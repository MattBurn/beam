pub mod diffuse;
pub mod metal;
pub mod glass;

use crate::drawable::Hit;
use crate::ray::Ray;
use cgmath::Vector3;
use cgmath::prelude::*;

pub use diffuse::Diffuse;
pub use metal::Metal;
pub use glass::Glass;

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

fn refract(v: Vector3<f32>, n: Vector3<f32>, ni_over_nt: f32) -> Option<Vector3<f32>> {
    let uv = v.normalize();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt*ni_over_nt*(1.0 - dt*dt);
    if discriminant > 0.0 {
        Some(
            ni_over_nt*(uv - n*dt) - n*discriminant.sqrt()
        )
    } else {
        None
    }
}

#[inline(always)]
fn schlick(cosine: f32, refraction_index: f32) -> f32 {
    let r0 = 2.0 * (1.0-refraction_index)/(1.0+refraction_index);
    r0 + (1.0 - r0)*(1.0 - cosine).powi(5)
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