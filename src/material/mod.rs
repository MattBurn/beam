pub mod diffuse;
pub mod metal;

use crate::drawable::Hit;
use crate::ray::Ray;
use cgmath::Vector3;
use cgmath::prelude::*;

pub use diffuse::Diffuse;
pub use metal::Metal;

pub struct Scatter<'a> {
    pub ray: Ray,
    pub attenuation: &'a Vector3<f32>,
}

impl<'a> Scatter<'a> {
    pub fn new(ray: Ray, attenuation: &'a Vector3<f32>) -> Self {
        Self {
            ray,
            attenuation,
        }
    }
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter>;
}

fn reflect(v: Vector3<f32>, n: Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * v.dot(n) * n
}

fn random_in_unit_sphere() -> Vector3<f32> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut point;
    loop {
        let rx = rng.gen::<f32>();
        let ry = rng.gen::<f32>();
        let rz = rng.gen::<f32>();
        point = 2.0 * Vector3::new(rx, ry, rz) - Vector3::new(1.0, 1.0, 1.0);

        if point.magnitude2() >= 1.0 { break }
    }
    point
}