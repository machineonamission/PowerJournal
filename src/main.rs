use dioxus::desktop::use_asset_handler;
use dioxus::document::Style;
use dioxus::prelude::*;
use dioxus_google_font_embedder::{asset_url, embed_google_font};
use sea_orm::DatabaseConnection;
use views::{Journal, Home, Navbar, TestPaginate, JournalPaginate};
use crate::blob_asset::register_blob_asset;
use crate::components::font::AHLFont;
use crate::components::icon::IconSheet;
use crate::database::init_db;

mod components;
mod views;
mod database;
mod importers;
pub mod blob_asset;

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
    // load db async and serve to children components
    let db_signal_raw = use_resource(move || async move {
        init_db().await.unwrap()
    });
    let db_signal = use_context_provider(|| db_signal_raw);
    
    register_blob_asset(db_signal);

    let dark_mode = use_signal(|| true);
    use_effect(move || {
        let value = if dark_mode() { "dark" } else { "light" };
        document::eval(&format!(
            r#"document.documentElement.setAttribute("data-bs-theme", "{value}");"#
        ));
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        Stylesheet { href: asset_url!("https://cdn.jsdelivr.net/npm/bootstrap@latest/dist/css/bootstrap.min.css") }
        AHLFont {}
        IconSheet {}
        Router::<Route> {}
    }
}
