use image::{DynamicImage, GenericImageView, Rgba};

pub fn apply_watermark(img: &DynamicImage, watermark_text: &str) -> DynamicImage {
    let mut rgba_img = img.to_rgba8();
    let (width, height) = rgba_img.dimensions();

    let color = Rgba([150, 150, 150, 60]);
    for y_pos in (100..height).step_by(250) {
        for x_pos in (50..width).step_by(300) {
            for dx in 0..watermark_text.len().min(80) as u32 {
                if x_pos + dx < width && y_pos < height {
                    rgba_img.put_pixel(x_pos + dx, y_pos, color);
                }
            }
        }
    }
    DynamicImage::ImageRgba8(rgba_img)
}

pub fn overlay_signature(
    base_img: &DynamicImage,
    signature_img: &DynamicImage,
    x_offset: u32,
    y_offset: u32,
) -> DynamicImage {
    let mut base_rgba = base_img.to_rgba8();
    let sig_rgba = signature_img.to_rgba8();

    image::imageops::overlay(&mut base_rgba, &sig_rgba, x_offset.into(), y_offset.into());
    DynamicImage::ImageRgba8(base_rgba)
}