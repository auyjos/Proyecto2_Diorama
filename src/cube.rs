use raylib::prelude::*;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::material::Material;
use crate::texture::Texture;

#[derive(Debug, Clone, Copy)]
pub struct Cube {
    pub min: Vector3,    // Minimum corner of the cube
    pub max: Vector3,    // Maximum corner of the cube
    pub material: Material,
    pub texture_id: Option<usize>, // Index into texture array
}

impl Cube {
    pub fn new(center: Vector3, size: f32, material: Material) -> Self {
        let half_size = size / 2.0;
        Cube {
            min: Vector3::new(
                center.x - half_size,
                center.y - half_size, 
                center.z - half_size
            ),
            max: Vector3::new(
                center.x + half_size,
                center.y + half_size,
                center.z + half_size
            ),
            material,
            texture_id: None,
        }
    }

    pub fn with_texture(mut self, texture_id: usize) -> Self {
        self.texture_id = Some(texture_id);
        self
    }

    // Calculate UV coordinates for a point on the cube surface
    pub fn get_uv(&self, point: Vector3, normal: Vector3) -> (f32, f32) {
        let epsilon = 1e-4;
        
        // Determine which face we're on based on the normal
        if (normal.x - 1.0).abs() < epsilon {
            // Right face (+X)
            let u = (point.z - self.min.z) / (self.max.z - self.min.z);
            let v = (point.y - self.min.y) / (self.max.y - self.min.y);
            (u, 1.0 - v)
        } else if (normal.x + 1.0).abs() < epsilon {
            // Left face (-X)
            let u = (self.max.z - point.z) / (self.max.z - self.min.z);
            let v = (point.y - self.min.y) / (self.max.y - self.min.y);
            (u, 1.0 - v)
        } else if (normal.y - 1.0).abs() < epsilon {
            // Top face (+Y)
            let u = (point.x - self.min.x) / (self.max.x - self.min.x);
            let v = (point.z - self.min.z) / (self.max.z - self.min.z);
            (u, v)
        } else if (normal.y + 1.0).abs() < epsilon {
            // Bottom face (-Y)
            let u = (point.x - self.min.x) / (self.max.x - self.min.x);
            let v = (self.max.z - point.z) / (self.max.z - self.min.z);
            (u, v)
        } else if (normal.z - 1.0).abs() < epsilon {
            // Front face (+Z)
            let u = (self.max.x - point.x) / (self.max.x - self.min.x);
            let v = (point.y - self.min.y) / (self.max.y - self.min.y);
            (u, 1.0 - v)
        } else {
            // Back face (-Z)
            let u = (point.x - self.min.x) / (self.max.x - self.min.x);
            let v = (point.y - self.min.y) / (self.max.y - self.min.y);
            (u, 1.0 - v)
        }
    }
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> Intersect {
        // Slab method for ray-cube intersection
        let mut t_min = f32::NEG_INFINITY;
        let mut t_max = f32::INFINITY;
        let mut normal = Vector3::new(0.0, 0.0, 0.0);
        let mut hit_normal = Vector3::new(0.0, 0.0, 0.0);

        // Check intersection with X slabs
        if ray_direction.x.abs() < 1e-8 {
            // Ray is parallel to X slabs
            if ray_origin.x < self.min.x || ray_origin.x > self.max.x {
                return Intersect::empty();
            }
        } else {
            let inv_dir = 1.0 / ray_direction.x;
            let mut t1 = (self.min.x - ray_origin.x) * inv_dir;
            let mut t2 = (self.max.x - ray_origin.x) * inv_dir;
            let mut face_normal = Vector3::new(-1.0, 0.0, 0.0);
            
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
                face_normal = Vector3::new(1.0, 0.0, 0.0);
            }
            
            if t1 > t_min {
                t_min = t1;
                hit_normal = face_normal;
            }
            if t2 < t_max {
                t_max = t2;
            }
            
            if t_min > t_max {
                return Intersect::empty();
            }
        }

        // Check intersection with Y slabs
        if ray_direction.y.abs() < 1e-8 {
            if ray_origin.y < self.min.y || ray_origin.y > self.max.y {
                return Intersect::empty();
            }
        } else {
            let inv_dir = 1.0 / ray_direction.y;
            let mut t1 = (self.min.y - ray_origin.y) * inv_dir;
            let mut t2 = (self.max.y - ray_origin.y) * inv_dir;
            let mut face_normal = Vector3::new(0.0, -1.0, 0.0);
            
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
                face_normal = Vector3::new(0.0, 1.0, 0.0);
            }
            
            if t1 > t_min {
                t_min = t1;
                hit_normal = face_normal;
            }
            if t2 < t_max {
                t_max = t2;
            }
            
            if t_min > t_max {
                return Intersect::empty();
            }
        }

        // Check intersection with Z slabs
        if ray_direction.z.abs() < 1e-8 {
            if ray_origin.z < self.min.z || ray_origin.z > self.max.z {
                return Intersect::empty();
            }
        } else {
            let inv_dir = 1.0 / ray_direction.z;
            let mut t1 = (self.min.z - ray_origin.z) * inv_dir;
            let mut t2 = (self.max.z - ray_origin.z) * inv_dir;
            let mut face_normal = Vector3::new(0.0, 0.0, -1.0);
            
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
                face_normal = Vector3::new(0.0, 0.0, 1.0);
            }
            
            if t1 > t_min {
                t_min = t1;
                hit_normal = face_normal;
            }
            if t2 < t_max {
                t_max = t2;
            }
            
            if t_min > t_max {
                return Intersect::empty();
            }
        }

        // We have an intersection
        let t = if t_min > 0.0 { t_min } else { t_max };
        
        if t > 0.0 {
            let point = *ray_origin + *ray_direction * t;
            normal = hit_normal;
            
            return Intersect::new(point, normal, t, self.material);
        }

        Intersect::empty()
    }
}