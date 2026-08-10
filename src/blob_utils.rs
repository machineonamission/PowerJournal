pub fn infer_mime_type(bytes: &Vec<u8>) -> &'static str {
    infer::get(bytes)
        .map(|kind| kind.mime_type())
        .unwrap_or("application/octet-stream")
}
