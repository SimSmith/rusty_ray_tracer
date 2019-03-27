extern crate image;

use vec3::Vec3;


fn main() {
    let nx = 200; // width
    let ny = 100; // height

    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let rgb = Vec3::new((x as f32) / (nx as f32), (y as f32) / (ny as f32), 0.2);
        let r = (255.99*rgb.x) as u8;
        let g = (255.99*rgb.y) as u8;
        let b = (255.99*rgb.z) as u8;
        *pixel = image::Rgb([r, g, b]);
    }

    // Flip the image on its y-axis, because I can
    let flipped = image::imageops::flip_vertical(&imgbuf);

    // Save the image, the format is deduced from the path
    flipped.save("../eye_candy/hello_world.png").unwrap();
}