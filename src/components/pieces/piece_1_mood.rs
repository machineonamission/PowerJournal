use crate::Route;
use crate::database::entity::prelude::*;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;

#[component]
pub fn Piece1Mood(piece: piece_1_mood::ModelEx) -> Element {
    // let pleasantness = piece.pleasantness;
    // let energy = piece.energy.map_or("None".to_string(), |energy| energy.to_string());
    rsx! {
        p {"mood: {piece:?}"}
    }
}
