use crate::ray::Ray;
use rand::Rng;
use vec3::Real;
use vec3::Vec3;

pub trait Material {
    fn scatter(&self, r_in: &Ray, point: Vec3, normal: Vec3) -> Option<(Vec3, Ray)>;
}

// the lambertian class does not make sense
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, point: Vec3, normal: Vec3) -> Option<(Vec3, Ray)> {
        let target = point + normal + crate::random_in_unit_sphere();
        let attenuation = self.albedo;
        let scattered = Ray::new(point, target - point);
        Some((attenuation, scattered))
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * v.dot(&n) * n
}

pub struct Metal {
    albedo: Vec3,
    fuzz: Real,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: Real) -> Self {
        Metal {
            albedo,
            fuzz: fuzz.min(1.),
        }
    }

    pub fn boxed(albedo: Vec3, fuzz: Real) -> Box<Self> {
        Box::new(Metal::new(albedo, fuzz))
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, point: Vec3, normal: Vec3) -> Option<(Vec3, Ray)> {
        let reflected = reflect(r_in.direction().unit_vec(), normal);
        let attenuation = self.albedo;
        let scattered = Ray::new(
            point,
            reflected + self.fuzz * crate::random_in_unit_sphere(),
        );
        if scattered.direction().dot(&normal) > 0. {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: Real) -> Option<Vec3> {
    let uv = v.unit_vec();
    let dt = uv.dot(&n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if discriminant > 0. {
        let refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}

fn schlick(cosine: Real, ref_idx: Real) -> Real {
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 *= r0;
    r0 + (1. - r0) * (1. - cosine).powi(5)
}

pub struct Dielectric {
    pub ref_idx: Real,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, point: Vec3, normal: Vec3) -> Option<(Vec3, Ray)> {
        let reflected = reflect(r_in.direction(), normal);
        let attenuation = Vec3::new(1., 1., 1.);
        let cosine = self.ref_idx * r_in.direction().dot(&normal) / r_in.direction().length();
        let (outward_normal, ni_over_nt, cosine) = if r_in.direction().dot(&normal) > 0. {
            (-normal, self.ref_idx, cosine)
        } else {
            (normal, 1. / self.ref_idx, -cosine)
        };
        let scattered =
            if let Some(refracted) = refract(r_in.direction(), outward_normal, ni_over_nt) {
                let reflect_prob = schlick(cosine, self.ref_idx);
                if rand::thread_rng().gen::<Real>() < reflect_prob {
                    Ray::new(point, reflected)
                } else {
                    Ray::new(point, refracted)
                }
            } else {
                Ray::new(point, reflected)
            };
        Some((attenuation, scattered))
    }
}
