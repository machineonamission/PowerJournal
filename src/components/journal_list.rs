use crate::components::pieces::Piece;
use crate::database::entity::prelude::*;
use crate::route::Route;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;

#[component]
pub fn JournalListEntry(journal: journal::ModelEx) -> Element {
    rsx! {
        Link {
            to: Route::JournalPaginate { id: Some(journal.id) },
            div {
                background_color: "gray",
                margin_bottom: "0.5 rem",
                "{journal.title}"
            }
        }
    }
}
