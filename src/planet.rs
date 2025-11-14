use cgmath::{Vector3, Rad};

pub struct Planet {
    pub name: String,
    pub radius: f32,
    pub distance_from_sun: f32,
    pub orbit_speed: f32,
    pub rotation_speed: f32,
    pub color: u32,   // color base del planeta (0xRRGGBB)
    pub is_sun: bool, // true si es el sol
}

impl Planet {
    pub fn new(
        name: &str,
        radius: f32,
        distance_from_sun: f32,
        orbit_speed: f32,
        rotation_speed: f32,
        color: u32,
        is_sun: bool,
    ) -> Planet {
        Planet {
            name: name.to_string(),
            radius,
            distance_from_sun,
            orbit_speed,
            rotation_speed,
            color,
            is_sun,
        }
    }

    // órbita circular en el plano XZ
    pub fn orbit_position(&self, time: f32) -> Vector3<f32> {
        if self.is_sun {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        let angle = time * self.orbit_speed;
        let x = self.distance_from_sun * angle.cos();
        let z = self.distance_from_sun * angle.sin();
        Vector3::new(x, 0.0, z)
    }

    pub fn rotation_angle(&self, time: f32) -> f32 {
        time * self.rotation_speed
    }
}