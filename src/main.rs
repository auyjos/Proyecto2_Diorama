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

    // Create textures for the Zen Garden diorama (6 materials: 5 unique + concrete base)
    let textures = vec![
        Texture::checkerboard(64, 64, Color::new(255, 255, 255), Color::new(0, 0, 0)),     // 0: Reference checkerboard
        Texture::zen_moss(64, 64),                                                          // 1: Natural moss vegetation
        Texture::brushed_metal(64, 64),                                                     // 2: Tech metal panels
        Texture::zen_water(64, 64),                                                         // 3: Calm water surface
        Texture::crystal_glass(64, 64),                                                     // 4: Prismatic crystal
        Texture::chrome_mirror(64, 64),                                                     // 5: Reflective chrome
        Texture::concrete_base(64, 64),                                                     // 6: Concrete foundation
    ];

    // === UNIFIED ZEN GARDEN - COHERENT MATERIAL GROUPS ===
    let mut objects = vec![];
    
    // === SOLID CONCRETE BASE (Foundation for everything) ===
    
    // Large concrete foundation platform (11x11)
    for x in -5..6 {
        for z in -5..6 {
            objects.push(Object::Cube(
                Cube::new(Vector3::new(x as f32, -1.0, z as f32), 1.0, Material::concrete_base())
                    .with_texture(6)
            ));
        }
    }
    
    // === INSTALLATION 1: CENTRAL REFLECTION POND ===
    // Combines: Water + Crystal + Chrome + Metal
    // Theme: Central focal point showing water reflections and crystal refractions
    
    // Water pond with integrated elements (3x3 arrangement)
    for x in -1..2 {
        for z in -1..2 {
            objects.push(Object::Cube(
                Cube::new(Vector3::new(x as f32, -0.5, z as f32), 1.0, Material::zen_water())
                    .with_texture(3)
            ));
        }
    }
    
    // Central crystal formation emerging from water
    objects.push(Object::Cube(
        Cube::new(Vector3::new(0.0, -0.2, 0.0), 0.8, Material::crystal_glass())
            .with_texture(4)
    ));
    
    // Chrome reflection panels around pond (N, S, E, W)
    objects.push(Object::Cube(
        Cube::new(Vector3::new(0.0, -0.3, -2.2), 1.5, Material::chrome_mirror())
            .with_texture(5)
    ));
    objects.push(Object::Cube(
        Cube::new(Vector3::new(0.0, -0.3, 2.2), 1.5, Material::chrome_mirror())
            .with_texture(5)
    ));
    objects.push(Object::Cube(
        Cube::new(Vector3::new(-2.2, -0.3, 0.0), 1.5, Material::chrome_mirror())
            .with_texture(5)
    ));
    objects.push(Object::Cube(
        Cube::new(Vector3::new(2.2, -0.3, 0.0), 1.5, Material::chrome_mirror())
            .with_texture(5)
    ));
    
    // Metal support structures for chrome panels
    objects.push(Object::Cube(
        Cube::new(Vector3::new(0.0, -0.5, -2.5), 0.6, Material::brushed_metal())
            .with_texture(2)
    ));
    objects.push(Object::Cube(
        Cube::new(Vector3::new(0.0, -0.5, 2.5), 0.6, Material::brushed_metal())
            .with_texture(2)
    ));
    objects.push(Object::Cube(
        Cube::new(Vector3::new(-2.5, -0.5, 0.0), 0.6, Material::brushed_metal())
            .with_texture(2)
    ));
    objects.push(Object::Cube(
        Cube::new(Vector3::new(2.5, -0.5, 0.0), 0.6, Material::brushed_metal())
            .with_texture(2)
    ));
    
    // === INSTALLATION 2: NORTHEAST ZEN GARDEN ===
    // Combines: Moss + Crystal + Water + Metal
    // Theme: Natural meditation area with technological accents
    
    // Moss garden cluster
    objects.push(Object::Cube(
        Cube::new(Vector3::new(3.5, -0.5, 3.5), 0.8, Material::zen_moss())
            .with_texture(1)
    ));
    objects.push(Object::Cube(
        Cube::new(Vector3::new(4.0, -0.5, 3.0), 0.6, Material::zen_moss())
            .with_texture(1)
    ));
    objects.push(Object::Cube(
        Cube::new(Vector3::new(3.0, -0.5, 4.0), 0.6, Material::zen_moss())
            .with_texture(1)
    ));
    
    // Small water feature integrated with moss
    objects.push(Object::Cube(
        Cube::new(Vector3::new(3.5, -0.3, 3.0), 0.4, Material::zen_water())
            .with_texture(3)
    ));
    
    // Crystal meditation point in moss
    objects.push(Object::Cube(
        Cube::new(Vector3::new(4.0, -0.2, 3.5), 0.3, Material::crystal_glass())
            .with_texture(4)
    ));
    
    // Metal accent element
    objects.push(Object::Cube(
        Cube::new(Vector3::new(3.0, -0.5, 3.5), 0.4, Material::brushed_metal())
            .with_texture(2)
    ));
    
    // === INSTALLATION 3: SOUTHWEST TECH GROVE ===
    // Combines: Metal + Chrome + Crystal + Moss
    // Theme: Technology integrated with nature
    
    // Metal platform base
    objects.push(Object::Cube(
        Cube::new(Vector3::new(-3.5, -0.5, -3.5), 1.2, Material::brushed_metal())
            .with_texture(2)
    ));
    
    // Chrome tech panel
    objects.push(Object::Cube(
        Cube::new(Vector3::new(-3.5, -0.2, -3.5), 1.0, Material::chrome_mirror())
            .with_texture(5)
    ));
    
    // Moss growing around tech
    objects.push(Object::Cube(
        Cube::new(Vector3::new(-3.0, -0.5, -3.0), 0.5, Material::zen_moss())
            .with_texture(1)
    ));
    objects.push(Object::Cube(
        Cube::new(Vector3::new(-4.0, -0.5, -3.0), 0.5, Material::zen_moss())
            .with_texture(1)
    ));
    objects.push(Object::Cube(
        Cube::new(Vector3::new(-3.0, -0.5, -4.0), 0.5, Material::zen_moss())
            .with_texture(1)
    ));
    
    // Crystal energy core
    objects.push(Object::Cube(
        Cube::new(Vector3::new(-3.5, 0.1, -3.5), 0.4, Material::crystal_glass())
            .with_texture(4)
    ));
    
    // === INSTALLATION 4: SOUTHEAST WATER CASCADE ===
    // Combines: Water + Crystal + Chrome + Metal
    // Theme: Flowing water with reflective and refractive elements
    
    // Stepped water pools
    objects.push(Object::Cube(
        Cube::new(Vector3::new(3.5, -0.4, -3.0), 0.8, Material::zen_water())
            .with_texture(3)
    ));
    objects.push(Object::Cube(
        Cube::new(Vector3::new(3.5, -0.5, -3.8), 0.6, Material::zen_water())
            .with_texture(3)
    ));
    objects.push(Object::Cube(
        Cube::new(Vector3::new(3.5, -0.3, -2.2), 0.6, Material::zen_water())
            .with_texture(3)
    ));
    
    // Crystal formations in water
    objects.push(Object::Cube(
        Cube::new(Vector3::new(3.5, -0.1, -3.0), 0.3, Material::crystal_glass())
            .with_texture(4)
    ));
    objects.push(Object::Cube(
        Cube::new(Vector3::new(3.5, -0.2, -3.8), 0.2, Material::crystal_glass())
            .with_texture(4)
    ));
    
    // Chrome reflection surface
    objects.push(Object::Cube(
        Cube::new(Vector3::new(4.2, -0.3, -3.0), 0.4, Material::chrome_mirror())
            .with_texture(5)
    ));
    
    // Metal support structure
    objects.push(Object::Cube(
        Cube::new(Vector3::new(4.2, -0.5, -3.0), 0.3, Material::brushed_metal())
            .with_texture(2)
    ));
    
    // === INSTALLATION 5: NORTHWEST HARMONY POINT ===
    // Combines: All 5 materials in one balanced composition
    // Theme: Complete material harmony showcase
    
    // Base moss platform
    objects.push(Object::Cube(
        Cube::new(Vector3::new(-3.5, -0.5, 3.5), 1.0, Material::zen_moss())
            .with_texture(1)
    ));
    
    // Metal support frame
    objects.push(Object::Cube(
        Cube::new(Vector3::new(-3.5, -0.3, 3.5), 0.8, Material::brushed_metal())
            .with_texture(2)
    ));
    
    // Small water feature
    objects.push(Object::Cube(
        Cube::new(Vector3::new(-3.5, -0.1, 3.5), 0.5, Material::zen_water())
            .with_texture(3)
    ));
    
    // Crystal centerpiece
    objects.push(Object::Cube(
        Cube::new(Vector3::new(-3.5, 0.1, 3.5), 0.3, Material::crystal_glass())
            .with_texture(4)
    ));
    
    // Chrome reflection accent
    objects.push(Object::Cube(
        Cube::new(Vector3::new(-3.0, -0.2, 3.0), 0.4, Material::chrome_mirror())
            .with_texture(5)
    ));

    // Camera positioned for optimal zen garden viewing
    let mut camera = CustomCamera::new(
        Vector3::new(8.0, 4.0, 8.0),     // Elevated position to see the full composition
        Vector3::new(0.0, 1.0, 0.0),     // Looking at the center of the garden
        Vector3::new(0.0, 1.0, 0.0)      // Up vector
    );
    let rotation_speed = PI / 60.0; // Smooth rotation for zen experience
    let zoom_speed = 0.3;

    // ZEN GARDEN LIGHTING - Serene and balanced illumination
    let lights = [
        // Overhead ambient light (soft white for natural feel)
        Light::new(
            Vector3::new(0.0, 8.0, 0.0),
            Color::new(240, 245, 255), // Cool white daylight
            1.8
        ),
        // Central water pool glow (blue-green reflection enhancer)
        Light::new(
            Vector3::new(0.0, 2.5, 0.0),
            Color::new(120, 180, 220), // Soft blue-cyan
            1.2
        ),
        // Corner tech tower lights (warm tech glow)
        Light::new(
            Vector3::new(-2.5, 2.5, 2.5),
            Color::new(200, 220, 255), // Cool tech blue
            0.8
        ),
        Light::new(
            Vector3::new(2.5, 2.5, 2.5),
            Color::new(200, 220, 255), // Cool tech blue
            0.8
        ),
        Light::new(
            Vector3::new(-2.5, 2.5, -2.5),
            Color::new(200, 220, 255), // Cool tech blue
            0.8
        ),
        Light::new(
            Vector3::new(2.5, 2.5, -2.5),
            Color::new(200, 220, 255), // Cool tech blue
            0.8
        )
    ];

    while !window.window_should_close() {
        // Smooth camera controls for zen garden exploration
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
