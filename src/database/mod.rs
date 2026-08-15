pub mod entity;
use anyhow::{Result, anyhow};

use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, Set};
#[cfg(not(target_arch = "wasm32"))]
use std::{fs, path::PathBuf};

// #[cfg(not(target_arch = "wasm32"))]
fn get_db_url() -> Result<String> {
    let mut path: PathBuf = dirs::data_local_dir().ok_or(anyhow!("No data local dir"))?;
    path.push("PowerJournal");
    fs::create_dir_all(&path)?;
    path.push("journal.powerjournal");

    Ok(format!("sqlite://{}?mode=rwc", path.to_string_lossy()))
}

// dioxus technically compiles to wasm, it is ultimately rendered by a browser, however
// sea-orm wont compile to wasm as far as i can tell. so who cares.
// #[cfg(target_arch = "wasm32")]
// fn get_db_url() -> Result<String> {
//     Ok("sqlite::memory:".to_string())
// }

async fn ensure_default_journal(db: &DatabaseConnection) -> Result<()> {
    use entity::journal;
    use sea_orm::EntityTrait;

    let default_journal = journal::Entity::find().one(db).await?;

    if default_journal.is_none() {
        let new_journal = journal::ActiveModel {
            title: Set("My Journal".to_string()),
            ..Default::default()
        };
        new_journal.insert(db).await?;
    }

    Ok(())
}

pub async fn init_db() -> Result<DatabaseConnection> {
    let db_url = get_db_url()?;

    let db = Database::connect(&db_url).await?;

    db.get_schema_registry(&format!("{}::entity", module_path!()))
        .sync(&db)
        .await?;

    ensure_default_journal(&db).await?;

    Ok(db)
}
