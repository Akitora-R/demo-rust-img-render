use std::borrow::Borrow;
use fontdue::Font;
use fontdue::layout::{CoordinateSystem, HorizontalAlign, Layout, LayoutSettings, TextStyle, VerticalAlign, WrapStyle};
use image::{DynamicImage, Rgb};

pub fn render() {
    // Read the font data.
    let font_byte = include_bytes!("../SourceHanSansCN-Light.otf") as &[u8];
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

    layout.append(fonts, &TextStyle::new("A".repeat(1).borrow(), 35.0, 0));
    // layout.append(fonts, &TextStyle::new("world!", 40.0, 0));
    // Prints the layout for "Hello world!"
    let mut image = DynamicImage::new_rgb8(100, 100).to_rgb8();
    for glyph in layout.glyphs() {
        let (metrics, bitmap) = fonts[0].rasterize_config(glyph.key);
        // Output
        dbg!(metrics);
        let mut row: u32 = 0;
        for (i, b) in bitmap.iter().enumerate() {
            if i > 0 && i % metrics.width == 0 {
                row += 1;
            }
            let p = *b;
            image.put_pixel((i % metrics.width) as u32, row, Rgb([p, p, p]))
        }
    }
    image.save("image_example.jpg").unwrap();
}
