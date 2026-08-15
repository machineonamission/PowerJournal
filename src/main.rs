#![feature(fn_traits)]

use crate::blob_asset::register_blob_asset;
use crate::components::font::FontStylesheets;
use crate::components::icon::IconSheet;
use crate::database::init_db;
use dioxus::desktop::use_asset_handler;
use dioxus::document::Style;
use dioxus::prelude::*;
use dioxus_google_font_embedder::{asset_url, embed_google_font};
use sea_orm::DatabaseConnection;
use views::*;

pub mod blob_asset;
pub mod blob_utils;
mod components;
mod database;
mod importers;
mod store_lenses;
pub mod text;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        JournalPaginateAll {},
        #[route("/debug")]
        DebugMenu {},
        #[route("/import")]
        ImportersView {},
        #[route("/journal/:id")]
        JournalPaginate { id: i64 },
        #[route("/journals")]
        JournalList {},
        #[route("/new")]
        NewEntry {},
        #[route("/settings")]
        Settings {},
}

const FAVICON: Asset = asset!("/assets/logo/logo.svg");

fn main() {
    dioxus::launch(App);
}

/// App is the main component of our app.
#[component]
fn App() -> Element {
    // load db async and serve to children components
    let db_signal_raw = use_resource(move || async move { init_db().await.unwrap() });
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
        FontStylesheets {}
        IconSheet {}
        Router::<Route> {}
    }
}
