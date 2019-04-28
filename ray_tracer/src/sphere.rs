use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use vec3::Real;
use vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: Real,
}

impl Sphere {
    pub fn new(center: Vec3, radius: Real) -> Self {
        Sphere { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: Real, t_max: Real) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0. {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord { t, p, normal });
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord { t, p, normal });
            }
        }
        None
    }
}
