use crate::database::entity::prelude::*;
use crate::Route;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Piece4Activities(piece: Vec<piece_4_activities::ModelEx>) -> Element {
    let len = piece.len();
    rsx! {
        div {
            for i in piece {
                {format!("activity id {}", i.activity_id)}
            }
        }
    }
}
