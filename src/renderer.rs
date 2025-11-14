use minifb::{Window, WindowOptions};
use cgmath::Vector3;
use crate::planet::Planet;

pub struct Renderer {
    pub window: Window,
    width: usize,
    height: usize,
    buffer: Vec<u32>,
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
    ) {
        let (cx, cy) = screen_pos;
        let r = planet.radius;

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

                // producto punto con la dirección de la luz
                let dot = (nx * lx + ny * ly + nz * lz).max(0.0);

                // un poco de luz ambiente para que el lado oscuro no sea negro total
                let ambient = 0.25;
                let intensity = ambient + (1.0 - ambient) * dot;

                let color = if planet.is_sun {
                    // el sol emite luz propia, lo dejamos brillante
                    planet.color
                } else {
                    Renderer::shade_color(planet.color, intensity)
                };

                self.put_pixel(x, y, color);
            }
        }
    }

    pub fn render_scene(&mut self, planets: &Vec<Planet>, time: f32) {
        self.clear();

        // centro de la pantalla = posición del sol
        let center_x = (self.width as f32) * 0.5;
        let center_y = (self.height as f32) * 0.5;
        let sun_screen = (center_x, center_y);

        for planet in planets {
            let pos = planet.orbit_position(time);
            // proyectamos el plano XZ a pantalla
            let screen_x = center_x + pos.x;
            let screen_y = center_y + pos.z;

            self.draw_planet((screen_x, screen_y), planet, sun_screen);
        }

        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }
}