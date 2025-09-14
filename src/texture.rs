use crate::color::Color;

#[derive(Debug, Clone)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Color>,
}

impl Texture {
    pub fn new(width: u32, height: u32) -> Self {
        Texture {
            width,
            height,
            data: vec![Color::new(255, 255, 255); (width * height) as usize],
        }
    }

    pub fn sample(&self, u: f32, v: f32) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);
        
        let x = ((u * (self.width as f32 - 1.0)) as u32).min(self.width - 1);
        let y = ((v * (self.height as f32 - 1.0)) as u32).min(self.height - 1);
        
        let index = (y * self.width + x) as usize;
        self.data[index]
    }

    // Create a checkerboard texture
    pub fn checkerboard(width: u32, height: u32, color1: Color, color2: Color) -> Self {
        let mut texture = Texture::new(width, height);
        let checker_size = 8;
        
        for y in 0..height {
            for x in 0..width {
                let checker_x = (x / checker_size) % 2;
                let checker_y = (y / checker_size) % 2;
                let color = if (checker_x + checker_y) % 2 == 0 {
                    color1
                } else {
                    color2
                };
                
                let index = (y * width + x) as usize;
                texture.data[index] = color;
            }
        }
        
        texture
    }

    // Create a brick texture
    pub fn brick(width: u32, height: u32) -> Self {
        let mut texture = Texture::new(width, height);
        
        let brick_color = Color::new(139, 69, 19);
        let mortar_color = Color::new(200, 200, 200);
        
        let brick_width = 16;
        let brick_height = 8;
        let mortar_width = 2;
        
        for y in 0..height {
            for x in 0..width {
                let row = y / (brick_height + mortar_width);
                let offset = if row % 2 == 0 { 0 } else { brick_width / 2 };
                
                let local_x = (x + offset) % (brick_width + mortar_width);
                let local_y = y % (brick_height + mortar_width);
                
                let color = if local_x < brick_width && local_y < brick_height {
                    // Add some variation to brick color
                    let variation = ((x * 7 + y * 11) % 20) as u8;
                    Color::new(
                        (brick_color.r as i32 + variation as i32 - 10).clamp(0, 255) as u8,
                        (brick_color.g as i32 + variation as i32 - 10).clamp(0, 255) as u8,
                        (brick_color.b as i32 + variation as i32 - 10).clamp(0, 255) as u8,
                    )
                } else {
                    mortar_color
                };
                
                let index = (y * width + x) as usize;
                texture.data[index] = color;
            }
        }
        
        texture
    }

    // Create a wood texture
    pub fn wood(width: u32, height: u32) -> Self {
        let mut texture = Texture::new(width, height);
        
        let base_color = Color::new(139, 115, 85);
        let ring_color = Color::new(101, 67, 33);
        
        for y in 0..height {
            for x in 0..width {
                let center_x = width as f32 / 2.0;
                let center_y = height as f32 / 2.0;
                
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;
                let distance = (dx * dx + dy * dy).sqrt();
                
                // Create wood rings
                let ring_pattern = ((distance / 4.0).sin() * 0.5 + 0.5) * 0.3 + 0.7;
                
                // Add some noise
                let noise = ((x * 7 + y * 11) % 100) as f32 / 100.0 * 0.1;
                
                let factor = (ring_pattern + noise).clamp(0.0, 1.0);
                
                let color = Color::new(
                    (base_color.r as f32 * factor + ring_color.r as f32 * (1.0 - factor)) as u8,
                    (base_color.g as f32 * factor + ring_color.g as f32 * (1.0 - factor)) as u8,
                    (base_color.b as f32 * factor + ring_color.b as f32 * (1.0 - factor)) as u8,
                );
                
                let index = (y * width + x) as usize;
                texture.data[index] = color;
            }
        }
        
        texture
    }

    // Create a marble texture
    pub fn marble(width: u32, height: u32) -> Self {
        let mut texture = Texture::new(width, height);
        
        let base_color = Color::new(240, 240, 255);
        let vein_color = Color::new(100, 100, 120);
        
        for y in 0..height {
            for x in 0..width {
                let u = x as f32 / width as f32;
                let v = y as f32 / height as f32;
                
                // Create marble veining pattern
                let vein1 = ((u * 10.0 + v * 3.0).sin() * 0.5 + 0.5);
                let vein2 = ((u * 7.0 - v * 5.0).sin() * 0.5 + 0.5);
                let noise = ((x * 13 + y * 17) % 100) as f32 / 100.0;
                
                let vein_intensity = ((vein1 * vein2 + noise * 0.3) * 2.0 - 1.0).abs();
                let factor = (1.0 - vein_intensity * 0.6).clamp(0.0, 1.0);
                
                let color = Color::new(
                    (base_color.r as f32 * factor + vein_color.r as f32 * (1.0 - factor)) as u8,
                    (base_color.g as f32 * factor + vein_color.g as f32 * (1.0 - factor)) as u8,
                    (base_color.b as f32 * factor + vein_color.b as f32 * (1.0 - factor)) as u8,
                );
                
                let index = (y * width + x) as usize;
                texture.data[index] = color;
            }
        }
        
        texture
    }

    // Create a metal texture
    pub fn metal(width: u32, height: u32) -> Self {
        let mut texture = Texture::new(width, height);
        
        let base_color = Color::new(180, 180, 200);
        
        for y in 0..height {
            for x in 0..width {
                // Create brushed metal effect
                let brush_pattern = ((y as f32 / 2.0).sin() * 0.1 + 0.9);
                let noise = ((x * 19 + y * 23) % 100) as f32 / 100.0 * 0.2 + 0.8;
                
                let factor = (brush_pattern * noise).clamp(0.0, 1.0);
                
                let color = Color::new(
                    (base_color.r as f32 * factor) as u8,
                    (base_color.g as f32 * factor) as u8,
                    (base_color.b as f32 * factor) as u8,
                );
                
                let index = (y * width + x) as usize;
                texture.data[index] = color;
            }
        }
        
        texture
    }

    // Create ancient stone texture for ruins
    pub fn ancient_stone(width: u32, height: u32) -> Self {
        let mut texture = Texture::new(width, height);
        
        let base_color = Color::new(120, 110, 95);
        let crack_color = Color::new(60, 55, 45);
        let moss_color = Color::new(40, 60, 30);
        
        for y in 0..height {
            for x in 0..width {
                let noise1 = ((x * 7 + y * 11) % 100) as f32 / 100.0;
                let noise2 = ((x * 13 + y * 17) % 100) as f32 / 100.0;
                
                // Create cracks and weathering
                let crack_factor = if noise1 > 0.85 { 0.3 } else { 1.0 };
                
                // Add moss patches
                let moss_factor = if noise2 > 0.9 { 0.7 } else { 1.0 };
                
                let color = if crack_factor < 1.0 {
                    crack_color
                } else if moss_factor < 1.0 {
                    Color::new(
                        ((base_color.r as f32 * 0.6) + (moss_color.r as f32 * 0.4)) as u8,
                        ((base_color.g as f32 * 0.6) + (moss_color.g as f32 * 0.4)) as u8,
                        ((base_color.b as f32 * 0.6) + (moss_color.b as f32 * 0.4)) as u8,
                    )
                } else {
                    base_color
                };
                
                let index = (y * width + x) as usize;
                texture.data[index] = color;
            }
        }
        
        texture
    }

    // Create rusted metal texture
    pub fn rusted_metal(width: u32, height: u32) -> Self {
        let mut texture = Texture::new(width, height);
        
        let metal_color = Color::new(140, 140, 150);
        let rust_color = Color::new(139, 69, 19);
        let dark_rust = Color::new(101, 45, 12);
        
        for y in 0..height {
            for x in 0..width {
                let noise1 = ((x * 9 + y * 13) % 100) as f32 / 100.0;
                let noise2 = ((x * 15 + y * 7) % 100) as f32 / 100.0;
                
                let rust_intensity = (noise1 + noise2 * 0.5) / 1.5;
                
                let color = if rust_intensity > 0.7 {
                    rust_color
                } else if rust_intensity > 0.4 {
                    Color::new(
                        ((metal_color.r as f32 * (1.0 - rust_intensity)) + (rust_color.r as f32 * rust_intensity)) as u8,
                        ((metal_color.g as f32 * (1.0 - rust_intensity)) + (rust_color.g as f32 * rust_intensity)) as u8,
                        ((metal_color.b as f32 * (1.0 - rust_intensity)) + (rust_color.b as f32 * rust_intensity)) as u8,
                    )
                } else if rust_intensity < 0.1 {
                    dark_rust
                } else {
                    metal_color
                };
                
                let index = (y * width + x) as usize;
                texture.data[index] = color;
            }
        }
        
        texture
    }

    // Create blood/water texture with ripple effect
    pub fn blood_water(width: u32, height: u32) -> Self {
        let mut texture = Texture::new(width, height);
        
        let blood_color = Color::new(120, 20, 20);
        let dark_blood = Color::new(80, 10, 10);
        
        for y in 0..height {
            for x in 0..width {
                let center_x = width as f32 / 2.0;
                let center_y = height as f32 / 2.0;
                
                let distance = ((x as f32 - center_x).powi(2) + (y as f32 - center_y).powi(2)).sqrt();
                let ripple = ((distance * 0.3).sin() * 0.3 + 0.7).clamp(0.0, 1.0);
                
                let color = Color::new(
                    ((blood_color.r as f32 * ripple) + (dark_blood.r as f32 * (1.0 - ripple))) as u8,
                    ((blood_color.g as f32 * ripple) + (dark_blood.g as f32 * (1.0 - ripple))) as u8,
                    ((blood_color.b as f32 * ripple) + (dark_blood.b as f32 * (1.0 - ripple))) as u8,
                );
                
                let index = (y * width + x) as usize;
                texture.data[index] = color;
            }
        }
        
        texture
    }

    // Create dark crystal texture
    pub fn dark_crystal(width: u32, height: u32) -> Self {
        let mut texture = Texture::new(width, height);
        
        let crystal_color = Color::new(40, 20, 60);
        let highlight_color = Color::new(120, 80, 140);
        let shadow_color = Color::new(20, 10, 30);
        
        for y in 0..height {
            for x in 0..width {
                let facet_x = (x / 8) * 8;
                let facet_y = (y / 8) * 8;
                
                let distance_to_facet = ((x as f32 - facet_x as f32).powi(2) + (y as f32 - facet_y as f32).powi(2)).sqrt();
                
                let color = if distance_to_facet < 2.0 {
                    highlight_color
                } else if distance_to_facet > 6.0 {
                    shadow_color
                } else {
                    crystal_color
                };
                
                let index = (y * width + x) as usize;
                texture.data[index] = color;
            }
        }
        
        texture
    }

    // Create charred wood texture
    pub fn charred_wood(width: u32, height: u32) -> Self {
        let mut texture = Texture::new(width, height);
        
        let char_color = Color::new(30, 25, 20);
        let burnt_color = Color::new(60, 45, 30);
        let ash_color = Color::new(80, 75, 70);
        
        for y in 0..height {
            for x in 0..width {
                // Create wood grain pattern
                let grain = ((y as f32 / 4.0).sin() * 0.5 + 0.5);
                let noise = ((x * 17 + y * 23) % 100) as f32 / 100.0;
                let burn_intensity = grain * noise;
                
                let color = if burn_intensity > 0.8 {
                    ash_color
                } else if burn_intensity > 0.4 {
                    burnt_color
                } else {
                    char_color
                };
                
                let index = (y * width + x) as usize;
                texture.data[index] = color;
            }
        }
        
        texture
    }
}