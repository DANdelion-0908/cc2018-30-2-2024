use nalgebra_glm::{Vec3, dot};
use crate::ray_intersect::RayIntersect;

pub struct Oval {
    pub center: Vec3,
    pub radius_x: f32,
    pub radius_y: f32,
    pub normal: Vec3,
    pub color: u32,
}

impl RayIntersect for Oval {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<u32> {
        let denom = dot(&self.normal, ray_direction);
        if denom.abs() > 1e-6 {
            let p0l0 = self.center - ray_origin;
            let t = dot(&p0l0, &self.normal) / denom;
            if t >= 0.0 {
                let p_intersection = ray_origin + ray_direction * t;

                // Verificar si el punto está dentro del óvalo utilizando la ecuación de un elipse
                let local_x = dot(&(p_intersection - self.center), &Vec3::new(1.0, 0.0, 0.0));
                let local_y = dot(&(p_intersection - self.center), &Vec3::new(0.0, 1.0, 0.0));

                let equation = (local_x * local_x) / (self.radius_x * self.radius_x)
                             + (local_y * local_y) / (self.radius_y * self.radius_y);

                if equation <= 1.0 {
                    return Some(self.color);
                }
            }
        }
        None
    }
}
