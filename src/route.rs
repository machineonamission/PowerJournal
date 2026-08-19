use dioxus::prelude::*;
use crate::views::*;


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
        #[route("/debug")]
        DebugMenu {},
        #[route("/import")]
        ImportersView {},
        #[redirect("/", || Route::JournalPaginate { id: None })]
        #[route("/journal?:id")]
        JournalPaginate { id: Option<i64> },
        #[route("/journals")]
        JournalList {},
        #[redirect("/new", || Route::Editor { id: None })]
        #[route("/edit?:id")]
        Editor {id:Option<i64>},
        #[route("/settings")]
        Settings {},
}