-- Phase 7: Knowledge Discovery (E-7-1)
-- pending_content table for discovered content items
CREATE TABLE IF NOT EXISTS pending_content (
    id                  TEXT PRIMARY KEY,
    title               TEXT NOT NULL,
    source_url          TEXT,
    source_name         TEXT,
    content_summary     TEXT,
    source_page_id      TEXT,
    source_page_title   TEXT,
    match_reason        TEXT,
    match_keywords      TEXT,
    relevance_score     REAL DEFAULT 0.5,
    full_content        TEXT,
    content_hash        TEXT,
    status              TEXT NOT NULL DEFAULT 'unread',
    read_at             TEXT,
    imported_content_id TEXT,
    discovered_at       TEXT NOT NULL,
    created_at          TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_pending_status ON pending_content(status);
CREATE INDEX IF NOT EXISTS idx_pending_page ON pending_content(source_page_id);
CREATE INDEX IF NOT EXISTS idx_pending_discovered ON pending_content(discovered_at);
CREATE UNIQUE INDEX IF NOT EXISTS idx_pending_url ON pending_content(source_url);

-- knowledge_monitor_source table for monitoring sources
CREATE TABLE IF NOT EXISTS knowledge_monitor_source (
    id                  TEXT PRIMARY KEY,
    page_id             TEXT REFERENCES wiki_pages(id) ON DELETE CASCADE,
    search_query        TEXT NOT NULL,
    source_type         TEXT NOT NULL,
    rss_url             TEXT,
    is_active           INTEGER DEFAULT 1,
    last_checked_at     TEXT,
    last_found_count    INTEGER DEFAULT 0,
    created_at          TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_monitor_page ON knowledge_monitor_source(page_id);
CREATE INDEX IF NOT EXISTS idx_monitor_active ON knowledge_monitor_source(is_active);

-- Add discovery fields to wiki_pages
ALTER TABLE wiki_pages ADD COLUMN monitor_enabled INTEGER DEFAULT 0;
ALTER TABLE wiki_pages ADD COLUMN monitor_query TEXT;
ALTER TABLE wiki_pages ADD COLUMN monitor_sources TEXT DEFAULT '[]';
ALTER TABLE wiki_pages ADD COLUMN last_discovered_at TEXT;
ALTER TABLE wiki_pages ADD COLUMN pending_count INTEGER DEFAULT 0;
