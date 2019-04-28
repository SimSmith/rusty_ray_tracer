use crate::ray::Ray;
use vec3::Real;
use vec3::Vec3;

pub struct HitRecord {
    pub t: Real,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: Real, t_max: Real) -> Option<HitRecord>;
}
