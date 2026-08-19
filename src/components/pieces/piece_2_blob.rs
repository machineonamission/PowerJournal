use crate::components::blobview::BlobView;
use crate::database::entity::prelude::*;
use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub fn Piece2Blob(piece: piece_2_blob::ModelEx) -> Element {
    // see blob_asset.rs
    let src = format!("/dbimage/{}", piece.id);

    rsx! {
        BlobView {
            src: src,
            mime: piece.mime_type,
            width: "300px"
        }
    }
}
