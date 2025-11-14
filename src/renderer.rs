use minifb::{Window, WindowOptions};
use cgmath::Vector3;
use crate::planet::Planet;

pub struct Renderer {
    pub window: Window,
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    // Cámara en el plano eclíptico (X,Z)
    pub camera_pos: Vector3<f32>,
    pub zoom: f32,
}

impl Renderer {
    pub fn new() -> Renderer {
        let width = 800;
        let height = 600;

        let window = Window::new(
            "Sistema Solar - Proyecto Rust",
            width,
            height,
            WindowOptions {
                resize: true,
                ..WindowOptions::default()
            },
        )
        .unwrap_or_else(|e| {
            panic!("Error al crear la ventana: {}", e);
        });

        let buffer = vec![0; width * height];

        Renderer {
            window,
            width,
            height,
            buffer,
            camera_pos: Vector3::new(0.0, 0.0, 0.0),
            zoom: 1.0,
        }
    }

    fn clear(&mut self) {
        self.buffer.fill(0x000000); // negro
    }

    fn put_pixel(&mut self, x: i32, y: i32, color: u32) {
        if x < 0 || y < 0 {
            return;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            return;
        }
        self.buffer[y * self.width + x] = color;
    }

    // Pequeño helper para aplicar intensidad de luz al color base
    fn shade_color(base: u32, intensity: f32) -> u32 {
        let i = intensity.clamp(0.0, 1.0);
        let r = ((base >> 16) & 0xFF) as f32 * i;
        let g = ((base >> 8) & 0xFF) as f32 * i;
        let b = (base & 0xFF) as f32 * i;

        ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }

    /// Dibuja un planeta con sombreado según la luz del sol.
    fn draw_planet(
        &mut self,
        screen_pos: (f32, f32),
        planet: &Planet,
        sun_screen: (f32, f32),
        radius: f32,
        rotation_phase: f32,
    ) {
        let (cx, cy) = screen_pos;
        let r = radius;

        let min_x = (cx - r).floor() as i32;
        let max_x = (cx + r).ceil() as i32;
        let min_y = (cy - r).floor() as i32;
        let max_y = (cy + r).ceil() as i32;

        // vector de luz desde el centro del planeta hacia el sol
        let mut lx = sun_screen.0 - cx;
        let mut ly = sun_screen.1 - cy;
        let len = (lx * lx + ly * ly).sqrt().max(0.0001);
        lx /= len;
        ly /= len;
        let lz = 0.4; // un poco de componente "hacia afuera" para que se vea bonito

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let dist2 = dx * dx + dy * dy;
                if dist2 > r * r {
                    continue;
                }

                // normal aproximada del punto en la esfera
                let nx = dx / r;
                let ny = dy / r;
                let nz_sq = 1.0 - nx * nx - ny * ny;
                if nz_sq <= 0.0 {
                    continue;
                }
                let nz = nz_sq.sqrt();

                // --- Patrón que depende del tipo de planeta y rota con él ---
                let pattern = if planet.is_sun {
                    1.0
                } else if planet.radius >= 16.0 {
                    // Gas giants (Júpiter, Saturno): bandas más suaves horizontales
                    let lat = ny; // -1..1
                    let long = (dy).atan2(dx) + rotation_phase;
                    let v = (lat * 6.0 + long * 2.0).sin() * 0.5 + 0.5; // 0..1
                    0.6 + 0.4 * v
                } else {
                    // Planetas rocosos: manchas irregulares
                    let v = ((nx * 8.0 + rotation_phase).sin() * (nz * 8.0).cos()).abs(); // 0..1
                    0.7 + 0.3 * v
                };

                // producto punto con la dirección de la luz
                let dot = (nx * lx + ny * ly + nz * lz).max(0.0);

                // un poco de luz ambiente para que el lado oscuro no sea negro total
                let ambient = 0.25;
                let base_intensity = ambient + (1.0 - ambient) * dot;

                let color = if planet.is_sun {
                    // el sol emite luz propia, lo dejamos brillante
                    planet.color
                } else {
                    // combinamos luz + patrón que rota
                    let final_intensity = (base_intensity * pattern).clamp(0.0, 1.0);
                    Renderer::shade_color(planet.color, final_intensity)
                };

                self.put_pixel(x, y, color);
            }
        }
    }

    fn draw_ring(&mut self, center: (f32, f32), inner_r: f32, outer_r: f32, color: u32) {
        let (cx, cy) = center;
        let inner2 = inner_r * inner_r;
        let outer2 = outer_r * outer_r;

        let min_x = (cx - outer_r).floor() as i32;
        let max_x = (cx + outer_r).ceil() as i32;
        let min_y = (cy - outer_r).floor() as i32;
        let max_y = (cy + outer_r).ceil() as i32;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let d2 = dx * dx + dy * dy;
                if d2 < inner2 || d2 > outer2 {
                    continue;
                }
                self.put_pixel(x, y, color);
            }
        }
    }

    pub fn render_scene(&mut self, planets: &Vec<Planet>, time: f32) {
        self.clear();

        let center_x = (self.width as f32) * 0.5;
        let center_y = (self.height as f32) * 0.5;

        // El sol lo seguimos dibujando en el centro de pantalla como referencia de luz
        let sun_screen = (center_x, center_y);

        for planet in planets {
            let pos = planet.orbit_position(time);

            // Posición relativa al centro de la cámara en el plano XZ
            let rel = Vector3::new(
                pos.x - self.camera_pos.x,
                pos.y - self.camera_pos.y,
                pos.z - self.camera_pos.z,
            );

            // Proyección al plano de pantalla usando zoom clásico:
            // todo se aleja/acerca del centro de pantalla
            let screen_x = center_x + rel.x * self.zoom;
            let screen_y = center_y + rel.z * self.zoom;

            // El radio también escala con el zoom
            let scaled_radius = planet.radius * self.zoom;

            // Fase de rotación del planeta sobre su eje
            let rotation_phase = planet.rotation_speed * time;

            self.draw_planet((screen_x, screen_y), planet, sun_screen, scaled_radius, rotation_phase);

            // Anillos para Saturno
            if planet.name == "Saturno" {
                // anillo más delgado
                let inner = scaled_radius * 1.6;
                let outer = scaled_radius * 1.9;
                let ring_color = 0xC8B090;
                self.draw_ring((screen_x, screen_y), inner, outer, ring_color);
            }
        }

        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }
}