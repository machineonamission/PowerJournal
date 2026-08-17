use crate::Route;
use crate::components::icon::Icon;
use crate::database::entity::entries::ActiveModelExStoreExt;
use crate::database::entity::piece::{ActiveModelExStoreExt as OtherActiveModelExStoreExt, Entity};
use crate::database::entity::prelude::*;
use crate::store_lenses::{ActiveHasManyStoreImplExt, ActiveHasOneStoreImplExt};
use dioxus::prelude::*;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use crate::components::editor::PieceEditor;

#[component]
pub fn NewEntry() -> Element {
    let db_signal = use_context::<Resource<DatabaseConnection>>();
    let mut navigator = use_navigator();
    let journals: Resource<Vec<journal::ModelEx>> = use_resource(move || async move {
        let Some(db) = db_signal() else { return vec![] };
        journal::Entity::load().all(&db).await.unwrap_or_default()
    });
    // MAIN STORE
    // TODO entry "editor" will load activemodelex from db
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
