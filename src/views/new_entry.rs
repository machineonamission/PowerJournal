use crate::database::entity::prelude::*;
use dioxus::prelude::*;
use sea_orm::DatabaseConnection;
use crate::components::icon::Icon;

#[component]
pub fn Piece0TextEditor(mut piece: Store<piece::ActiveModelEx>) -> Element {
    rsx! {
        textarea {
            oninput: move |e| {
                let val = e.value();
                piece.write().piece_0_text.as_mut().unwrap().content = sea_orm::ActiveValue::Set(val);
            }
        }
    }
}

#[component]
pub fn PieceEditor(mut piece: Store<piece::ActiveModelEx>) -> Element {
    let piece_type = piece.read().piece_type.clone().unwrap(); // ActiveValue<i64> -> i64
    rsx! {
        match piece_type {
            0 => rsx! { Piece0TextEditor { piece: piece} },
            // 1 => rsx! { Piece1MoodEditor { piece } },
            // 2 => rsx! { Piece2BlobEditor { piece } },
            // 3 => rsx! { Piece3LocationEditor { piece } },
            // 4 => rsx! { Piece4ActivityEditor { piece } },
            _ => rsx! {},
        }
    }
}

#[component]
pub fn NewEntry() -> Element {
    let db_signal = use_context::<Resource<DatabaseConnection>>();
    let journals: Resource<Vec<journal::ModelEx>> = use_resource(move || async move {
        let Some(db) = db_signal() else { return vec![] };
        journal::Entity::load().all(&db).await.unwrap_or_default()
    });
    // partial store for entry like, title?
    let mut entry = use_signal(entries::ActiveModel::builder);
    // main editor store, each piece as a signal
    let mut pieces = use_store(Vec::<piece::ActiveModelEx>::new);
    // let mut first: Write<Piece, _> = entry_pieces.map_mut(|v| &mut v[0]);
    use_effect(move || {
        dbg!(&pieces());
    });
    rsx! {
        h1 {"New Entry"}
        div {
            label { for: "journal", class:"form-label", "Journal:"  }
            select {
                name: "journal",
                class: "form-select",
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
        for piece in pieces.iter() {
            PieceEditor {piece:piece}
        }
        button {
            r#type: "button",
            class: "btn btn-success",
            onclick: move |_| async move {
                pieces.write().push(piece::ActiveModel::builder().set_piece_type(0).set_piece_0_text(piece_0_text::ActiveModel::builder()));
            },
            Icon { icon: "plus" }
            "Add Piece"
        }
    }
}
