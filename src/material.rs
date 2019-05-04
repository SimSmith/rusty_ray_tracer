use crate::ray::Ray;
use vec3::Real;
use vec3::Vec3;

pub trait Material {
    fn scatter(&self, r_in: Ray, point: Vec3, normal: Vec3) -> Option<(Vec3, Ray)>;
}

// the lambertian class does not make sense
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: Ray, point: Vec3, normal: Vec3) -> Option<(Vec3, Ray)> {
        let target = point + normal + crate::random_in_unit_sphere();
        let attenuation = self.albedo;
        let scattered = Ray::new(point, target - point);
        Some((attenuation, scattered))
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * v.dot(n) * n
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: Real,
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, point: Vec3, normal: Vec3) -> Option<(Vec3, Ray)> {
        let reflected = reflect(r_in.direction().unit_vec(), normal);
        let attenuation = self.albedo;
        let scattered = Ray::new(
            point,
            reflected + self.fuzz * crate::random_in_unit_sphere(),
        );
        if scattered.direction().dot(normal) > 0. {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
