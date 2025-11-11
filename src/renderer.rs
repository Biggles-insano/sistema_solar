use minifb::{Window, WindowOptions};
use cgmath::{Matrix4, Vector3};
use crate::planet::Planet;

pub struct Renderer {
    pub window: Window,
}

impl Renderer {
    // Crear una nueva ventana
    pub fn new() -> Renderer {
        let window = Window::new(
            "Sistema Solar - Proyecto Rust",
            800,
            600,
            WindowOptions {
                resize: true,
                ..WindowOptions::default()
            },
        )
        .unwrap_or_else(|e| {
            panic!("Error al crear la ventana: {}", e);
        });

        Renderer { window }
    }

    // Dibujar un círculo que representa un planeta
    pub fn draw_planet(&mut self, position: Vector3<f32>, radius: f32) {
        let mut buffer: Vec<u32> = vec![0; 800 * 600]; // Crear un buffer de píxeles

        let cx = 400; // Centro de la ventana en el eje X
        let cy = 300; // Centro de la ventana en el eje Y

        // Dibujar un círculo
        for y in 0..600 {
            for x in 0..800 {
                let dx = x as f32 - cx as f32;
                let dy = y as f32 - cy as f32;
                if (dx * dx + dy * dy).sqrt() < radius {
                    buffer[y * 800 + x] = 0xFFFFFF; // Color blanco para el planeta
                }
            }
        }

        // Actualizar la ventana con el nuevo buffer
        self.window
            .update_with_buffer(&buffer, 800, 600)
            .unwrap();
    }

    // Renderizar un planeta
    pub fn render_planet(&mut self, planet: &Planet, time: f32) {
        let position = planet.orbit_position(time);
        let rotation_matrix = planet.rotation_matrix(time);

        // Convertir las coordenadas 3D a 2D (solo X y Z)
        let x = position.x + 400.0; // Centrar en la ventana
        let y = position.z + 300.0; // Centrar en la ventana

        // Dibujar el planeta como un círculo
        self.draw_planet(Vector3::new(x, y, 0.0), planet.radius);
    }

    // Hacer mutable la referencia de self para modificar la ventana
    // Método actualizado para tomar una referencia mutable de self
    pub fn render_scene(&mut self, planets: &Vec<Planet>, time: f32) {
        // Limpiar la ventana antes de renderizar
        self.window.update();

        // Renderizar cada planeta
        for planet in planets {
            self.render_planet(planet, time);
        }
    }
}