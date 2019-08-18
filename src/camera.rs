use crate::ray::Ray;
use crate::vec3::Real;
use crate::vec3::Vec3;
use rand::Rng;
use std::f32::consts;

#[derive(Debug)]
pub struct Camera {
    upper_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    lens_radius: Real,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {

    pub fn viewport(
        look_from: Vec3,
        look_at: Vec3,
        v_up: Vec3,
        v_fov: Real,
        aspect: Real,
        aperture: Real,
        focus_dist: Real
    ) -> Self {
        let lens_radius = aperture / 2.0;

        let theta = v_fov * consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).unit_vec();
        let u = v_up.cross(&w).unit_vec();
        let v = w.cross(&u);

        let origin = look_from;
        let upper_left_corner = origin - half_width * focus_dist * u + half_height * focus_dist * v - focus_dist * w;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = -2.0 * half_height * focus_dist * v;

        Camera {
            upper_left_corner,
            horizontal,
            vertical,
            origin,
            lens_radius,
            u, v, w,
        }
    }

    pub fn get_ray(&self, s: Real, t: Real) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.upper_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut p: Vec3;
    let mut rng = rand::thread_rng();
    let mut noise = || rng.gen::<Real>();
    while {
        p = 2. * Vec3::new(noise(), noise(), 0.) - Vec3::new(1., 1., 0.);
        p.dot(&p) >= 1.
    } { /* Black magic; do-while loop. */ }
    p
}
