use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    rsx! {
        h1 {"Setings"}
        Link {
            to: Route::ImportersView {},
            "open importers"
        }
    }
}
