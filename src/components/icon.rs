use dioxus::document::Style;
use dioxus::prelude::*;
use dioxus_google_font_embedder::embed_google_font;

/// https://fonts.google.com/icons?icon.style=Rounded
#[component]
pub fn Icon(
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        span {
            class: "material-symbols-rounded",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn IconSheet() -> Element {
    rsx! {
        {
            embed_google_font!("https://fonts.googleapis.com/css2?family=Material+Symbols+Rounded:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200")
        }
        style {
            r#"
                .material-symbols-rounded {{
                  font-variation-settings: 'FILL' 1, 'wght' 400;
                  font-optical-sizing: auto;
                }}
            "#
        }
    }
}
