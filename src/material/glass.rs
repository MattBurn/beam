use super::{
    Material,
    Scatter,
};
use crate::drawable::Hit;
use crate::ray::Ray;

use cgmath::Vector3;
use cgmath::prelude::*;

use rand::Rng;

pub struct Glass {
    refractive_index: f32,
    _attenuation: Vector3<f32>,
}

impl Glass {
    pub fn new(refractive_index: f32) -> Self {
        Self {
            refractive_index,
            _attenuation: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

impl Material for Glass {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let (
            outward_normal, 
            ni_over_nt, 
            cosine
        ) = if ray.direction.dot(hit.normal) > 0.0 {
            (
                -hit.normal, 
                self.refractive_index,
                self.refractive_index * ray.direction.dot(hit.normal)/ray.direction.magnitude(),
            )
        } else {
            (
                hit.normal, 
                self.refractive_index.recip(),
                -ray.direction.dot(hit.normal)/ray.direction.magnitude(),
            )
        };
        let mut rng = rand::thread_rng();
        if let Some(refracted) = super::refract(ray.direction, outward_normal, ni_over_nt) {
            let reflect_probability = super::schlick(cosine, ni_over_nt);
            if rng.gen::<f32>() > reflect_probability {
                return Some(
                    Scatter::new(
                        Ray::new(
                            hit.point,
                            refracted,
                        ),
                        &self._attenuation,
                    )
                )
            }
        }
        let reflected = super::reflect(ray.direction, hit.normal);
        Some(
            Scatter::new(
                Ray::new(
                    hit.point,
                    reflected,
                ), 
                &self._attenuation,
            )
        )
    }
}