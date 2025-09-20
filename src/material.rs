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
    
        // === ZEN GARDEN MATERIALS (5 UNIQUE MATERIALS FOR MAXIMUM POINTS) ===
    
    // 1. CRYSTAL GLASS - High refraction for energy crystals [REFRACTION MATERIAL]
    pub fn crystal_glass() -> Self {
        Material::new(
            Color::new(220, 240, 255), // Clear blue-white
            300.0,
            [0.1, 0.1, 0.2, 0.8], // High refraction
            1.8, // High refractive index like diamond
            0.95, // Very transparent
        )
    }

    // 2. CHROME MIRROR - Perfect reflection for tech panels [REFLECTION MATERIAL]
    pub fn chrome_mirror() -> Self {
        Material::new(
            Color::new(250, 250, 250), // Bright chrome
            1000.0,
            [0.05, 0.1, 0.85, 0.0], // High reflection
            1.0,
            0.0, // Opaque
        )
    }

    // 3. ZEN WATER - Transparent water with subtle reflection
    pub fn zen_water() -> Self {
        Material::new(
            Color::new(60, 120, 140), // Calm blue-green
            80.0,
            [0.2, 0.2, 0.3, 0.6], // Balanced reflection/refraction
            1.33, // Water refractive index
            0.8, // Mostly transparent
        )
    }

    // 4. ZEN MOSS - Natural organic vegetation with soft appearance
    pub fn zen_moss() -> Self {
        Material::new(
            Color::new(85, 120, 70), // Natural moss green
            30.0,
            [0.7, 0.2, 0.1, 0.0], // Mostly diffuse, very natural
            1.0,
            0.0, // Opaque
        )
    }

    // 5. BRUSHED METAL - Technological elements with directional reflection
    pub fn brushed_metal() -> Self {
        Material::new(
            Color::new(170, 180, 190), // Cool metal tone
            120.0,
            [0.5, 0.3, 0.2, 0.0], // Moderate reflection
            1.0,
            0.0, // Opaque
        )
    }

    // 6. CONCRETE BASE - Solid foundation material
    pub fn concrete_base() -> Self {
        Material::new(
            Color::new(180, 180, 175), // Neutral concrete gray
            25.0,
            [0.8, 0.1, 0.1, 0.0], // Mostly diffuse, minimal reflection
            1.0,
            0.0, // Opaque
        )
    }

    // === LEGACY MATERIALS (for backward compatibility) ===
    
    pub fn concrete() -> Self {
        Self::concrete_base() // Use concrete base
    }

    pub fn polished_metal() -> Self {
        Self::brushed_metal() // Use brushed metal
    }

    pub fn rusted_metal() -> Self {
        Self::brushed_metal() // Use brushed metal for consistency
    }

    pub fn dark_water() -> Self {
        Self::zen_water() // Use zen water
    }

    pub fn red_crystal() -> Self {
        Self::crystal_glass() // Use crystal glass
    }

    // More legacy redirects
    pub fn ancient_stone() -> Self {
        Self::zen_moss()
    }

    pub fn blood_water() -> Self {
        Self::zen_water()
    }

    pub fn dark_crystal() -> Self {
        Self::crystal_glass()
    }

    pub fn charred_wood() -> Self {
        Self::brushed_metal()
    }
}