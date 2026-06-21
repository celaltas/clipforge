CREATE VIRTUAL TABLE clipboard_entries_fts USING fts5(
    id UNINDEXED,
    content
);

CREATE TRIGGER After_Insert_ClipboardEntries
AFTER INSERT ON clipboard_entries
BEGIN
    INSERT INTO clipboard_entries_fts (id, content) VALUES (new.id, new.content);
END;

CREATE TRIGGER After_Delete_ClipboardEntries
AFTER DELETE ON clipboard_entries
BEGIN
    DELETE FROM clipboard_entries_fts WHERE id = old.id;
END;