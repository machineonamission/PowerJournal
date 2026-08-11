use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    rsx! {
        Link {
            to: Route::ImportersView {},
            "open importers"
        }
    }
}
