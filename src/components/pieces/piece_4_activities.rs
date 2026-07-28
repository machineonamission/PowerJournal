use crate::database::entity::prelude::*;
use crate::Route;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Piece4Activities(piece: Vec<piece_4_activities::ModelEx>) -> Element {
    // TODO: piece 4 is weird cause its many-many so
    // rsx! {
    //     {piece.content}
    // }
    rsx! {
        p {"{piece:?}"}
    }
}
