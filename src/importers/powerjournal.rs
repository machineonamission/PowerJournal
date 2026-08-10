use crate::importers::common::ImporterArgs;
use dioxus::prelude::WritableExt;

pub async fn import_powerjournal(mut args: ImporterArgs<'_>) -> anyhow::Result<()> {
    let ImporterArgs {
        file,
        db,
        mut log_signal,
        mut current_prog_signal,
        mut max_prog_signal,
        importer_options,
    } = args;

    let mut log = move |message: String| {
        log_signal.write().push(message);
    };

    let mut log_str = move |message: &str| {
        log(message.to_string());
    };
    log_str("beginning import...");
    todo!()
}
