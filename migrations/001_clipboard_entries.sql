CREATE TABLE clipboard_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content TEXT NOT NULL,
    content_type TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    pinned INTEGER NOT NULL DEFAULT 0
);
