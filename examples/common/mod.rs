pub mod png;

pub fn write_png(pixels: shiny::pixel_buffer::PixelBuffer, filename: &str) {
    use self::png::encode_png;
    use std::fs::File;

    let mut file = File::create(format!("sample_{}.png", filename)).unwrap();
    encode_png(pixels, &mut file);
}
