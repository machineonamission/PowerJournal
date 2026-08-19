use crate::database::entity::prelude::*;
use crate::route::Route;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;

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
