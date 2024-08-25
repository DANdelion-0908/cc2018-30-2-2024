
mod framebuffer;
mod ray_intersect;
mod sphere;
mod square;
mod oval;

use minifb::{ Window, WindowOptions, Key };
use nalgebra_glm::{Vec3, normalize};
use std::time::Duration;
use crate::ray_intersect::RayIntersect;
use crate::sphere::Sphere;
use crate::square::Square;
use crate::oval::Oval;

use crate::framebuffer::Framebuffer;

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Box<dyn RayIntersect>]) -> u32 {
    for object in objects {
        if let Some(color) = object.ray_intersect(ray_origin, ray_direction) {
            return color; // Devuelve el color del objeto si hay una intersección
        }
    }

    0x000000 // Negro si no hay intersección
}


pub fn render(framebuffer: &mut Framebuffer, objects: &[Box<dyn RayIntersect>]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio;

            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));

            let pixel_color = cast_ray(&Vec3::new(0.0, 0.0, 0.0), &ray_direction, objects);

            framebuffer.set_current_color(pixel_color);
            framebuffer.point(x, y);
        }
    }
}


fn main() {
    let window_width = 1300;
    let window_height = 900;
    let framebuffer_width = 1300;
    let framebuffer_height = 900;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Caster",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    // Almacena diferentes objetos en un Vec<Box<dyn RayIntersect>>
    let objects: Vec<Box<dyn RayIntersect>> = vec![
        Box::new(Sphere { // Pupila
            center: Vec3::new(0.0, 0.0, -5.0),
            radius: 0.2,
            color: 0x000000,
        }),
        Box::new(Sphere { // Ojo
            center: Vec3::new(0.0, 0.0, -5.0),
            radius: 1.2,
            color: 0xFFFFFF,
        }),
        Box::new(Square { // Cruz tornillo inferior izquierdo 1
            center: Vec3::new(-1.8, -2.2, -5.0),
            height: 0.8,
            width: 0.4,
            normal: Vec3::new(0.0, 0.0, -1.0),
            color: 0x515050,
        }),
        Box::new(Square { // Cruz tornillo inferior izquierdo 2
            center: Vec3::new(-1.8, -2.2, -5.0),
            height: 0.4,
            width: 0.8,
            normal: Vec3::new(0.0, 0.0, -1.0),
            color: 0x515050,
        }),
        Box::new(Sphere { // Tornillo inferior izquierdo
            center: Vec3::new(-4.5, -5.5, -13.0),
            radius: 1.5,
            color: 0x7C7C7C,
        }),
        Box::new(Square { // Cruz tornillo inferior derecho 1
            center: Vec3::new(1.8, -2.2, -5.0),
            height: 0.8,
            width: 0.4,
            normal: Vec3::new(0.0, 0.0, -1.0),
            color: 0x515050,
        }),
        Box::new(Square { // Cruz tornillo inferior derecho 2
            center: Vec3::new(1.8, -2.2, -5.0),
            height: 0.4,
            width: 0.8,
            normal: Vec3::new(0.0, 0.0, -1.0),
            color: 0x515050,
        }),
        Box::new(Sphere { // Tornillo inferior derecho
            center: Vec3::new(4.5, -5.5, -13.0),
            radius: 1.5,
            color: 0x7C7C7C,
        }),
        Box::new(Oval { // Chapuz tornillo superior
            center: Vec3::new(0.0, 4.3, -5.0),
            radius_x: 0.4,
            radius_y: 1.0,
            normal: Vec3::new(0.0, 0.0, -5.0),
            color: 0x334854,
        }),
        Box::new(Square { // Cruz tornillo superior 1
            center: Vec3::new(0.0, 3.23, -5.0),
            height: 0.5,
            width: 0.4,
            normal: Vec3::new(0.0, 0.0, -1.0),
            color: 0x515050,
        }),
        Box::new(Square { // Cruz tornillo superior 2
            center: Vec3::new(0.0, 4.01, -6.0),
            height: 0.3,
            width: 0.8,
            normal: Vec3::new(0.0, 0.0, -1.0),
            color: 0x515050,
        }),
        Box::new(Oval { // Tornillo superior
            center: Vec3::new(0.0, 3.0, -5.0),
            radius_x: 0.7,
            radius_y: 0.5,
            normal: Vec3::new(0.0, 0.0, -5.0),
            color: 0x7C7C7C,
        }),
        Box::new(Square { // Base de tornilo superior
            center: Vec3::new(0.0, 2.5, -5.0),
            height: 0.8,
            width: 0.8,
            normal: Vec3::new(0.0, 0.0, -1.0),
            color: 0x686767,
        }),
        Box::new(Square { // Polo azul imán derecho
            center: Vec3::new(3.6, 0.6, -5.0),
            height: 0.6,
            width: 0.8,
            normal: Vec3::new(0.0, 0.0, -1.0),
            color: 0x038FD1,
        }),
        Box::new(Square { // Polo rojo imán derecho
            center: Vec3::new(3.6, -0.6, -5.0),
            height: 0.6,
            width: 0.8,
            normal: Vec3::new(0.0, 0.0, -1.0),
            color: 0xFD3748,
        }),
        Box::new(Oval { // Chapuz imán derecho 
            center: Vec3::new(4.0, 0.0, -5.0),
            radius_x: 0.9,
            radius_y: 0.44,
            normal: Vec3::new(3.0, 0.0, -5.0),
            color: 0x334854,
        }),
        Box::new(Sphere { // Imán derecho
            center: Vec3::new(3.0, 0.0, -5.0),
            radius: 0.9,
            color: 0x73A4BF,
        }),
        Box::new(Square { // Polo azul imán izquierdo
            center: Vec3::new(-3.6, 0.6, -5.0),
            height: 0.6,
            width: 0.8,
            normal: Vec3::new(0.0, 0.0, -1.0),
            color: 0x038FD1,
        }),
        Box::new(Square { // Polo rojo imán izquierdo
            center: Vec3::new(-3.6, -0.6, -5.0),
            height: 0.6,
            width: 0.8,
            normal: Vec3::new(0.0, 0.0, -1.0),
            color: 0xFD3748,
        }),
        Box::new(Oval { // Chapuz imán izquierdo 
            center: Vec3::new(-4.0, 0.0, -5.0),
            radius_x: 1.5,
            radius_y: 0.5,
            normal: Vec3::new(3.0, 0.0, -5.0),
            color: 0x334854,
        }),
        Box::new(Sphere { // Imán izquierdo
            center: Vec3::new(-3.0, 0.0, -5.0),
            radius: 0.9,
            color: 0x73A4BF,
        }),
        Box::new(Sphere { // Cabeza
            center: Vec3::new(0.0, 0.0, -2.0),
            radius: 1.0,
            color: 0xCBE5E1,
        }),
        Box::new(Sphere { // Fondo
            center: Vec3::new(-4.5, -5.5, -13.0),
            radius: 100.5,
            color: 0x334854,
        }),
    ];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        render(&mut framebuffer, &objects);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
