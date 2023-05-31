use image::{Rgb, RgbImage};
use rusttype::{Font, Scale};
use std::path::Path;
mod klasseboks;

fn main() {
    let padding: u32 = 20;  // padding rundt bildet
    let save_path: &Path = Path::new("./bilde.png");
    let font_size: f32 = 48.0;

    let height: u32 = 1000;
    let width: u32 = 1000;

    let mut image = RgbImage::new(width, height);

    let jonas_klasse = klasseboks::KlasseBoks::new("Jonas".to_string(), vec!["main()".to_string()]);
    jonas_klasse.draw_box(&mut image, [10, 10]);

    image.save(save_path).unwrap();
}

fn render_tekst(bilde: &mut RgbImage, tekst: String, storrelse: f32, posisjon: [f32;2]) {
    let scale = Scale::uniform(storrelse);
    let font_data: &[u8] = include_bytes!("../../../.fonts/Roboto-Regular.ttf");
    let font: Font<'static> = match Font::try_from_bytes(font_data) {
        None => return,
        Some(jonas) => jonas
    };

    for glyph in font.layout(&tekst, scale, rusttype::point(posisjon[0], posisjon[1])) {
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, _| {
                let x = x as i32 + bb.min.x;
                let y = y as i32 + bb.min.y;
                let pixel = bilde.get_pixel_mut(x as u32, y as u32);
                *pixel = Rgb([255, 255, 255]);
            });
        }
    }
}
