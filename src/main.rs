use image::{DynamicImage, Rgba, Rgb};
use rusttype::{point, Font, Scale};

fn main() {
    // Load the font
    let font_data = include_bytes!("../SourceHanSansSC-Light.ttf");
    // This only succeeds if collection consists of one font
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

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

mod tests {
    use fontdue::Font;
    use fontdue::layout::{CoordinateSystem, HorizontalAlign, Layout, LayoutSettings, TextStyle, VerticalAlign, WrapStyle};

    #[test]
    fn fontdue() {
        // Read the font data.
        let font_byte = include_bytes!("../SourceHanSansSC-Light.ttf") as &[u8];
        // Parse it into the font type.
        let font = Font::from_bytes(font_byte, fontdue::FontSettings::default()).unwrap();

        // The list of fonts that will be used during layout.
        let fonts = &[font];
        // Create a layout context. Laying out text needs some heap allocations; reusing this context
        // reduces the need to reallocate space. We inform layout of which way the Y axis points here.
        let mut layout = Layout::new(CoordinateSystem::PositiveYUp);
        // By default, layout is initialized with the default layout settings. This call is redundant, but
        // demonstrates setting the value with your custom settings.
        layout.reset(&LayoutSettings {
            // ..LayoutSettings::default()
            x: 0.0,
            y: 0.0,
            max_width: Some(10.0),
            max_height: None,
            horizontal_align: HorizontalAlign::Left,
            vertical_align: VerticalAlign::Top,
            wrap_style: WrapStyle::Word,
            wrap_hard_breaks: true,
        });
        // The text that will be laid out, its size, and the index of the font in the font list to use for
        // that section of text.
        layout.append(fonts, &TextStyle::new("Hello Hello Hello HelloHelloHelloHelloHelloHello ", 35.0, 0));
        // layout.append(fonts, &TextStyle::new("world!", 40.0, 0));
        // Prints the layout for "Hello world!"
        for glyph in layout.glyphs() {
            let (metrics,vec) = fonts[0].rasterize_config(glyph.key);
            println!("{:?}", metrics)
        }
    }
}
