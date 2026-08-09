use std::pin::Pin;
use bytes::Bytes;
use dioxus::prelude::*;
use sea_orm::DatabaseConnection;
use crate::importers::{daylio, applejournal};
use crate::Route;

// so the Bytes object just HAPPENS to be what Dioxus file uploads are. so fucking, whatever. its a fine object for this.
type ImportFuture<'a> = Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + 'a>>;
type ImportFn = for<'a> fn(Bytes, &'a DatabaseConnection) -> ImportFuture<'a>;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Importer {
    pub name: &'static str,
    pub function: ImportFn,

}

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn ImportersView() -> Element {
    let IMPORTERS: [Importer; 2] = [
        Importer {
            name: "Daylio",
            function: |file, db| Box::pin(daylio::import_daylio(file, db)),
        },
        Importer {
            name: "Apple Journal",
            function: |file, db| Box::pin(applejournal::import_apple_journal(file, db)),
        },
    ];
    let mut selection = use_signal(|| 0usize);
    let selected_importer = use_memo(move || IMPORTERS[selection()]);
    let mut file = use_signal(|| Option::<Bytes>::None);

    let db_signal = use_context::<Resource<DatabaseConnection>>();
    rsx! {
        h1 {
            "3rd-party Journal Importer"
        }
        form {
            for (i, importer) in IMPORTERS.iter().enumerate() {
                div {
                    class: "form-check",
                    input {
                        class: "form-check-input",
                        r#type: "radio",
                        name: "importer",
                        id: "{importer.name}",
                        oninput: move |evt| {
                           selection.set(i);
                        },
                        checked: i == 0
                    }
                    label {
                        class: "form-check-label",
                        r#for: "{importer.name}",
                        "{importer.name}"
                    }
                }
            }
            p {
                "{selected_importer().name}"
            }
            input {
                // tell the input to pick a file
                type: "file",
                // list the accepted extensions
                accept: "*",
                // pick multiple files
                multiple: false,
                onchange: move |evt| async move {
                    let bytes = evt.files()[0].read_bytes().await.unwrap();
                    file.set(Some(bytes))
                }
            }
            if file().is_some() {
                button {
                    r#type: "button",
                    class: "btn btn-primary",
                    onclick: move |_| async move {
                        let db = db_signal().unwrap();
                        selected_importer().function.call((file().as_ref().unwrap().clone(), &db)).await.unwrap();
                    },
                    "Import"
                }
            }
        }
    }
}
