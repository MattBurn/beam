use cgmath::Vector3;
use crate::ray::Ray;

pub struct Camera {
    origin: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
    lower_left_corner: Vector3<f32>,
}

impl Camera {
    pub fn new(origin: Vector3<f32>, horizontal: Vector3<f32>, vertical: Vector3<f32>, lower_left_corner: Vector3<f32>) -> Self {
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin,
        )
    }
}