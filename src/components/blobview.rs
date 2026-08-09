use dioxus::document::Style;
use dioxus::prelude::*;
use dioxus_google_font_embedder::{asset_url, embed_google_font};

#[component]
pub fn BlobView(
    src: String,
    mime: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let top_level = mime.split('/').next().unwrap();
    rsx! {
        match top_level {
            "image" => rsx! { img { src: "{src}", ..attributes} },
            "video" => rsx! { video { src: "{src}", controls: true, ..attributes } },
            "audio" => rsx! { audio { src: "{src}", controls: true, ..attributes } },
            _ => rsx! {
                div { class: "unsupported-media", ..attributes,
                    p { "Can't preview this file ({mime})." }
                    a { href: "{src}", download: true, "Download" }
                }
            }
        }
    }
}
