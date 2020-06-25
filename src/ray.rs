pub struct Ray {
    pub origin: cgmath::Vector3<f32>,
    pub direction: cgmath::Vector3<f32>,
}

impl Ray {
    pub fn new(origin: cgmath::Vector3<f32>, direction: cgmath::Vector3<f32>) -> Self {
        Self {
            origin,
            direction
        }
    }

    pub fn point_at_parameter(&self, t: f32) -> cgmath::Vector3<f32> {
        self.origin + t * self.direction
    }
}