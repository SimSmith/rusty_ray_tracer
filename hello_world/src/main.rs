use vec3::Vec3;
use std::path::Path;

extern crate image;


fn main() {
    let nx = 200; // width
    let ny = 100; // height

    let mut rbg_data = vec![0u8; (nx*ny*3) as usize];
    let mut indexer: usize = 0;

    for j in (0..ny).rev() {
        for i in 0..nx {
            let rgb = Vec3::new((i as f32) / (nx as f32), (j as f32) / (ny as f32), 0.2);
            let ir = (255.99*rgb.x) as u8;
            let ig = (255.99*rgb.y) as u8;
            let ib = (255.99*rgb.z) as u8;
            rbg_data[indexer] = ir;
            rbg_data[indexer+1] = ig;
            rbg_data[indexer+2] = ib;
            indexer += 3;
        }
    }

    let path = Path::new(r"../eye_candy/hello_world.png");
    image::save_buffer(&path, &rbg_data, nx, ny, image::RGB(8)).unwrap();
}