use crate::database::entity::prelude::*;
use crate::Route;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Piece2Blob(id: i64) -> Element {
    
    rsx! {
        img { src: "/dbimage/{id}" }
    }
}
