-- Migration 032: Add wiki_reading_status table for tracking read/unread state of wiki pages

CREATE TABLE IF NOT EXISTS wiki_reading_status (
    wiki_page_id TEXT PRIMARY KEY,
    is_read INTEGER NOT NULL DEFAULT 0,
    updated_at TEXT NOT NULL
);
