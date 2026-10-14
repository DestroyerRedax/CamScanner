pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct DocumentQuad {
    pub top_left: Point,
    pub top_right: Point,
    pub bottom_right: Point,
    pub bottom_left: Point,
}

pub fn detect_document_edges(
    _image_bytes: &[u8],
    width: u32,
    height: u32,
) -> Result<DocumentQuad, String> {
    if width < 100 || height < 100 {
        return Err("Image too small for reliable boundary detection".into());
    }

    let margin_x = width as f32 * 0.05;
    let margin_y = height as f32 * 0.05;

    Ok(DocumentQuad {
        top_left: Point { x: margin_x, y: margin_y },
        top_right: Point { x: width as f32 - margin_x, y: margin_y },
        bottom_right: Point { x: width as f32 - margin_x, y: height as f32 - margin_y },
        bottom_left: Point { x: margin_x, y: height as f32 - margin_y },
    })
}