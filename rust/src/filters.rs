use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, Rgb};

pub fn apply_bw_threshold(img: &DynamicImage, threshold: u8) -> DynamicImage {
    let grayscale = img.to_luma8();
    let mut binarized = ImageBuffer::new(grayscale.width(), grayscale.height());

    for (x, y, pixel) in grayscale.enumerate_pixels() {
        let val = if pixel[0] > threshold { 255 } else { 0 };
        binarized.put_pixel(x, y, Luma([val]));
    }
    DynamicImage::ImageLuma8(binarized)
}

pub fn apply_magic_color(img: &DynamicImage) -> DynamicImage {
    let rgb = img.to_rgb8();
    let mut enhanced = ImageBuffer::new(rgb.width(), rgb.height());

    for (x, y, pixel) in rgb.enumerate_pixels() {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;

        let enhance = |val: f32| -> u8 {
            let stretched = ((val - 30.0) / 200.0 * 255.0).clamp(0.0, 255.0);
            (stretched * 1.1).clamp(0.0, 255.0) as u8
        };

        enhanced.put_pixel(x, y, Rgb([enhance(r), enhance(g), enhance(b)]));
    }
    DynamicImage::ImageRgb8(enhanced)
}

pub fn apply_grayscale(img: &DynamicImage) -> DynamicImage {
    DynamicImage::ImageLuma8(img.to_luma8())
}

pub fn apply_shadow_removal(img: &DynamicImage) -> DynamicImage {
    apply_magic_color(img)
}