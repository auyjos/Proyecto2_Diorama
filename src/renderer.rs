use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::material::Material;
use crate::light::Light;
use crate::color::Color;
use crate::camera::CustomCamera;
use crate::sphere::Sphere;
use crate::cube::Cube;
use crate::texture::Texture;
use std::f32::consts::PI;
use rayon::prelude::*;

const SHADOW_BIAS: f32 = 1e-4;
const MAX_RECURSION_DEPTH: i32 = 3;

// World-space AABB that bounds our diorama. Rays that don't enter this box will
// skip object intersection entirely and sample the skybox. This creates a
// compact, focused rendering area showing only the diorama like a microcube.
const SCENE_MIN: Vector3 = Vector3 { x: -3.8, y: -1.0, z: -3.8 };
const SCENE_MAX: Vector3 = Vector3 { x:  3.8, y:  3.2, z:  3.8 };

#[inline]
fn point_in_aabb(p: &Vector3, min: &Vector3, max: &Vector3) -> bool {
    p.x >= min.x && p.x <= max.x &&
    p.y >= min.y && p.y <= max.y &&
    p.z >= min.z && p.z <= max.z
}

// Ray vs AABB using the slab method
#[inline]
fn ray_aabb_intersect(ro: &Vector3, rd: &Vector3, min: &Vector3, max: &Vector3) -> bool {
    // Robust slab method with proper handling for near-zero components
    let inv_x = if rd.x != 0.0 { 1.0 / rd.x } else { f32::INFINITY };
    let inv_y = if rd.y != 0.0 { 1.0 / rd.y } else { f32::INFINITY };
    let inv_z = if rd.z != 0.0 { 1.0 / rd.z } else { f32::INFINITY };

    let mut t1 = (min.x - ro.x) * inv_x;
    let mut t2 = (max.x - ro.x) * inv_x;
    if t1 > t2 { std::mem::swap(&mut t1, &mut t2); }

    let mut ty1 = (min.y - ro.y) * inv_y;
    let mut ty2 = (max.y - ro.y) * inv_y;
    if ty1 > ty2 { std::mem::swap(&mut ty1, &mut ty2); }

    if (t1 > ty2) || (ty1 > t2) { return false; }
    if ty1 > t1 { t1 = ty1; }
    if ty2 < t2 { t2 = ty2; }

    let mut tz1 = (min.z - ro.z) * inv_z;
    let mut tz2 = (max.z - ro.z) * inv_z;
    if tz1 > tz2 { std::mem::swap(&mut tz1, &mut tz2); }

    if (t1 > tz2) || (tz1 > t2) { return false; }
    if tz1 > t1 { t1 = tz1; }
    if tz2 < t2 { t2 = tz2; }

    // We consider an intersection only if the box is hit at t >= 0 (forward ray)
    t2 >= 0.0
}

