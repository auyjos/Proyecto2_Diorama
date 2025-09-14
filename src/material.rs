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

    // Eclipse-themed materials for the Berserk diorama
    
    // Concrete base for modern brutalist foundation
    pub fn concrete() -> Self {
        Material::new(
            Color::new(160, 160, 155),
            8.0,
            [0.85, 0.1, 0.03, 0.0], // Very diffuse, minimal reflection
            1.0,
            0.0,
        )
    }

    // Polished metal for structural elements (performance optimized)
    pub fn polished_metal() -> Self {
        Material::new(
            Color::new(180, 180, 190),
            60.0,
            [0.4, 0.3, 0.2, 0.0], // Reduced reflection for better performance
            1.0,
            0.0,
        )
    }

    // Rusted metal for weathered elements
    pub fn rusted_metal() -> Self {
        Material::new(
            Color::new(139, 69, 19),
            25.0,
            [0.7, 0.2, 0.15, 0.0], // Reduced reflection for better performance
            1.0,
            0.0,
        )
    }

    // Dark water for pools
    pub fn dark_water() -> Self {
        Material::new(
            Color::new(40, 60, 80),
            50.0,
            [0.3, 0.2, 0.15, 0.5], // Subtle reflection and refraction
            1.33, // Water refractive index
            0.7,
        )
    }

    // Red crystal for mystical accent
    pub fn red_crystal() -> Self {
        Material::new(
            Color::new(120, 20, 30),
            200.0,
            [0.1, 0.1, 0.1, 0.8], // High refraction for glow effect
            1.6,
            0.85,
        )
    }

    // Legacy materials for compatibility
    pub fn ancient_stone() -> Self {
        Self::concrete() // Redirect to concrete for coherence
    }

    pub fn blood_water() -> Self {
        Self::dark_water() // Redirect to dark water
    }

    pub fn dark_crystal() -> Self {
        Self::red_crystal() // Redirect to red crystal
    }

    pub fn charred_wood() -> Self {
        Self::rusted_metal() // Redirect to rusted metal for consistency
    }
}