use std::borrow::Borrow;
use fontdue::Font;
use fontdue::layout::{CoordinateSystem, HorizontalAlign, Layout, LayoutSettings, TextStyle, VerticalAlign, WrapStyle};
use image::{DynamicImage, Rgb};

pub fn render() {
    // Read the font data.
    let font_byte = std::fs::read("./SourceHanSansCN-Light.otf").unwrap();
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
        max_width: Some(80.0),
        max_height: None,
        horizontal_align: HorizontalAlign::Left,
        vertical_align: VerticalAlign::Top,
        wrap_style: WrapStyle::Word,
        wrap_hard_breaks: true,
    });
    // The text that will be laid out, its size, and the index of the font in the font list to use for
    // that section of text.

    layout.append(fonts, &TextStyle::new("水木清华".repeat(1).borrow(), 35.0, 0));
    let baseline: i32 = 40;
    // layout.append(fonts, &TextStyle::new("world!", 40.0, 0));
    let mut image = DynamicImage::new_rgb8(200, 100).to_rgb8();
    let mut base_x_offset: i32 = 0;
    for glyph in layout.glyphs() {
        let (metrics, bitmap) = fonts[0].rasterize_config(glyph.key);
        let mut base_y_offset: i32 = baseline + metrics.ymin;
        base_x_offset += metrics.xmin;
        for (i, b) in bitmap.iter().enumerate() {
            if i > 0 && i % metrics.width == 0 {
                base_y_offset += 1;
            }
            let p = *b;
            let vx = (i % metrics.width) as i32 + base_x_offset;
            let real_x = if vx <= 0 { 0 } else { vx as u32 };
            let real_y = base_y_offset as u32 - metrics.advance_height as u32;
            image.put_pixel(real_x, real_y, Rgb([p, p, p]));
        }
        base_x_offset += metrics.width as i32;
    }
    image.save("image_example.jpg").unwrap();
}

pub fn lined_render() {
    // Read the font data.
    let font_byte = std::fs::read("./SourceHanSansCN-Light.otf").unwrap();
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
        max_width: Some(200.0),
        max_height: None,
        horizontal_align: HorizontalAlign::Left,
        vertical_align: VerticalAlign::Top,
        wrap_style: WrapStyle::Word,
        wrap_hard_breaks: true,
    });
    // The text that will be laid out, its size, and the index of the font in the font list to use for
    // that section of text.

    let text = "仁义道德";
    layout.append(fonts, &TextStyle::new(text.repeat(20).borrow(), 35.0, 0));
    let mut image = DynamicImage::new_rgb8(200, layout.height() as u32).to_rgb8();
    for lp in layout.lines().unwrap() {
        dbg!(lp);
        let baseline: i32 = -lp.baseline_y as i32;
        let mut base_x_offset: i32 = 0;
        for glyph in &layout.glyphs()[lp.glyph_start..=lp.glyph_end] {
            let (metrics, bitmap) = fonts[0].rasterize_config(glyph.key);
            let mut base_y_offset: i32 = baseline + metrics.ymin;
            base_x_offset += metrics.xmin;
            for (i, b) in bitmap.iter().enumerate() {
                if i > 0 && i % metrics.width == 0 {
                    base_y_offset += 1;
                }
                let p = *b;
                let vx = (i % metrics.width) as i32 + base_x_offset;
                let real_x = if vx <= 0 { 0 } else { vx as u32 };
                let real_y = base_y_offset as u32 - metrics.advance_height as u32;
                dbg!(base_y_offset);
                image.put_pixel(real_x, real_y, Rgb([p, p, p]));
            }
            base_x_offset += metrics.width as i32;
        }
    }
    image.save("image_example.jpg").unwrap();
}
