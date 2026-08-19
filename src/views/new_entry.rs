use crate::components::editor::PieceEditor;
use crate::components::icon::Icon;
use crate::database::entity::entries::ActiveModelExStoreExt;
use crate::database::entity::piece::{ActiveModelExStoreExt as OtherActiveModelExStoreExt, Entity};
use crate::database::entity::prelude::*;
use crate::route::Route;
use crate::store_lenses::{ActiveHasManyStoreImplExt, ActiveHasOneStoreImplExt};
use dioxus::prelude::*;
use sea_orm::{DatabaseConnection, EntityLoaderTrait, EntityTrait, IntoActiveModel, Set};

#[component]
pub fn Editor(id: Option<i64>) -> Element {
    let db_signal = use_context::<Resource<DatabaseConnection>>();

    // Move the fetching logic here
    let entry_resource: Resource<entries::ActiveModelEx> = use_resource(move || async move {
        if let Some(id) = id {
            if let Some(db) = db_signal() {
                return entries::Entity::load()
                    .filter_by_id(id)
                    .with((piece::Entity, piece_0_text::Entity))
                    .with((piece::Entity, piece_1_mood::Entity))
                    .with((piece::Entity, piece_2_blob::Entity))
                    .with((piece::Entity, piece_3_location::Entity))
                    .with((piece::Entity, piece_4_activities::Entity))
                    .one(&db)
                    .await
                    .unwrap()
                    .unwrap()
                    .into_active_model()
            }
        }
        // If no ID, immediately return a new builder
        entries::ActiveModel::builder()
    });

    // Don't render the editor until the resource is ready.
    // Early returns are completely fine here because we haven't hit the child's hooks yet!
    let Some(initial_entry) = entry_resource() else {
        return rsx! { div { class: "loading", "Loading entry..." } };
    };

    rsx! {
        // Pass the resolved data into the actual editor
        InnerEditor {
            initial_entry: initial_entry
        }
    }
}

// 2. The Actual Editor Component
#[component]
fn InnerEditor(initial_entry: entries::ActiveModelEx) -> Element {
    let db_signal = use_context::<Resource<DatabaseConnection>>();
    let mut navigator = use_navigator();
    let journals: Resource<Vec<journal::ModelEx>> = use_resource(move || async move {
        let Some(db) = db_signal() else { return vec![] };
        journal::Entity::load().all(&db).await.unwrap_or_default()
    });

    // MAIN STORE
    // This now works perfectly because initial_entry is fully resolved synchronously
    let mut entry = use_store(move || initial_entry.clone());
    let mut p = entry.pieces();

    use_effect(move || {
        // Your journal sync effect
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
                navigator.push(Route::JournalPaginate { id: Some(journal_id) });
            },
            Icon { "add" }
            "Save Entry"
        }
    }
}
