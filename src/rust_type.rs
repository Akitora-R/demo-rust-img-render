use std::borrow::Borrow;
use std::fs;
use image::{DynamicImage, Rgb};
use rusttype::{Font, point, Scale};

pub fn render(){
    // Load the font
    // let font_data = include_bytes!("../SourceHanSansSC-Light.ttf");
    let font_data = fs::read("./SourceHanSansSC-Light.ttf").unwrap();
    // This only succeeds if collection consists of one font
    let font = Font::try_from_bytes(font_data.borrow()).expect("Error constructing Font");

    // The font size to use
    let scale = Scale::uniform(64.0);

    // The text to render
    let text = "This is RustType rendered into a jpg!";

    let colour = (255, 255, 255);

    let v_metrics = font.v_metrics(scale);

    // layout the glyphs in a line with 20 pixels padding
    let glyphs: Vec<_> = font
        .layout(text, scale, point(20.0, 20.0 + v_metrics.ascent))
        .collect();

    // work out the layout size
    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.x)
            .unwrap();
        let max_x = glyphs
            .last()
            .map(|g| g.pixel_bounding_box().unwrap().max.x)
            .unwrap();
        (max_x - min_x) as u32
    };

    // Create a new rgba image with some padding
    let mut image = DynamicImage::new_rgb8(glyphs_width + 40, glyphs_height + 40).to_rgb8();

    // Loop through the glyphs in the text, positing each one on a line
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            // Draw the glyph into the image per-pixel by using the draw closure
            glyph.draw(|x, y, v| {
                println!("{:?} {:?} {:?}", x, y, v);
                image.put_pixel(
                    // Offset the position by the glyph bounding box
                    x + bounding_box.min.x as u32,
                    y + bounding_box.min.y as u32,
                    // Turn the coverage into an alpha value
                    Rgb([(colour.0 as f32 * v) as u8, (colour.1 as f32 * v) as u8, (colour.2 as f32 * v) as u8]),
                )
            });
        }
    }

    // Save the image to a png file
    image.save("image_example.jpg").unwrap();
    println!("Generated: image_example.jpg");
}
