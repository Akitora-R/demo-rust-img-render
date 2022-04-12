use std::num::NonZeroU32;
use std::time::SystemTime;
use image::{DynamicImage, GenericImageView, ImageFormat, Pixel, Rgb};
use image::imageops::Nearest;

pub fn render_img() {
    let mut image = DynamicImage::new_rgb8(1000, 2000).to_rgb8();
    let start = SystemTime::now();
    let img_vec = std::fs::read("./orig_img.jpg").unwrap();
    let i = image::load_from_memory_with_format(&img_vec, ImageFormat::Jpeg).unwrap();
    let (x, y) = i.dimensions();
    let d = 900.0 / (x as f64);
    let nx = 900 as u32;
    let ny = (d * y as f64) as u32;
    let ib = image::imageops::resize(&i, nx, ny, image::imageops::Nearest);
    let mut row = 0;
    for px in image.pixels_mut() {
        *px = Rgb([255, 255, 255]);
    }
    for (i, p) in ib.pixels().into_iter().enumerate() {
        let x = (i % 900) as u32;
        if i > 0 && x == 0 {
            row += 1;
        }
        image.put_pixel(x, row, p.to_rgb());
    }
    image.save("image_example.jpg").unwrap();
    println!("耗时 {:#?}", SystemTime::now().duration_since(start).unwrap());
}
