use crate::ray::Ray;
use vec3::Real;
use vec3::Vec3;

#[derive(Debug)]
pub struct Camera {
    upper_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn default() -> Self {
        Camera {
            upper_left_corner: Vec3::new(-2., 1., -1.),
            horizontal: Vec3::new(4., 0., 0.),
            vertical: Vec3::new(0., -2., 0.),
            origin: Vec3::new(0., 0., 0.),
        }
    }

    pub fn viewport(look_from: Vec3, look_at: Vec3, v_up: Vec3, v_fov: Real, aspect: Real) -> Self {
        let theta = v_fov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).unit_vec();
        let u = v_up.cross(w).unit_vec();
        let v = w.cross(u);

        Camera {
            upper_left_corner: look_from - half_width * u + half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: -2.0 * half_height * v,
            origin: look_from,
        }
    }

    pub fn get_ray(&self, u: Real, v: Real) -> Ray {
        Ray::new(
            self.origin,
            self.upper_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
