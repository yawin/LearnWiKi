-- Phase 7: Knowledge Discovery Suppression (E-7-7)
-- Tracks user dismissals of discovered content to auto-ignore noisy sources
CREATE TABLE IF NOT EXISTS discovery_suppression (
    id                  TEXT PRIMARY KEY,
    source_page_id      TEXT NOT NULL REFERENCES wiki_pages(id) ON DELETE CASCADE,
    dismiss_count       INTEGER NOT NULL DEFAULT 1,
    is_auto_ignored     INTEGER NOT NULL DEFAULT 0,
    created_at          TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at          TEXT NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_suppression_source ON discovery_suppression(source_page_id);
