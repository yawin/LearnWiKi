-- goals 表
CREATE TABLE IF NOT EXISTS goals (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    keywords TEXT NOT NULL DEFAULT '[]',
    status TEXT NOT NULL DEFAULT 'active' CHECK(status IN ('active', 'achieved', 'archived')),
    progress REAL NOT NULL DEFAULT 0.0 CHECK(progress >= 0.0 AND progress <= 100.0),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_goals_status ON goals(status);

-- goal_wiki_links 表
CREATE TABLE IF NOT EXISTS goal_wiki_links (
    id TEXT PRIMARY KEY NOT NULL,
    goal_id TEXT NOT NULL REFERENCES goals(id) ON DELETE CASCADE,
    wiki_page_id TEXT NOT NULL,
    relevance_score REAL NOT NULL DEFAULT 0.0,
    source TEXT NOT NULL DEFAULT 'auto' CHECK(source IN ('auto', 'manual')),
    is_new INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    UNIQUE(goal_id, wiki_page_id)
);
CREATE INDEX IF NOT EXISTS idx_goal_wiki_links_goal ON goal_wiki_links(goal_id);
CREATE INDEX IF NOT EXISTS idx_goal_wiki_links_wiki ON goal_wiki_links(wiki_page_id);
