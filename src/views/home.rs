use dioxus::prelude::*;
use crate::Route;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        h1 {
            "PowerJournal indev debug menu"
        }
        div {
            display: "flex",
            flex_direction: "column",
            gap: "1rem",
            Link {
                to: Route::JournalPaginate { id: 1 },
                "open journal 1"
            }
            Link {
                to: Route::TestPaginate {},
                "open test paginate"
            }
            Link {
                to: Route::ImportersView {},
                "open importers"
            }
            Link {
                to: Route::JournalList {},
                "open journal list"
            }
        }
    }
}
