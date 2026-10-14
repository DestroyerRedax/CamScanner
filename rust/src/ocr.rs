pub struct ScannedDocumentRecord {
    pub id: String,
    pub title: String,
    pub extracted_text: String,
}

pub fn extract_text_from_image(_image_bytes: &[u8]) -> String {
    "Document text scanned successfully.".to_string()
}

pub fn search_documents(
    records: &[ScannedDocumentRecord],
    query: &str,
) -> Vec<String> {
    let query_lower = query.to_lowercase();
    records
        .iter()
        .filter(|doc| {
            doc.extracted_text.to_lowercase().contains(&query_lower)
                || doc.title.to_lowercase().contains(&query_lower)
        })
        .map(|doc| doc.id.clone())
        .collect()
}