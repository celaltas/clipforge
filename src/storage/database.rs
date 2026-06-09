use anyhow::{Result, anyhow};
use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

use crate::storage::migration::run_migrations as run_sql_migrations;

pub struct Database {
    connection: Mutex<Connection>,
}

impl Database {
    pub fn new() -> Result<Self> {
        let path = Self::database_path()?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut connection = Connection::open(path)?;

        Self::run_migrations(&mut connection)?;

        Ok(Self {
            connection: Mutex::new(connection),
        })
    }
    fn database_path() -> Result<PathBuf> {
        let data_dir =
            dirs::data_local_dir().ok_or_else(|| anyhow!("failed to resolve data directory"))?;

        Ok(data_dir.join("clipforge").join("clipforge.db"))
    }

    pub fn run_migrations(connection: &mut Connection) -> Result<()> {
        tracing::info!("Running database migrations...");
        run_sql_migrations(connection)?;
        tracing::info!("Migrations completed successfully");
        Ok(())
    }

    pub fn run<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Connection) -> R,
    {
        let conn = self.connection.lock().unwrap();
        f(&conn)
    }
}