// Zen Cosmic skybox function - Creates a serene futuristic atmosphere
fn skybox_color(ray_direction: &Vector3) -> Color {
    // Normalize ray direction
    let dir = ray_direction.normalized();
    
    // Calculate spherical coordinates
    let theta = dir.y.asin(); // Elevation angle (-π/2 to π/2)
    let phi = dir.z.atan2(dir.x); // Azimuthal angle (-π to π)
    
    // Cosmic nebula centers for depth
    let nebula1_theta = 0.8; // High elevation
    let nebula1_phi = 1.0;   // Eastern side
    let nebula2_theta = -0.3; // Lower elevation
    let nebula2_phi = -1.5;   // Western side
    
    // Calculate distances to nebula centers
    let d1_theta = theta - nebula1_theta;
    let d1_phi = phi - nebula1_phi;
    let dist1 = (d1_theta * d1_theta + d1_phi * d1_phi).sqrt();
    
    let d2_theta = theta - nebula2_theta;
    let d2_phi = phi - nebula2_phi;
    let dist2 = (d2_theta * d2_theta + d2_phi * d2_phi).sqrt();
    
    // Nebula parameters
    let nebula_radius = 0.6;
    let nebula_intensity = 0.4;
    
    // Base cosmic gradient
    let elevation_factor = (theta + 1.57) / 3.14; // Normalize to 0-1
    
    let mut base_color = if elevation_factor > 0.8 {
        // Deep space - dark blue with purple hints
        let factor = (elevation_factor - 0.8) / 0.2;
        let r = (15.0 * (1.0 - factor) + 25.0 * factor) as u8;
        let g = (25.0 * (1.0 - factor) + 15.0 * factor) as u8;
        let b = (45.0 * (1.0 - factor) + 55.0 * factor) as u8;
        Color::new(r, g, b)
    } else if elevation_factor > 0.4 {
        // Mid sky - gentle blue gradient
        let factor = (elevation_factor - 0.4) / 0.4;
        let r = (30.0 * (1.0 - factor) + 15.0 * factor) as u8;
        let g = (45.0 * (1.0 - factor) + 25.0 * factor) as u8;
        let b = (80.0 * (1.0 - factor) + 45.0 * factor) as u8;
        Color::new(r, g, b)
    } else {
        // Horizon - warmer cosmic tones
        let factor = elevation_factor / 0.4;
        let r = (45.0 * factor + 35.0 * (1.0 - factor)) as u8;
        let g = (55.0 * factor + 40.0 * (1.0 - factor)) as u8;
        let b = (85.0 * factor + 65.0 * (1.0 - factor)) as u8;
        Color::new(r, g, b)
    };
    
    // Add cyan nebula effect (complements zen water)
    if dist1 < nebula_radius {
        let nebula_factor = 1.0 - (dist1 / nebula_radius);
        let intensity = nebula_factor * nebula_intensity;
        base_color.r = ((base_color.r as f32) * (1.0 - intensity) + 60.0 * intensity) as u8;
        base_color.g = ((base_color.g as f32) * (1.0 - intensity) + 120.0 * intensity) as u8;
        base_color.b = ((base_color.b as f32) * (1.0 - intensity) + 140.0 * intensity) as u8;
    }
    
    // Add purple nebula effect (complements crystal refractions)
    if dist2 < nebula_radius {
        let nebula_factor = 1.0 - (dist2 / nebula_radius);
        let intensity = nebula_factor * nebula_intensity;
        base_color.r = ((base_color.r as f32) * (1.0 - intensity) + 100.0 * intensity) as u8;
        base_color.g = ((base_color.g as f32) * (1.0 - intensity) + 50.0 * intensity) as u8;
        base_color.b = ((base_color.b as f32) * (1.0 - intensity) + 120.0 * intensity) as u8;
    }
    
    // Add subtle stars effect with noise
    let star_noise = ((phi * 50.0).sin() * (theta * 30.0).cos() * (phi * theta * 100.0).sin()).abs();
    if star_noise > 0.98 {
        let star_intensity = (star_noise - 0.98) / 0.02;
        base_color.r = (base_color.r as f32 + 40.0 * star_intensity) as u8;
        base_color.g = (base_color.g as f32 + 40.0 * star_intensity) as u8;
        base_color.b = (base_color.b as f32 + 40.0 * star_intensity) as u8;
    }
    
    base_color
}

// Enum to handle different object types
#[derive(Clone)]
pub enum Object {
    Sphere(Sphere),
    Cube(Cube),
}

impl RayIntersect for Object {
    fn ray_intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> Intersect {
        match self {
            Object::Sphere(sphere) => sphere.ray_intersect(ray_origin, ray_direction),
            Object::Cube(cube) => cube.ray_intersect(ray_origin, ray_direction),
        }
    }
}

impl Object {
    pub fn get_material(&self) -> Material {
        match self {
            Object::Sphere(sphere) => sphere.material,
            Object::Cube(cube) => cube.material,
        }
    }

    pub fn get_texture_color(&self, intersect: &Intersect, textures: &[Texture]) -> Color {
        match self {
            Object::Sphere(_) => intersect.material.diffuse, // Spheres use solid colors
            Object::Cube(cube) => {
                if let Some(texture_id) = cube.texture_id {
                    if texture_id < textures.len() {
                        let (u, v) = cube.get_uv(intersect.point, intersect.normal);
                        return textures[texture_id].sample(u, v);
                    }
                }
                intersect.material.diffuse // Fallback to material color
            }
        }
    }
}

// Función de reflexión siguiendo la fórmula: R = I - 2(I·N)N
fn reflect(incident: &Vector3, normal: &Vector3) -> Vector3 {
    *incident - *normal * 2.0 * incident.dot(*normal)
}

