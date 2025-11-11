use cgmath::{Matrix4, Vector3};
use std::f32::consts::PI;

pub struct Planet {
    pub name: String,
    pub radius: f32,
    pub distance_from_sun: f32,
    pub orbit_speed: f32,
    pub rotation_speed: f32,
}

impl Planet {
    // Crear un nuevo planeta
    pub fn new(name: &str, radius: f32, distance_from_sun: f32, orbit_speed: f32, rotation_speed: f32) -> Planet {
        Planet {
            name: name.to_string(),
            radius,
            distance_from_sun,
            orbit_speed,
            rotation_speed,
        }
    }

    // Calcular la posición del planeta en su órbita (simplificada como una órbita circular)
    pub fn orbit_position(&self, time: f32) -> Vector3<f32> {
        let angle = time * self.orbit_speed; // velocidad de órbita multiplicada por el tiempo
        let x = self.distance_from_sun * angle.cos();
        let z = self.distance_from_sun * angle.sin();
        Vector3::new(x, 0.0, z) // Regresar la posición en el plano XZ
    }

    // Calcular la rotación del planeta sobre su eje
    pub fn rotation_matrix(&self, time: f32) -> Matrix4<f32> {
        let angle = time * self.rotation_speed;
        Matrix4::from_angle_y(cgmath::Rad(angle)) // Rotación alrededor del eje Y
    }
}