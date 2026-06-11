-- Migration 028: Add exam tables for goal-based testing system

CREATE TABLE IF NOT EXISTS exams (
    id TEXT PRIMARY KEY NOT NULL,
    goal_id TEXT NOT NULL REFERENCES goals(id) ON DELETE CASCADE,
    title TEXT,
    total_questions INTEGER NOT NULL DEFAULT 0,
    score REAL,
    grade TEXT CHECK(grade IN ('A', 'B', 'C', 'D') OR grade IS NULL),
    status TEXT NOT NULL DEFAULT 'in_progress' CHECK(status IN ('in_progress', 'completed')),
    started_at TEXT NOT NULL,
    completed_at TEXT,
    diagnosis_json TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_exams_goal ON exams(goal_id);
CREATE INDEX IF NOT EXISTS idx_exams_status ON exams(status);

CREATE TABLE IF NOT EXISTS exam_questions (
    id TEXT PRIMARY KEY NOT NULL,
    exam_id TEXT NOT NULL REFERENCES exams(id) ON DELETE CASCADE,
    wiki_page_id TEXT NOT NULL,
    question_type TEXT NOT NULL CHECK(question_type IN ('choice', 'judgment', 'essay')),
    question_json TEXT NOT NULL,
    user_answer TEXT,
    correct_answer TEXT,
    is_correct INTEGER,
    score REAL,
    ai_feedback TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0,
    answered_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_exam_questions_exam ON exam_questions(exam_id);
