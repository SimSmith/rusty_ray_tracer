use vec3::Vec3;
// For reading and opening files
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
// To use encoder.set()
use png::HasParameters;


fn main() {
    let nx = 200;
    let ny = 100;

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

    print_image(&rbg_data, nx, ny);
}


fn print_image(data: &Vec<u8>, width: u32, height: u32) {
    let path = Path::new(r"../eye_candy/hello_world.png");
    let file = File::create(path).unwrap();
    let ref mut bw = BufWriter::new(file);

    let mut encoder = png::Encoder::new(bw, width, height);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(data).unwrap(); // Save
}