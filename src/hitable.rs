use crate::material::Material;
use crate::ray::Ray;
use vec3::Real;
use vec3::Vec3;

pub struct HitRecord<'a> {
    pub t: Real,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: &'a Box<Material>,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: Real, t_max: Real) -> Option<HitRecord>;
}

pub type HitableList = Vec<Box<dyn Hitable>>;

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: Real, t_max: Real) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut rec_opt = None;
        for item in self.iter() {
            if let Some(rec) = item.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                rec_opt = Some(rec);
            }
        }
        rec_opt
    }
}

pub struct Sphere {
    center: Vec3,
    radius: Real,
    mat: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: Real, mat: Box<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            mat,
        }
    }

    pub fn boxed(center: Vec3, radius: Real, mat: Box<dyn Material>) -> Box<Self> {
        Box::new(Sphere::new(center, radius, mat))
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: Real, t_max: Real) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(&r.direction());
        let b = oc.dot(&r.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0. {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord {
                    t,
                    p,
                    normal,
                    mat: &self.mat,
                });
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord {
                    t,
                    p,
                    normal,
                    mat: &self.mat,
                });
            }
        }
        None
    }
}
