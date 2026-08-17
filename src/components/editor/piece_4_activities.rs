use crate::database::entity::piece;
use crate::database::entity::piece::ActiveModelExStoreExt;
use crate::store_lenses::ActiveHasManyStoreImplExt;
use dioxus::prelude::*;


#[component]
pub fn Piece4ActivityEditor(mut piece: Store<piece::ActiveModelEx>) -> Element {
    let mut store = piece.piece_4_activity().model();

    rsx! {
        "TODO"
    }
}