use crate::ray::Ray;
use vec3::Real;
use vec3::Vec3;

pub struct Camera {
    upper_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            upper_left_corner: Vec3::new(-2., 1., -1.),
            horizontal: Vec3::new(4., 0., 0.),
            vertical: Vec3::new(0., -2., 0.),
            origin: Vec3::new(0., 0., 0.),
        }
    }

    pub fn get_ray(&self, u: Real, v: Real) -> Ray {
        Ray::new(
            self.origin,
            self.upper_left_corner + u * self.horizontal + v * self.vertical,
        )
    }
}
