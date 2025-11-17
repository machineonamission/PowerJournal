use std::collections::HashMap;

#[derive(Default)]
pub struct TextPiece {
    title: String,
    content: String,
}
#[derive(Default)]
pub struct MoodPiece {
    pleasantness: f64,
    energy: f64,
}
#[derive(Default)]
pub struct BlobPiece {
    data: Vec<u8>,
}
#[derive(Default)]
pub struct LocationPiece {
    name: String,
    latitude: f64,
    longitude: f64,
}
#[derive(Default)]
pub struct ActivityPiece {
    values: HashMap<Activity, i32>,
}
#[derive(Default)]
enum PieceContent {
    #[default]
    Unknown,
    Text(TextPiece),
    Mood(MoodPiece),
    Blob(BlobPiece),
    Location(LocationPiece),
    Activity(ActivityPiece),
}

#[derive(Default)]
enum ID {
    Exists(i32),
    ToBeUpdated(i32),
    ToBeDeleted(i32),
    #[default]
    ToBeCreated,
}

#[derive(Default)]
pub struct Piece {
    id: ID,
    content: PieceContent,
}

#[derive(Default)]
pub struct Activity {
    id: ID,
    name: String,
    emoji: String,
}

#[derive(Default)]
pub struct Entry {
    id: ID,
    title: String,
    pieces: Vec<Piece>,
}

impl Entry {
    fn commit(&self) {
        let statement = String::new();
    }
}