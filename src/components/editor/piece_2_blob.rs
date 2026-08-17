use crate::database::entity::piece;
use crate::database::entity::piece::ActiveModelExStoreExt;
use crate::store_lenses::ActiveHasOneStoreImplExt;
use dioxus::prelude::*;


#[component]
pub fn Piece2BlobEditor(mut piece: Store<piece::ActiveModelEx>) -> Element {
    let mut store = piece.piece_2_blob().model_or_default();

    rsx! {
        "TODO"
    }
}