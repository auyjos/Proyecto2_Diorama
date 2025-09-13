use raylib::prelude::*;
use crate::material::Material;

#[derive(Debug, Clone)]
pub struct Intersect {
    pub distance: f32,
    pub is_intersecting: bool,
    pub material: Material,
    pub point: Vector3,
    pub normal: Vector3,
}

impl Intersect {
    pub fn new(point: Vector3, normal: Vector3, distance: f32, material: Material) -> Self {
        Intersect {
            distance,
            is_intersecting: true,
            material,
            point,
            normal,
        }
    }

    pub fn empty() -> Self {
        Intersect {
            distance: 0.0,
            is_intersecting: false,
            material: Material::rubber(), // Usar un material válido
            point: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> Intersect;
}
