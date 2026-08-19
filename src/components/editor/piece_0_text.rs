use crate::database::entity::piece;
use crate::database::entity::piece::ActiveModelExStoreExt;
use crate::store_lenses::ActiveHasOneStoreImplExt;
use dioxus::prelude::*;
use sea_orm::Set;
use crate::database::entity::piece_0_text::ActiveModelExStoreExt as OtherActiveModelExStoreExt;

#[component]
pub fn Piece0TextEditor(mut piece: Store<piece::ActiveModelEx>) -> Element {
    // 1. Get the store for the HasOne field (assuming the macro generates `piece_0_text()`)
    let mut store = piece.piece_0_text().model_or_default();
    // store.title().write().set_if_unset_default();
    store.content().write().set_if_unset_default();
    store.title().write().set_if_unset_default();

    rsx! {
        input {
            type: "text",
            class: "form-control",
            value: store.title()().unwrap(),
            oninput: move |e| {
                let val = e.value();
                *store.title().write() = Set(Some(val));
            }
        }
        textarea {
            class: "form-control",
            value: store.content()().unwrap(),
            oninput: move |e| {
                let val = e.value();
                *store.content().write() = Set(val);
            }
        }
    }
}