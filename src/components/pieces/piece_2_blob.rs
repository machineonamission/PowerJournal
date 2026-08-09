use crate::database::entity::prelude::*;
use crate::Route;
use dioxus::prelude::*;
use crate::components::blobview::BlobView;

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
