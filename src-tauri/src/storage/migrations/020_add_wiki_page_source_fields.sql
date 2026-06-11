-- Migration 020: Add source fields (author_name, author_url, source_type, source_task_id)
-- to wiki_pages for E-2-6 (author attribution) and E-3-2 (task experience → wiki).

ALTER TABLE wiki_pages ADD COLUMN author_name TEXT;
ALTER TABLE wiki_pages ADD COLUMN author_url TEXT;
ALTER TABLE wiki_pages ADD COLUMN source_type TEXT NOT NULL DEFAULT 'user';
ALTER TABLE wiki_pages ADD COLUMN source_task_id TEXT;
