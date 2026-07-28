use crate::database::entity::prelude::*;
use crate::Route;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Piece1Mood(piece: piece_1_mood::ModelEx) -> Element {
    // let pleasantness = piece.pleasantness;
    // let energy = piece.energy.map_or("None".to_string(), |energy| energy.to_string());
    rsx! {
        p {"mood: {piece:?}"}
    }
}
