mod hitable;
mod hitable_list;
mod ray;
mod sphere;

extern crate image;

use hitable::Hitable;
use hitable_list::HitableList;
use ray::Ray;
use sphere::Sphere;
use vec3::Real;
use vec3::Vec3;

fn main() {
    let width = 200;
    let height = 100;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let upper_left_corner = Vec3::new(-2., 1., -1.);
    let horizontal = Vec3::new(4., 0., 0.);
    let vertical = Vec3::new(0., -2., 0.);
    let origin = Vec3::new(0., 0., 0.);
    let world: HitableList = vec![
        Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)),
        Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)),
    ];

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let u = (x as Real) / (width as Real);
        let v = (y as Real) / (height as Real);

        let r = Ray::new(origin, upper_left_corner + u * horizontal + v * vertical);
        let col = color(&r, &world);
        let rgb = 255.99 * col;

        *pixel = image::Rgb([rgb.x as u8, rgb.y as u8, rgb.z as u8]);
    }

    // Save the image, the format is deduced from the path
    imgbuf.save("../eye_candy/normal_sphere.png").unwrap();
}

fn color(r: &Ray, world: &HitableList) -> Vec3 {
    if let Some(rec) = world.hit(r, 0., std::f32::MAX) {
        0.5 * (rec.normal + Vec3::new(1., 1., 1.)) // map from [-1, 1] to [0, 1] for x,y,z
    } else {
        let unit_direction = r.direction().unit_vec();
        let t = 0.5 * (unit_direction.y + 1.); // map from [-1, 1] to [0, 1]
                                               // return lerp
        t * Vec3::new(0.5, 0.7, 1.) + (1. - t) * Vec3::new(1., 1., 1.)
    }
}
