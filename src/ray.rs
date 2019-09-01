use crate::vec3::Real;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    o: Vec3,
    d: Vec3,
}

impl Ray {
    pub fn new(o: Vec3, d: Vec3) -> Self {
        Ray { o, d: d.unit_vec() }
    }
    pub fn origin(&self) -> Vec3 {
        self.o
    }
    pub fn direction(&self) -> Vec3 {
        self.d
    }
    pub fn point_at_parameter(&self, t: Real) -> Vec3 {
        self.o + t * self.d
    }
}
