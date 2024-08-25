
use nalgebra_glm::{Vec3, dot};
use crate::ray_intersect::RayIntersect;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub color: u32,
}

impl RayIntersect for Sphere {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<u32> {
        let oc = ray_origin - self.center;
        let a = dot(ray_direction, ray_direction);
        let b = 2.0 * dot(&oc, ray_direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            return Some(self.color); // Devuelve el color si hay una intersección
        }

        None // No hay intersección
    }
}
