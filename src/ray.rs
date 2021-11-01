use crate::vec3;

pub struct Ray {
    pub A: vec3::Vec3,
    pub B: vec3::Vec3,
}

impl Ray {
    pub fn direction(&self) -> vec3::Vec3 { self.B }
    pub fn origin(&self) -> vec3::Vec3 { self.A }

    pub fn point_at_parameter(&self, t: f64) -> vec3::Vec3 { self.A + t * self.B }
}