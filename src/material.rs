use crate::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32; 4], // [diffuse, specular, reflective, refractive]
    pub refractive_index: f32,
    pub transparency: f32, // 0.0 = opaque, 1.0 = fully transparent
}

impl Material {
    pub fn new(diffuse: Color, specular: f32, albedo: [f32; 4], refractive_index: f32, transparency: f32) -> Self {
        Material {
            diffuse,
            specular,
            albedo,
            refractive_index,
            transparency,
        }
    }

    // Material presets
    pub fn rubber() -> Self {
        Material::new(
            Color::new(80, 0, 0),
            1.0,
            [0.9, 0.1, 0.0, 0.0], // No reflection, no refraction
            1.0,
            0.0,
        )
    }

    pub fn ivory() -> Self {
        Material::new(
            Color::new(100, 100, 80),
            50.0,
            [0.6, 0.3, 0.1, 0.0], // Slight reflection
            1.0,
            0.0,
        )
    }

    pub fn mirror() -> Self {
        Material::new(
            Color::new(240, 240, 240), // Más brillante
            1425.0,
            [0.0, 5.0, 0.9, 0.0], // Muy reflectante
            1.0,
            0.0,
        )
    }

    pub fn glass() -> Self {
        Material::new(
            Color::new(200, 220, 240), // Más visible
            125.0,
            [0.0, 0.2, 0.05, 0.9], // Menos reflexión, más refracción
            1.5, // Glass refractive index
            0.9, // Más transparente
        )
    }

    pub fn water() -> Self {
        Material::new(
            Color::new(100, 150, 200),
            80.0,
            [0.1, 0.3, 0.2, 0.6],
            1.33, // Water refractive index
            0.7,
        )
    }
}