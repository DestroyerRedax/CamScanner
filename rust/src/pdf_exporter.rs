use lopdf::{Document, Object, Dictionary, Stream, dictionary};

pub fn create_pdf_from_images(image_buffers: &[Vec<u8>], output_path: &str) -> Result<(), String> {
    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let mut page_ids = Vec::new();

    for img_bytes in image_buffers {
        let stream = Stream::new(dictionary! {
            "Type" => Object::Name(b"XObject".to_vec()),
            "Subtype" => Object::Name(b"Image".to_vec()),
            "Width" => 800,
            "Height" => 1200,
            "ColorSpace" => Object::Name(b"DeviceRGB".to_vec()),
            "BitsPerComponent" => 8,
            "Filter" => Object::Name(b"DCTDecode".to_vec()),
        }, img_bytes.clone());

        let image_id = doc.add_object(stream);

        let page_dict = dictionary! {
            "Type" => Object::Name(b"Page".to_vec()),
            "Parent" => Object::Reference(pages_id),
            "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
            "Resources" => dictionary! {
                "XObject" => dictionary! {
                    "Im1" => Object::Reference(image_id),
                },
            },
        };
        page_ids.push(doc.add_object(page_dict));
    }

    let pages_dict = dictionary! {
        "Type" => Object::Name(b"Pages".to_vec()),
        "Count" => Object::Integer(page_ids.len() as i64),
        "Kids" => Object::Array(page_ids.into_iter().map(Object::Reference).collect()),
    };
    doc.objects.insert(pages_id, Object::Dictionary(pages_dict));

    let catalog_id = doc.add_object(dictionary! {
        "Type" => Object::Name(b"Catalog".to_vec()),
        "Pages" => Object::Reference(pages_id),
    });
    doc.trailer.set("Root", catalog_id);
    doc.save(output_path).map_err(|e| e.to_string())?;
    Ok(())
}