use dioxus::prelude::*;
use crate::Route;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        Link {
            to: Route::Journal { id: 1 },
            "open journal 1"
        }
    }
}
