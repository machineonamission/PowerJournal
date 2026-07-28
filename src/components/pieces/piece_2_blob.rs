use crate::database::entity::prelude::*;
use crate::Route;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use crate::components::blobview::ImageView;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Piece2Blob(piece: piece_2_blob::ModelEx) -> Element {
    
    rsx! {
        ImageView { bytes: piece.data }
    }
}
