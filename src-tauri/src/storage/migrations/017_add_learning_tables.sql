-- Migration 017: Add learning data models (LearningPath, Module, PracticeTask, TaskDailyLog)
--
-- These tables support structured learning paths (10-20-70 model),
-- modular content units, practice tasks with status tracking, and
-- daily activity logs.

-- LearningPath: top-level learning journey
CREATE TABLE IF NOT EXISTS learning_paths (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    topic TEXT NOT NULL DEFAULT '',
    difficulty TEXT NOT NULL DEFAULT 'beginner'
        CHECK(difficulty IN ('beginner', 'intermediate', 'advanced')),
    estimated_days INTEGER NOT NULL DEFAULT 0,
    module_count INTEGER NOT NULL DEFAULT 0,
    completion_rate REAL NOT NULL DEFAULT 0.0
        CHECK(completion_rate >= 0.0 AND completion_rate <= 1.0),
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_learning_paths_active
    ON learning_paths(is_active);

-- Module: a unit within a learning path (10-20-70 unit)
CREATE TABLE IF NOT EXISTS modules (
    id TEXT PRIMARY KEY NOT NULL,
    path_id TEXT NOT NULL REFERENCES learning_paths(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    sort_order INTEGER NOT NULL DEFAULT 0,
    description TEXT NOT NULL DEFAULT '',
    theory_markdown TEXT NOT NULL DEFAULT '',
    reading_list_json TEXT NOT NULL DEFAULT '[]',
    estimated_read_minutes INTEGER NOT NULL DEFAULT 0,
    discussion_prompts TEXT NOT NULL DEFAULT '[]',
    community_solutions TEXT NOT NULL DEFAULT '[]',
    task_ids TEXT NOT NULL DEFAULT '[]',
    status TEXT NOT NULL DEFAULT 'locked'
        CHECK(status IN ('locked', 'available', 'completed')),
    completed_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_modules_path_id
    ON modules(path_id);
CREATE INDEX IF NOT EXISTS idx_modules_sort_order
    ON modules(path_id, sort_order);

-- PracticeTask: a concrete exercise tied to a module
CREATE TABLE IF NOT EXISTS practice_tasks (
    id TEXT PRIMARY KEY NOT NULL,
    module_id TEXT NOT NULL REFERENCES modules(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    difficulty TEXT NOT NULL DEFAULT 'easy'
        CHECK(difficulty IN ('easy', 'medium', 'hard')),
    estimated_minutes INTEGER NOT NULL DEFAULT 0,
    prerequisites TEXT NOT NULL DEFAULT '[]',
    hint_content TEXT,
    reference_links TEXT,
    status TEXT NOT NULL DEFAULT 'not_started'
        CHECK(status IN ('not_started', 'in_progress', 'completed', 'reviewed')),
    started_at TEXT,
    completed_at TEXT,
    attempt_count INTEGER NOT NULL DEFAULT 0,
    is_starred INTEGER NOT NULL DEFAULT 0,
    reflection TEXT,
    code_snippets TEXT,
    screenshots_json TEXT,
    created_wiki_pages TEXT NOT NULL DEFAULT '[]',
    related_wiki_pages TEXT NOT NULL DEFAULT '[]',
    tags TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_practice_tasks_module_id
    ON practice_tasks(module_id);
CREATE INDEX IF NOT EXISTS idx_practice_tasks_status
    ON practice_tasks(status);

-- TaskDailyLog: daily summary of learning activity
CREATE TABLE IF NOT EXISTS task_daily_logs (
    id TEXT PRIMARY KEY NOT NULL,
    date TEXT NOT NULL UNIQUE,
    total_minutes INTEGER NOT NULL DEFAULT 0,
    tasks_completed INTEGER NOT NULL DEFAULT 0,
    tasks_in_progress INTEGER NOT NULL DEFAULT 0,
    streak_day INTEGER NOT NULL DEFAULT 0,
    reflection TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_task_daily_logs_date
    ON task_daily_logs(date DESC);
