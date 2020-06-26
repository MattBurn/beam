use cgmath::Vector3;
use super::{
    Scatter,
    Material,
};
use crate::ray::Ray;
use crate::drawable::{
    Hit,
};

pub struct Diffuse {
    albedo: Vector3<f32>,
}

impl Diffuse {
    pub fn new(albedo: Vector3<f32>) -> Self {
        Self {
            albedo,
        }
    }
}

impl Material for Diffuse {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let target = hit.point + hit.normal + super::random_in_unit_sphere();
        Some(Scatter::new(
            Ray::new(
                hit.point,
                target - hit.point,
            ),
            &self.albedo,
        ))
    }
}