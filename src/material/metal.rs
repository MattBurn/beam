use super::{
    Material,
    Scatter,
};
use crate::ray::Ray;
use crate::drawable::Hit;
use cgmath::Vector3;
use cgmath::prelude::*;

pub struct Metal {
    albedo: Vector3<f32>,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vector3<f32>, fuzz: f32) -> Self {
        let fuzz = if fuzz < 1.0 {
            fuzz
        } else {
            1.0
        };
        Self {
            albedo,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let reflected = super::reflect(ray.direction, hit.normal);
        let scattered = Ray::new(
            hit.point,
            reflected + self.fuzz * super::random_in_unit_sphere(),
        );
        if scattered.direction.dot(hit.normal) > 0.0 {
            Some(Scatter::new(
                scattered,
                &self.albedo,
            ))
        } else {
            None
        }
    }
}
