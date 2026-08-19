use crate::database::entity::prelude::*;
use crate::route::Route;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;

#[component]
pub fn Piece3Location(piece: piece_3_location::ModelEx) -> Element {
    rsx! {
        p {"location: {piece:?}"}
    }
}
