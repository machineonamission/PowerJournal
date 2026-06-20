// use anyhow::Result;
// use rusqlite::Connection;
// use std::path::PathBuf;
// use std::sync::{Mutex, OnceLock};
//
static FILENAME: &str = "sqlite://data.powerjournal";

fn database_file() -> Result<PathBuf> {
    Ok(crate::path::data_dir()?.join(FILENAME))
}
//
// fn init(conn: &Connection) -> Result<()> {
//     println!("Initializing database");
//     conn.execute_batch(include_str!("init.sql"))?;
//     Ok(())
// }
// pub fn connect() -> Result<Connection> {
//     let path = database_file()?;
//     println!("Database path: {:?}", path);
//     let exists = path.try_exists()?;
//     let conn = Connection::open(path)?;
//     if !exists {
//         init(&conn)?;
//     }
//     Ok(conn)
// }
//
static DB: OnceLock<Mutex<Result<DatabaseConnection>>> = OnceLock::new();

pub fn get_db() -> &'static Mutex<Result<Connection>> {
    DB.get_or_init(|| Mutex::new(init_db()))
}

pub mod entity;
use anyhow::Result;

// static DATABASE: OnceCell<DatabaseConnection> = OnceCell::const_new();

use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

pub async fn init_db() -> Result<DatabaseConnection> {
    let db = Database::connect(database_file()?).await?;

    // synchronizes database schema with entity definitions
    db.get_schema_registry(&format!("{}::entity", module_path!()))
        .sync(&db)
        .await?;

    // runs migrations (db stuff i cant do in seaorm)
    // Migrator::up(&db, None).await?;

    // DATABASE.set(db)?;
    Ok(db)
}
