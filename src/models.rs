struct TextPiece {
    title: Option<String>,
    content: String,
}
struct MoodPiece {
    pleasantness: i16,
    energy: i16,
}
struct BlobPiece {
    data: Vec<u8>,
}
struct Entry {
    id: i32,
    title: String,
    content: String,
}