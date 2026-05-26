use anyhow::{Result, anyhow};
use std::path::PathBuf;
use std::fs;
use rusqlite::Connection;

pub fn database_path() -> Result<PathBuf> {
    let data_dir =
        dirs::data_local_dir().ok_or_else(|| anyhow!("failed to resolve data directory"))?;

    Ok(data_dir.join("clipforge").join("clipforge.db"))
}



pub fn initialize_database() -> Result<Connection> {
    let path = database_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let connection = Connection::open(path)?;
    Ok(connection)
}