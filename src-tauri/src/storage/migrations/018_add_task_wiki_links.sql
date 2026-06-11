-- Migration 018: Add task_wiki_links table for bidirectional linking
-- between practice tasks and wiki knowledge base pages (E-2-3)

CREATE TABLE IF NOT EXISTS task_wiki_links (
    id TEXT PRIMARY KEY NOT NULL,
    task_id TEXT NOT NULL REFERENCES practice_tasks(id) ON DELETE CASCADE,
    wiki_id TEXT NOT NULL REFERENCES wiki_pages(id) ON DELETE CASCADE,
    relevance_score REAL NOT NULL DEFAULT 0.5,
    created_at TEXT NOT NULL,
    UNIQUE(task_id, wiki_id)
);

CREATE INDEX IF NOT EXISTS idx_task_wiki_links_task_id
    ON task_wiki_links(task_id);

CREATE INDEX IF NOT EXISTS idx_task_wiki_links_wiki_id
    ON task_wiki_links(wiki_id);
