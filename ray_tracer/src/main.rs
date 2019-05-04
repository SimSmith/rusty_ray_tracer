mod hitable;
mod hitable_list;
mod ray;
mod sphere;
mod camera;

extern crate image;
extern crate rand;


use rand::Rng;
use camera::Camera;
use hitable::Hitable;
use hitable_list::HitableList;
use ray::Ray;
use sphere::Sphere;
use vec3::Real;
use vec3::Vec3;

fn main() {
    let width = 200;
    let height = 100;
    let n_samples = 100;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let world: HitableList = vec![
        Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)),
        Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)),
    ];
    let cam = Camera::new();
    let mut rng = rand::thread_rng();
    let mut noise = || rng.gen_range(0., 1.);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut col = Vec3::new(0., 0., 0.);
        for _ in 0..n_samples {
            let u = (x as Real + noise()) / (width as Real);
            let v = (y as Real + noise()) / (height as Real);
            let r = cam.get_ray(u, v);
            col += color(&r, &world);
        }
        col /= n_samples as Real;
        col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt()); // gamma 2
        let rgb = 255.99 * col;

        *pixel = image::Rgb([rgb.x as u8, rgb.y as u8, rgb.z as u8]);
    }

    // Save the image, the format is deduced from the path
    imgbuf.save("../eye_candy/diffuse_sphere.png").unwrap();
}

fn color(r: &Ray, world: &HitableList) -> Vec3 {
    let eps = 0.001; // to get rid of shadow acne
    if let Some(rec) = world.hit(r, eps, std::f32::MAX) {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        0.5 * color(&Ray::new(rec.p, target - rec.p), world)
    } else {
        let unit_direction = r.direction().unit_vec();
        let t = 0.5 * (unit_direction.y + 1.); // map from [-1, 1] to [0, 1]
        t * Vec3::new(0.5, 0.7, 1.) + (1. - t) * Vec3::new(1., 1., 1.) // return lerp
    }
}

/// Sample a random point in a unit radius sphere using a rejection method.
fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    let mut rng = rand::thread_rng();
    let mut noise = || rng.gen_range(0., 1.);
    while {
        p = 2. * Vec3::new(noise(), noise(), noise()) - Vec3::new(1., 1., 1.);
        p.dot(p) >= 1.
    } { /* Black magic; do-while loop. */ }
    p
}