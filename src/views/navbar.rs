use crate::Route;
use crate::components::icon::Icon;
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
                to: Route::JournalPaginateAll {},
                icon: "home",
            }
            NavItem {
                to: Route::JournalList {},
                icon: "newsstand",
            }
            NavItem {
                to: Route::NewEntry {},
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
