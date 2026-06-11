CREATE TABLE IF NOT EXISTS review_schedule (
    id TEXT PRIMARY KEY,
    wiki_page_id TEXT NOT NULL REFERENCES wiki_pages(id) ON DELETE CASCADE,
    ease_factor REAL NOT NULL DEFAULT 2.5,
    interval_days INTEGER NOT NULL DEFAULT 0,
    next_review_at TEXT NOT NULL,
    review_count INTEGER NOT NULL DEFAULT 0,
    last_reviewed_at TEXT,
    mastery REAL NOT NULL DEFAULT 0.0,
    is_archived INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_review_schedule_next ON review_schedule(next_review_at);
CREATE INDEX IF NOT EXISTS idx_review_schedule_wiki ON review_schedule(wiki_page_id);

CREATE TABLE IF NOT EXISTS review_logs (
    id TEXT PRIMARY KEY,
    schedule_id TEXT NOT NULL REFERENCES review_schedule(id) ON DELETE CASCADE,
    quality INTEGER NOT NULL CHECK(quality IN (0,1,2)),
    interval_before INTEGER NOT NULL DEFAULT 0,
    interval_after INTEGER NOT NULL,
    ease_factor_before REAL NOT NULL,
    ease_factor_after REAL NOT NULL,
    reviewed_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS daily_review_summary (
    id TEXT PRIMARY KEY,
    date TEXT NOT NULL UNIQUE,
    total_reviewed INTEGER NOT NULL DEFAULT 0,
    correct_count INTEGER NOT NULL DEFAULT 0,
    streak_day INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);
