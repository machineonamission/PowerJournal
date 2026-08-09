use dioxus::document::Style;
use dioxus::prelude::*;
use dioxus_google_font_embedder::embed_google_font;

#[component]
pub fn Icon(
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        span {
            class: "material-symbols",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn IconSheet() -> Element {
    rsx! {
        {embed_google_font!("https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200")}
    }
}
