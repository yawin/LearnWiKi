-- Migration 019: Add task_solutions table and author/source fields to wiki_pages (E-2-6)

CREATE TABLE IF NOT EXISTS task_solutions (
    id TEXT PRIMARY KEY NOT NULL,
    task_id TEXT NOT NULL REFERENCES practice_tasks(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    author TEXT,
    source_url TEXT,
    content TEXT NOT NULL,
    solution_type TEXT NOT NULL DEFAULT 'reference'
        CHECK(solution_type IN ('reference', 'my_attempt', 'community')),
    difficulty_rating REAL,
    quality_rating REAL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_task_solutions_task_id
    ON task_solutions(task_id);
