use cgmath::Vector3;
use crate::ray::Ray;
use crate::material::Material;

pub trait Drawable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

pub struct Hit<'a> {
    pub t: f32,
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>, 
    pub material: &'a dyn Material,
}

impl<'a> Hit<'a> {
    pub fn new(t: f32, point: Vector3<f32>, normal: Vector3<f32>, material: &'a dyn Material) -> Self {
        Self {
            t,
            point,
            normal,
            material,
        }
    }
}

pub struct Drawables<'a> {
    _drawables: Vec<&'a dyn Drawable>,
}

impl<'a> Drawables<'a> {
    pub fn new() -> Self {
        Self {
            _drawables: Vec::new(),
        }
    }

    pub fn new_from(drawables: Vec<&'a dyn Drawable>) -> Self {
        Self {
            _drawables: drawables,
        }
    }
}

impl<'a> Drawable for Drawables<'a> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut closest_so_far = t_max;
        let mut hit = None;
        for drawable in self.iter() {
            if let Some(h) = drawable.hit(ray, t_min, closest_so_far) {
                closest_so_far = h.t;
                hit = Some(h)
            }
        }
        hit
    }
}

impl<'a> std::ops::Deref for Drawables<'a> {
    type Target = Vec<&'a dyn Drawable>;

    fn deref(&self) -> &Self::Target {
        &self._drawables
    }
}

impl<'a> std::ops::DerefMut for Drawables<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._drawables
    }
}
