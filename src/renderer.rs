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

const SHADOW_BIAS: f32 = 1e-4;
const MAX_RECURSION_DEPTH: i32 = 3;

// Eclipse skybox function - Creates the dramatic Berserk Eclipse atmosphere
fn skybox_color(ray_direction: &Vector3) -> Color {
    // Normalize ray direction
    let dir = ray_direction.normalized();
    
    // Calculate spherical coordinates
    let theta = dir.y.asin(); // Elevation angle (-π/2 to π/2)
    let phi = dir.z.atan2(dir.x); // Azimuthal angle (-π to π)
    
    // Eclipse position (high in the sky, slightly offset)
    let eclipse_theta = 0.7; // High elevation
    let eclipse_phi = 0.2;   // Slightly to the right
    
    // Calculate angular distance to eclipse center
    let d_theta = theta - eclipse_theta;
    let d_phi = phi - eclipse_phi;
    let angular_dist = (d_theta * d_theta + d_phi * d_phi).sqrt();
    
    // Eclipse parameters
    let eclipse_radius = 0.15; // Size of the eclipse
    let corona_radius = 0.25;  // Corona effect radius
    
    if angular_dist < eclipse_radius {
        // Inside the eclipse - very dark with subtle red rim
        if angular_dist > eclipse_radius * 0.8 {
            // Red rim of the eclipse
            return Color::new(80, 20, 10);
        } else {
            // Dark center
            return Color::new(20, 5, 5);
        }
    } else if angular_dist < corona_radius {
        // Corona effect - bright red/orange glow
        let corona_factor = 1.0 - (angular_dist - eclipse_radius) / (corona_radius - eclipse_radius);
        let intensity = (corona_factor * 255.0) as u8;
        return Color::new(intensity, (intensity as f32 * 0.6) as u8, (intensity as f32 * 0.2) as u8);
    }
    
    // Sky gradient based on elevation
    let elevation_factor = (theta + 1.57) / 3.14; // Normalize to 0-1
    
    if elevation_factor > 0.7 {
        // Upper sky - dark red/black
        let factor = (elevation_factor - 0.7) / 0.3;
        let r = (80.0 * (1.0 - factor) + 20.0 * factor) as u8;
        let g = (20.0 * (1.0 - factor) + 5.0 * factor) as u8;
        let b = (10.0 * (1.0 - factor) + 5.0 * factor) as u8;
        Color::new(r, g, b)
    } else if elevation_factor > 0.3 {
        // Middle sky - deep red
        let factor = (elevation_factor - 0.3) / 0.4;
        let r = (120.0 * (1.0 - factor) + 80.0 * factor) as u8;
        let g = (30.0 * (1.0 - factor) + 20.0 * factor) as u8;
        let b = (15.0 * (1.0 - factor) + 10.0 * factor) as u8;
        Color::new(r, g, b)
    } else {
        // Lower sky/horizon - darker red with some orange
        let factor = elevation_factor / 0.3;
        let r = (60.0 * factor + 40.0 * (1.0 - factor)) as u8;
        let g = (15.0 * factor + 8.0 * (1.0 - factor)) as u8;
        let b = (8.0 * factor + 5.0 * (1.0 - factor)) as u8;
        Color::new(r, g, b)
    }
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
    let width = framebuffer.width() as f32;
    let height = framebuffer.height() as f32;
    let aspect_ratio = width / height;
    let fov = PI / 3.0;
    let perspective_scale = (fov * 0.5).tan();

    for y in 0..framebuffer.height() {
        for x in 0..framebuffer.width() {
            // Map the pixel coordinate to screen space [-1, 1]
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            // Adjust for aspect ratio and perspective 
            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            // Calculate the direction of the ray for this pixel
            let mut ray_direction = Vector3::new(screen_x, screen_y, -1.0);
            ray_direction.normalize();

            // Apply camera rotation to the ray direction
            let rotated_direction = camera.basis_change(&ray_direction);

            // Cast the ray and get the pixel color
            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, lights, textures, MAX_RECURSION_DEPTH);

            // Draw the pixel on screen with the returned color
            framebuffer.set_pixel_with_color(x as u32, y as u32, pixel_color);
        }
    }
}