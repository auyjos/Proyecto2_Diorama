mod framebuffer;
mod sphere;
mod cube;
mod texture;
mod renderer;
mod ray_intersect;
mod color;
mod light;
mod material;
mod camera;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use renderer::{render, Object};
use cube::Cube;
use texture::Texture;
use camera::CustomCamera;
use light::Light;
use material::Material;
use color::Color;
use std::f32::consts::PI;

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 600; // Reducido para mejor rendimiento
    let framebuffer_height = 450;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Rust Graphics - Raytracer with Reflections & Refractions")
        .log_level(TraceLogLevel::LOG_WARNING)
        .resizable()
        .build();

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height, raylib::color::Color::BLACK);

    framebuffer.set_background_color(raylib::color::Color::new(135, 206, 235, 255));
    framebuffer.clear();

    // Create textures for the Eclipse diorama
    let textures = vec![
        Texture::checkerboard(64, 64, Color::new(255, 255, 255), Color::new(0, 0, 0)),     // 0: Checkerboard (kept for reference)
        Texture::ancient_stone(64, 64),                                                     // 1: Ancient stone for ruins
        Texture::rusted_metal(64, 64),                                                      // 2: Rusted metal for weapons/armor
        Texture::blood_water(64, 64),                                                       // 3: Blood water for pools
        Texture::dark_crystal(64, 64),                                                      // 4: Dark crystal for mystical elements
        Texture::charred_wood(64, 64),                                                      // 5: Charred wood for burnt structures
    ];

    // BERSERK ECLIPSE DIORAMA - COHERENT CONCRETE & METAL DESIGN
    let mut objects = vec![];
    
    // === SOLID FOUNDATION PLATFORM (No floating elements) ===
    
    // Main concrete foundation (7x7 platform on ground level)
    for x in -3..4 {
        for z in -3..4 {
            objects.push(Object::Cube(
                Cube::new(Vector3::new(x as f32, -1.0, z as f32), 1.0, Material::concrete())
                    .with_texture(1)
            ));
        }
    }
    
    // === CENTRAL STEPPED PLATFORM (Metal reinforcement) ===
    
    // Level 1: Metal reinforced concrete (5x5)
    for x in -2..3 {
        for z in -2..3 {
            objects.push(Object::Cube(
                Cube::new(Vector3::new(x as f32, 0.0, z as f32), 1.0, Material::polished_metal())
                    .with_texture(2)
            ));
        }
    }
    
    // Level 2: Central altar (3x3 concrete)
    for x in -1..2 {
        for z in -1..2 {
            objects.push(Object::Cube(
                Cube::new(Vector3::new(x as f32, 1.0, z as f32), 1.0, Material::concrete())
                    .with_texture(1)
            ));
        }
    }
    
    // === CENTRAL ECLIPSE MONUMENT ===
    
    // Concrete pedestal
    objects.push(Object::Cube(
        Cube::new(Vector3::new(0.0, 2.0, 0.0), 1.0, Material::concrete())
            .with_texture(1)
    ));
    
    // Eclipse crystal on top
    objects.push(Object::Cube(
        Cube::new(Vector3::new(0.0, 3.0, 0.0), 0.6, Material::red_crystal())
            .with_texture(4)
    ));
    
    // === CORNER METAL PILLARS (Properly supported on foundation) ===
    
    // Four corner pillars on the foundation corners
    let corner_positions = [(-3.0, 3.0), (3.0, 3.0), (-3.0, -3.0), (3.0, -3.0)];
    
    for (x, z) in corner_positions.iter() {
        // Metal pillar base (on foundation level)
        objects.push(Object::Cube(
            Cube::new(Vector3::new(*x, -0.5, *z), 0.8, Material::polished_metal())
                .with_texture(2)
        ));
        
        // Metal pillar middle
        objects.push(Object::Cube(
            Cube::new(Vector3::new(*x, 0.5, *z), 0.8, Material::polished_metal())
                .with_texture(2)
        ));
        
        // Metal pillar top
        objects.push(Object::Cube(
            Cube::new(Vector3::new(*x, 1.5, *z), 0.8, Material::polished_metal())
                .with_texture(2)
        ));
        
        // Rusted metal cap
        objects.push(Object::Cube(
            Cube::new(Vector3::new(*x, 2.5, *z), 0.6, Material::rusted_metal())
                .with_texture(5)
        ));
    }
    
    // === DECORATIVE WATER FEATURES (On platform surfaces) ===
    
    // Small water pools on the metal platform (Level 1)
    objects.push(Object::Cube(
        Cube::new(Vector3::new(-1.5, 0.5, 0.0), 0.4, Material::dark_water())
            .with_texture(3)
    ));
    objects.push(Object::Cube(
        Cube::new(Vector3::new(1.5, 0.5, 0.0), 0.4, Material::dark_water())
            .with_texture(3)
    ));
    objects.push(Object::Cube(
        Cube::new(Vector3::new(0.0, 0.5, -1.5), 0.4, Material::dark_water())
            .with_texture(3)
    ));
    objects.push(Object::Cube(
        Cube::new(Vector3::new(0.0, 0.5, 1.5), 0.4, Material::dark_water())
            .with_texture(3)
    ));
    
    // Crystal accents on the concrete level (Level 2)
    objects.push(Object::Cube(
        Cube::new(Vector3::new(-0.7, 1.5, 0.7), 0.3, Material::red_crystal())
            .with_texture(4)
    ));
    objects.push(Object::Cube(
        Cube::new(Vector3::new(0.7, 1.5, -0.7), 0.3, Material::red_crystal())
            .with_texture(4)
    ));
    
    // === GROUND PLANE ===
    objects.push(Object::Cube(
        Cube::new(Vector3::new(0.0, -1002.0, 0.0), 2000.0, Material::ancient_stone())
            .with_texture(1)
    ));

    // Camera positioned for optimal Eclipse diorama view
    let mut camera = CustomCamera::new(
        Vector3::new(5.0, 5.0, 5.0),     // Better angle to see all pillars and structure
        Vector3::new(0.0, 1.5, 0.0),     // Looking slightly up at the monument
        Vector3::new(0.0, 1.0, 0.0)      // Up vector
    );
    let rotation_speed = PI / 60.0; // Slower rotation for cinematic feel
    let zoom_speed = 0.3;

        // ECLIPSE DRAMATIC LIGHTING - Final optimized version
    let lights = [
        // Main Eclipse light from above (dramatic red-orange)
        Light::new(
            Vector3::new(0.0, 10.0, 0.0),
            Color::new(255, 200, 150),
            2.2
        ),
        // Central crystal glow (red mystical light)
        Light::new(
            Vector3::new(0.0, 3.5, 0.0),
            Color::new(200, 50, 80),
            1.5
        ),
        // Corner pillar lights (warm metal glow) - optimized positions
        Light::new(
            Vector3::new(-3.0, 2.8, 3.0),
            Color::new(255, 180, 120), // Warm orange glow
            1.0
        ),
        Light::new(
            Vector3::new(3.0, 2.8, 3.0),
            Color::new(255, 180, 120), // Warm orange glow
            1.0
        ),
        Light::new(
            Vector3::new(-3.0, 2.8, -3.0),
            Color::new(255, 180, 120), // Warm orange glow
            1.0
        ),
        Light::new(
            Vector3::new(3.0, 2.8, -3.0),
            Color::new(255, 180, 120), // Warm orange glow
            1.0
        )
    ];

    while !window.window_should_close() {
        // Enhanced camera controls for the Eclipse diorama
        if window.is_key_down(KeyboardKey::KEY_LEFT) || window.is_key_down(KeyboardKey::KEY_A) {
            camera.orbit(rotation_speed, 0.0);
        }
        if window.is_key_down(KeyboardKey::KEY_RIGHT) || window.is_key_down(KeyboardKey::KEY_D) {
            camera.orbit(-rotation_speed, 0.0);
        }
        if window.is_key_down(KeyboardKey::KEY_UP) || window.is_key_down(KeyboardKey::KEY_W) {
            camera.orbit(0.0, -rotation_speed);
        }
        if window.is_key_down(KeyboardKey::KEY_DOWN) || window.is_key_down(KeyboardKey::KEY_S) {
            camera.orbit(0.0, rotation_speed);
        }
        
        // Zoom controls
        if window.is_key_down(KeyboardKey::KEY_Q) || window.is_key_down(KeyboardKey::KEY_Z) {
            // Zoom in - move camera closer to center
            let direction = (camera.center - camera.eye).normalized();
            camera.eye = camera.eye + direction * zoom_speed;
        }
        if window.is_key_down(KeyboardKey::KEY_E) || window.is_key_down(KeyboardKey::KEY_X) {
            // Zoom out - move camera away from center
            let direction = (camera.center - camera.eye).normalized();
            camera.eye = camera.eye - direction * zoom_speed;
        }

        // Check if window was resized
        let current_width = window.get_screen_width();
        let current_height = window.get_screen_height();
        
        if current_width != framebuffer.width() || current_height != framebuffer.height() {
            framebuffer.resize(current_width as u32, current_height as u32);
            framebuffer.clear();
        }

        // Render the scene
        render(&mut framebuffer, &objects, &camera, &lights, &textures);
        
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}
