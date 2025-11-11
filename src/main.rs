extern crate minifb;
use minifb::{Window, WindowOptions};
use cgmath::{Matrix4, Vector3};
use std::f32::consts::PI;
use crate::planet::Planet;
use crate::renderer::Renderer;

mod planet;
mod renderer;

fn main() {
    // Crear los planetas
    let earth = Planet::new("Tierra", 1.0, 5.0, 0.01, 0.1);
    let mars = Planet::new("Marte", 0.5, 8.0, 0.008, 0.12);

    let planets = vec![earth, mars];

    // Crear el renderizador
    let mut renderer = Renderer::new();

    let mut time: f32 = 0.0;

    // Bucle principal de la ventana
    while renderer.window.is_open() && !renderer.window.is_key_down(minifb::Key::Escape) {
        // Incrementar el tiempo
        time += 0.01;

        // Renderizar la escena
        renderer.render_scene(&planets, time);
    }
}