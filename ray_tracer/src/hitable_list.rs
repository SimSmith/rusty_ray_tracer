use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use vec3::Real;

pub type HitableList = Vec<Box<Hitable>>;

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
