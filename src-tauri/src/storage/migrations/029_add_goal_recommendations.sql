-- Migration 029: Add goal_recommendations table for search-based learning resources

CREATE TABLE IF NOT EXISTS goal_recommendations (
    id TEXT PRIMARY KEY NOT NULL,
    goal_id TEXT NOT NULL REFERENCES goals(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    url TEXT,
    summary TEXT,
    difficulty TEXT NOT NULL DEFAULT 'beginner' CHECK(difficulty IN ('beginner', 'intermediate', 'advanced')),
    sort_order INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'pending' CHECK(status IN ('pending', 'imported', 'dismissed')),
    imported_content_id TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_goal_recommendations_goal ON goal_recommendations(goal_id);
CREATE INDEX IF NOT EXISTS idx_goal_recommendations_status ON goal_recommendations(goal_id, status);
