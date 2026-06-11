use rusqlite::{functions::FunctionFlags, Connection};
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

/// Insert a space between every adjacent CJK ideograph so that the
/// FTS5 unicode61 tokenizer treats each character as its own token.
/// Without this, a continuous Chinese sequence like "整理设计风格" is
/// indexed as a single mega-token and searches for "设计" miss it.
///
/// English words and digits pass through unchanged, since unicode61
/// already tokenizes them correctly at whitespace/punctuation.
///
/// Idempotent: applying it twice produces the same string.
pub fn cjk_segment(input: &str) -> String {
    fn is_cjk(c: char) -> bool {
        matches!(c,
            '\u{3400}'..='\u{4DBF}' |   // CJK Unified Ideographs Extension A
            '\u{4E00}'..='\u{9FFF}' |   // CJK Unified Ideographs
            '\u{F900}'..='\u{FAFF}' |   // CJK Compatibility Ideographs
            '\u{20000}'..='\u{2FFFF}'   // Extensions B-F (supplementary plane)
        )
    }
    let mut out = String::with_capacity(input.len() + input.len() / 4);
    let mut prev_cjk = false;
    for c in input.chars() {
        let cur_cjk = is_cjk(c);
        if cur_cjk && prev_cjk {
            out.push(' ');
        }
        out.push(c);
        prev_cjk = cur_cjk;
    }
    out
}

impl Database {
    /// Register the cjk_seg() SQL function on the given connection.
    /// Must be called once per Connection — used by both the on-disk
    /// and in-memory constructors so the FTS triggers can call it.
    fn register_functions(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
        conn.create_scalar_function(
            "cjk_seg",
            1,
            FunctionFlags::SQLITE_DETERMINISTIC | FunctionFlags::SQLITE_UTF8,
            |ctx| {
                let input: String = ctx.get(0).unwrap_or_default();
                Ok(cjk_segment(&input))
            },
        )?;
        Ok(())
    }

    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db_path = Self::get_db_path()?;

        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(&db_path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        Self::register_functions(&conn)?;

        let db = Database {
            conn: Mutex::new(conn),
        };
        db.run_migrations()?;

        Ok(db)
    }

