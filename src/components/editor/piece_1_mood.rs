use crate::database::entity::piece;
use crate::database::entity::piece::ActiveModelExStoreExt;
use crate::store_lenses::ActiveHasOneStoreImplExt;
use dioxus::prelude::*;
use sea_orm::Set;
use crate::database::entity::piece_1_mood::ActiveModelExStoreExt as OtherActiveModelExStoreExt;

#[component]
pub fn Piece1MoodEditor(mut piece: Store<piece::ActiveModelEx>) -> Element {
    let mut store = piece.piece_1_mood().model_or_default();

    store.pleasantness().write().set_if_unset_default();


    rsx! {
        input {
            type: "range",
            class: "form-range",
            min: -1,
            max: 1,
            step: 0.0001,
            value: store.pleasantness()().unwrap(),
            oninput: move |e| {
                let val = e.value();
                *store.pleasantness().write() = Set(val.parse::<f64>().unwrap_or_default());
            }
        }
    }
}