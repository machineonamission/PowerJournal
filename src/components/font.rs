use dioxus::prelude::*;
use dioxus::document::Style;
use dioxus_google_font_embedder::{asset_url, embed_google_font};

#[component]
pub fn AHLFont() -> Element {
    rsx! {
        {embed_google_font!("https://fonts.googleapis.com/css2?family=Atkinson+Hyperlegible+Mono:ital,wght@0,200..800;1,200..800&family=Atkinson+Hyperlegible+Next:ital,wght@0,200..800;1,200..800&display=swap")}
        Style {
            r#":root {{
                --bs-font-sans-serif: "Atkinson Hyperlegible Next", system-ui, -apple-system, "Segoe UI", Roboto, "Helvetica Neue", "Noto Sans", "Liberation Sans", Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji" !important;
                --bs-font-monospace: "Atkinson Hyperlegible Mono", SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace !important;
            }}"#
        }
    }
}