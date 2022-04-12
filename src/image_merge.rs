use std::num::NonZeroU32;
use std::time::SystemTime;
use image::{DynamicImage, GenericImageView, ImageFormat, Pixel};
use image::imageops::Nearest;

#[test]
pub fn render_img() {
    // let mut image = DynamicImage::new_rgb8(1000, 1000).to_rgb8();
    let start = SystemTime::now();
    let img_vec = std::fs::read("./orig_img.jpg").unwrap();
    let i = image::load_from_memory_with_format(&img_vec,ImageFormat::Jpeg).unwrap();
    let (x, y) = i.dimensions();
    let d = 900.0 / (x as f64);
    let nx = 900 as u32;
    let ny = (d * y as f64) as u32;
    let ib = image::imageops::resize(&i, nx, ny, image::imageops::Nearest);
    for p in ib.pixels() {
        let [r, g, b] = p.to_rgb().0;
    }
    ib.save("image_example.jpg").unwrap();
    println!("耗时 {:#?}",SystemTime::now().duration_since(start).unwrap());
}
