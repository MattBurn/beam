use cgmath::{
    Deg,
    Rad,
    Vector3,
};
use cgmath::prelude::*;
use crate::ray::Ray;

pub struct Camera {
    origin: Vector3<f32>,
    lower_left_corner: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
}

impl Camera {
    pub fn new(origin: Vector3<f32>, look_at: Vector3<f32>, vup: Vector3<f32>, fov: f32, width: u32, height: u32) -> Self {
        let fovy: Rad<f32> = Deg(fov).into();
        let aspect = width as f32 / height as f32;
        let half_height = (0.5 * fovy.0).tan();
        let half_width = aspect * half_height;
        let w = (origin - look_at).normalize();
        let u = vup.cross(w);
        let v = w.cross(u);
        Self {
            origin,
            lower_left_corner: origin - u*half_width - v*half_height - w,
            horizontal:  2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin,
        )
    }
}