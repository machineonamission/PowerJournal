use crate::Route;
use crate::components::blobview::BlobView;
use crate::database::entity::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Piece2Blob(id: i64, mime: String) -> Element {
    // see blob_asset.rs
    let src = format!("/dbimage/{id}");

    rsx! {
        BlobView {
            src: src,
            mime: mime,
            width: "300px"
        }
    }
}
