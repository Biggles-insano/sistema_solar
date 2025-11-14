extern crate minifb;

use minifb::Key;
use crate::planet::Planet;
use crate::renderer::Renderer;

mod planet;
mod renderer;

fn main() {
    // Crear los planetas (sol + 6 planetas) con colores
    let sun     = Planet::new("Sol",      30.0,    0.0,    0.0,   0.0,   0xFFCC33, true);
    let mercury = Planet::new("Mercurio",  6.0,   150.0,   0.05,  0.10,  0xAAAAAA, false);
    let venus   = Planet::new("Venus",    10.0,   260.0,   0.035, 0.09, 0xFFCC88, false);
    let earth   = Planet::new("Tierra",   11.0,   380.0,   0.03,  0.12, 0x3366FF, false);
    let mars    = Planet::new("Marte",     9.0,   500.0,   0.026, 0.11, 0xCC5533, false);
    let jupiter = Planet::new("Júpiter",  18.0,   650.0,   0.018, 0.20, 0xDDBB88, false);
    let saturn  = Planet::new("Saturno",  16.0,   820.0,   0.014, 0.18, 0xEEDD99, false);

    let planets = vec![sun, mercury, venus, earth, mars, jupiter, saturn];

    let mut renderer = Renderer::new();
    let mut time: f32 = 0.0;

    // Velocidades de cámara y zoom
    let camera_speed: f32 = 4.0;
    let zoom_step: f32 = 0.03;

    // Mostrar controles en pantalla
    println!("Controles:");
    println!("W/S: Mover la cámara hacia adelante/atrás");
    println!("A/D: Mover la cámara a la izquierda/derecha");
    println!("Q/E: Zoom");
    println!("T: Teletransportarse a otro planeta");
    println!("Esc: Salir");

    // Bucle principal de la ventana
    while renderer.window.is_open() && !renderer.window.is_key_down(Key::Escape) {
        // Movimiento acelerado de la cámara
        thread_local! {
            static CAM_VEL_X: std::cell::RefCell<f32> = std::cell::RefCell::new(0.0);
            static CAM_VEL_Z: std::cell::RefCell<f32> = std::cell::RefCell::new(0.0);
        }

        let accel: f32 = 0.5;
        let friction: f32 = 0.9;

        CAM_VEL_X.with(|vx| {
            let mut vx = vx.borrow_mut();
            if renderer.window.is_key_down(Key::A) {
                *vx -= accel;
            }
            if renderer.window.is_key_down(Key::D) {
                *vx += accel;
            }
            *vx *= friction; // fricción
            renderer.camera_pos.x += *vx;
        });

        CAM_VEL_Z.with(|vz| {
            let mut vz = vz.borrow_mut();
            if renderer.window.is_key_down(Key::W) {
                *vz -= accel;
            }
            if renderer.window.is_key_down(Key::S) {
                *vz += accel;
            }
            *vz *= friction; // fricción
            renderer.camera_pos.z += *vz;
        });

        // Zoom con Q/E
        if renderer.window.is_key_down(Key::Q) {
            renderer.zoom += zoom_step;
        }
        if renderer.window.is_key_down(Key::E) {
            renderer.zoom -= zoom_step;
        }
        // Limitar el zoom a un rango razonable
        if renderer.zoom < 0.3 {
            renderer.zoom = 0.3;
        }
        if renderer.zoom > 3.0 {
            renderer.zoom = 3.0;
        }

        // Avanzar el tiempo para las órbitas/rotaciones
        time += 0.01;

        // Renderizar la escena
        renderer.render_scene(&planets, time);
    }
}