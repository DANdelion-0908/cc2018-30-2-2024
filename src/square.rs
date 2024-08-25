use nalgebra_glm::{Vec3, dot};
use crate::ray_intersect::RayIntersect;

pub struct Square {
    pub center: Vec3,
    pub height: f32,
    pub width: f32,
    pub normal: Vec3,
    pub color: u32,
}

impl RayIntersect for Square {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<u32> {
        let denom = dot(&self.normal, ray_direction);
        if denom.abs() > 1e-6 {
            let p0l0 = self.center - ray_origin;
            let t = dot(&p0l0, &self.normal) / denom;
            if t >= 0.0 {
                // Punto de intersección en el plano
                let p_intersection = ray_origin + ray_direction * t;

                // Verificar si el punto está dentro del cuadrado
                let half_width = self.width / 2.0;
                let half_height = self.height / 2.0;

                // Proyección del punto de intersección en el sistema local del cuadrado
                let local_x = dot(&(p_intersection - self.center), &Vec3::new(1.0, 0.0, 0.0));
                let local_y = dot(&(p_intersection - self.center), &Vec3::new(0.0, 1.0, 0.0));

                if local_x.abs() <= half_width && local_y.abs() <= half_height {
                    return Some(self.color); // Devuelve el color si hay una intersección
                }
            }
        }
        None // No hay intersección
    }
}
