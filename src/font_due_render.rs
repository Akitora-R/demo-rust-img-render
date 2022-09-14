use fontdue::layout::{
    CoordinateSystem, HorizontalAlign, Layout, LayoutSettings, TextStyle, VerticalAlign, WrapStyle,
};
use fontdue::Font;
use image::{DynamicImage, Rgb};
use once_cell::sync::Lazy;
use std::borrow::Borrow;
use std::time::SystemTime;

pub static FONT: Lazy<Font> = Lazy::new(|| {
    let font_byte = std::fs::read("./SourceHanSansCN-Light.otf").unwrap();
    Font::from_bytes(font_byte, fontdue::FontSettings::default()).unwrap()
});

#[test]
pub fn render() {
    let (metrics, bitmap) = FONT.rasterize('っ', 30.0);
    dbg!(metrics);
    let mut image = DynamicImage::new_rgb8(100, 100).to_rgb8();
    let baseline: i32 = 40;
    let mut base_y_offset: i32 = baseline + metrics.ymin;
    let mut base_x_offset: i32 = 0;
    for (i, b) in bitmap.iter().enumerate() {
        if i > 0 && i % metrics.width == 0 {
            base_y_offset += 1;
        }
        let p = *b;
        let vx = (i % metrics.width) as i32 + base_x_offset;
        let real_x = if vx <= 0 { 0 } else { vx as u32 };
        let real_y = base_y_offset as u32 - metrics.height as u32;
        image.put_pixel(real_x, real_y, Rgb([p, p, p]));
    }
    image.save("image_example.jpg").unwrap();
}

#[test]
pub fn lined_render() {
    const W: f32 = 1000.0;
    // Read the font data.
    let font_byte = std::fs::read("./SourceHanSansCN-Light.otf").unwrap();
    // Parse it into the font type.
    let font = Font::from_bytes(font_byte, fontdue::FontSettings::default()).unwrap();

    let start = SystemTime::now();

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
        max_width: Some(W),
        max_height: None,
        horizontal_align: HorizontalAlign::Left,
        vertical_align: VerticalAlign::Bottom,
        wrap_style: WrapStyle::Word,
        wrap_hard_breaks: true,
    });
    // The text that will be laid out, its size, and the index of the font in the font list to use for
    // that section of text.

    let text = "あけましておめでとうございます🐯\nケンカっぱやい虎さん描きました\n去年は全然絵が描けなかったから今年はもう少しくらいは描きたい……\n";
    layout.append(fonts, &TextStyle::new(text.repeat(10).borrow(), 35.0, 0));
    let mut image = DynamicImage::new_rgb8(W as u32, layout.height() as u32).to_rgb8();
    for lp in layout.lines().unwrap() {
        dbg!(lp);
        let baseline: i32 = -lp.baseline_y as i32;
        let mut base_x_offset: i32 = 0;
        for glyph in &layout.glyphs()[lp.glyph_start..=lp.glyph_end] {
            let (metrics, bitmap) = fonts[0].rasterize_config(glyph.key);
            let mut base_y_offset: i32 = baseline;
            base_x_offset += metrics.xmin;
            for (i, b) in bitmap.iter().enumerate() {
                if i > 0 && i % metrics.width == 0 {
                    base_y_offset += 1;
                }
                let p = *b;
                let vx = (i % metrics.width) as i32 + base_x_offset;
                let real_x = if vx <= 0 { 0 } else { vx as u32 };
                let real_y = base_y_offset as u32 - metrics.height as u32;
                image.put_pixel(real_x, real_y, Rgb([p, p, p]));
            }
            base_x_offset += metrics.width as i32;
        }
    }
    image.save("image_example.jpg").unwrap();
    println!("{:#?}", SystemTime::now().duration_since(start).unwrap());
}
