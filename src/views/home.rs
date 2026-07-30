use dioxus::prelude::*;
use crate::Route;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        Link {
            to: Route::JournalPaginate { id: 1 },
            "open journal 1"
        }
        Link {
            to: Route::TestPaginate {},
            "open test paginate"
        }
    }
}