    /// Create an in-memory database for testing.
    pub fn new_in_memory() -> Result<Self, Box<dyn std::error::Error>> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        Self::register_functions(&conn)?;
        let db = Database {
            conn: Mutex::new(conn),
        };
        db.run_migrations()?;
        Ok(db)
    }

    fn get_db_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let data_dir = dirs::data_dir()
            .ok_or("Could not find data directory")?
            .join("com.learnwiki.app");
        Ok(data_dir.join("learnwiki.db"))
    }

    fn run_migrations(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.conn.lock().map_err(|e| format!("Lock error: {}", e))?;

        let migration_sql = include_str!("migrations/001_initial.sql");
        conn.execute_batch(migration_sql)?;

        // Migration 002: Add user_note column (idempotent check)
        let has_user_note: bool = conn
            .prepare("SELECT COUNT(*) FROM pragma_table_info('captured_content') WHERE name = 'user_note'")?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|count| count > 0)
            .unwrap_or(false);

        if !has_user_note {
            let migration_002 = include_str!("migrations/002_add_user_note.sql");
            conn.execute_batch(migration_002)?;
            log::info!("Migration 002 applied: added user_note column");
        }

        // Migration 003: Add chat_messages table
        let has_chat_messages: bool = conn
            .prepare(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='chat_messages'",
            )?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|count| count > 0)
            .unwrap_or(false);

        if !has_chat_messages {
            let migration_003 = include_str!("migrations/003_add_chat_messages.sql");
            conn.execute_batch(migration_003)?;
            log::info!("Migration 003 applied: added chat_messages table");
        }

        // Migration 004: Add digest fields (digested_at, digest_action)
        let has_digested_at: bool = conn
            .prepare("SELECT COUNT(*) FROM pragma_table_info('captured_content') WHERE name = 'digested_at'")?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|count| count > 0)
            .unwrap_or(false);

        if !has_digested_at {
            let migration_004 = include_str!("migrations/004_add_digest_fields.sql");
            conn.execute_batch(migration_004)?;
            log::info!("Migration 004 applied: added digest fields");
        }

        // Migration 005: Add attention_insights table
        let has_attention_insights: bool = conn
            .prepare("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='attention_insights'")?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|count| count > 0)
            .unwrap_or(false);

        if !has_attention_insights {
            let migration_005 = include_str!("migrations/005_add_attention_insights.sql");
            conn.execute_batch(migration_005)?;
            log::info!("Migration 005 applied: added attention_insights table");
        }

        // Migration 006: Add summary and tags columns to captured_content
        let has_summary: bool = conn
            .prepare(
                "SELECT COUNT(*) FROM pragma_table_info('captured_content') WHERE name='summary'",
            )?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|c| c > 0)
            .unwrap_or(false);
        let has_tags: bool = conn
            .prepare(
                "SELECT COUNT(*) FROM pragma_table_info('captured_content') WHERE name='tags'",
            )?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|c| c > 0)
            .unwrap_or(false);

        if !has_summary || !has_tags {
            if !has_summary {
                conn.execute_batch("ALTER TABLE captured_content ADD COLUMN summary TEXT;")?;
            }
            if !has_tags {
                conn.execute_batch("ALTER TABLE captured_content ADD COLUMN tags TEXT;")?;
            }
            log::info!("Migration 006 applied: added summary/tags columns");
        }

        // Migration 007: Add digest column to captured_content
        let has_digest: bool = conn
            .prepare(
                "SELECT COUNT(*) FROM pragma_table_info('captured_content') WHERE name='digest'",
            )?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|c| c > 0)
            .unwrap_or(false);

        if !has_digest {
            conn.execute_batch("ALTER TABLE captured_content ADD COLUMN digest TEXT;")?;
            log::info!("Migration 007 applied: added digest column");
        }

        // Migration 008: Add wiki tables
        let has_wiki_pages: bool = conn
            .prepare("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='wiki_pages'")?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|count| count > 0)
            .unwrap_or(false);

        if !has_wiki_pages {
            let migration_008 = include_str!("migrations/008_add_wiki.sql");
            conn.execute_batch(migration_008)?;
            log::info!("Migration 008 applied: added wiki tables");
        }

        // Migration 009: Add wiki hash columns to captured_content
        let has_wiki_compile_hash: bool = conn
            .prepare(
                "SELECT COUNT(*) FROM pragma_table_info('captured_content') WHERE name='wiki_compile_hash'",
            )?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|c| c > 0)
            .unwrap_or(false);

        if !has_wiki_compile_hash {
            conn.execute_batch(
                "ALTER TABLE captured_content ADD COLUMN wiki_compile_hash TEXT;
                 ALTER TABLE captured_content ADD COLUMN wiki_assessed_hash TEXT;",
            )?;
            log::info!("Migration 009 applied: added wiki hash columns");
        }

        // Migration 010: Add multi-turn chat tables
        let has_chat_sessions: bool = conn
            .prepare(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='wiki_chat_sessions'",
            )?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|count| count > 0)
            .unwrap_or(false);

        if !has_chat_sessions {
            let migration_010 = include_str!("migrations/010_add_chat_tables.sql");
            conn.execute_batch(migration_010)?;
            log::info!("Migration 010 applied: added chat session tables");
        }

        // Migration 011: Add source_message_id to wiki_pages
        let has_source_message_id: bool = conn
            .prepare(
                "SELECT COUNT(*) FROM pragma_table_info('wiki_pages') WHERE name='source_message_id'",
            )?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|c| c > 0)
            .unwrap_or(false);

        if !has_source_message_id {
            let migration_011 = include_str!("migrations/011_add_source_message_id.sql");
            conn.execute_batch(migration_011)?;
            log::info!("Migration 011 applied: added source_message_id to wiki_pages");
        }

        // Migration 012: Add clean_content column to captured_content
        let has_clean_content: bool = conn
            .prepare(
                "SELECT COUNT(*) FROM pragma_table_info('captured_content') WHERE name='clean_content'",
            )?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|c| c > 0)
            .unwrap_or(false);

        if !has_clean_content {
            conn.execute_batch("ALTER TABLE captured_content ADD COLUMN clean_content TEXT;")?;
            log::info!("Migration 012 applied: added clean_content column");
        }

        // Migration 013: Add locale columns to content and AI-generated tables
        let has_content_locale: bool = conn
            .prepare(
                "SELECT COUNT(*) FROM pragma_table_info('captured_content') WHERE name='locale'",
            )?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|c| c > 0)
            .unwrap_or(false);

        if !has_content_locale {
            conn.execute_batch(
                "ALTER TABLE captured_content ADD COLUMN locale TEXT NOT NULL DEFAULT 'zh-CN';",
            )?;
            // weekly_reports may not exist yet in some setups, so check first
            let has_reports: bool = conn
                .prepare("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='weekly_reports'")
                .and_then(|mut s| s.query_row([], |row| row.get::<_, i32>(0)))
                .map(|c| c > 0)
                .unwrap_or(false);
            if has_reports {
                conn.execute_batch(
                    "ALTER TABLE weekly_reports ADD COLUMN locale TEXT NOT NULL DEFAULT 'zh-CN';",
                )?;
            }
            let has_insights: bool = conn
                .prepare("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='attention_insights'")
                .and_then(|mut s| s.query_row([], |row| row.get::<_, i32>(0)))
                .map(|c| c > 0)
                .unwrap_or(false);
            if has_insights {
                conn.execute_batch(
                    "ALTER TABLE attention_insights ADD COLUMN locale TEXT NOT NULL DEFAULT 'zh-CN';",
                )?;
            }
            let has_wiki: bool = conn
                .prepare(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='wiki_pages'",
                )
                .and_then(|mut s| s.query_row([], |row| row.get::<_, i32>(0)))
                .map(|c| c > 0)
                .unwrap_or(false);
            if has_wiki {
                conn.execute_batch(
                    "ALTER TABLE wiki_pages ADD COLUMN locale TEXT NOT NULL DEFAULT 'zh-CN';",
                )?;
            }
            log::info!("Migration 013 applied: added locale columns");
        }

        // Migration 014: Add FTS5 virtual table for wiki_pages.
        // Wrapped in fallible block — if FTS5 is unavailable in the sqlite
        // build (shouldn't happen with rusqlite "bundled"), we log and
        // continue in degraded mode. The repository layer will detect the
        // missing table and fall back to LIKE-based search.
        let has_wiki_fts: bool = conn
            .prepare(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='wiki_pages_fts'",
            )?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|c| c > 0)
            .unwrap_or(false);

        if !has_wiki_fts {
            let migration_014 = include_str!("migrations/014_add_wiki_fts.sql");
            match conn.execute_batch(migration_014) {
                Ok(_) => log::info!("Migration 014 applied: added wiki_pages_fts"),
                Err(e) => log::warn!(
                    "Migration 014 skipped (FTS5 unavailable, falling back to LIKE search): {}",
                    e
                ),
            }
        }

        // Migration 015: rebuild FTS with CJK character segmentation.
        // We detect "already applied" by checking whether the trigger
        // body references cjk_seg — that's the marker of v2 layout.
        let needs_015: bool = conn
            .prepare(
                "SELECT COUNT(*) FROM sqlite_master \
                 WHERE type='trigger' AND name='wiki_pages_fts_insert' \
                 AND sql LIKE '%cjk_seg%'",
            )
            .and_then(|mut s| s.query_row([], |row| row.get::<_, i32>(0)))
            .map(|c| c == 0)
            .unwrap_or(true);

        if needs_015 {
            let migration_015 = include_str!("migrations/015_wiki_fts_cjk_segment.sql");
            match conn.execute_batch(migration_015) {
                Ok(_) => log::info!(
                    "Migration 015 applied: rebuilt wiki_pages_fts with CJK segmentation"
                ),
                Err(e) => log::warn!(
                    "Migration 015 skipped (FTS5 unavailable, keeping legacy index): {}",
                    e
                ),
            }
        }

        // Migration 016: bump stale Anthropic model IDs to current 4.X
        // family. The old dated IDs (claude-sonnet-4-20250514 etc.) are
        // discontinued — leaving them as the saved default would cause
        // every API call to fail with "model not found". We rewrite
        // exact matches only; user-chosen custom IDs are left alone.
        let _ = conn.execute(
            "UPDATE app_settings SET value = 'claude-sonnet-4-6' \
             WHERE key = 'ai_model' AND value = 'claude-sonnet-4-20250514'",
            [],
        );
        let _ = conn.execute(
            "UPDATE app_settings SET value = 'claude-opus-4-7' \
             WHERE key = 'ai_model' AND value = 'claude-opus-4-20250514'",
            [],
        );
        let _ = conn.execute(
            "UPDATE app_settings SET value = 'claude-haiku-4-5-20251001' \
             WHERE key = 'ai_model' AND value = 'claude-3-5-haiku-20241022'",
            [],
        );

        // Migration 017: Create learning data tables (LearningPath, Module, PracticeTask, TaskDailyLog)
        let has_learning_paths: bool = conn
            .prepare(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='learning_paths'",
            )?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|count| count > 0)
            .unwrap_or(false);

        if !has_learning_paths {
            let migration_017 = include_str!("migrations/017_add_learning_tables.sql");
            conn.execute_batch(migration_017)?;
            log::info!("Migration 017 applied: created learning data tables");
        }

        // Migration 018: Create task_wiki_links table
        let has_task_wiki_links: bool = conn
            .prepare(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='task_wiki_links'",
            )?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|count| count > 0)
            .unwrap_or(false);

        if !has_task_wiki_links {
            let migration_018 = include_str!("migrations/018_add_task_wiki_links.sql");
            conn.execute_batch(migration_018)?;
            log::info!("Migration 018 applied: created task_wiki_links table");
        }

        // Migration 019: Create task_solutions table
        let has_task_solutions: bool = conn
            .prepare(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='task_solutions'",
            )?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|count| count > 0)
            .unwrap_or(false);

        if !has_task_solutions {
            let migration_019 = include_str!("migrations/019_add_task_solutions_and_author.sql");
            conn.execute_batch(migration_019)?;
            log::info!("Migration 019 applied: created task_solutions table");
        }

        // Migration 020: Add source fields (author_name, author_url, source_type, source_task_id) to wiki_pages
        let has_author_name: bool = conn
            .prepare(
                "SELECT COUNT(*) FROM pragma_table_info('wiki_pages') WHERE name='author_name'",
            )?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|c| c > 0)
            .unwrap_or(false);

        if !has_author_name {
            let migration_020 = include_str!("migrations/020_add_wiki_page_source_fields.sql");
            conn.execute_batch(migration_020)?;
            log::info!("Migration 020 applied: added source fields to wiki_pages");
        }

        // Migration 021: Create task_recommendations table
        let has_task_recommendations: bool = conn
            .prepare(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='task_recommendations'",
            )?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|count| count > 0)
            .unwrap_or(false);

        if !has_task_recommendations {
            let migration_021 = include_str!("migrations/021_create_task_recommendations.sql");
            conn.execute_batch(migration_021)?;
            log::info!("Migration 021 applied: created task_recommendations table");
        }

        // Migration 022: Create review tables (Sprint 4, E-4-1)
        let has_review_schedule: bool = conn
            .prepare(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='review_schedule'",
            )?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|count| count > 0)
            .unwrap_or(false);

        if !has_review_schedule {
            let migration_022 = include_str!("migrations/022_add_review_tables.sql");
            conn.execute_batch(migration_022)?;
            log::info!("Migration 022 applied: created review tables");
        }

        // Migration 023: Add review format fields (Sprint 5A, E-5-5)
        let has_last_format: bool = conn
            .prepare(
                "SELECT COUNT(*) FROM pragma_table_info('review_schedule') WHERE name='last_format'",
            )?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|c| c > 0)
            .unwrap_or(false);

        if !has_last_format {
            let migration_023 = include_str!("migrations/023_add_review_format.sql");
            conn.execute_batch(migration_023)?;
            log::info!("Migration 023 applied: added review format fields");
        }

        // Migration 024: Add variant tracking fields (Sprint 6A, E-6-3)
        let has_variant_streak: bool = conn
            .prepare(
                "SELECT COUNT(*) FROM pragma_table_info('review_schedule') WHERE name='variant_streak'",
            )?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|c| c > 0)
            .unwrap_or(false);

        if !has_variant_streak {
            let migration_024 = include_str!("migrations/024_add_variant_tracking.sql");
            conn.execute_batch(migration_024)?;
            log::info!("Migration 024 applied: added variant tracking fields");
        }

        // Migration 025: Add discovery tables (Phase 7, E-7-1)
        let has_pending_content: bool = conn
            .prepare(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='pending_content'",
            )?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|count| count > 0)
            .unwrap_or(false);

        if !has_pending_content {
            let migration_025 = include_str!("migrations/025_add_discovery_tables.sql");
            conn.execute_batch(migration_025)?;
            log::info!("Migration 025 applied: added discovery tables and fields");
        }

        // Migration 026: Add discovery_suppression table (E-7-7)
        let has_suppression: bool = conn
            .prepare(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='discovery_suppression'",
            )?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|count| count > 0)
            .unwrap_or(false);

        if !has_suppression {
            let migration_026 = include_str!("migrations/026_add_discovery_suppression.sql");
            conn.execute_batch(migration_026)?;
            log::info!("Migration 026 applied: added discovery_suppression table");
        }

        // Migration 027: Add goals and goal_wiki_links tables
        let has_goals: bool = conn
            .prepare("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='goals'")?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|count| count > 0)
            .unwrap_or(false);

        if !has_goals {
            let migration_027 = include_str!("migrations/027_add_goals.sql");
            conn.execute_batch(migration_027)?;
            log::info!("Migration 027 applied: added goals and goal_wiki_links tables");
        }

        // Migration 028: Add exam tables
        let has_exams: bool = conn
            .prepare("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='exams'")?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|count| count > 0)
            .unwrap_or(false);

        if !has_exams {
            let migration_028 = include_str!("migrations/028_add_exam_tables.sql");
            conn.execute_batch(migration_028)?;
            log::info!("Migration 028 applied: added exam tables");
        }

        // Migration 029: Add goal_recommendations table
        let has_goal_recommendations: bool = conn
            .prepare("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='goal_recommendations'")?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|count| count > 0)
            .unwrap_or(false);

        if !has_goal_recommendations {
            let migration_029 = include_str!("migrations/029_add_goal_recommendations.sql");
            conn.execute_batch(migration_029)?;
            log::info!("Migration 029 applied: added goal_recommendations table");
        }

        // Migration 030: Add sync tables
        let has_sync_folders: bool = conn
            .prepare("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='sync_folders'")?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|count| count > 0)
            .unwrap_or(false);

        if !has_sync_folders {
            let migration_030 = include_str!("migrations/030_add_sync_tables.sql");
            conn.execute_batch(migration_030)?;
            log::info!("Migration 030 applied: added sync tables");
        }

        // Migration 031: Drop old learning system tables (replaced by Goal-based system)
        // Run only if old tables still exist
        let table_exists: bool = conn
            .query_row(
                "SELECT COUNT(*) > 0 FROM sqlite_master WHERE type='table' AND name='learning_paths'",
                [],
                |row| row.get(0),
            )
            .unwrap_or(false);
        if table_exists {
            let migration_031 = include_str!("migrations/031_drop_old_learning_tables.sql");
            conn.execute_batch(migration_031)?;
            log::info!("Migration 031 applied: dropped old learning tables");
        }

        // Migration 032: Add wiki_reading_status table
        let has_wiki_reading_status: bool = conn
            .prepare("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='wiki_reading_status'")?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|count| count > 0)
            .unwrap_or(false);

        if !has_wiki_reading_status {
            let migration_032 = include_str!("migrations/032_add_wiki_reading_status.sql");
            conn.execute_batch(migration_032)?;
            log::info!("Migration 032 applied: added wiki_reading_status table");
        }

        // Migration 033: Add version and question_config columns to exams
        let has_exam_version: bool = conn
            .prepare("SELECT COUNT(*) FROM pragma_table_info('exams') WHERE name = 'version'")?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|c| c > 0)
            .unwrap_or(false);

        if !has_exam_version {
            let migration_033 = include_str!("migrations/033_add_exam_version.sql");
            conn.execute_batch(migration_033)?;
            log::info!("Migration 033 applied: added version and question_config to exams");
        }

        // Migration 034: Create wiki_mastery_flags table
        let has_mastery_flags: bool = conn
            .prepare("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='wiki_mastery_flags'")?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|count| count > 0)
            .unwrap_or(false);

        if !has_mastery_flags {
            let migration_034 = include_str!("migrations/034_add_mastery_flags.sql");
            conn.execute_batch(migration_034)?;
            log::info!("Migration 034 applied: created wiki_mastery_flags table");
        }

        // Migration 035: Add question_snapshot column to review_logs
        let has_question_snapshot: bool = conn
            .prepare("SELECT COUNT(*) FROM pragma_table_info('review_logs') WHERE name = 'question_snapshot'")?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|c| c > 0)
            .unwrap_or(false);

        if !has_question_snapshot {
            let migration_035 = include_str!("migrations/035_add_question_snapshot.sql");
            conn.execute_batch(migration_035)?;
            log::info!("Migration 035 applied: added question_snapshot to review_logs");
        }

        // Migration 036: Add session_id column to review_logs
        let has_session_id: bool = conn
            .prepare("SELECT COUNT(*) FROM pragma_table_info('review_logs') WHERE name = 'session_id'")?
            .query_row([], |row| row.get::<_, i32>(0))
            .map(|c| c > 0)
            .unwrap_or(false);

        if !has_session_id {
            let migration_036 = include_str!("migrations/036_add_review_session_id.sql");
            conn.execute_batch(migration_036)?;
            log::info!("Migration 036 applied: added session_id to review_logs");
        }

        log::info!("Database migrations completed successfully");
        Ok(())
    }
}
