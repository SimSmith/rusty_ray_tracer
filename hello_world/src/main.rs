use std::io::prelude::*;
use std::fs::File;
use std::io::LineWriter;
use vec3::Vec3;


fn main() -> std::io::Result<()>{
    let nx = 200;
    let ny = 100;

    let file = File::create("../eye_candy/hello_world.ppm")?;
    let mut file = LineWriter::new(file);

    // Write ppm format
    write!(file, "P3\n{} {}\n255\n", nx, ny)?;
    for j in (0..ny).rev() {
        for i in 0..nx {
            let rgb = Vec3::new((i as f32) / (nx as f32), (j as f32) / (ny as f32), 0.2);
            let ir = (255.99*rgb.x) as i32;
            let ig = (255.99*rgb.y) as i32;
            let ib = (255.99*rgb.z) as i32;
            write!(file, "{} {} {}\n", ir, ig, ib)?;
        }
    }
    Ok(())
}