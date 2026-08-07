use dioxus::prelude::*;
use sea_orm::DatabaseConnection;
use views::{Journal, Home, Navbar, TestPaginate, JournalPaginate};
use crate::database::init_db;

mod components;
mod views;
mod database;
mod importers;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/journal/:id")]
        JournalPaginate { id: i32 },
        #[route("/testpaginate")]
        TestPaginate {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

/// App is the main component of our app. 
#[component]
fn App() -> Element {
    // 1. Unconditionally create and provide a global DB signal
    let mut db_signal = use_context_provider(|| Signal::<Option<DatabaseConnection>>::new(None));

    // 2. Lazily load the connection asynchronously
    let _ = use_resource(move || async move {
        match init_db().await {
            Ok(db) => db_signal.set(Some(db)),
            Err(err) => eprintln!("Failed to initialize database: {err}"),
        }
    });
    // dioxus_core::spawn_forever(importers::daylio::main());
    
    rsx! {
        document::Link { rel: "icon", href: FAVICON }

        Router::<Route> {}
    }
}
