use std::fs;
use std::path::PathBuf;
use std::sync::RwLock;
use anyhow::Result;
use directories::ProjectDirs;
use rusqlite::Connection;
use rusqlite::ffi::sqlite3;

static FILENAME: &str = "data.powerjournal";

fn database_file() -> Result<PathBuf> {
    Ok(crate::path::data_dir()?.join(FILENAME))
}

pub fn connect() -> Result<Connection> {
    let path = database_file()?;
    println!("Database path: {:?}", path);
    let conn = Connection::open(path)?;

    Ok(conn)
}