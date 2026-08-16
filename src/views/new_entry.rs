use crate::Route;
use crate::components::icon::Icon;
use crate::database::entity::entries::ActiveModelExStoreExt;
use crate::database::entity::piece::{ActiveModelExStoreExt as OtherActiveModelExStoreExt, Entity};
use crate::database::entity::prelude::*;
use crate::store_lenses::{ActiveHasManyStoreImplExt, ActiveHasOneStoreImplExt};
use dioxus::prelude::*;
use sea_orm::{DatabaseConnection, EntityTrait, Set};

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
#[component]
pub fn Piece1MoodEditor(mut piece: Store<piece::ActiveModelEx>) -> Element {
    let mut store = piece.piece_1_mood().model_or_default();

    rsx! {
        input {
            type: "range",
            class: "form-range",
            min: -1,
            max: 1,
            step: 0.0001,
            oninput: move |e| {
                let val = e.value();
                store.write().pleasantness = Set(val.parse::<f64>().unwrap_or_default());
            }
        }
    }
}


#[component]
pub fn Piece2BlobEditor(mut piece: Store<piece::ActiveModelEx>) -> Element {
    let mut store = piece.piece_2_blob().model_or_default();

    rsx! {
        "TODO"
    }
}
#[component]
pub fn Piece3LocationEditor(mut piece: Store<piece::ActiveModelEx>) -> Element {
    let mut store = piece.piece_3_location().model_or_default();

    rsx! {
        "TODO"
    }
}
#[component]
pub fn Piece4ActivityEditor(mut piece: Store<piece::ActiveModelEx>) -> Element {
    let mut store = piece.piece_4_activity().model();

    rsx! {
        "TODO"
    }
}

#[component]
pub fn PieceEditor(mut piece: Store<piece::ActiveModelEx>) -> Element {
    rsx! {
        match piece().piece_type.unwrap() {
            0 => rsx! { Piece0TextEditor { piece: piece} },
            1 => rsx! { Piece1MoodEditor {piece:  piece } },
            2 => rsx! { Piece2BlobEditor {piece:  piece } },
            3 => rsx! { Piece3LocationEditor { piece: piece } },
            4 => rsx! { Piece4ActivityEditor {piece:  piece } },
            _ => rsx! {},
        }
    }
}

#[component]
pub fn NewEntry() -> Element {
    let db_signal = use_context::<Resource<DatabaseConnection>>();
    let mut navigator = use_navigator();
    let journals: Resource<Vec<journal::ModelEx>> = use_resource(move || async move {
        let Some(db) = db_signal() else { return vec![] };
        journal::Entity::load().all(&db).await.unwrap_or_default()
    });
    // MAIN STORE
    let mut entry = use_store(entries::ActiveModel::builder);
    let mut p = entry.pieces();

    use_effect(move || {
        if let Some(journals) = journals()
            && let Some(first_journal) = journals.first()
            && entry().journal_id.is_not_set()
        {
            entry.set(entry().set_journal_id(first_journal.id));
        }
    });

    rsx! {
        h1 {"New Entry"}
        div {
            label { for: "journal", class:"form-label", "Journal:"  }
            select {
                name: "journal",
                class: "form-select",
                onchange: move |e| {
                    let val = e.value();
                    entry.set(entry().set_journal_id(val.parse::<i64>().unwrap_or_default()));
                },
                if let Some(journals) = journals() {
                    for journal in journals {
                        option {
                            label: &journal.title,
                            value: &journal.id,
                        }
                    }
                }
            }
        }
        for piece in entry.pieces().model() {
            PieceEditor {piece:piece}
        }
        for i in 0..=4 {
            button {
                r#type: "button",
                class: "btn btn-success",
                onclick: move |_| async move {
                    p.write().append(piece::ActiveModel::builder().set_piece_type(i));
                },
                Icon { "add" }
                "Add Piece {i}"
            }
        }
        br{}
        button {
            r#type: "button",
            class: "btn btn-success",
            onclick: move |_| async move {
                dbg!(&entry());
                let journal_id = entry().journal_id.unwrap();
                entry.set(entry().set_datetime(chrono::Utc::now().timestamp()));
                entry().insert(&db_signal().unwrap()).await.expect("TODO: panic message");
                navigator.push(Route::JournalPaginate { id: journal_id });
            },
            Icon { "add" }
            "Save Entry"
        }
    }
}
