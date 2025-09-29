use std::fs;
use std::path::PathBuf;
use std::sync::RwLock;
use anyhow::Result;
use directories::ProjectDirs;
use rusqlite::Connection;
use rusqlite::ffi::sqlite3;

static FILENAME: &str = "data.powerjournal";

fn database_file() -> Result<PathBuf> {
    let binding = match ProjectDirs::from("me", "machineonamission", "powerjournal") {
        Some(dirs) => dirs,
        None => return Err(anyhow::anyhow!("Could not find data directory")),
    };
    let dir = binding.data_dir();
    fs::create_dir_all(dir)?;
    Ok(dir.join(FILENAME))
}

fn connect() -> Result<Connection> {
    let path = database_file()?;
    let conn = Connection::open(path)?;
    // PWRJRNL in phone keys
    conn.pragma_update(None, "application_id", 7975765)?;
    // increment with each version, todo handle changes
    conn.pragma_update(None, "user_version", 0)?;
    Ok(conn)
}