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
                let vein1 = (u * 10.0 + v * 3.0).sin() * 0.5 + 0.5;
                let vein2 = (u * 7.0 - v * 5.0).sin() * 0.5 + 0.5;
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
                let brush_pattern = (y as f32 / 2.0).sin() * 0.1 + 0.9;
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
                let grain = (y as f32 / 4.0).sin() * 0.5 + 0.5;
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

    // === ZEN GARDEN TEXTURES (5 UNIQUE TEXTURES FOR MATERIALS) ===

    // 1. Crystal Glass - Prismatic crystal patterns
    pub fn crystal_glass(width: u32, height: u32) -> Self {
        let mut texture = Texture::new(width, height);
        
        let base_color = Color::new(220, 240, 255);
        let refract_color = Color::new(180, 220, 255);
        let highlight_color = Color::new(255, 255, 255);
        
        for y in 0..height {
            for x in 0..width {
                let u = x as f32 / width as f32;
                let v = y as f32 / height as f32;
                
                // Create prismatic crystal facets
                let facet_x = ((u * 6.0).floor() + (v * 4.0).floor()) % 2.0;
                let crystal_pattern = ((u * 12.0).sin() * (v * 8.0).cos()).abs();
                
                let color = if crystal_pattern > 0.8 {
                    highlight_color
                } else if facet_x > 0.5 {
                    refract_color
                } else {
                    base_color
                };
                
                let index = (y * width + x) as usize;
                texture.data[index] = color;
            }
        }
        
        texture
    }

    // 2. Chrome Mirror - Polished reflective surface
    pub fn chrome_mirror(width: u32, height: u32) -> Self {
        let mut texture = Texture::new(width, height);
        
        let base_color = Color::new(250, 250, 250);
        let highlight_color = Color::new(255, 255, 255);
        let shadow_color = Color::new(220, 220, 220);
        
        for y in 0..height {
            for x in 0..width {
                // Create polished metal surface with subtle gradients
                let gradient = ((x + y) as f32 / (width + height) as f32);
                let noise = ((x * 3 + y * 7) % 10) as f32 / 10.0 * 0.1;
                
                let intensity = (gradient + noise).clamp(0.0, 1.0);
                
                let color = if intensity > 0.7 {
                    highlight_color
                } else if intensity < 0.3 {
                    shadow_color
                } else {
                    base_color
                };
                
                let index = (y * width + x) as usize;
                texture.data[index] = color;
            }
        }
        
        texture
    }

    // 3. Zen Water - Calm water with gentle ripples
    pub fn zen_water(width: u32, height: u32) -> Self {
        let mut texture = Texture::new(width, height);
        
        let deep_water = Color::new(40, 80, 120);
        let surface_water = Color::new(80, 140, 180);
        let reflection = Color::new(120, 180, 220);
        
        for y in 0..height {
            for x in 0..width {
                let center_x = width as f32 / 2.0;
                let center_y = height as f32 / 2.0;
                
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;
                let distance = (dx * dx + dy * dy).sqrt();
                
                // Create concentric ripples
                let ripple1 = ((distance * 0.2).sin() * 0.3 + 0.7).clamp(0.0, 1.0);
                let ripple2 = ((distance * 0.4 + 1.0).sin() * 0.2 + 0.8).clamp(0.0, 1.0);
                
                let intensity = ripple1 * ripple2;
                
                let color = if intensity > 0.8 {
                    reflection
                } else if intensity > 0.5 {
                    surface_water
                } else {
                    deep_water
                };
                
                let index = (y * width + x) as usize;
                texture.data[index] = color;
            }
        }
        
        texture
    }

    // 4. Marble Stone - Elegant veined marble
    pub fn marble_stone(width: u32, height: u32) -> Self {
        let mut texture = Texture::new(width, height);
        
        let marble_white = Color::new(240, 235, 220);
        let vein_gray = Color::new(180, 175, 160);
        let dark_vein = Color::new(120, 115, 100);
        
        for y in 0..height {
            for x in 0..width {
                let u = x as f32 / width as f32;
                let v = y as f32 / height as f32;
                
                // Create natural marble veining
                let vein1 = ((u * 8.0 + v * 2.0).sin() * 0.5 + 0.5);
                let vein2 = ((u * 3.0 - v * 6.0).sin() * 0.3 + 0.7);
                let vein3 = ((u * 12.0 + v * 4.0).cos() * 0.2 + 0.8);
                
                let vein_intensity = vein1 * vein2 * vein3;
                
                let color = if vein_intensity < 0.3 {
                    dark_vein
                } else if vein_intensity < 0.6 {
                    vein_gray
                } else {
                    marble_white
                };
                
                let index = (y * width + x) as usize;
                texture.data[index] = color;
            }
        }
        
        texture
    }

    // 5. Brushed Metal - Technological directional metal
    pub fn brushed_metal(width: u32, height: u32) -> Self {
        let mut texture = Texture::new(width, height);
        
        let base_metal = Color::new(170, 180, 190);
        let bright_metal = Color::new(200, 210, 220);
        let dark_metal = Color::new(140, 150, 160);
        
        for y in 0..height {
            for x in 0..width {
                // Create horizontal brushed pattern
                let brush_pattern = ((y as f32 / 3.0).sin() * 0.4 + 0.6).clamp(0.0, 1.0);
                let vertical_noise = ((x * 11 + y * 13) % 20) as f32 / 20.0 * 0.3;
                
                let intensity = (brush_pattern + vertical_noise).clamp(0.0, 1.0);
                
                let color = if intensity > 0.7 {
                    bright_metal
                } else if intensity < 0.4 {
                    dark_metal
                } else {
                    base_metal
                };
                
                let index = (y * width + x) as usize;
                texture.data[index] = color;
            }
        }
        
        texture
    }

    // 6. Zen Moss - Natural organic vegetation texture
    pub fn zen_moss(width: u32, height: u32) -> Self {
        let mut texture = Texture::new(width, height);
        
        let base_moss = Color::new(85, 120, 70);    // Base moss green
        let light_moss = Color::new(105, 140, 90);  // Lighter moss areas
        let dark_moss = Color::new(65, 100, 50);    // Darker moss patches
        let earth_brown = Color::new(90, 70, 50);   // Earth between moss
        
        for y in 0..height {
            for x in 0..width {
                let u = x as f32 / width as f32;
                let v = y as f32 / height as f32;
                
                // Create organic moss pattern with multiple noise layers
                let moss_pattern1 = ((u * 15.0).sin() * (v * 12.0).cos() * 0.5 + 0.5);
                let moss_pattern2 = ((u * 8.0 + v * 6.0).sin() * 0.3 + 0.7);
                let moss_pattern3 = ((u * 25.0).cos() * (v * 20.0).sin() * 0.2 + 0.8);
                
                // Combine patterns for natural variation
                let moss_density = (moss_pattern1 * moss_pattern2 * moss_pattern3).clamp(0.0, 1.0);
                
                // Add small scale texture details
                let detail_noise = ((u * 50.0 + v * 45.0).sin() * (u * 35.0 - v * 40.0).cos() * 0.1 + 0.9);
                let final_density = (moss_density * detail_noise).clamp(0.0, 1.0);
                
                let color = if final_density > 0.8 {
                    light_moss  // Thick healthy moss
                } else if final_density > 0.5 {
                    base_moss   // Normal moss coverage
                } else if final_density > 0.25 {
                    dark_moss   // Sparse moss
                } else {
                    earth_brown // Exposed earth
                };
                
                let index = (y * width + x) as usize;
                texture.data[index] = color;
            }
        }
        
        texture
    }

    // 7. Concrete Base - Solid foundation texture
    pub fn concrete_base(width: u32, height: u32) -> Self {
        let mut texture = Texture::new(width, height);
        
        let base_concrete = Color::new(180, 180, 175);  // Base concrete gray
        let light_concrete = Color::new(200, 200, 195); // Lighter patches
        let dark_concrete = Color::new(160, 160, 155);  // Darker areas
        let speckle = Color::new(140, 140, 135);        // Small speckles
        
        for y in 0..height {
            for x in 0..width {
                let u = x as f32 / width as f32;
                let v = y as f32 / height as f32;
                
                // Create subtle concrete texture with minimal variation
                let noise1 = ((u * 20.0).sin() * (v * 18.0).cos() * 0.15 + 0.85);
                let noise2 = ((u * 35.0 + v * 25.0).sin() * 0.1 + 0.9);
                let speckle_pattern = ((u * 80.0).sin() * (v * 90.0).cos()).abs();
                
                let intensity = (noise1 * noise2).clamp(0.0, 1.0);
                
                let color = if speckle_pattern > 0.95 {
                    speckle      // Small dark speckles
                } else if intensity > 0.92 {
                    light_concrete  // Light patches
                } else if intensity < 0.88 {
                    dark_concrete   // Dark areas
                } else {
                    base_concrete   // Base concrete
                };
                
                let index = (y * width + x) as usize;
                texture.data[index] = color;
            }
        }
        
        texture
    }
}