// Función de refracción siguiendo la Ley de Snell
fn refract(incident: &Vector3, normal: &Vector3, eta: f32) -> Option<Vector3> {
    let cos_i = -incident.dot(*normal).max(-1.0).min(1.0);
    let sin_t2 = eta * eta * (1.0 - cos_i * cos_i);
    
    // Verificar reflexión total interna (RTI)
    if sin_t2 > 1.0 {
        return None; // RTI - no hay refracción
    }
    
    let cos_t = (1.0 - sin_t2).sqrt();
    Some(*incident * eta + *normal * (eta * cos_i - cos_t))
}

// Ecuaciones de Fresnel para determinar qué tanto se refleja vs refracta
fn fresnel(incident: &Vector3, normal: &Vector3, ior: f32) -> f32 {
    let cos_i = incident.dot(*normal).abs().max(-1.0).min(1.0);
    let eta_i = 1.0;
    let eta_t = ior;
    
    let sin_t = eta_i / eta_t * (1.0 - cos_i * cos_i).sqrt();
    
    if sin_t >= 1.0 {
        return 1.0; // Reflexión total
    }
    
    let cos_t = (1.0 - sin_t * sin_t).sqrt();
    let cos_i = cos_i.abs();
    
    let r_ortho = ((eta_t * cos_i) - (eta_i * cos_t)) / ((eta_t * cos_i) + (eta_i * cos_t));
    let r_para = ((eta_i * cos_i) - (eta_t * cos_t)) / ((eta_i * cos_i) + (eta_t * cos_t));
    
    (r_ortho * r_ortho + r_para * r_para) / 2.0
}

fn cast_shadow(
    intersect: &Intersect,
    light: &Light,
    objects: &[Object],
) -> f32 {
    let mut light_dir = light.position - intersect.point;
    light_dir.normalize();
    let light_distance = (light.position - intersect.point).length();

    let offset_normal = intersect.normal * SHADOW_BIAS;
    let shadow_ray_origin = if light_dir.dot(intersect.normal) < 0.0 {
        intersect.point - offset_normal
    } else {
        intersect.point + offset_normal
    };

    let mut shadow_intensity = 0.0;

    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        if shadow_intersect.is_intersecting && shadow_intersect.distance < light_distance {
            let distance_ratio = shadow_intersect.distance / light_distance;
            shadow_intensity = 1.0 - distance_ratio.powf(2.0).min(1.0);
            break;
        }
    }

    shadow_intensity
}

