use std::path::PathBuf;
use dioxus::prelude::*;
use sea_orm::DatabaseConnection;
use crate::Route;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn DebugMenu() -> Element {
    let db_signal = use_context::<Resource<DatabaseConnection>>();
    rsx! {
        h1 {
            img {
                src: asset!("/assets/logo/logo.svg"),
                display: "inline-block",
                // can't do width: shorthand cause its ambiguous with html width attr, so this is workaround
                style: "width: 1em; height: 1em;",
                vertical_align: "-0.125em",
                object_fit: "contain",
            }
            span {
                font_weight: "900",
                font_family: "var(--display-font)",
                " PowerJournal"
            }
        }
        div {
            display: "flex",
            flex_direction: "column",
            gap: "1rem",
            Link {
                to: Route::JournalPaginate { id: 1 },
                "open journal 1"
            }
            Link {
                to: Route::TestPaginate {},
                "open test paginate"
            }
            Link {
                to: Route::ImportersView {},
                "open importers"
            }
            Link {
                to: Route::JournalList {},
                "open journal list"
            }
            button {
                onclick: move |_| {
                    let mut path: PathBuf = dirs::data_local_dir().unwrap();
                    path.push("PowerJournal");
                    open::that(path).expect("Failed to open folder");
                },
                "open db folder"
            }
            button {
                onclick: move |_| async move {
                    db_signal().unwrap().close().await.unwrap();
                    let mut path: PathBuf = dirs::data_local_dir().unwrap();
                    path.push("PowerJournal");
                    path.push("journal.powerjournal");
                    tokio::fs::remove_file(path).await.unwrap();
                },
                "delete db"
            }
        }
    }
}
