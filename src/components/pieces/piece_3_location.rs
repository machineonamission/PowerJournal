use crate::database::entity::prelude::*;
use crate::Route;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Piece3Location(piece: piece_3_location::ModelEx) -> Element {
    rsx! {
        p {"location: {piece:?}"}
    }
}
