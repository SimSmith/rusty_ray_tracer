mod camera;
mod hitable;
mod material;
mod ray;
mod vec3;

use camera::Camera;
use hitable::{Hitable, HitableList, Sphere};
use image::{ImageBuffer, Rgb};
use material::{Dielectric, Lambertian, Metal};
use rand::{Rng, SeedableRng, rngs::StdRng};
use ray::Ray;
use rayon::prelude::*;
use std::time::Instant;
use vec3::Real;
use vec3::Vec3;

fn main() {
    let width = 200;
    let height = 100;
    let n_samples = 100;

    let world = random_scene();

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let aperture = 0.1;
    let vertical_fov = 20.;
    let dist_to_focus = (look_from - look_at).length();
    let cam = Camera::viewport(
        look_from,
        look_at,
        Vec3::new(0., 1., 0.),
        vertical_fov,
        width as Real / height as Real,
        aperture,
        dist_to_focus,
    );

    println!("Let's go, rays!");
    let now = Instant::now();
    // Iterate over the coordinates and pixels of the image
    let pixels: Vec<Vec<Rgb<u8>>> = (0..width)
        .into_par_iter()
        .map(|x|{
            let mut rng = rand::thread_rng();
            (0..height).map(|y| {
                let mut col = Vec3::new(0., 0., 0.);
                for _ in 0..n_samples {
                    let u = (x as Real + rng.gen::<Real>()) / (width as Real);
                    let v = (y as Real + rng.gen::<Real>()) / (height as Real);
                    let r = cam.get_ray(u, v);
                    col += color(&r, &world, 0);
                }
                col /= n_samples as Real;
                col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt()); // gamma 2
                let rgb = 255.99 * col;

                Rgb([rgb.x as u8, rgb.y as u8, rgb.z as u8])
            }).collect()
        }).collect();

    // Fill image buffer and save the image
    let mut imgbuf = ImageBuffer::new(width, height);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = pixels[x as usize][y as usize];
    }
    // The format is deduced from the path
    imgbuf.save("eye_candy/ball_heaven.png").unwrap();

    println!("Time: {} ms", now.elapsed().as_millis());
}

fn color(r: &Ray, world: &Hitable, depth: usize) -> Vec3 {
    let eps = 0.001; // to get rid of shadow acne
    if let Some(rec) = world.hit(r, eps, std::f32::MAX) {
        let opt = rec.mat.scatter(r, rec.p, rec.normal);
        if opt.is_some() && depth < 50 {
            let (attenuation, scattered) = opt.unwrap();
            attenuation * color(&scattered, world, depth + 1)
        } else {
            Vec3::new(0., 0., 0.)
        }
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
    let mut noise = || rng.gen::<Real>();
    while {
        p = 2. * Vec3::new(noise(), noise(), noise()) - Vec3::new(1., 1., 1.);
        p.dot(&p) >= 1.
    } { /* Black magic; do-while loop. */ }
    p
}

pub fn random_scene() -> HitableList {
    let mut list: HitableList = Vec::new();
    list.push(Sphere::boxed(
        Vec3::new(0., -1000., 0.),
        1000.,
        Box::new(Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        }),
    ));
    let mut rng: StdRng = SeedableRng::seed_from_u64(0);
    // let mut rng = rand::thread_rng();
    let mut noise = || rng.gen::<Real>();
    for a in -11..11 {
        for b in -11..11 {
            let a = a as Real;
            let b = b as Real; 
            let choosen_mat = noise();
            let center = Vec3::new(
                a + 0.9 * noise(),
                0.2,
                b + 0.9 * noise()
            );
            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                if choosen_mat < 0.8 {  // diffuse
                    list.push(Sphere::boxed(
                        center,
                        0.2,
                        Box::new(Lambertian {
                            albedo: Vec3::new(
                                noise() * noise(),
                                noise() * noise(),
                                noise() * noise()
                            ),
                        }),
                    ));
                }
                else if choosen_mat < 0.95 {  // metal
                    list.push(Sphere::boxed(
                        center,
                        0.2,
                        Box::new(Metal::new(
                            Vec3::new(
                                0.5 * (1. + noise()),
                                0.5 * (1. + noise()),
                                0.5 * noise()
                            ),
                            0.5 * noise()
                        )),
                    ));
                }
                else {  // glass
                    list.push(Sphere::boxed(
                        center,
                        0.2,
                        Box::new(Dielectric {
                            ref_idx: 1.5,
                        }),
                    ));
                }
            }
        }
    }

    list.push(Sphere::boxed(
        Vec3::new(0., 1., 0.),
        1.0,
        Box::new(Dielectric {
            ref_idx: 1.5,
        }),
    ));
    list.push(Sphere::boxed(
        Vec3::new(-4., 1., 0.),
        1.0,
        Box::new(Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        }),
    ));
    list.push(Sphere::boxed(
        Vec3::new(4., 1., 0.),
        1.0,
        Metal::boxed( Vec3::new(0.7, 0.6, 0.5), 0. ),
    ));

    list
}
