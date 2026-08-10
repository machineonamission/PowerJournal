use crate::Route;
use crate::components::pieces::Piece;
use crate::database::entity::prelude::*;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;

#[component]
pub fn Entry(entry: entries::ModelEx) -> Element {
    let dt = DateTime::from_timestamp(entry.datetime, 0).unwrap();
    rsx! {
        div {
            background_color: "gray",
            margin_bottom: "0.5 rem",
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
