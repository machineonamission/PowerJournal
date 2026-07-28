use crate::database::entity::prelude::*;
use crate::Route;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Piece0Text(piece: piece_0_text::ModelEx) -> Element {
    rsx! {
        if let Some(t) = piece.title {
            h1 { {t} }
        }

        {piece.content}
    }
}
