use dioxus::prelude::*;
use base64::{engine::general_purpose, Engine as _};

#[component]
pub fn ImageView(bytes: Vec<u8>) -> Element {
    let encoded = general_purpose::STANDARD.encode(&bytes);
    let data_url = format!("data:image/heic;base64,{encoded}");

    rsx! {
        img { src: "{data_url}" }
    }
}