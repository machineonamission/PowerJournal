use crate::database::entity::piece;
use crate::database::entity::piece::ActiveModelExStoreExt;
use crate::store_lenses::ActiveHasOneStoreImplExt;
use dioxus::prelude::*;
use sea_orm::Set;

#[component]
pub fn Piece0TextEditor(mut piece: Store<piece::ActiveModelEx>) -> Element {
    // 1. Get the store for the HasOne field (assuming the macro generates `piece_0_text()`)
    let mut store = piece.piece_0_text().model_or_default();

    rsx! {
        input {
            type: "text",
            class: "form-control",
            oninput: move |e| {
                let val = e.value();
                store.write().title = Set(Some(val));
            }
        }
        textarea {
            class: "form-control",
            oninput: move |e| {
                let val = e.value();
                store.write().content = Set(val);
            }
        }
    }
}