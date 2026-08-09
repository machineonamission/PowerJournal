use crate::database::entity::prelude::*;
use crate::Route;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;

#[component]
pub fn Piece0Text(piece: piece_0_text::ModelEx) -> Element {
    rsx! {
        if let Some(t) = piece.title {
            h1 { {t} }
        }

        {piece.content}
    }
}
