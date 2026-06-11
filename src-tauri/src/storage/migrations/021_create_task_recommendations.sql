-- Migration 021: Create task_recommendations table for E-3-1 (AI Task Recommendations).
--
-- This table stores AI-generated suggestions that connect wiki knowledge base
-- pages to actionable practice tasks. Each recommendation is derived from
-- keyword overlap between a wiki page and existing tasks.

CREATE TABLE IF NOT EXISTS task_recommendations (
    id TEXT PRIMARY KEY NOT NULL,
    wiki_page_id TEXT NOT NULL REFERENCES wiki_pages(id) ON DELETE CASCADE,
    task_template_title TEXT NOT NULL,
    task_template_description TEXT NOT NULL DEFAULT '',
    task_template_difficulty TEXT NOT NULL DEFAULT 'medium'
        CHECK(task_template_difficulty IN ('easy', 'medium', 'hard')),
    task_template_tags TEXT NOT NULL DEFAULT '',
    score REAL NOT NULL DEFAULT 0.0,
    matched_keywords TEXT NOT NULL DEFAULT '[]',
    status TEXT NOT NULL DEFAULT 'recommended'
        CHECK(status IN ('recommended', 'accepted', 'dismissed')),
    ignore_count INTEGER NOT NULL DEFAULT 0,
    created_task_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_task_recommendations_wiki_page
    ON task_recommendations(wiki_page_id);

CREATE INDEX IF NOT EXISTS idx_task_recommendations_status
    ON task_recommendations(status);
