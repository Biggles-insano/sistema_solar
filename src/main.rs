extern crate minifb;
use minifb::{Window, WindowOptions};
use cgmath::{Matrix4, Vector3};
use std::f32::consts::PI;
use crate::planet::Planet;
use crate::renderer::Renderer;

mod planet;
mod renderer;

fn main() {
    // Crear los planetas (sol + 6 planetas) con colores
    let sun     = Planet::new("Sol",      30.0,   0.0,   0.0,   0.0,   0xFFCC33, true);
    let mercury = Planet::new("Mercurio",  6.0,  60.0,  0.05,  0.10,  0xAAAAAA, false);
    let venus   = Planet::new("Venus",    10.0,  90.0,  0.035, 0.09,  0xFFCC88, false);
    let earth   = Planet::new("Tierra",   11.0, 120.0,  0.03,  0.12,  0x3366FF, false);
    let mars    = Planet::new("Marte",     9.0, 150.0,  0.026, 0.11,  0xCC5533, false);
    let jupiter = Planet::new("Júpiter",  18.0, 190.0,  0.018, 0.20,  0xDDBB88, false);
    let saturn  = Planet::new("Saturno",  16.0, 230.0,  0.014, 0.18,  0xEEDD99, false);

    let planets = vec![sun, mercury, venus, earth, mars, jupiter, saturn];

    let mut renderer = Renderer::new();
    let mut time: f32 = 0.0;

    while renderer.window.is_open() && !renderer.window.is_key_down(minifb::Key::Escape) {
        time += 0.01;
        renderer.render_scene(&planets, time);
    }
}