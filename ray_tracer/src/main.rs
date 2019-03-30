mod ray;

extern crate image;

use ray::Ray;
use vec3::Vec3;


fn main() {
    let width = 200;
    let height = 100;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let lower_left_corner = Vec3::new(-2., -1., -1.);
    let horizontal = Vec3::new(4., 0., 0.);
    let vertical = Vec3::new(0., 2., 0.);
    let origin = Vec3::new(0., 0., 0.);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let u = (x as f32) / (width as f32);
        let v = (y as f32) / (height as f32);
        
        let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);
        let col = color(r);
        let rgb = 255.99 * col;

        *pixel = image::Rgb([
            rgb.x as u8,
            rgb.y as u8,
            rgb.z as u8,
        ]);
    }

    // Save the image, the format is deduced from the path
    imgbuf.save("../eye_candy/sky.png").unwrap();
}


fn color(r: Ray) -> Vec3 {
    let unit_direction = r.direction().unit_vec();
    let t = 0.5*(unit_direction.y + 1.);
    // return lerp
    t * Vec3::new(1., 1., 1.) + (1. - t) * Vec3::new(0.5, 0.7, 1.)
}