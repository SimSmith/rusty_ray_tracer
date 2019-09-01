use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Real;
use crate::vec3::Vec3;

pub struct HitRecord<'a> {
    pub t: Real,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: &'a Box<dyn Material>,
}

pub trait Hitable: Sync {
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

        let oc =  self.center - r.origin();
        let s = oc.dot(&r.direction());
        let oc_squared_len = oc.dot(&oc);
        let radius_squared = self.radius * self.radius;
        
        if s < 0. && oc_squared_len > radius_squared {
            return None
        }

        let m_squared = oc_squared_len - s * s;
        if  m_squared > radius_squared {
            return None
        }

        let q = (radius_squared - m_squared).sqrt();
        let t = if oc_squared_len > radius_squared {
            s - q
        } else {
            s + q
        };

        if t < t_min || t > t_max {
            return None
        }

        let p = r.point_at_parameter(t);
        let normal = (p - self.center) / self.radius;
        Some(HitRecord {
            t,
            p,
            normal,
            mat: &self.mat,
        })
    }
}
