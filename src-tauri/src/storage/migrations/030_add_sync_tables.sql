-- Migration 030: Add folder sync tables

CREATE TABLE IF NOT EXISTS sync_folders (
    id TEXT PRIMARY KEY NOT NULL,
    path TEXT NOT NULL UNIQUE,
    enabled INTEGER NOT NULL DEFAULT 1,
    last_synced_at TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sync_records (
    id TEXT PRIMARY KEY NOT NULL,
    folder_id TEXT NOT NULL REFERENCES sync_folders(id) ON DELETE CASCADE,
    file_path TEXT NOT NULL,
    file_name TEXT NOT NULL,
    file_size INTEGER,
    file_mtime TEXT NOT NULL,
    file_type TEXT NOT NULL CHECK(file_type IN ('md', 'txt', 'pdf', 'docx', 'epub', 'image')),
    content_id TEXT,
    status TEXT NOT NULL DEFAULT 'imported' CHECK(status IN ('imported', 'updated', 'error')),
    synced_at TEXT NOT NULL,
    UNIQUE(folder_id, file_path)
);

CREATE INDEX IF NOT EXISTS idx_sync_records_folder ON sync_records(folder_id);
CREATE INDEX IF NOT EXISTS idx_sync_records_content ON sync_records(content_id);
