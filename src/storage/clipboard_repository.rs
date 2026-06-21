use crate::storage::{database::Database, models::ClipboardEntry};
use rusqlite::params;
use std::sync::Arc;

pub struct ClipboardRepository {
    database: Arc<Database>,
}

impl ClipboardRepository {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }

    pub fn insert(&self, entry: ClipboardEntry) -> anyhow::Result<i64> {
        self.database.run(|conn| {
            conn.execute(
                "INSERT INTO clipboard_entries (content, content_type, created_at, pinned) VALUES (?1, ?2, ?3, ?4)",
                params![entry.content, entry.content_type, entry.created_at, entry.pinned],
            )?;

            let generated_id = conn.last_insert_rowid();
            Ok(generated_id)
        })
    }

    pub fn get_latest(&self, limit: usize, offset: usize) -> anyhow::Result<Vec<ClipboardEntry>> {
        self.database.run(|conn| {
            let sql = "SELECT id, content, content_type, created_at, pinned \
                       FROM clipboard_entries \
                       ORDER BY pinned DESC, created_at DESC \
                       LIMIT ?1 OFFSET ?2";

            let mut stmt = conn.prepare(sql)?;
            let entries = stmt
                .query_map(params![limit as i64, offset as i64], |row| {
                    Ok(ClipboardEntry {
                        id: row.get(0)?,
                        content: row.get(1)?,
                        content_type: row.get(2)?,
                        created_at: row.get(3)?,
                        pinned: row.get(4)?,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?;

            Ok(entries)
        })
    }

    pub fn search_entries(
        &self,
        query: &str,
        limit: usize,
        offset: usize,
    ) -> anyhow::Result<Vec<ClipboardEntry>> {
        self.database.run(|conn| {
            let sql = "SELECT e.id, e.content, e.content_type, e.created_at, e.pinned 
                            FROM clipboard_entries e 
                            JOIN clipboard_entries_fts fts ON e.id = fts.id 
                            WHERE fts.content MATCH ?1 
                            ORDER BY e.pinned DESC, fts.rank ASC, e.created_at DESC 
                            LIMIT ?2 OFFSET ?3";

            let mut stmt = conn.prepare(sql)?;

            let formatted_query = format!("{}*", query.trim());

            let entries = stmt
                .query_map(
                    params![formatted_query, limit as i64, offset as i64],
                    |row| {
                        Ok(ClipboardEntry {
                            id: row.get(0)?,
                            content: row.get(1)?,
                            content_type: row.get(2)?,
                            created_at: row.get(3)?,
                            pinned: row.get(4)?,
                        })
                    },
                )?
                .collect::<Result<Vec<_>, _>>()?;

            Ok(entries)
        })
    }

    pub fn toggle_pin(&self, id: i64, is_pinned: bool) -> anyhow::Result<()> {
        self.database.run(|conn| {
            conn.execute(
                "UPDATE clipboard_entries SET pinned = ?1 WHERE id = ?2",
                params![is_pinned, id],
            )?;
            Ok(())
        })
    }

    pub fn delete_entry(&self, id: i64) -> anyhow::Result<()> {
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

    pub fn get_by_id(&self, id: i64) -> anyhow::Result<Option<ClipboardEntry>> {
        self.database.run(|conn| {
            let mut stmt = conn
                .prepare("SELECT id, content, content_type, created_at, pinned FROM clipboard_entries WHERE id = ?1")?;

            let mut rows = stmt.query_map(params![id], |row| {
                Ok(ClipboardEntry {
                    id: row.get(0)?,
                    content: row.get(1)?,
                    content_type: row.get(2)?,
                    created_at: row.get(3)?,
                    pinned: row.get(4)?,
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
