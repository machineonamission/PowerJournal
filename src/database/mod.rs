use anyhow::Result;
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

static FILENAME: &str = "data.powerjournal";

fn database_file() -> Result<PathBuf> {
    Ok(crate::path::data_dir()?.join(FILENAME))
}

fn init(conn: &Connection) -> Result<()> {
    println!("Initializing database");
    conn.execute_batch(include_str!("init.sql"))?;
    Ok(())
}
pub fn connect() -> Result<Connection> {
    let path = database_file()?;
    println!("Database path: {:?}", path);
    let exists = path.try_exists()?;
    let conn = Connection::open(path)?;
    if !exists {
        init(&conn)?;
    }
    Ok(conn)
}

static DB: OnceLock<Mutex<Result<Connection>>> = OnceLock::new();

pub fn get_db() -> &'static Mutex<Result<Connection>> {
    DB.get_or_init(|| Mutex::new(connect()))
}
