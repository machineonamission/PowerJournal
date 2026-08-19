use crate::components::icon::Icon;
use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub fn NavItem(to: Route, icon: String) -> Element {
    rsx! {
        Link {
            class: "nav-link active",
            to: to,
            Icon { "{icon}" }
        }
    }
}

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            margin: "1rem",
            Outlet::<Route> {}
        }
        nav {
            class: "nav nav-justified",
            position: "fixed",
            bottom: 0,
            left: 0,
            right: 0,

            NavItem {
                to: Route::JournalPaginate { id: None },
                icon: "home",
            }
            NavItem {
                to: Route::JournalList {},
                icon: "newsstand",
            }
            NavItem {
                to: Route::Editor {id:None},
                icon: "add",
            }
            NavItem {
                to: Route::DebugMenu {},
                icon: "bug_report",
            }
            NavItem {
                to: Route::Settings {},
                icon: "settings",
            }
        }
    }
}
