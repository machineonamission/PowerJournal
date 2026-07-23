pub mod entity;
use anyhow::{anyhow, Result};

use sea_orm::{Database, DatabaseConnection};
#[cfg(not(target_arch = "wasm32"))]
use std::{fs, path::PathBuf};

/// The struct we will provide to Dioxus via use_context_provider
#[derive(Clone, Debug)]
pub struct DbContext {
    pub conn: DatabaseConnection,
}

#[cfg(not(target_arch = "wasm32"))]
fn get_db_url() -> Result<String> {
    let mut path: PathBuf = dirs::data_local_dir().ok_or(anyhow!("No data local dir"))?;
    path.push("PowerJournal");
    fs::create_dir_all(&path)?;
    path.push("journal.powerjournal");

    Ok(format!("sqlite://{}?mode=rwc", path.to_string_lossy()))
}

#[cfg(target_arch = "wasm32")]
fn get_db_url() -> Result<String> {
    Ok("sqlite::memory:".to_string())
}

/// Initializes the connection. You can call this from use_resource in your UI.
pub async fn init_db() -> Result<DatabaseConnection> {
    let db_url = get_db_url()?;

    dbg!(&db_url);


    let db = Database::connect(&db_url)
        .await?;

    db.get_schema_registry(&format!("{}::entity", module_path!()))
        .sync(&db)
        .await?;

    // Example schema sync:
    // db.get_schema_registry(&format!("{}::entity", module_path!()))
    //     .sync(&db)
    //     .await
    //     .expect("Failed to sync schema");

    Ok(db)
}