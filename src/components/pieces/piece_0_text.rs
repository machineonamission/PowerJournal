use crate::Route;
use crate::database::entity::prelude::*;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use crate::components::markdown::Markdown;

#[component]
pub fn Piece0Text(piece: piece_0_text::ModelEx) -> Element {
    rsx! {
        if let Some(t) = piece.title {
            h1 { {t} }
        }
        div {
            Markdown {
                md: piece.content
            }
        }
    }
}
