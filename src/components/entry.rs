use dioxus::prelude::*;
use crate::Route;
use crate::database::entity::prelude::*;
use chrono::{DateTime, Utc};
use crate::components::pieces::Piece;

#[component]
pub fn Entry(entry: entries::ModelEx) -> Element {
    let dt = DateTime::from_timestamp(entry.datetime, 0).unwrap();
    rsx! {
        div {
            class: "bg-gray-800 mb-2",
            if let Some(title) = entry.title {
                h2 { "{title}" }
            }
            p { "{dt}" }
            div {
                for piece in entry.pieces {
                    Piece { piece }
                }
            }
        }
    }
}