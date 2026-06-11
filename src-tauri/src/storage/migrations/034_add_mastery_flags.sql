CREATE TABLE IF NOT EXISTS wiki_mastery_flags (
    id TEXT PRIMARY KEY,
    wiki_page_id TEXT NOT NULL,
    goal_id TEXT NOT NULL,
    exam_id TEXT NOT NULL,
    is_resolved INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    resolved_at TEXT
);
