-- Sprint 5A: Review Format Rotation
-- Adds last_format to review_schedule, review_format_history table,
-- and review_format + response_time_seconds to review_logs

ALTER TABLE review_schedule ADD COLUMN last_format TEXT DEFAULT 'quiz';

CREATE TABLE IF NOT EXISTS review_format_history (
    id              TEXT PRIMARY KEY,
    schedule_id     TEXT REFERENCES review_schedule(id) ON DELETE CASCADE,
    format          TEXT NOT NULL,
    used_at         TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_review_format_history_schedule ON review_format_history(schedule_id);

-- Add review_format and response_time_seconds to review_logs
ALTER TABLE review_logs ADD COLUMN review_format TEXT;
ALTER TABLE review_logs ADD COLUMN response_time_seconds INTEGER;
