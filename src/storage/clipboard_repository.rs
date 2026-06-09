use crate::{service::clipboard_service::ClipboardEntry, storage::database::Database};
use rusqlite::params;
use std::sync::Arc;

pub struct ClipboardRepository {
    database: Arc<Database>,
}

impl ClipboardRepository {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }

    pub fn insert(&self, entry: ClipboardEntry) -> anyhow::Result<()> {
        self.database.run(|conn| {
            conn.execute(
                "INSERT INTO clipboard_entries (id, content, created_at) VALUES (?1, ?2, ?3)",
                params![entry.id, entry.content, entry.created_at],
            )?;
            Ok(())
        })
    }

    pub fn get_latest(&self, limit: Option<usize>) -> anyhow::Result<Vec<ClipboardEntry>> {
        self.database.run(|conn| {
            let (sql, params) = match limit {
                Some(limit_val) => (
                    "SELECT id, content, created_at FROM clipboard_entries ORDER BY created_at DESC LIMIT ?1",
                    params![limit_val as i64],
                ),
                None => (
                    "SELECT id, content, created_at FROM clipboard_entries ORDER BY created_at DESC",
                    params![],
                ),
            };

            let mut stmt = conn.prepare(sql)?;
            let entries = stmt
                .query_map(params, |row| {
                    Ok(ClipboardEntry {
                        id: row.get(0)?,
                        content: row.get(1)?,
                        created_at: row.get(2)?,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?;

            Ok(entries)
        })
    }

    pub fn delete(&self, id: &str) -> anyhow::Result<()> {
        self.database.run(|conn| {
            conn.execute("DELETE FROM clipboard_entries WHERE id = ?1", params![id])?;
            Ok(())
        })
    }

    pub fn count(&self) -> anyhow::Result<usize> {
        self.database.run(|conn| {
            let mut stmt = conn.prepare("SELECT COUNT(*) FROM clipboard_entries")?;
            let count: i64 = stmt.query_row([], |row| row.get(0))?;
            Ok(count as usize)
        })
    }

    pub fn delete_all(&self) -> anyhow::Result<()> {
        self.database.run(|conn| {
            conn.execute("DELETE FROM clipboard_entries", [])?;
            Ok(())
        })
    }

    pub fn get_by_id(&self, id: &str) -> anyhow::Result<Option<ClipboardEntry>> {
        self.database.run(|conn| {
            let mut stmt = conn
                .prepare("SELECT id, content, created_at FROM clipboard_entries WHERE id = ?1")?;

            let mut rows = stmt.query_map(params![id], |row| {
                Ok(ClipboardEntry {
                    id: row.get(0)?,
                    content: row.get(1)?,
                    created_at: row.get(2)?,
                })
            })?;

            if let Some(row) = rows.next() {
                Ok(Some(row?))
            } else {
                Ok(None)
            }
        })
    }
}
