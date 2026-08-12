use crate::Route;
use crate::components::icon::Icon;
use crate::database::entity::prelude::*;
use dioxus::prelude::*;
use sea_orm::DatabaseConnection;

#[component]
pub fn Piece0TextEditor(mut piece: Store<piece::ActiveModelEx>) -> Element {
    rsx! {
        textarea {
            class: "form-control",
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
    let mut navigator = use_navigator();
    let journals: Resource<Vec<journal::ModelEx>> = use_resource(move || async move {
        let Some(db) = db_signal() else { return vec![] };
        journal::Entity::load().all(&db).await.unwrap_or_default()
    });
    // partial store for entry like, title?
    let mut entry = use_signal(entries::ActiveModel::builder);
    // main editor store, each piece as a signal
    let mut pieces = use_store(Vec::<piece::ActiveModelEx>::new);

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
        for piece in pieces.iter() {
            PieceEditor {piece:piece}
        }
        button {
            r#type: "button",
            class: "btn btn-success",
            onclick: move |_| async move {
                pieces.write().push(piece::ActiveModel::builder().set_piece_type(0).set_piece_0_text(piece_0_text::ActiveModel::builder()));
            },
            Icon { "add" }
            "Add Piece"
        }
        br{}
        button {
            r#type: "button",
            class: "btn btn-success",
            onclick: move |_| async move {
                let journal_id = match entry().journal_id {
                    sea_orm::ActiveValue::Set(id) => id,
                    _ => 0,
                };
                for piece in pieces.iter() {
                    entry.set(entry().add_piece(piece()))
                }
                entry.set(entry().set_datetime(chrono::Utc::now().timestamp()));
                entry().insert(&db_signal().unwrap()).await.expect("TODO: panic message");
                navigator.push(Route::JournalPaginate { id: journal_id });
            },
            Icon { "add" }
            "Save Entry"
        }
    }
}