pub fn cast_ray(
    ray_origin: &Vector3,
    ray_direction: &Vector3,
    objects: &[Object],
    lights: &[Light],
    textures: &[Texture],
    depth: i32,
) -> Color {
    if depth <= 0 {
        return Color::new(0, 0, 0); // Negro si alcanzamos máxima profundidad
    }

    // Scene culling: if the ray origin is outside the scene AABB and the ray
    // doesn't intersect the AABB, skip object tests and sample the skybox.
    // This preserves the skybox while limiting rendering to our diorama.
    if !point_in_aabb(ray_origin, &SCENE_MIN, &SCENE_MAX) {
        if !ray_aabb_intersect(ray_origin, ray_direction, &SCENE_MIN, &SCENE_MAX) {
            return skybox_color(ray_direction);
        }
    }

    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;
    let mut closest_object: Option<&Object> = None;

    // Encontrar la intersección más cercana
    for object in objects {
        let i = object.ray_intersect(ray_origin, ray_direction);
        if i.is_intersecting && i.distance < zbuffer {
            zbuffer = i.distance;
            intersect = i;
            closest_object = Some(object);
        }
    }

    if !intersect.is_intersecting {
        return skybox_color(ray_direction); // Usar skybox en lugar de color fijo
    }

    let closest_object = closest_object.unwrap();

    // Get texture color for the surface
    let surface_color = closest_object.get_texture_color(&intersect, textures);
    
    // Color local (iluminación Phong)
    let mut color = Color::new(0, 0, 0);
    
    // Iluminación ambiente
    let ambient = surface_color * 0.1;
    color = color + ambient;

    // Calcular iluminación directa para cada luz
    for light in lights {
        let mut light_dir = light.position - intersect.point;
        light_dir.normalize();
        let view_dir = (*ray_origin - intersect.point).normalized();
        let reflect_dir = reflect(&-light_dir, &intersect.normal);

        let shadow_intensity = cast_shadow(&intersect, light, objects);
        let light_intensity = light.intensity * (1.0 - shadow_intensity);

        // Componente difusa usando color de textura
        let diffuse_intensity = intersect.normal.dot(light_dir).max(0.0);
        let diffuse = surface_color * intersect.material.albedo[0] * diffuse_intensity * light_intensity;

        // Componente especular
        let specular_intensity = view_dir.dot(reflect_dir).max(0.0).powf(intersect.material.specular);
        let specular = light.color * intersect.material.albedo[1] * specular_intensity * light_intensity;

        color = color + diffuse + specular;
    }

    // Calcular reflexión - Solo si vale la pena
    let mut reflect_color = Color::new(0, 0, 0);
    if intersect.material.albedo[2] > 0.01 {
        let reflect_dir = reflect(ray_direction, &intersect.normal);
        let reflect_origin = intersect.point + intersect.normal * SHADOW_BIAS;
        reflect_color = cast_ray(&reflect_origin, &reflect_dir, objects, lights, textures, depth - 1);
    }

    // Calcular refracción - Solo si vale la pena
    let mut refract_color = Color::new(0, 0, 0);
    if intersect.material.albedo[3] > 0.01 && intersect.material.transparency > 0.01 {
        let mut normal = intersect.normal;
        let mut eta = 1.0 / intersect.material.refractive_index;
        
        if ray_direction.dot(intersect.normal) > 0.0 {
            normal = -normal;
            eta = intersect.material.refractive_index;
        }

        if let Some(refract_dir) = refract(ray_direction, &normal, eta) {
            let refract_origin = intersect.point - normal * SHADOW_BIAS;
            refract_color = cast_ray(&refract_origin, &refract_dir, objects, lights, textures, depth - 1);
        }
    }

    // Aplicar ecuaciones de Fresnel para mezclar reflexión y refracción
    let kr = if intersect.material.transparency > 0.0 {
        fresnel(ray_direction, &intersect.normal, intersect.material.refractive_index)
    } else {
        intersect.material.albedo[2]
    };

    // Combinar todos los componentes
    let local_contribution = 1.0 - intersect.material.albedo[2] - intersect.material.albedo[3];
    color = color * local_contribution + 
            reflect_color * kr + 
            refract_color * (1.0 - kr) * intersect.material.transparency;

    color
}

pub fn render(
    framebuffer: &mut Framebuffer, 
    objects: &[Object], 
    camera: &CustomCamera, 
    lights: &[Light],
    textures: &[Texture]
) {
    let width = framebuffer.width() as usize;
    let height = framebuffer.height() as usize;
    let aspect_ratio = width as f32 / height as f32;
    let fov = PI / 3.0;
    let perspective_scale = (fov * 0.5).tan();

    // Create a buffer to store all pixel colors
    // This allows parallel computation without concurrent writes to framebuffer
    let pixels: Vec<(u32, u32, Color)> = (0..height)
        .into_par_iter()
        .flat_map(|y| {
            (0..width)
                .into_par_iter()
                .map(move |x| {
                    // Map the pixel coordinate to screen space [-1, 1]
                    let screen_x = (2.0 * x as f32) / width as f32 - 1.0;
                    let screen_y = -(2.0 * y as f32) / height as f32 + 1.0;

                    // Adjust for aspect ratio and perspective 
                    let screen_x = screen_x * aspect_ratio * perspective_scale;
                    let screen_y = screen_y * perspective_scale;

                    // Calculate the direction of the ray for this pixel
                    let mut ray_direction = Vector3::new(screen_x, screen_y, -1.0);
                    ray_direction.normalize();

                    // Apply camera rotation to the ray direction
                    let rotated_direction = camera.basis_change(&ray_direction);

                    // Cast the ray and get the pixel color
                    let pixel_color = cast_ray(
                        &camera.eye, 
                        &rotated_direction, 
                        objects, 
                        lights, 
                        textures, 
                        MAX_RECURSION_DEPTH
                    );

                    (x as u32, y as u32, pixel_color)
                })
                .collect::<Vec<_>>()
        })
        .collect();

    // Write all computed pixels to the framebuffer (sequential, but fast)
    for (x, y, color) in pixels {
        framebuffer.set_pixel_with_color(x, y, color);
    }
}