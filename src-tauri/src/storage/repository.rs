use super::database::Database;
use super::models::{
    AdaptiveRecommendation, AttentionInsight, CapturedContent, ContentForAnalysis, ContentType,
    KnowledgeMonitorSource, PendingContent, ReportSection, UserFeedback, UserPreference, WeeklyReport,
};
use rusqlite::params;
use serde_json;
use std::sync::Arc;

pub struct Repository {
    db: Arc<Database>,
}

impl Repository {
    pub fn new(db: Arc<Database>) -> Self {
        Repository { db }
    }

    // ========== Captured Content ==========

    pub fn save_content(
        &self,
        content: &CapturedContent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "INSERT INTO captured_content (id, content_type, raw_text, image_path, thumbnail_path, source_app, source_bundle_id, source_url, user_note, captured_at, content_hash, byte_size, is_deleted, created_at, updated_at, digested_at, digest_action, summary, tags, digest, wiki_compile_hash, wiki_assessed_hash, clean_content)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23)",
            params![
                content.id,
                content.content_type.as_str(),
                content.raw_text,
                content.image_path,
                content.thumbnail_path,
                content.source_app,
                content.source_bundle_id,
                content.source_url,
                content.user_note,
                content.captured_at,
                content.content_hash,
                content.byte_size,
                content.is_deleted,
                content.created_at,
                content.updated_at,
                content.digested_at,
                content.digest_action,
                content.summary,
                content.tags,
                content.digest,
                content.wiki_compile_hash,
                content.wiki_assessed_hash,
                content.clean_content,
            ],
        )?;
        Ok(())
    }

    pub fn get_all_content(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<CapturedContent>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, content_type, raw_text, image_path, thumbnail_path, source_app, source_bundle_id, source_url, user_note, captured_at, content_hash, byte_size, is_deleted, created_at, updated_at, digested_at, digest_action, summary, tags, digest, wiki_compile_hash, wiki_assessed_hash, clean_content
             FROM captured_content WHERE is_deleted = 0 ORDER BY captured_at DESC LIMIT ?1 OFFSET ?2"
        )?;

        let rows = stmt.query_map(params![limit, offset], |row| {
            Ok(CapturedContent {
                id: row.get(0)?,
                content_type: ContentType::from_str(&row.get::<_, String>(1)?),
                raw_text: row.get(2)?,
                image_path: row.get(3)?,
                thumbnail_path: row.get(4)?,
                source_app: row.get(5)?,
                source_bundle_id: row.get(6)?,
                source_url: row.get(7)?,
                user_note: row.get(8)?,
                captured_at: row.get(9)?,
                content_hash: row.get(10)?,
                byte_size: row.get(11)?,
                is_deleted: row.get::<_, i32>(12)? != 0,
                created_at: row.get(13)?,
                updated_at: row.get(14)?,
                digested_at: row.get(15).unwrap_or(None),
                digest_action: row.get(16).unwrap_or(None),
                summary: row.get(17).unwrap_or(None),
                tags: row.get(18).unwrap_or(None),
                digest: row.get(19).unwrap_or(None),
                wiki_compile_hash: row.get(20).unwrap_or(None),
                wiki_assessed_hash: row.get(21).unwrap_or(None),
                clean_content: row.get(22).unwrap_or(None),
            })
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    /// Search content by keyword across raw_text, source_url, source_app, and user_note.
    pub fn search_content(
        &self,
        query: &str,
        limit: i64,
    ) -> Result<Vec<CapturedContent>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let pattern = format!("%{}%", query);
        let mut stmt = conn.prepare(
            "SELECT id, content_type, raw_text, image_path, thumbnail_path, source_app, source_bundle_id, source_url, user_note, captured_at, content_hash, byte_size, is_deleted, created_at, updated_at, digested_at, digest_action, summary, tags, digest, wiki_compile_hash, wiki_assessed_hash, clean_content
             FROM captured_content
             WHERE is_deleted = 0
               AND (raw_text LIKE ?1 OR source_url LIKE ?1 OR source_app LIKE ?1 OR user_note LIKE ?1)
             ORDER BY captured_at DESC LIMIT ?2"
        )?;

        let rows = stmt.query_map(params![pattern, limit], |row| {
            Ok(CapturedContent {
                id: row.get(0)?,
                content_type: ContentType::from_str(&row.get::<_, String>(1)?),
                raw_text: row.get(2)?,
                image_path: row.get(3)?,
                thumbnail_path: row.get(4)?,
                source_app: row.get(5)?,
                source_bundle_id: row.get(6)?,
                source_url: row.get(7)?,
                user_note: row.get(8)?,
                captured_at: row.get(9)?,
                content_hash: row.get(10)?,
                byte_size: row.get(11)?,
                is_deleted: row.get::<_, i32>(12)? != 0,
                created_at: row.get(13)?,
                updated_at: row.get(14)?,
                digested_at: row.get(15).unwrap_or(None),
                digest_action: row.get(16).unwrap_or(None),
                summary: row.get(17).unwrap_or(None),
                tags: row.get(18).unwrap_or(None),
                digest: row.get(19).unwrap_or(None),
                wiki_compile_hash: row.get(20).unwrap_or(None),
                wiki_assessed_hash: row.get(21).unwrap_or(None),
                clean_content: row.get(22).unwrap_or(None),
            })
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    /// Update the raw_text and source_url of an existing content item.
    /// Used by the URL reader to fill in fetched article content.
    pub fn update_content_for_url(
        &self,
        id: &str,
        raw_text: &str,
        source_url: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE captured_content SET raw_text = ?1, source_url = ?2, byte_size = ?3, updated_at = datetime('now') WHERE id = ?4",
            params![raw_text, source_url, raw_text.len() as i64, id],
        )?;
        Ok(())
    }

    /// Move a content item to the top by updating its captured_at to now.
    pub fn touch_captured_at(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let now = chrono::Utc::now().to_rfc3339();
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE captured_content SET captured_at = ?1, updated_at = datetime('now') WHERE id = ?2",
            rusqlite::params![now, id],
        )?;
        Ok(())
    }

    /// Update the AI-generated summary, tags, and digest for a content item.
    pub fn update_summary_and_tags(
        &self,
        id: &str,
        summary: &str,
        tags: &str,
        digest: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE captured_content SET summary = ?1, tags = ?2, digest = ?3, updated_at = datetime('now') WHERE id = ?4",
            rusqlite::params![summary, tags, digest, id],
        )?;
        Ok(())
    }

    /// Update the AI-cleaned content and optionally clear wiki hash to trigger recompilation.
    pub fn update_clean_content(
        &self,
        id: &str,
        clean_content: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE captured_content SET clean_content = ?1, wiki_assessed_hash = NULL, updated_at = datetime('now') WHERE id = ?2",
            rusqlite::params![clean_content, id],
        )?;
        Ok(())
    }

    /// Update the raw_text of an existing content item (e.g. OCR result for images).
    pub fn update_raw_text(
        &self,
        id: &str,
        raw_text: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE captured_content SET raw_text = ?1, byte_size = ?2, updated_at = datetime('now') WHERE id = ?3",
            params![raw_text, raw_text.len() as i64, id],
        )?;
        Ok(())
    }

    pub fn delete_content(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE captured_content SET is_deleted = 1, updated_at = datetime('now') WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    /// Update the user_note for an existing content item.
    pub fn update_user_note(&self, id: &str, note: &str) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE captured_content SET user_note = ?1, updated_at = datetime('now') WHERE id = ?2",
            params![note, id],
        )?;
        Ok(())
    }

    /// Find an existing content item by its content_hash (for dedup in spotlight).
    pub fn find_content_by_hash(
        &self,
        hash: &str,
    ) -> Result<Option<CapturedContent>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, content_type, raw_text, image_path, thumbnail_path, source_app, source_bundle_id, source_url, user_note, captured_at, content_hash, byte_size, is_deleted, created_at, updated_at, digested_at, digest_action, summary, tags, digest, wiki_compile_hash, wiki_assessed_hash, clean_content
             FROM captured_content WHERE content_hash = ?1 AND is_deleted = 0 LIMIT 1"
        )?;

        let mut rows = stmt.query_map(params![hash], |row| {
            Ok(CapturedContent {
                id: row.get(0)?,
                content_type: ContentType::from_str(&row.get::<_, String>(1)?),
                raw_text: row.get(2)?,
                image_path: row.get(3)?,
                thumbnail_path: row.get(4)?,
                source_app: row.get(5)?,
                source_bundle_id: row.get(6)?,
                source_url: row.get(7)?,
                user_note: row.get(8)?,
                captured_at: row.get(9)?,
                content_hash: row.get(10)?,
                byte_size: row.get(11)?,
                is_deleted: row.get::<_, i32>(12)? != 0,
                created_at: row.get(13)?,
                updated_at: row.get(14)?,
                digested_at: row.get(15).unwrap_or(None),
                digest_action: row.get(16).unwrap_or(None),
                summary: row.get(17).unwrap_or(None),
                tags: row.get(18).unwrap_or(None),
                digest: row.get(19).unwrap_or(None),
                wiki_compile_hash: row.get(20).unwrap_or(None),
                wiki_assessed_hash: row.get(21).unwrap_or(None),
                clean_content: row.get(22).unwrap_or(None),
            })
        })?;

        match rows.next() {
            Some(row) => Ok(Some(row?)),
            None => Ok(None),
        }
    }

    pub fn content_exists_by_hash(&self, hash: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM captured_content WHERE content_hash = ?1 AND is_deleted = 0",
            params![hash],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    /// Get all content captured between week_start and week_end (inclusive).
    /// Dates should be in ISO 8601 / RFC 3339 format (e.g. "2025-01-06T00:00:00+00:00").
    pub fn get_content_for_week(
        &self,
        week_start: &str,
        week_end: &str,
    ) -> Result<Vec<CapturedContent>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, content_type, raw_text, image_path, thumbnail_path, source_app, source_bundle_id, source_url, user_note, captured_at, content_hash, byte_size, is_deleted, created_at, updated_at, digested_at, digest_action, summary, tags, digest, wiki_compile_hash, wiki_assessed_hash, clean_content
             FROM captured_content
             WHERE is_deleted = 0 AND captured_at >= ?1 AND captured_at <= ?2
             ORDER BY captured_at DESC"
        )?;

        let rows = stmt.query_map(params![week_start, week_end], |row| {
            Ok(CapturedContent {
                id: row.get(0)?,
                content_type: ContentType::from_str(&row.get::<_, String>(1)?),
                raw_text: row.get(2)?,
                image_path: row.get(3)?,
                thumbnail_path: row.get(4)?,
                source_app: row.get(5)?,
                source_bundle_id: row.get(6)?,
                source_url: row.get(7)?,
                user_note: row.get(8)?,
                captured_at: row.get(9)?,
                content_hash: row.get(10)?,
                byte_size: row.get(11)?,
                is_deleted: row.get::<_, i32>(12)? != 0,
                created_at: row.get(13)?,
                updated_at: row.get(14)?,
                digested_at: row.get(15).unwrap_or(None),
                digest_action: row.get(16).unwrap_or(None),
                summary: row.get(17).unwrap_or(None),
                tags: row.get(18).unwrap_or(None),
                digest: row.get(19).unwrap_or(None),
                wiki_compile_hash: row.get(20).unwrap_or(None),
                wiki_assessed_hash: row.get(21).unwrap_or(None),
                clean_content: row.get(22).unwrap_or(None),
            })
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    /// Get a single content item by its ID.
    pub fn get_content_by_id(
        &self,
        id: &str,
    ) -> Result<Option<CapturedContent>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, content_type, raw_text, image_path, thumbnail_path, source_app, source_bundle_id, source_url, user_note, captured_at, content_hash, byte_size, is_deleted, created_at, updated_at, digested_at, digest_action, summary, tags, digest, wiki_compile_hash, wiki_assessed_hash, clean_content
             FROM captured_content WHERE id = ?1 AND is_deleted = 0"
        )?;

        let mut rows = stmt.query_map(params![id], |row| {
            Ok(CapturedContent {
                id: row.get(0)?,
                content_type: ContentType::from_str(&row.get::<_, String>(1)?),
                raw_text: row.get(2)?,
                image_path: row.get(3)?,
                thumbnail_path: row.get(4)?,
                source_app: row.get(5)?,
                source_bundle_id: row.get(6)?,
                source_url: row.get(7)?,
                user_note: row.get(8)?,
                captured_at: row.get(9)?,
                content_hash: row.get(10)?,
                byte_size: row.get(11)?,
                is_deleted: row.get::<_, i32>(12)? != 0,
                created_at: row.get(13)?,
                updated_at: row.get(14)?,
                digested_at: row.get(15).unwrap_or(None),
                digest_action: row.get(16).unwrap_or(None),
                summary: row.get(17).unwrap_or(None),
                tags: row.get(18).unwrap_or(None),
                digest: row.get(19).unwrap_or(None),
                wiki_compile_hash: row.get(20).unwrap_or(None),
                wiki_assessed_hash: row.get(21).unwrap_or(None),
                clean_content: row.get(22).unwrap_or(None),
            })
        })?;

        match rows.next() {
            Some(row) => Ok(Some(row?)),
            None => Ok(None),
        }
    }

    // ========== Weekly Reports ==========

    /// Save a complete weekly report with its sections to the database.
    pub fn save_report(&self, report: &WeeklyReport) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;

        // Insert the report
        conn.execute(
            "INSERT OR REPLACE INTO weekly_reports (id, week_start, week_end, summary_text, report_json, content_count, model_used, tokens_used, generated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                report.id,
                report.week_start,
                report.week_end,
                report.summary_text,
                report.report_json.to_string(),
                report.content_count,
                report.model_used,
                report.tokens_used,
                report.generated_at,
            ],
        )?;

        // Delete old sections for this report (in case of regeneration)
        conn.execute(
            "DELETE FROM report_sections WHERE report_id = ?1",
            params![report.id],
        )?;

        // Insert sections
        for section in &report.sections {
            let content_ids_json =
                serde_json::to_string(&section.content_ids).unwrap_or_else(|_| "[]".to_string());

            conn.execute(
                "INSERT INTO report_sections (id, report_id, section_type, title, body, relevance_score, sort_order, content_ids)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    section.id,
                    section.report_id,
                    section.section_type,
                    section.title,
                    section.body,
                    section.relevance_score,
                    section.sort_order,
                    content_ids_json,
                ],
            )?;
        }

        Ok(())
    }

    /// Get a weekly report for a specific week_start date.
    pub fn get_report_by_week(
        &self,
        week_start: &str,
    ) -> Result<Option<WeeklyReport>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT id, week_start, week_end, summary_text, report_json, content_count, model_used, tokens_used, generated_at
             FROM weekly_reports WHERE week_start = ?1"
        )?;

        let mut rows = stmt.query_map(params![week_start], |row| {
            let report_json_str: String = row.get(4)?;
            let report_json: serde_json::Value =
                serde_json::from_str(&report_json_str).unwrap_or(serde_json::Value::Null);

            Ok(WeeklyReport {
                id: row.get(0)?,
                week_start: row.get(1)?,
                week_end: row.get(2)?,
                summary_text: row.get(3)?,
                report_json,
                content_count: row.get(5)?,
                model_used: row.get(6)?,
                tokens_used: row.get(7)?,
                generated_at: row.get(8)?,
                sections: Vec::new(), // filled below
            })
        })?;

        let report = match rows.next() {
            Some(row) => row?,
            None => return Ok(None),
        };

        // Load sections for this report
        let sections = self.get_sections_for_report_inner(&conn, &report.id)?;

        Ok(Some(WeeklyReport { sections, ..report }))
    }

    /// List all weekly reports (without full sections, just metadata).
    pub fn get_all_reports(&self) -> Result<Vec<WeeklyReport>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT id, week_start, week_end, summary_text, report_json, content_count, model_used, tokens_used, generated_at
             FROM weekly_reports ORDER BY week_start DESC"
        )?;

        let rows = stmt.query_map([], |row| {
            let report_json_str: String = row.get(4)?;
            let report_json: serde_json::Value =
                serde_json::from_str(&report_json_str).unwrap_or(serde_json::Value::Null);

            Ok(WeeklyReport {
                id: row.get(0)?,
                week_start: row.get(1)?,
                week_end: row.get(2)?,
                summary_text: row.get(3)?,
                report_json,
                content_count: row.get(5)?,
                model_used: row.get(6)?,
                tokens_used: row.get(7)?,
                generated_at: row.get(8)?,
                sections: Vec::new(),
            })
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    /// Internal helper: load sections for a report using an already-locked connection.
    fn get_sections_for_report_inner(
        &self,
        conn: &rusqlite::Connection,
        report_id: &str,
    ) -> Result<Vec<ReportSection>, Box<dyn std::error::Error>> {
        let mut stmt = conn.prepare(
            "SELECT id, report_id, section_type, title, body, relevance_score, sort_order, content_ids
             FROM report_sections WHERE report_id = ?1 ORDER BY sort_order"
        )?;

        let rows = stmt.query_map(params![report_id], |row| {
            let content_ids_str: Option<String> = row.get(7)?;
            let content_ids: Vec<String> = content_ids_str
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default();

            Ok(ReportSection {
                id: row.get(0)?,
                report_id: row.get(1)?,
                section_type: row.get(2)?,
                title: row.get(3)?,
                body: row.get(4)?,
                relevance_score: row.get(5)?,
                sort_order: row.get(6)?,
                content_ids,
            })
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    // ========== User Feedback ==========

    /// Save user feedback (interested/dismissed/bookmarked) for a content or section.
    pub fn save_feedback(&self, feedback: &UserFeedback) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;

        conn.execute(
            "INSERT INTO user_feedback (id, content_id, section_id, feedback_type, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                feedback.id,
                feedback.content_id,
                feedback.section_id,
                feedback.feedback_type.as_str(),
                feedback.created_at,
            ],
        )?;

        Ok(())
    }

    // ========== User Preferences ==========

    /// Update or insert a topic preference. Increases weight by weight_delta
    /// and increments occurrence_count.
    pub fn update_preference(
        &self,
        topic: &str,
        weight_delta: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;

        // Try to update existing preference
        let rows_updated = conn.execute(
            "UPDATE user_preferences SET weight = weight + ?1, occurrence_count = occurrence_count + 1, last_updated = datetime('now')
             WHERE topic = ?2",
            params![weight_delta, topic],
        )?;

        // If no existing row, insert a new one
        if rows_updated == 0 {
            let id = uuid::Uuid::new_v4().to_string();
            conn.execute(
                "INSERT INTO user_preferences (id, topic, weight, occurrence_count, last_updated)
                 VALUES (?1, ?2, ?3, 1, datetime('now'))",
                params![id, topic, weight_delta],
            )?;
        }

        Ok(())
    }

    /// Get all user preferences ordered by weight descending.
    pub fn get_all_preferences(&self) -> Result<Vec<UserPreference>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT id, topic, weight, occurrence_count, last_updated
             FROM user_preferences ORDER BY weight DESC",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(UserPreference {
                id: row.get(0)?,
                topic: row.get(1)?,
                weight: row.get(2)?,
                occurrence_count: row.get(3)?,
                last_updated: row.get(4)?,
            })
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    // ========== Data Hub ==========

    /// Get all dates that have captured content, with counts.
    pub fn get_dates_with_content(&self) -> Result<Vec<(String, i64)>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT DATE(captured_at) as day, COUNT(*) as cnt FROM captured_content
             WHERE is_deleted = 0 GROUP BY day ORDER BY day DESC",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    /// Get all content for a specific date.
    pub fn get_content_for_date(
        &self,
        date: &str,
    ) -> Result<Vec<CapturedContent>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, content_type, raw_text, image_path, thumbnail_path, source_app, source_bundle_id, source_url, user_note, captured_at, content_hash, byte_size, is_deleted, created_at, updated_at, digested_at, digest_action, summary, tags, digest, wiki_compile_hash, wiki_assessed_hash, clean_content
             FROM captured_content WHERE DATE(captured_at) = ?1 AND is_deleted = 0 ORDER BY captured_at ASC",
        )?;

        let rows = stmt.query_map(params![date], |row| {
            Ok(CapturedContent {
                id: row.get(0)?,
                content_type: ContentType::from_str(&row.get::<_, String>(1)?),
                raw_text: row.get(2)?,
                image_path: row.get(3)?,
                thumbnail_path: row.get(4)?,
                source_app: row.get(5)?,
                source_bundle_id: row.get(6)?,
                source_url: row.get(7)?,
                user_note: row.get(8)?,
                captured_at: row.get(9)?,
                content_hash: row.get(10)?,
                byte_size: row.get(11)?,
                is_deleted: row.get::<_, i32>(12)? != 0,
                created_at: row.get(13)?,
                updated_at: row.get(14)?,
                digested_at: row.get(15).unwrap_or(None),
                digest_action: row.get(16).unwrap_or(None),
                summary: row.get(17).unwrap_or(None),
                tags: row.get(18).unwrap_or(None),
                digest: row.get(19).unwrap_or(None),
                wiki_compile_hash: row.get(20).unwrap_or(None),
                wiki_assessed_hash: row.get(21).unwrap_or(None),
                clean_content: row.get(22).unwrap_or(None),
            })
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    // ========== Digest ==========

    /// Get undigested content items, ordered by oldest first.
    /// Used by the "消化" feature to surface content for review.
    pub fn get_undigested_content(
        &self,
        limit: i64,
    ) -> Result<Vec<CapturedContent>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, content_type, raw_text, image_path, thumbnail_path, source_app, source_bundle_id, source_url, user_note, captured_at, content_hash, byte_size, is_deleted, created_at, updated_at, digested_at, digest_action, summary, tags, digest, wiki_compile_hash, wiki_assessed_hash, clean_content
             FROM captured_content
             WHERE is_deleted = 0 AND digested_at IS NULL
             ORDER BY captured_at ASC LIMIT ?1"
        )?;

        let rows = stmt.query_map(params![limit], |row| {
            Ok(CapturedContent {
                id: row.get(0)?,
                content_type: ContentType::from_str(&row.get::<_, String>(1)?),
                raw_text: row.get(2)?,
                image_path: row.get(3)?,
                thumbnail_path: row.get(4)?,
                source_app: row.get(5)?,
                source_bundle_id: row.get(6)?,
                source_url: row.get(7)?,
                user_note: row.get(8)?,
                captured_at: row.get(9)?,
                content_hash: row.get(10)?,
                byte_size: row.get(11)?,
                is_deleted: row.get::<_, i32>(12)? != 0,
                created_at: row.get(13)?,
                updated_at: row.get(14)?,
                digested_at: row.get(15).unwrap_or(None),
                digest_action: row.get(16).unwrap_or(None),
                summary: row.get(17).unwrap_or(None),
                tags: row.get(18).unwrap_or(None),
                digest: row.get(19).unwrap_or(None),
                wiki_compile_hash: row.get(20).unwrap_or(None),
                wiki_assessed_hash: row.get(21).unwrap_or(None),
                clean_content: row.get(22).unwrap_or(None),
            })
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    /// Get undigested content from the last N days, ordered oldest first.
    pub fn get_undigested_content_recent(
        &self,
        days: i64,
    ) -> Result<Vec<CapturedContent>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, content_type, raw_text, image_path, thumbnail_path, source_app, source_bundle_id, source_url, user_note, captured_at, content_hash, byte_size, is_deleted, created_at, updated_at, digested_at, digest_action, summary, tags, digest, wiki_compile_hash, wiki_assessed_hash, clean_content
             FROM captured_content
             WHERE is_deleted = 0 AND digested_at IS NULL
               AND captured_at >= datetime('now', '-' || ?1 || ' days')
             ORDER BY captured_at ASC"
        )?;

        let rows = stmt.query_map(params![days], |row| {
            Ok(CapturedContent {
                id: row.get(0)?,
                content_type: ContentType::from_str(&row.get::<_, String>(1)?),
                raw_text: row.get(2)?,
                image_path: row.get(3)?,
                thumbnail_path: row.get(4)?,
                source_app: row.get(5)?,
                source_bundle_id: row.get(6)?,
                source_url: row.get(7)?,
                user_note: row.get(8)?,
                captured_at: row.get(9)?,
                content_hash: row.get(10)?,
                byte_size: row.get(11)?,
                is_deleted: row.get::<_, i32>(12)? != 0,
                created_at: row.get(13)?,
                updated_at: row.get(14)?,
                digested_at: row.get(15).unwrap_or(None),
                digest_action: row.get(16).unwrap_or(None),
                summary: row.get(17).unwrap_or(None),
                tags: row.get(18).unwrap_or(None),
                digest: row.get(19).unwrap_or(None),
                wiki_compile_hash: row.get(20).unwrap_or(None),
                wiki_assessed_hash: row.get(21).unwrap_or(None),
                clean_content: row.get(22).unwrap_or(None),
            })
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    /// Mark a content item as digested with the given action (keep/archive/pin).
    pub fn update_digest_action(
        &self,
        id: &str,
        action: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let rows = conn.execute(
            "UPDATE captured_content SET digested_at = datetime('now'), digest_action = ?1, updated_at = datetime('now') WHERE id = ?2 AND is_deleted = 0",
            params![action, id],
        )?;
        if rows == 0 {
            return Err(format!("Content not found: {}", id).into());
        }
        Ok(())
    }

    /// Count total undigested content items.
    pub fn count_undigested(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM captured_content WHERE is_deleted = 0 AND digested_at IS NULL",
            [],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    // ========== App Settings ==========

    /// Get a setting value by key.
    pub fn get_setting(&self, key: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;

        let mut stmt = conn.prepare("SELECT value FROM app_settings WHERE key = ?1")?;
        let mut rows = stmt.query_map(params![key], |row| row.get::<_, String>(0))?;

        match rows.next() {
            Some(row) => Ok(Some(row?)),
            None => Ok(None),
        }
    }

    /// Set a setting value by key (insert or replace).
    pub fn set_setting(&self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;

        conn.execute(
            "INSERT OR REPLACE INTO app_settings (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
        Ok(())
    }

    /// Get all settings as key-value pairs.
    pub fn get_all_settings(
        &self,
    ) -> Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;

        let mut stmt = conn.prepare("SELECT key, value FROM app_settings")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;

        let mut settings = std::collections::HashMap::new();
        for row in rows {
            let (key, value) = row?;
            settings.insert(key, value);
        }
        Ok(settings)
    }

    /// Update a setting value by key.
    pub fn update_setting(&self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;

        conn.execute(
            "INSERT INTO app_settings (key, value, updated_at) VALUES (?1, ?2, datetime('now'))
             ON CONFLICT(key) DO UPDATE SET value = ?2, updated_at = datetime('now')",
            params![key, value],
        )?;
        Ok(())
    }

    // ========== Attention Insights ==========

    /// Get recent content for attention analysis (rich fields for v2).
    pub fn get_recent_content_for_analysis(
        &self,
        days: i64,
        limit: usize,
    ) -> Result<Vec<ContentForAnalysis>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let cutoff = (chrono::Utc::now() - chrono::TimeDelta::days(days)).to_rfc3339();
        let mut stmt = conn.prepare(
            "SELECT id, raw_text, source_url, captured_at, summary, tags, user_note, source_app, content_type
             FROM captured_content
             WHERE is_deleted = 0 AND captured_at >= ?1
             ORDER BY captured_at DESC
             LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![cutoff, limit as i64], |row| {
            Ok(ContentForAnalysis {
                id: row.get(0)?,
                raw_text: row.get(1)?,
                source_url: row.get(2)?,
                captured_at: row.get(3)?,
                summary: row.get(4)?,
                tags: row.get(5)?,
                user_note: row.get(6)?,
                source_app: row.get(7)?,
                content_type: row.get(8)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    /// Compute stats for radar v2 prompt from content items.
    pub fn get_content_stats(items: &[ContentForAnalysis]) -> serde_json::Value {
        use std::collections::HashMap;

        let total = items.len();
        if total == 0 {
            return serde_json::json!({});
        }

        // Source distribution
        let mut source_map: HashMap<&str, usize> = HashMap::new();
        for item in items {
            *source_map.entry(item.source_app.as_str()).or_default() += 1;
        }
        let source_count = source_map.len();
        let sources: Vec<serde_json::Value> = {
            let mut v: Vec<_> = source_map.iter().collect();
            v.sort_by(|a, b| b.1.cmp(a.1));
            v.iter()
                .map(|(name, count)| serde_json::json!({"name": name, "count": count}))
                .collect()
        };

        // Content type distribution
        let mut content_type_map: HashMap<&str, usize> = HashMap::new();
        for item in items {
            *content_type_map
                .entry(item.content_type.as_str())
                .or_default() += 1;
        }
        let content_types: Vec<serde_json::Value> = {
            let mut v: Vec<_> = content_type_map.iter().collect();
            v.sort_by(|a, b| b.1.cmp(a.1));
            v.iter()
                .map(|(name, count)| serde_json::json!({"name": name, "count": count}))
                .collect()
        };

        // Hour distribution
        let (mut morning, mut afternoon, mut evening, mut midnight) = (0usize, 0, 0, 0);
        for item in items {
            // Try to parse hour from ISO timestamp
            if let Some(t_pos) = item.captured_at.find('T') {
                if let Ok(hour) = item.captured_at[t_pos + 1..]
                    .get(..2)
                    .unwrap_or("0")
                    .parse::<u32>()
                {
                    match hour {
                        6..=11 => morning += 1,
                        12..=17 => afternoon += 1,
                        18..=23 => evening += 1,
                        _ => midnight += 1,
                    }
                }
            }
        }

        // Active days + peak day
        let mut day_counts: HashMap<String, usize> = HashMap::new();
        for item in items {
            let day = item.captured_at.get(..10).unwrap_or("").to_string();
            if !day.is_empty() {
                *day_counts.entry(day).or_default() += 1;
            }
        }
        let day_keys: Vec<&str> = day_counts.keys().map(|s| s.as_str()).collect();
        let min_day = day_keys.iter().min().copied().unwrap_or("");
        let max_day = day_keys.iter().max().copied().unwrap_or("");
        let active_days = day_counts.len();
        let total_days = if min_day.len() >= 10 && max_day.len() >= 10 {
            let start = chrono::NaiveDate::parse_from_str(&min_day[..10], "%Y-%m-%d").ok();
            let end = chrono::NaiveDate::parse_from_str(&max_day[..10], "%Y-%m-%d").ok();
            match (start, end) {
                (Some(s), Some(e)) => ((e - s).num_days().max(0) + 1) as usize,
                _ => active_days,
            }
        } else {
            active_days
        };

        let peak_day = day_counts
            .iter()
            .max_by_key(|(_, c)| *c)
            .map(|(d, c)| serde_json::json!({"date": d, "count": c}))
            .unwrap_or(serde_json::json!(null));

        let annotated = items
            .iter()
            .filter(|i| {
                i.user_note.as_ref().is_some_and(|n| !n.is_empty())
                    || i.tags.as_ref().is_some_and(|t| !t.is_empty())
            })
            .count();
        let annotation_rate = ((annotated as f64 / total as f64) * 100.0).round();
        let avg_per_active = if active_days > 0 {
            total as f64 / active_days as f64
        } else {
            0.0
        };

        serde_json::json!({
            "date_range": format!("{} 至 {}", min_day, max_day),
            "total_items": total,
            "active_days": active_days,
            "total_days": total_days,
            "annotated_items": annotated,
            "annotation_rate": format!("{}%", annotation_rate as i64),
            "source_count": source_count,
            "sources": sources,
            "content_types": content_types,
            "peak_day": peak_day,
            "avg_per_active_day": (avg_per_active * 10.0).round() / 10.0,
            "hour_distribution": {
                "morning": morning,
                "afternoon": afternoon,
                "evening": evening,
                "midnight": midnight,
            }
        })
    }

    /// Save a new attention insight, marking all previous as not current.
    pub fn save_attention_insight(
        &self,
        analysis_json: Option<&str>,
        status: &str,
        error_message: Option<&str>,
        window_start: &str,
        window_end: &str,
        content_count: i32,
        model_used: &str,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute("UPDATE attention_insights SET is_current = 0", [])?;
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO attention_insights (analysis_json, status, error_message, analyzed_at, window_start, window_end, content_count, model_used, is_current)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 1)",
            params![analysis_json, status, error_message, now, window_start, window_end, content_count, model_used],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// Update the status of an insight.
    pub fn update_insight_status(
        &self,
        id: i64,
        status: &str,
        analysis_json: Option<&str>,
        error_message: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE attention_insights SET status = ?1, analysis_json = ?2, error_message = ?3 WHERE id = ?4",
            params![status, analysis_json, error_message, id],
        )?;
        Ok(())
    }

    /// Get the most recent current insight.
    pub fn get_current_insight(
        &self,
    ) -> Result<Option<AttentionInsight>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, analysis_json, status, error_message, analyzed_at, window_start, window_end, content_count, model_used, is_current
             FROM attention_insights
             WHERE is_current = 1
             ORDER BY analyzed_at DESC
             LIMIT 1",
        )?;
        let mut rows = stmt.query_map([], |row| {
            Ok(AttentionInsight {
                id: row.get(0)?,
                analysis_json: row.get(1)?,
                status: row.get(2)?,
                error_message: row.get(3)?,
                analyzed_at: row.get(4)?,
                window_start: row.get(5)?,
                window_end: row.get(6)?,
                content_count: row.get(7)?,
                model_used: row.get(8)?,
                is_current: row.get::<_, i32>(9)? == 1,
            })
        })?;
        match rows.next() {
            Some(Ok(insight)) => Ok(Some(insight)),
            Some(Err(e)) => Err(Box::new(e)),
            None => Ok(None),
        }
    }

    /// Check if any content was saved or updated after the given timestamp.
    pub fn has_new_content_since(&self, since: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM captured_content WHERE is_deleted = 0 AND (captured_at > ?1 OR updated_at > ?1)",
            params![since],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    // ========== Wiki Pages ==========

    pub fn save_wiki_page(
        &self,
        page: &super::models::WikiPage,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "INSERT INTO wiki_pages (id, title, slug, page_type, body_markdown, summary, tags, status, confidence, created_at, updated_at, last_compiled_at, source_message_id, author_name, author_url, source_type, source_task_id, monitor_enabled, monitor_query, monitor_sources, last_discovered_at, pending_count)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, COALESCE(?16, 'user'), ?17, ?18, ?19, ?20, ?21, ?22)",
            params![
                page.id, page.title, page.slug, page.page_type, page.body_markdown,
                page.summary, page.tags, page.status, page.confidence,
                page.created_at, page.updated_at, page.last_compiled_at, page.source_message_id,
                page.author_name, page.author_url, page.source_type, page.source_task_id,
                page.monitor_enabled, page.monitor_query, page.monitor_sources, page.last_discovered_at, page.pending_count,
            ],
        )?;
        Ok(())
    }

    pub fn update_wiki_page(
        &self,
        page: &super::models::WikiPage,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE wiki_pages SET title=?1, body_markdown=?2, summary=?3, tags=?4, status=?5, confidence=?6, updated_at=datetime('now'), last_compiled_at=?7, monitor_enabled=?8, monitor_query=?9, monitor_sources=?10, last_discovered_at=?11, pending_count=?12 WHERE id=?13",
            params![page.title, page.body_markdown, page.summary, page.tags, page.status, page.confidence, page.last_compiled_at, page.monitor_enabled, page.monitor_query, page.monitor_sources, page.last_discovered_at, page.pending_count, page.id],
        )?;
        Ok(())
    }

    pub fn update_wiki_page_status(
        &self,
        page_id: &str,
        status: &str,
        confidence: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE wiki_pages SET status=?1, confidence=?2, updated_at=datetime('now') WHERE id=?3",
            params![status, confidence, page_id],
        )?;
        Ok(())
    }

    pub fn get_wiki_page_by_id(
        &self,
        id: &str,
    ) -> Result<Option<super::models::WikiPage>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, title, slug, page_type, body_markdown, summary, tags, status, confidence, created_at, updated_at, last_compiled_at, source_message_id, author_name, author_url, source_type, source_task_id, monitor_enabled, monitor_query, monitor_sources, last_discovered_at, pending_count
             FROM wiki_pages WHERE id = ?1"
        )?;
        let mut rows = stmt.query_map(params![id], |row| {
            Ok(super::models::WikiPage {
                id: row.get(0)?,
                title: row.get(1)?,
                slug: row.get(2)?,
                page_type: row.get(3)?,
                body_markdown: row.get(4)?,
                summary: row.get(5)?,
                tags: row.get(6)?,
                status: row.get(7)?,
                confidence: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
                last_compiled_at: row.get(11)?,
                source_message_id: row.get(12).unwrap_or(None),
                author_name: row.get(13).unwrap_or(None),
                author_url: row.get(14).unwrap_or(None),
                source_type: row.get(15).unwrap_or(None),
                source_task_id: row.get(16).unwrap_or(None),
                monitor_enabled: row.get::<_, Option<i32>>(17).unwrap_or(Some(0)) != Some(0),
                monitor_query: row.get(18).unwrap_or(None),
                monitor_sources: row.get::<_, Option<String>>(19).unwrap_or(Some("[]".to_string())).unwrap_or_else(|| "[]".to_string()),
                last_discovered_at: row.get(20).unwrap_or(None),
                pending_count: row.get::<_, Option<i32>>(21).unwrap_or(Some(0)).unwrap_or(0),
            })
        })?;
        match rows.next() {
            Some(Ok(page)) => Ok(Some(page)),
            Some(Err(e)) => Err(Box::new(e)),
            None => Ok(None),
        }
    }

    pub fn get_wiki_page_by_slug(
        &self,
        slug: &str,
    ) -> Result<Option<super::models::WikiPage>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, title, slug, page_type, body_markdown, summary, tags, status, confidence, created_at, updated_at, last_compiled_at, source_message_id, author_name, author_url, source_type, source_task_id, monitor_enabled, monitor_query, monitor_sources, last_discovered_at, pending_count
             FROM wiki_pages WHERE slug = ?1"
        )?;
        let mut rows = stmt.query_map(params![slug], |row| {
            Ok(super::models::WikiPage {
                id: row.get(0)?,
                title: row.get(1)?,
                slug: row.get(2)?,
                page_type: row.get(3)?,
                body_markdown: row.get(4)?,
                summary: row.get(5)?,
                tags: row.get(6)?,
                status: row.get(7)?,
                confidence: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
                last_compiled_at: row.get(11)?,
                source_message_id: row.get(12).unwrap_or(None),
                author_name: row.get(13).unwrap_or(None),
                author_url: row.get(14).unwrap_or(None),
                source_type: row.get(15).unwrap_or(None),
                source_task_id: row.get(16).unwrap_or(None),
                monitor_enabled: row.get::<_, Option<i32>>(17).unwrap_or(Some(0)) != Some(0),
                monitor_query: row.get(18).unwrap_or(None),
                monitor_sources: row.get::<_, Option<String>>(19).unwrap_or(Some("[]".to_string())).unwrap_or_else(|| "[]".to_string()),
                last_discovered_at: row.get(20).unwrap_or(None),
                pending_count: row.get::<_, Option<i32>>(21).unwrap_or(Some(0)).unwrap_or(0),
            })
        })?;
        match rows.next() {
            Some(Ok(page)) => Ok(Some(page)),
            Some(Err(e)) => Err(Box::new(e)),
            None => Ok(None),
        }
    }

    pub fn get_all_wiki_pages(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<super::models::WikiPage>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, title, slug, page_type, body_markdown, summary, tags, status, confidence, created_at, updated_at, last_compiled_at, source_message_id, author_name, author_url, source_type, source_task_id, monitor_enabled, monitor_query, monitor_sources, last_discovered_at, pending_count
             FROM wiki_pages WHERE status IN ('active', 'needs_recompile') ORDER BY updated_at DESC LIMIT ?1 OFFSET ?2"
        )?;
        let rows = stmt.query_map(params![limit, offset], |row| {
            Ok(super::models::WikiPage {
                id: row.get(0)?,
                title: row.get(1)?,
                slug: row.get(2)?,
                page_type: row.get(3)?,
                body_markdown: row.get(4)?,
                summary: row.get(5)?,
                tags: row.get(6)?,
                status: row.get(7)?,
                confidence: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
                last_compiled_at: row.get(11)?,
                source_message_id: row.get(12).unwrap_or(None),
                author_name: row.get(13).unwrap_or(None),
                author_url: row.get(14).unwrap_or(None),
                source_type: row.get(15).unwrap_or(None),
                source_task_id: row.get(16).unwrap_or(None),
                monitor_enabled: row.get::<_, Option<i32>>(17).unwrap_or(Some(0)) != Some(0),
                monitor_query: row.get(18).unwrap_or(None),
                monitor_sources: row.get::<_, Option<String>>(19).unwrap_or(Some("[]".to_string())).unwrap_or_else(|| "[]".to_string()),
                last_discovered_at: row.get(20).unwrap_or(None),
                pending_count: row.get::<_, Option<i32>>(21).unwrap_or(Some(0)).unwrap_or(0),
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn search_wiki_pages(
        &self,
        query: &str,
        limit: i64,
    ) -> Result<Vec<super::models::WikiPage>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let pattern = format!("%{}%", query);
        let mut stmt = conn.prepare(
            "SELECT id, title, slug, page_type, body_markdown, summary, tags, status, confidence, created_at, updated_at, last_compiled_at, source_message_id, author_name, author_url, source_type, source_task_id, monitor_enabled, monitor_query, monitor_sources, last_discovered_at, pending_count
             FROM wiki_pages WHERE status IN ('active', 'needs_recompile')
             AND page_type != 'qa'
             AND (title LIKE ?1 OR summary LIKE ?1 OR tags LIKE ?1 OR body_markdown LIKE ?1)
             ORDER BY confidence DESC, updated_at DESC LIMIT ?2"
        )?;
        let rows = stmt.query_map(params![pattern, limit], |row| {
            Ok(super::models::WikiPage {
                id: row.get(0)?,
                title: row.get(1)?,
                slug: row.get(2)?,
                page_type: row.get(3)?,
                body_markdown: row.get(4)?,
                summary: row.get(5)?,
                tags: row.get(6)?,
                status: row.get(7)?,
                confidence: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
                last_compiled_at: row.get(11)?,
                source_message_id: row.get(12).unwrap_or(None),
                author_name: row.get(13).unwrap_or(None),
                author_url: row.get(14).unwrap_or(None),
                source_type: row.get(15).unwrap_or(None),
                source_task_id: row.get(16).unwrap_or(None),
                monitor_enabled: row.get::<_, Option<i32>>(17).unwrap_or(Some(0)) != Some(0),
                monitor_query: row.get(18).unwrap_or(None),
                monitor_sources: row.get::<_, Option<String>>(19).unwrap_or(Some("[]".to_string())).unwrap_or_else(|| "[]".to_string()),
                last_discovered_at: row.get(20).unwrap_or(None),
                pending_count: row.get::<_, Option<i32>>(21).unwrap_or(Some(0)).unwrap_or(0),
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn get_wiki_pages_by_type(
        &self,
        page_type: &str,
    ) -> Result<Vec<super::models::WikiPage>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, title, slug, page_type, body_markdown, summary, tags, status, confidence, created_at, updated_at, last_compiled_at, source_message_id, author_name, author_url, source_type, source_task_id, monitor_enabled, monitor_query, monitor_sources, last_discovered_at, pending_count
             FROM wiki_pages WHERE page_type = ?1 AND status IN ('active', 'needs_recompile') ORDER BY updated_at DESC"
        )?;
        let rows = stmt.query_map(params![page_type], |row| {
            Ok(super::models::WikiPage {
                id: row.get(0)?,
                title: row.get(1)?,
                slug: row.get(2)?,
                page_type: row.get(3)?,
                body_markdown: row.get(4)?,
                summary: row.get(5)?,
                tags: row.get(6)?,
                status: row.get(7)?,
                confidence: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
                last_compiled_at: row.get(11)?,
                source_message_id: row.get(12).unwrap_or(None),
                author_name: row.get(13).unwrap_or(None),
                author_url: row.get(14).unwrap_or(None),
                source_type: row.get(15).unwrap_or(None),
                source_task_id: row.get(16).unwrap_or(None),
                monitor_enabled: row.get::<_, Option<i32>>(17).unwrap_or(Some(0)) != Some(0),
                monitor_query: row.get(18).unwrap_or(None),
                monitor_sources: row.get::<_, Option<String>>(19).unwrap_or(Some("[]".to_string())).unwrap_or_else(|| "[]".to_string()),
                last_discovered_at: row.get(20).unwrap_or(None),
                pending_count: row.get::<_, Option<i32>>(21).unwrap_or(Some(0)).unwrap_or(0),
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn get_wiki_pages_by_status(
        &self,
        status: &str,
    ) -> Result<Vec<super::models::WikiPage>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, title, slug, page_type, body_markdown, summary, tags, status, confidence, created_at, updated_at, last_compiled_at, source_message_id, author_name, author_url, source_type, source_task_id, monitor_enabled, monitor_query, monitor_sources, last_discovered_at, pending_count
             FROM wiki_pages WHERE status = ?1 ORDER BY updated_at DESC"
        )?;
        let rows = stmt.query_map(params![status], |row| {
            Ok(super::models::WikiPage {
                id: row.get(0)?,
                title: row.get(1)?,
                slug: row.get(2)?,
                page_type: row.get(3)?,
                body_markdown: row.get(4)?,
                summary: row.get(5)?,
                tags: row.get(6)?,
                status: row.get(7)?,
                confidence: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
                last_compiled_at: row.get(11)?,
                source_message_id: row.get(12).unwrap_or(None),
                author_name: row.get(13).unwrap_or(None),
                author_url: row.get(14).unwrap_or(None),
                source_type: row.get(15).unwrap_or(None),
                source_task_id: row.get(16).unwrap_or(None),
                monitor_enabled: row.get::<_, Option<i32>>(17).unwrap_or(Some(0)) != Some(0),
                monitor_query: row.get(18).unwrap_or(None),
                monitor_sources: row.get::<_, Option<String>>(19).unwrap_or(Some("[]".to_string())).unwrap_or_else(|| "[]".to_string()),
                last_discovered_at: row.get(20).unwrap_or(None),
                pending_count: row.get::<_, Option<i32>>(21).unwrap_or(Some(0)).unwrap_or(0),
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn delete_wiki_page(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute("DELETE FROM wiki_pages WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn get_wiki_stats(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let total_pages: i64 = conn.query_row(
            "SELECT COUNT(*) FROM wiki_pages WHERE status IN ('active', 'needs_recompile')",
            [],
            |r| r.get(0),
        )?;
        let total_edges: i64 =
            conn.query_row("SELECT COUNT(*) FROM wiki_edges", [], |r| r.get(0))?;
        let total_sources: i64 = conn.query_row(
            "SELECT COUNT(DISTINCT content_id) FROM wiki_page_sources WHERE source_status = 'active'", [], |r| r.get(0)
        )?;
        let needs_recompile: i64 = conn.query_row(
            "SELECT COUNT(*) FROM wiki_pages WHERE status = 'needs_recompile'",
            [],
            |r| r.get(0),
        )?;
        let lint_open: i64 = conn.query_row(
            "SELECT COUNT(*) FROM wiki_lint_results WHERE status = 'open'",
            [],
            |r| r.get(0),
        )?;
        Ok(serde_json::json!({
            "total_pages": total_pages,
            "total_edges": total_edges,
            "total_sources": total_sources,
            "needs_recompile": needs_recompile,
            "lint_open": lint_open,
        }))
    }

    /// Returns (id, title, summary) for all active pages — used as compile context.
    pub fn get_wiki_page_summaries(
        &self,
    ) -> Result<Vec<(String, String, String)>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, title, COALESCE(summary, '') FROM wiki_pages WHERE status = 'active' ORDER BY title"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    // ========== Wiki Page Sources ==========

    pub fn add_page_source(
        &self,
        page_id: &str,
        content_id: &str,
        compile_hash: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "INSERT OR REPLACE INTO wiki_page_sources (page_id, content_id, compile_hash, source_status, contributed_at)
             VALUES (?1, ?2, ?3, 'active', datetime('now'))",
            params![page_id, content_id, compile_hash],
        )?;
        Ok(())
    }

    pub fn get_sources_for_page(
        &self,
        page_id: &str,
    ) -> Result<Vec<super::models::WikiPageSource>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, page_id, content_id, compile_hash, source_status, contributed_at FROM wiki_page_sources WHERE page_id = ?1"
        )?;
        let rows = stmt.query_map(params![page_id], |row| {
            Ok(super::models::WikiPageSource {
                id: row.get(0)?,
                page_id: row.get(1)?,
                content_id: row.get(2)?,
                compile_hash: row.get(3)?,
                source_status: row.get(4)?,
                contributed_at: row.get(5)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn get_pages_for_content(
        &self,
        content_id: &str,
    ) -> Result<Vec<super::models::WikiPageSource>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, page_id, content_id, compile_hash, source_status, contributed_at FROM wiki_page_sources WHERE content_id = ?1"
        )?;
        let rows = stmt.query_map(params![content_id], |row| {
            Ok(super::models::WikiPageSource {
                id: row.get(0)?,
                page_id: row.get(1)?,
                content_id: row.get(2)?,
                compile_hash: row.get(3)?,
                source_status: row.get(4)?,
                contributed_at: row.get(5)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn update_source_status(
        &self,
        page_id: &str,
        content_id: &str,
        status: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE wiki_page_sources SET source_status = ?1 WHERE page_id = ?2 AND content_id = ?3",
            params![status, page_id, content_id],
        )?;
        Ok(())
    }

    pub fn update_source_status_by_content(
        &self,
        content_id: &str,
        status: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE wiki_page_sources SET source_status = ?1 WHERE content_id = ?2",
            params![status, content_id],
        )?;
        Ok(())
    }

    pub fn count_active_sources(
        &self,
        page_id: &str,
    ) -> Result<(i64, i64), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let active: i64 = conn.query_row(
            "SELECT COUNT(*) FROM wiki_page_sources WHERE page_id = ?1 AND source_status = 'active'",
            params![page_id], |r| r.get(0),
        )?;
        let total: i64 = conn.query_row(
            "SELECT COUNT(*) FROM wiki_page_sources WHERE page_id = ?1",
            params![page_id],
            |r| r.get(0),
        )?;
        Ok((active, total))
    }

    pub fn delete_sources_for_page(&self, page_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "DELETE FROM wiki_page_sources WHERE page_id = ?1",
            params![page_id],
        )?;
        Ok(())
    }

    // ========== Wiki Edges ==========

    pub fn save_wiki_edge(
        &self,
        source_id: &str,
        target_id: &str,
        relation: &str,
        weight: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "INSERT OR REPLACE INTO wiki_edges (source_page_id, target_page_id, relation, weight, created_at)
             VALUES (?1, ?2, ?3, ?4, datetime('now'))",
            params![source_id, target_id, relation, weight],
        )?;
        Ok(())
    }

    pub fn get_edges_for_page(
        &self,
        page_id: &str,
    ) -> Result<Vec<super::models::WikiEdge>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, source_page_id, target_page_id, relation, weight, created_at
             FROM wiki_edges WHERE source_page_id = ?1 OR target_page_id = ?1",
        )?;
        let rows = stmt.query_map(params![page_id], |row| {
            Ok(super::models::WikiEdge {
                id: row.get(0)?,
                source_page_id: row.get(1)?,
                target_page_id: row.get(2)?,
                relation: row.get(3)?,
                weight: row.get(4)?,
                created_at: row.get(5)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn get_all_wiki_edges(
        &self,
    ) -> Result<Vec<super::models::WikiEdge>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, source_page_id, target_page_id, relation, weight, created_at FROM wiki_edges"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(super::models::WikiEdge {
                id: row.get(0)?,
                source_page_id: row.get(1)?,
                target_page_id: row.get(2)?,
                relation: row.get(3)?,
                weight: row.get(4)?,
                created_at: row.get(5)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn delete_edges_for_page(&self, page_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "DELETE FROM wiki_edges WHERE source_page_id = ?1 OR target_page_id = ?1",
            params![page_id],
        )?;
        Ok(())
    }

    /// Delete all edges of a given relation type. Used when rebuilding the
    /// tag-based "related" graph from scratch with a new algorithm.
    pub fn delete_edges_by_relation(
        &self,
        relation: &str,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let n = conn.execute(
            "DELETE FROM wiki_edges WHERE relation = ?1",
            params![relation],
        )?;
        Ok(n)
    }

    // ========== Wiki Compile Log ==========

    pub fn acquire_compile_lock(
        &self,
        content_id: &str,
        content_hash: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        match conn.execute(
            "INSERT INTO wiki_compile_log (content_id, content_hash, status, created_at)
             VALUES (?1, ?2, 'compiling', datetime('now'))",
            params![content_id, content_hash],
        ) {
            Ok(_) => Ok(true),
            Err(rusqlite::Error::SqliteFailure(e, _))
                if e.code == rusqlite::ErrorCode::ConstraintViolation =>
            {
                Ok(false)
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn release_compile_lock(
        &self,
        content_id: &str,
        status: &str,
        pages_touched: Option<&str>,
        model_used: Option<&str>,
        error_message: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE wiki_compile_log SET status=?1, pages_touched=?2, model_used=?3, error_message=?4, compiled_at=datetime('now')
             WHERE content_id=?5 AND status='compiling'",
            params![status, pages_touched, model_used, error_message, content_id],
        )?;
        Ok(())
    }

    pub fn cleanup_stale_compile_locks(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let count = conn.execute(
            "UPDATE wiki_compile_log SET status='error', error_message='stale lock cleaned on startup' WHERE status='compiling'",
            [],
        )?;
        Ok(count as u64)
    }

    // ========== Wiki Hash Updates ==========

    pub fn update_content_compile_hash(
        &self,
        content_id: &str,
        hash: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE captured_content SET wiki_compile_hash=?1, wiki_assessed_hash=?1 WHERE id=?2",
            params![hash, content_id],
        )?;
        Ok(())
    }

    pub fn update_content_assessed_hash(
        &self,
        content_id: &str,
        hash: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE captured_content SET wiki_assessed_hash=?1 WHERE id=?2",
            params![hash, content_id],
        )?;
        Ok(())
    }

    // ========== Wiki Conversations ==========

    pub fn save_wiki_conversation(
        &self,
        conv: &super::models::WikiConversation,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "INSERT INTO wiki_conversations (id, question, answer, pages_used, saved_as_page, model_used, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![conv.id, conv.question, conv.answer, conv.pages_used, conv.saved_as_page, conv.model_used, conv.created_at],
        )?;
        Ok(())
    }

    pub fn get_wiki_conversations(
        &self,
        limit: i64,
    ) -> Result<Vec<super::models::WikiConversation>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, question, answer, pages_used, saved_as_page, model_used, created_at
             FROM wiki_conversations ORDER BY created_at DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit], |row| {
            Ok(super::models::WikiConversation {
                id: row.get(0)?,
                question: row.get(1)?,
                answer: row.get(2)?,
                pages_used: row.get(3)?,
                saved_as_page: row.get(4)?,
                model_used: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn update_conversation_saved_page(
        &self,
        conv_id: &str,
        page_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE wiki_conversations SET saved_as_page=?1 WHERE id=?2",
            params![page_id, conv_id],
        )?;
        Ok(())
    }

    // ========== Wiki Lint ==========

    pub fn save_lint_result(
        &self,
        lint_type: &str,
        severity: &str,
        title: &str,
        description: &str,
        page_ids: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "INSERT INTO wiki_lint_results (lint_type, severity, title, description, page_ids, status, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, 'open', datetime('now'))",
            params![lint_type, severity, title, description, page_ids],
        )?;
        Ok(())
    }

    pub fn get_open_lint_results(
        &self,
    ) -> Result<Vec<super::models::WikiLintResult>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, lint_type, severity, title, description, page_ids, status, created_at
             FROM wiki_lint_results WHERE status = 'open' ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(super::models::WikiLintResult {
                id: row.get(0)?,
                lint_type: row.get(1)?,
                severity: row.get(2)?,
                title: row.get(3)?,
                description: row.get(4)?,
                page_ids: row.get(5)?,
                status: row.get(6)?,
                created_at: row.get(7)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn resolve_lint_result(&self, id: i64) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE wiki_lint_results SET status='resolved' WHERE id=?1",
            params![id],
        )?;
        Ok(())
    }

    /// Batch-resolve all open lint results of a given type.
    /// Used at app startup to clean up stale "source deleted" notifications
    /// from before we stopped auto-generating them on content deletion.
    pub fn resolve_lint_results_by_type(
        &self,
        lint_type: &str,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let n = conn.execute(
            "UPDATE wiki_lint_results SET status='resolved' WHERE status='open' AND lint_type=?1",
            params![lint_type],
        )?;
        Ok(n)
    }

    /// Recalculate confidence for a page based on its source health.
    pub fn recalculate_page_confidence(
        &self,
        page_id: &str,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let (active, total) = self.count_active_sources(page_id)?;
        let confidence = if total == 0 {
            0.3
        } else {
            active as f64 / total as f64
        };
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE wiki_pages SET confidence=?1, updated_at=datetime('now') WHERE id=?2",
            params![confidence, page_id],
        )?;
        Ok(confidence)
    }

    pub fn get_pages_needing_recompile(
        &self,
    ) -> Result<Vec<super::models::WikiPage>, Box<dyn std::error::Error>> {
        self.get_wiki_pages_by_status("needs_recompile")
    }

    // ========== Wiki Chat Sessions ==========

    pub fn create_chat_session(
        &self,
        id: &str,
        title: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "INSERT INTO wiki_chat_sessions (id, title, created_at, updated_at) VALUES (?1, ?2, datetime('now'), datetime('now'))",
            params![id, title],
        )?;
        Ok(())
    }

    pub fn get_chat_sessions(
        &self,
        limit: i64,
    ) -> Result<Vec<super::models::WikiChatSession>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, title, created_at, updated_at FROM wiki_chat_sessions ORDER BY updated_at DESC LIMIT ?1"
        )?;
        let rows = stmt.query_map(params![limit], |row| {
            Ok(super::models::WikiChatSession {
                id: row.get(0)?,
                title: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn update_chat_session_title(
        &self,
        session_id: &str,
        title: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE wiki_chat_sessions SET title=?1, updated_at=datetime('now') WHERE id=?2",
            params![title, session_id],
        )?;
        Ok(())
    }

    pub fn touch_chat_session(&self, session_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE wiki_chat_sessions SET updated_at=datetime('now') WHERE id=?1",
            params![session_id],
        )?;
        Ok(())
    }

    pub fn delete_chat_session(&self, session_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "DELETE FROM wiki_chat_sessions WHERE id=?1",
            params![session_id],
        )?;
        Ok(())
    }

    // ========== Wiki Chat Messages ==========

    pub fn add_chat_message(
        &self,
        msg: &super::models::WikiChatMessage,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "INSERT INTO wiki_chat_messages (id, session_id, role, content, pages_used, source_mode, turn_index, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, datetime('now'))",
            params![msg.id, msg.session_id, msg.role, msg.content, msg.pages_used, msg.source_mode, msg.turn_index],
        )?;
        Ok(())
    }

    pub fn get_chat_messages(
        &self,
        session_id: &str,
    ) -> Result<Vec<super::models::WikiChatMessage>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, session_id, role, content, pages_used, source_mode, turn_index, created_at
             FROM wiki_chat_messages WHERE session_id=?1 ORDER BY turn_index ASC",
        )?;
        let rows = stmt.query_map(params![session_id], |row| {
            Ok(super::models::WikiChatMessage {
                id: row.get(0)?,
                session_id: row.get(1)?,
                role: row.get(2)?,
                content: row.get(3)?,
                pages_used: row.get(4)?,
                source_mode: row.get(5)?,
                turn_index: row.get(6)?,
                created_at: row.get(7)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn update_chat_message_sources(
        &self,
        message_id: &str,
        pages_used: &str,
        source_mode: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE wiki_chat_messages
             SET pages_used = ?1, source_mode = ?2
             WHERE id = ?3",
            params![pages_used, source_mode, message_id],
        )?;
        Ok(())
    }

    pub fn get_next_turn_index(&self, session_id: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let max: Option<i32> = conn.query_row(
            "SELECT MAX(turn_index) FROM wiki_chat_messages WHERE session_id=?1",
            params![session_id],
            |r| r.get(0),
        )?;
        Ok(max.unwrap_or(-1) + 1)
    }

    pub fn get_wiki_page_by_source_message_id(
        &self,
        message_id: &str,
    ) -> Result<Option<super::models::WikiPage>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, title, slug, page_type, body_markdown, summary, tags, status, confidence, created_at, updated_at, last_compiled_at, source_message_id, author_name, author_url, source_type, source_task_id, monitor_enabled, monitor_query, monitor_sources, last_discovered_at, pending_count
             FROM wiki_pages WHERE source_message_id = ?1"
        )?;
        let mut rows = stmt.query_map(params![message_id], |row| {
            Ok(super::models::WikiPage {
                id: row.get(0)?,
                title: row.get(1)?,
                slug: row.get(2)?,
                page_type: row.get(3)?,
                body_markdown: row.get(4)?,
                summary: row.get(5)?,
                tags: row.get(6)?,
                status: row.get(7)?,
                confidence: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
                last_compiled_at: row.get(11)?,
                source_message_id: row.get(12).unwrap_or(None),
                author_name: row.get(13).unwrap_or(None),
                author_url: row.get(14).unwrap_or(None),
                source_type: row.get(15).unwrap_or(None),
                source_task_id: row.get(16).unwrap_or(None),
                monitor_enabled: row.get::<_, Option<i32>>(17).unwrap_or(Some(0)) != Some(0),
                monitor_query: row.get(18).unwrap_or(None),
                monitor_sources: row.get::<_, Option<String>>(19).unwrap_or(Some("[]".to_string())).unwrap_or_else(|| "[]".to_string()),
                last_discovered_at: row.get(20).unwrap_or(None),
                pending_count: row.get::<_, Option<i32>>(21).unwrap_or(Some(0)).unwrap_or(0),
            })
        })?;
        match rows.next() {
            Some(Ok(page)) => Ok(Some(page)),
            Some(Err(e)) => Err(Box::new(e)),
            None => Ok(None),
        }
    }

    pub fn get_active_wiki_page_titles(
        &self,
    ) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, title
             FROM wiki_pages
             WHERE status IN ('active', 'needs_recompile') AND page_type != 'qa'",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    /// Get page summaries for Q&A retrieval, excluding qa-type pages.
    pub fn get_wiki_page_summaries_for_qa(
        &self,
    ) -> Result<Vec<(String, String, String)>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, title, COALESCE(summary, substr(body_markdown, 1, 100))
             FROM wiki_pages WHERE status = 'active' AND page_type != 'qa' ORDER BY title",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    /// Whether the FTS5 virtual table exists. False means migration 014
    /// failed to apply (older sqlite without FTS5) — callers should fall
    /// back to the full-index APIs above.
    pub fn fts_available(&self) -> bool {
        let conn = match self.db.conn.lock() {
            Ok(c) => c,
            Err(_) => return false,
        };
        conn.prepare("SELECT 1 FROM sqlite_master WHERE type='table' AND name='wiki_pages_fts'")
            .and_then(|mut s| s.query_row([], |row| row.get::<_, i32>(0)))
            .is_ok()
    }

    /// Build an FTS5 MATCH expression from a free-form user query.
    ///
    /// Two transforms applied:
    ///
    /// 1. **CJK segmentation** — same `cjk_segment()` used at index time
    ///    (registered as `cjk_seg()` SQL function and applied by triggers).
    ///    This guarantees query tokens line up with index tokens. Without
    ///    it, querying "设计" would never match "整理设计风格" because
    ///    unicode61 indexes the latter as one big token.
    ///
    /// 2. **Syntax sanitization** — FTS5 reserves a handful of chars
    ///    (`" * : ^ ( ) - +`) that would either be parsed specially or
    ///    raise a syntax error. We strip them.
    ///
    /// Then split on whitespace, quote each token (which becomes an
    /// adjacent-token phrase match for CJK after segmentation), and OR
    /// them together for broad recall — Q&A wants high recall, the AI
    /// does the precision step downstream.
    fn build_fts_match(query: &str) -> String {
        // 1. Strip FTS5 syntax chars first so they don't survive into the
        //    phrase quoting step.
        let cleaned: String = query
            .chars()
            .map(|c| match c {
                '"' | '\'' | '*' | ':' | '^' | '(' | ')' | '-' | '+' => ' ',
                _ => c,
            })
            .collect();
        // 2. Split on the user's original whitespace into words. Each
        //    word becomes a quoted phrase in the OR'd MATCH expression.
        //    For CJK words, cjk_segment turns each character into its
        //    own token, and the quoted phrase becomes an adjacency
        //    match (e.g. "设" followed immediately by "计").
        let phrases: Vec<String> = cleaned
            .split_whitespace()
            .filter(|w| !w.is_empty())
            .map(|w| format!("\"{}\"", super::database::cjk_segment(w)))
            .collect();
        phrases.join(" OR ")
    }

    /// Extract the best clickable link from a wiki page body. Pages
    /// frequently include a "## 项目地址" or "## Links" section with
    /// the actual project URL — that's far more useful to surface than
    /// the source_url (which is often the user's tweet/article they
    /// happened to copy from).
    ///
    /// Priority:
    ///   1. First github.com / gitlab.com / bitbucket.org link
    ///   2. First non-social-media http(s) link
    ///   3. None (caller falls back to source_url)
    fn extract_project_url_from_body(body: &str) -> Option<String> {
        use std::sync::OnceLock;
        static RE: OnceLock<regex::Regex> = OnceLock::new();
        let re = RE.get_or_init(|| {
            // URL stops at whitespace, Markdown bracket close, or CJK punctuation.
            // Inside [...], `)` `]` `>` don't need escaping.
            regex::Regex::new(r"https?://[^\s)\]>，。、；：！？]+").expect("regex compiles")
        });

        let trim_trailing = |s: &str| -> String {
            s.trim_end_matches(|c: char| {
                matches!(
                    c,
                    '.' | ','
                        | ';'
                        | ':'
                        | '?'
                        | '!'
                        | '。'
                        | '，'
                        | '、'
                        | '；'
                        | '：'
                        | '！'
                        | '？'
                )
            })
            .to_string()
        };

        let is_repo = |u: &str| {
            u.contains("github.com")
                || u.contains("gitlab.com")
                || u.contains("bitbucket.org")
                || u.contains("huggingface.co")
        };
        let is_social = |u: &str| {
            u.contains("twitter.com")
                || u.contains("x.com/")
                || u.contains("weibo.com")
                || u.contains("mp.weixin.qq.com")
                || u.contains("xiaohongshu.com")
                || u.contains("douyin.com")
                || u.contains("youtube.com/watch")
                || u.contains("bilibili.com")
        };

        let mut fallback: Option<String> = None;
        for m in re.find_iter(body) {
            let url = trim_trailing(m.as_str());
            if is_repo(&url) {
                return Some(url);
            }
            if fallback.is_none() && !is_social(&url) {
                fallback = Some(url);
            }
        }
        fallback
    }

    /// Returns (id, title, summary, created_at, best_url) candidates
    /// for AI prompts, pre-filtered in SQL.
    ///
    /// `best_url` priority:
    ///   1. GitHub / GitLab / HuggingFace project link extracted from body_markdown
    ///   2. Any non-social-media http link in body_markdown
    ///   3. source_url (the page the user originally captured from)
    ///   4. None
    ///
    /// Why: when a user asks "what was that design skill", they want to
    /// click through to the actual project (a GitHub repo) — not back to
    /// the tweet they happened to save it from.
    ///
    /// - `fts_query`: optional free-form text. None = no FTS filter.
    /// - `date_start`/`date_end`: optional ISO-8601 timestamps. Filters
    ///   `created_at` lexically (works because we store ISO-8601 UTC).
    /// - `exclude_qa`: true for Q&A retrieval (Q&A pages would create a
    ///   feedback loop), false for compile (compile may merge into Q&A
    ///   pages legitimately).
    /// - `limit`: max candidates to return. Recommend 50–100 for AI prompts.
    ///
    /// Falls back to a non-FTS query when FTS is unavailable or when
    /// `fts_query` produces no usable tokens.
    pub fn get_wiki_page_candidates(
        &self,
        fts_query: Option<&str>,
        date_start: Option<&str>,
        date_end: Option<&str>,
        exclude_qa: bool,
        limit: i64,
    ) -> Result<Vec<(String, String, String, String, Option<String>)>, Box<dyn std::error::Error>>
    {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;

        let match_expr: Option<String> = fts_query
            .map(Self::build_fts_match)
            .filter(|s| !s.is_empty());

        let fts_table_exists = conn
            .prepare("SELECT 1 FROM sqlite_master WHERE type='table' AND name='wiki_pages_fts'")
            .and_then(|mut s| s.query_row([], |row| row.get::<_, i32>(0)))
            .is_ok();
        let use_fts = match_expr.is_some() && fts_table_exists;

        // We pull the source URL via a correlated subquery — picks one
        // active source per page (most recently contributed). NULL when
        // a page has no active source (rare but possible for pages built
        // entirely from chat answers).
        let url_subq = "(SELECT cc.source_url FROM wiki_page_sources wps \
            JOIN captured_content cc ON cc.id = wps.content_id \
            WHERE wps.page_id = wp.id AND wps.source_status = 'active' \
            AND cc.source_url IS NOT NULL AND cc.source_url != '' \
            ORDER BY wps.contributed_at DESC LIMIT 1)";

        let mut sql = String::new();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        // We also fetch the first ~3000 chars of body_markdown so we can
        // mine a project link from it in Rust. Project links typically
        // sit in a "## 项目地址" section near the top.
        if use_fts {
            sql.push_str(&format!(
                "SELECT wp.id, wp.title, COALESCE(wp.summary, substr(wp.body_markdown, 1, 100)), wp.created_at, {}, substr(wp.body_markdown, 1, 3000) \
                 FROM wiki_pages_fts fts \
                 JOIN wiki_pages wp ON wp.id = fts.page_id \
                 WHERE wiki_pages_fts MATCH ? AND wp.status = 'active'",
                url_subq
            ));
            params.push(Box::new(match_expr.clone().unwrap()));
        } else {
            sql.push_str(&format!(
                "SELECT wp.id, wp.title, COALESCE(wp.summary, substr(wp.body_markdown, 1, 100)), wp.created_at, {}, substr(wp.body_markdown, 1, 3000) \
                 FROM wiki_pages wp \
                 WHERE wp.status = 'active'",
                url_subq
            ));
        }

        if exclude_qa {
            sql.push_str(" AND wp.page_type != 'qa'");
        }
        if let Some(s) = date_start {
            sql.push_str(" AND wp.created_at >= ?");
            params.push(Box::new(s.to_string()));
        }
        if let Some(e) = date_end {
            sql.push_str(" AND wp.created_at <= ?");
            params.push(Box::new(e.to_string()));
        }

        if use_fts {
            sql.push_str(" ORDER BY rank");
        } else if date_start.is_some() || date_end.is_some() {
            sql.push_str(" ORDER BY wp.created_at DESC");
        } else {
            sql.push_str(" ORDER BY wp.title");
        }
        sql.push_str(" LIMIT ?");
        params.push(Box::new(limit));

        let mut stmt = conn.prepare(&sql)?;
        let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|b| b.as_ref()).collect();
        let rows = stmt.query_map(param_refs.as_slice(), |row| {
            // Tuple shape returned to caller stays at 5 elements; the
            // body excerpt is consumed here to compute best_url and not
            // forwarded.
            let id: String = row.get(0)?;
            let title: String = row.get(1)?;
            let summary: String = row.get(2)?;
            let created_at: String = row.get(3)?;
            let source_url: Option<String> = row.get(4)?;
            let body_excerpt: String = row.get::<_, Option<String>>(5)?.unwrap_or_default();
            let best_url = Self::extract_project_url_from_body(&body_excerpt).or(source_url);
            Ok((id, title, summary, created_at, best_url))
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    // ========== Learning Data Models (E-2, E-3) ==========

    // ----- Goal -----

    pub fn save_goal(&self, goal: &super::models::Goal) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "INSERT INTO goals (id, title, description, keywords, status, progress, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![goal.id, goal.title, goal.description, goal.keywords, goal.status, goal.progress, goal.created_at, goal.updated_at],
        )?;
        Ok(())
    }

    pub fn update_goal(&self, goal: &super::models::Goal) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE goals SET title=?1, description=?2, keywords=?3, status=?4, progress=?5, updated_at=?6 WHERE id=?7",
            params![goal.title, goal.description, goal.keywords, goal.status, goal.progress, goal.updated_at, goal.id],
        )?;
        Ok(())
    }

    pub fn get_all_goals(&self) -> Result<Vec<super::models::Goal>, Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, title, description, keywords, status, progress, created_at, updated_at FROM goals ORDER BY created_at DESC"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(super::models::Goal {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                keywords: row.get(3)?,
                status: row.get(4)?,
                progress: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn get_goals_by_status(&self, status: &str) -> Result<Vec<super::models::Goal>, Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, title, description, keywords, status, progress, created_at, updated_at FROM goals WHERE status = ?1 ORDER BY created_at DESC"
        )?;
        let rows = stmt.query_map(params![status], |row| {
            Ok(super::models::Goal {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                keywords: row.get(3)?,
                status: row.get(4)?,
                progress: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn get_goal_by_id(&self, id: &str) -> Result<Option<super::models::Goal>, Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, title, description, keywords, status, progress, created_at, updated_at FROM goals WHERE id = ?1"
        )?;
        let mut rows = stmt.query_map(params![id], |row| {
            Ok(super::models::Goal {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                keywords: row.get(3)?,
                status: row.get(4)?,
                progress: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?;
        match rows.next() {
            Some(Ok(g)) => Ok(Some(g)),
            Some(Err(e)) => Err(Box::new(e)),
            None => Ok(None),
        }
    }

    pub fn delete_goal(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        conn.execute("DELETE FROM goals WHERE id = ?1", params![id])?;
        Ok(())
    }

    // ----- GoalWikiLink -----

    pub fn save_goal_wiki_link(&self, link: &super::models::GoalWikiLink) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "INSERT OR IGNORE INTO goal_wiki_links (id, goal_id, wiki_page_id, relevance_score, source, is_new, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![link.id, link.goal_id, link.wiki_page_id, link.relevance_score, link.source, link.is_new as i32, link.created_at],
        )?;
        Ok(())
    }

    pub fn get_goal_wiki_links(&self, goal_id: &str) -> Result<Vec<super::models::GoalWikiLink>, Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, goal_id, wiki_page_id, relevance_score, source, is_new, created_at FROM goal_wiki_links WHERE goal_id = ?1 ORDER BY created_at DESC"
        )?;
        let rows = stmt.query_map(params![goal_id], |row| {
            Ok(super::models::GoalWikiLink {
                id: row.get(0)?,
                goal_id: row.get(1)?,
                wiki_page_id: row.get(2)?,
                relevance_score: row.get(3)?,
                source: row.get(4)?,
                is_new: row.get::<_, i32>(5)? != 0,
                created_at: row.get(6)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn get_goal_wiki_links_with_titles(
        &self,
        goal_id: &str,
    ) -> Result<Vec<super::models::GoalWikiLinkWithTitle>, Box<dyn std::error::Error>> {
        let links = self.get_goal_wiki_links(goal_id)?;
        let mut enriched = Vec::new();
        for link in links {
            let title = self
                .get_wiki_page_by_id(&link.wiki_page_id)
                .ok()
                .flatten()
                .map(|p| p.title)
                .unwrap_or_else(|| link.wiki_page_id.clone());
            let review_count = self.get_review_count_for_wiki(&link.wiki_page_id).unwrap_or(0);
            let schedule = self.get_review_schedule(&link.wiki_page_id).ok().flatten();
            let next_review_at = schedule.as_ref().map(|s| s.next_review_at.clone());
            let last_reviewed_at = schedule.as_ref().and_then(|s| s.last_reviewed_at.clone());
            enriched.push(super::models::GoalWikiLinkWithTitle {
                id: link.id,
                goal_id: link.goal_id,
                wiki_page_id: link.wiki_page_id,
                relevance_score: link.relevance_score,
                source: link.source,
                is_new: link.is_new,
                created_at: link.created_at,
                wiki_title: title,
                review_count,
                next_review_at,
                last_reviewed_at,
            });
        }
        Ok(enriched)
    }

    pub fn get_goal_review_logs(
        &self,
        goal_id: &str,
        limit: i64,
    ) -> Result<Vec<super::models::GoalReviewLogItem>, Box<dyn std::error::Error>> {
        let links = self.get_goal_wiki_links(goal_id)?;
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        let mut results = Vec::new();
        for link in links {
            // Use direct SQL to avoid re-locking self.db.conn (std::sync::Mutex is non-reentrant)
            let title = {
                let mut stmt = conn.prepare(
                    "SELECT title FROM wiki_pages WHERE id = ?1"
                )?;
                stmt.query_row(params![link.wiki_page_id], |row| row.get::<_, String>(0))
                    .unwrap_or_else(|_| link.wiki_page_id.clone())
            };
            let schedule = {
                let mut stmt = conn.prepare(
                    "SELECT id FROM review_schedule WHERE wiki_page_id = ?1 AND is_archived = 0"
                )?;
                stmt.query_row(params![link.wiki_page_id], |row| row.get::<_, String>(0)).ok()
            };
            if let Some(schedule_id) = schedule {
                let mut stmt = conn.prepare(
                    "SELECT id, schedule_id, quality, reviewed_at, review_format, response_time_seconds
                     FROM review_logs WHERE schedule_id = ?1 ORDER BY reviewed_at DESC LIMIT ?2"
                )?;
                let rows = stmt.query_map(params![schedule_id, limit], |row| {
                    Ok(super::models::GoalReviewLogItem {
                        id: row.get(0)?,
                        schedule_id: row.get(1)?,
                        wiki_page_id: link.wiki_page_id.clone(),
                        wiki_title: title.clone(),
                        quality: row.get(2)?,
                        reviewed_at: row.get(3)?,
                        review_format: row.get(4)?,
                        response_time_seconds: row.get(5)?,
                    })
                })?;
                for row in rows {
                    results.push(row?);
                }
            }
        }
        results.sort_by(|a, b| b.reviewed_at.cmp(&a.reviewed_at));
        results.truncate(limit as usize);
        Ok(results)
    }

    pub fn get_goal_review_sessions(
        &self,
        goal_id: &str,
        limit: i64,
    ) -> Result<Vec<super::models::ReviewSessionRecord>, Box<dyn std::error::Error>> {
        use std::collections::HashMap;
        let links = self.get_goal_wiki_links(goal_id)?;
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        // Gather all review_logs for all schedules linked to this goal
        let mut all_logs: Vec<(String, Option<String>, String, String, String, i32, Option<String>)> = Vec::new();
        // (log_id, session_id, wiki_page_id, wiki_title, reviewed_at, quality, review_format)
        for link in links {
            let title = {
                let mut stmt = conn.prepare("SELECT title FROM wiki_pages WHERE id = ?1")?;
                stmt.query_row(params![link.wiki_page_id], |row| row.get::<_, String>(0))
                    .unwrap_or_else(|_| link.wiki_page_id.clone())
            };
            let schedule_id = {
                let mut stmt = conn.prepare(
                    "SELECT id FROM review_schedule WHERE wiki_page_id = ?1 AND is_archived = 0"
                )?;
                stmt.query_row(params![link.wiki_page_id], |row| row.get::<_, String>(0)).ok()
            };
            if let Some(sid) = schedule_id {
                let mut stmt = conn.prepare(
                    "SELECT id, session_id, reviewed_at, quality, review_format
                     FROM review_logs WHERE schedule_id = ?1 ORDER BY reviewed_at DESC"
                )?;
                let rows = stmt.query_map(params![sid], |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, Option<String>>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, i32>(3)?,
                        row.get::<_, Option<String>>(4)?,
                    ))
                })?;
                for row in rows {
                    if let Ok((log_id, session_id, reviewed_at, quality, review_format)) = row {
                        all_logs.push((log_id, session_id, link.wiki_page_id.clone(), title.clone(), reviewed_at, quality, review_format));
                    }
                }
            }
        }
        drop(conn);

        // Group by session_id; treat NULL as individual sessions (legacy data)
        let mut sessions: HashMap<String, (String, i32, i32, Vec<super::models::ReviewSessionItem>)> = HashMap::new();
        for (log_id, session_id, wiki_page_id, wiki_title, reviewed_at, quality, review_format) in &all_logs {
            let key = session_id.clone().unwrap_or_else(|| format!("legacy-{}", uuid::Uuid::new_v4()));
            let entry = sessions.entry(key.clone()).or_insert_with(|| {
                (reviewed_at.clone(), 0, 0, Vec::new())
            });
            entry.1 += 1; // total_count
            if *quality >= 1 { entry.2 += 1; } // correct_count
            // Keep the latest reviewed_at
            if reviewed_at > &entry.0 { entry.0 = reviewed_at.clone(); }
            entry.3.push(super::models::ReviewSessionItem {
                log_id: log_id.clone(),
                wiki_page_id: wiki_page_id.clone(),
                wiki_title: wiki_title.clone(),
                quality: *quality,
                review_format: review_format.clone(),
            });
        }

        let mut result: Vec<super::models::ReviewSessionRecord> = sessions
            .into_iter()
            .map(|(session_id, (reviewed_at, total_count, correct_count, items))| {
                super::models::ReviewSessionRecord {
                    session_id: if session_id.starts_with("legacy-") { "".to_string() } else { session_id },
                    reviewed_at,
                    total_count,
                    correct_count,
                    items,
                }
            })
            .collect();
        result.sort_by(|a, b| b.reviewed_at.cmp(&a.reviewed_at));
        result.truncate(limit as usize);
        Ok(result)
    }

    pub fn get_review_log_by_id(
        &self,
        log_id: &str,
    ) -> Result<Option<super::models::ReviewLogDetail>, Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT rl.id, rl.schedule_id, rl.quality, rl.interval_before, rl.interval_after,
                    rl.ease_factor_before, rl.ease_factor_after, rl.reviewed_at,
                    rl.review_format, rl.response_time_seconds,
                    rs.wiki_page_id, wp.title, wp.summary, wp.tags
             FROM review_logs rl
             JOIN review_schedule rs ON rs.id = rl.schedule_id
             JOIN wiki_pages wp ON wp.id = rs.wiki_page_id
             WHERE rl.id = ?1"
        )?;
        let result = stmt.query_row(params![log_id], |row| {
            Ok(super::models::ReviewLogDetail {
                id: row.get(0)?,
                schedule_id: row.get(1)?,
                quality: row.get(2)?,
                interval_before: row.get(3)?,
                interval_after: row.get(4)?,
                ease_factor_before: row.get(5)?,
                ease_factor_after: row.get(6)?,
                reviewed_at: row.get(7)?,
                review_format: row.get(8)?,
                response_time_seconds: row.get(9)?,
                wiki_page_id: row.get(10)?,
                wiki_title: row.get(11)?,
                wiki_summary: row.get(12)?,
                wiki_tags: row.get(13)?,
            })
        }).ok();
        Ok(result)
    }

    pub fn get_goal_wiki_links_for_page(
        &self,
        wiki_page_id: &str,
    ) -> Result<Vec<super::models::GoalWikiLink>, Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, goal_id, wiki_page_id, relevance_score, source, is_new, created_at
             FROM goal_wiki_links WHERE wiki_page_id = ?1",
        )?;
        let rows = stmt.query_map([wiki_page_id], |row| {
            Ok(super::models::GoalWikiLink {
                id: row.get(0)?,
                goal_id: row.get(1)?,
                wiki_page_id: row.get(2)?,
                relevance_score: row.get(3)?,
                source: row.get(4)?,
                is_new: row.get::<_, i32>(5)? != 0,
                created_at: row.get(6)?,
            })
        })?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }

    pub fn delete_goal_wiki_link(&self, goal_id: &str, wiki_page_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "DELETE FROM goal_wiki_links WHERE goal_id = ?1 AND wiki_page_id = ?2",
            params![goal_id, wiki_page_id],
        )?;
        Ok(())
    }

    pub fn mark_goal_wiki_links_seen(&self, goal_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE goal_wiki_links SET is_new = 0 WHERE goal_id = ?1 AND is_new = 1",
            params![goal_id],
        )?;
        Ok(())
    }

    pub fn set_wiki_read_status(
        &self,
        wiki_page_id: &str,
        is_read: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO wiki_reading_status (wiki_page_id, is_read, updated_at)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(wiki_page_id) DO UPDATE SET is_read = excluded.is_read, updated_at = excluded.updated_at",
            params![wiki_page_id, is_read as i32, now],
        )?;
        Ok(())
    }

    pub fn get_wiki_read_status(&self, wiki_page_id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT is_read FROM wiki_reading_status WHERE wiki_page_id = ?1"
        )?;
        let mut rows = stmt.query_map(params![wiki_page_id], |row| {
            Ok(row.get::<_, i32>(0)? != 0)
        })?;
        Ok(rows.next().and_then(|r| r.ok()).unwrap_or(false))
    }

    // ----- Exam -----

    pub fn save_exam(&self, exam: &super::models::Exam) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "INSERT INTO exams (id, goal_id, title, total_questions, score, grade, status, started_at, completed_at, diagnosis_json, created_at, version, question_config)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![exam.id, exam.goal_id, exam.title, exam.total_questions, exam.score, exam.grade, exam.status, exam.started_at, exam.completed_at, exam.diagnosis_json, exam.created_at, exam.version, exam.question_config],
        )?;
        Ok(())
    }

    pub fn update_exam(&self, exam: &super::models::Exam) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE exams SET title=?1, score=?2, grade=?3, status=?4, completed_at=?5, diagnosis_json=?6 WHERE id=?7",
            params![exam.title, exam.score, exam.grade, exam.status, exam.completed_at, exam.diagnosis_json, exam.id],
        )?;
        Ok(())
    }

    pub fn get_exam_by_id(&self, id: &str) -> Result<Option<super::models::Exam>, Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, goal_id, title, total_questions, score, grade, status, started_at, completed_at, diagnosis_json, created_at, version, question_config FROM exams WHERE id = ?1"
        )?;
        let mut rows = stmt.query_map(params![id], |row| {
            Ok(super::models::Exam {
                id: row.get(0)?,
                goal_id: row.get(1)?,
                title: row.get(2)?,
                total_questions: row.get(3)?,
                score: row.get(4)?,
                grade: row.get(5)?,
                status: row.get(6)?,
                started_at: row.get(7)?,
                completed_at: row.get(8)?,
                diagnosis_json: row.get(9)?,
                created_at: row.get(10)?,
                version: row.get(11)?,
                question_config: row.get(12)?,
            })
        })?;
        match rows.next() {
            Some(Ok(e)) => Ok(Some(e)),
            Some(Err(e)) => Err(Box::new(e)),
            None => Ok(None),
        }
    }

    pub fn get_exams_by_goal(&self, goal_id: &str) -> Result<Vec<super::models::ExamSummary>, Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, title, score, grade, total_questions, status, created_at FROM exams WHERE goal_id = ?1 ORDER BY created_at DESC"
        )?;
        let rows = stmt.query_map(params![goal_id], |row| {
            Ok(super::models::ExamSummary {
                id: row.get(0)?,
                title: row.get(1)?,
                score: row.get(2)?,
                grade: row.get(3)?,
                total_questions: row.get(4)?,
                status: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    // ----- ExamQuestion -----

    pub fn save_exam_question(&self, q: &super::models::ExamQuestion) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "INSERT INTO exam_questions (id, exam_id, wiki_page_id, question_type, question_json, user_answer, correct_answer, is_correct, score, ai_feedback, sort_order, answered_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![q.id, q.exam_id, q.wiki_page_id, q.question_type, q.question_json, q.user_answer, q.correct_answer, q.is_correct, q.score, q.ai_feedback, q.sort_order, q.answered_at],
        )?;
        Ok(())
    }

    pub fn update_exam_question_answer(&self, question_id: &str, user_answer: &str, is_correct: Option<bool>, score: Option<f64>, ai_feedback: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE exam_questions SET user_answer=?1, is_correct=?2, score=?3, ai_feedback=?4, answered_at=?5 WHERE id=?6",
            params![user_answer, is_correct.map(|b| b as i32), score, ai_feedback, now, question_id],
        )?;
        Ok(())
    }

    pub fn get_exam_questions(&self, exam_id: &str) -> Result<Vec<super::models::ExamQuestion>, Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, exam_id, wiki_page_id, question_type, question_json, user_answer, correct_answer, is_correct, score, ai_feedback, sort_order, answered_at FROM exam_questions WHERE exam_id = ?1 ORDER BY sort_order"
        )?;
        let rows = stmt.query_map(params![exam_id], |row| {
            Ok(super::models::ExamQuestion {
                id: row.get(0)?,
                exam_id: row.get(1)?,
                wiki_page_id: row.get(2)?,
                question_type: row.get(3)?,
                question_json: row.get(4)?,
                user_answer: row.get(5)?,
                correct_answer: row.get(6)?,
                is_correct: row.get::<_, Option<i32>>(7)?.map(|v| v != 0),
                score: row.get(8)?,
                ai_feedback: row.get(9)?,
                sort_order: row.get(10)?,
                answered_at: row.get(11)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    // ----- GoalRecommendation -----

    pub fn save_goal_recommendation(&self, rec: &super::models::GoalRecommendation) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "INSERT INTO goal_recommendations (id, goal_id, title, url, summary, difficulty, sort_order, status, imported_content_id, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![rec.id, rec.goal_id, rec.title, rec.url, rec.summary, rec.difficulty, rec.sort_order, rec.status, rec.imported_content_id, rec.created_at],
        )?;
        Ok(())
    }

    pub fn get_goal_recommendations(&self, goal_id: &str) -> Result<Vec<super::models::GoalRecommendation>, Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, goal_id, title, url, summary, difficulty, sort_order, status, imported_content_id, created_at FROM goal_recommendations WHERE goal_id = ?1 ORDER BY sort_order"
        )?;
        let rows = stmt.query_map(params![goal_id], |row| {
            Ok(super::models::GoalRecommendation {
                id: row.get(0)?,
                goal_id: row.get(1)?,
                title: row.get(2)?,
                url: row.get(3)?,
                summary: row.get(4)?,
                difficulty: row.get(5)?,
                sort_order: row.get(6)?,
                status: row.get(7)?,
                imported_content_id: row.get(8)?,
                created_at: row.get(9)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn update_goal_recommendation_status(&self, id: &str, status: &str, imported_content_id: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE goal_recommendations SET status = ?1, imported_content_id = ?2 WHERE id = ?3",
            params![status, imported_content_id, id],
        )?;
        Ok(())
    }

    pub fn delete_goal_recommendations(&self, goal_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        conn.execute("DELETE FROM goal_recommendations WHERE goal_id = ?1", params![goal_id])?;
        Ok(())
    }

    // ----- LearningPath -----

    pub fn save_learning_path(
        &self,
        lp: &super::models::LearningPath,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "INSERT INTO learning_paths (id, title, description, topic, difficulty, estimated_days, module_count, completion_rate, is_active, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                lp.id, lp.title, lp.description, lp.topic, lp.difficulty,
                lp.estimated_days, lp.module_count, lp.completion_rate, lp.is_active,
                lp.created_at, lp.updated_at,
            ],
        )?;
        Ok(())
    }

    pub fn update_learning_path(
        &self,
        lp: &super::models::LearningPath,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE learning_paths SET title=?1, description=?2, topic=?3, difficulty=?4, estimated_days=?5, module_count=?6, completion_rate=?7, is_active=?8, updated_at=?9 WHERE id=?10",
            params![
                lp.title, lp.description, lp.topic, lp.difficulty,
                lp.estimated_days, lp.module_count, lp.completion_rate, lp.is_active,
                lp.updated_at, lp.id,
            ],
        )?;
        Ok(())
    }

    pub fn get_all_learning_paths(
        &self,
    ) -> Result<Vec<super::models::LearningPath>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, title, description, topic, difficulty, estimated_days, module_count, completion_rate, is_active, created_at, updated_at
             FROM learning_paths ORDER BY created_at DESC"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(super::models::LearningPath {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                topic: row.get(3)?,
                difficulty: row.get(4)?,
                estimated_days: row.get(5)?,
                module_count: row.get(6)?,
                completion_rate: row.get(7)?,
                is_active: row.get::<_, i32>(8)? != 0,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn get_learning_path_by_id(
        &self,
        id: &str,
    ) -> Result<Option<super::models::LearningPath>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, title, description, topic, difficulty, estimated_days, module_count, completion_rate, is_active, created_at, updated_at
             FROM learning_paths WHERE id = ?1"
        )?;
        let mut rows = stmt.query_map(params![id], |row| {
            Ok(super::models::LearningPath {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                topic: row.get(3)?,
                difficulty: row.get(4)?,
                estimated_days: row.get(5)?,
                module_count: row.get(6)?,
                completion_rate: row.get(7)?,
                is_active: row.get::<_, i32>(8)? != 0,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
            })
        })?;
        match rows.next() {
            Some(Ok(lp)) => Ok(Some(lp)),
            Some(Err(e)) => Err(Box::new(e)),
            None => Ok(None),
        }
    }

    pub fn delete_learning_path(
        &self,
        id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute("DELETE FROM learning_paths WHERE id = ?1", params![id])?;
        Ok(())
    }

    // ----- Module -----

    pub fn save_modules_by_path(
        &self,
        modules: &[super::models::Module],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        for m in modules {
            conn.execute(
                "INSERT INTO modules (id, path_id, title, sort_order, description, theory_markdown, reading_list_json, estimated_read_minutes, discussion_prompts, community_solutions, task_ids, status, completed_at, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
                params![
                    m.id, m.path_id, m.title, m.sort_order, m.description,
                    m.theory_markdown, m.reading_list_json, m.estimated_read_minutes,
                    m.discussion_prompts, m.community_solutions, m.task_ids, m.status,
                    m.completed_at, m.created_at, m.updated_at,
                ],
            )?;
        }
        Ok(())
    }

    pub fn get_modules_by_path_id(
        &self,
        path_id: &str,
    ) -> Result<Vec<super::models::Module>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, path_id, title, sort_order, description, theory_markdown, reading_list_json, estimated_read_minutes, discussion_prompts, community_solutions, task_ids, status, completed_at, created_at, updated_at
             FROM modules WHERE path_id = ?1 ORDER BY sort_order"
        )?;
        let rows = stmt.query_map(params![path_id], |row| {
            Ok(super::models::Module {
                id: row.get(0)?,
                path_id: row.get(1)?,
                title: row.get(2)?,
                sort_order: row.get(3)?,
                description: row.get(4)?,
                theory_markdown: row.get(5)?,
                reading_list_json: row.get(6)?,
                estimated_read_minutes: row.get(7)?,
                discussion_prompts: row.get(8)?,
                community_solutions: row.get(9)?,
                task_ids: row.get(10)?,
                status: row.get(11)?,
                completed_at: row.get(12)?,
                created_at: row.get(13)?,
                updated_at: row.get(14)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn get_module_by_id(
        &self,
        id: &str,
    ) -> Result<Option<super::models::Module>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, path_id, title, sort_order, description, theory_markdown, reading_list_json, estimated_read_minutes, discussion_prompts, community_solutions, task_ids, status, completed_at, created_at, updated_at
             FROM modules WHERE id = ?1"
        )?;
        let mut rows = stmt.query_map(params![id], |row| {
            Ok(super::models::Module {
                id: row.get(0)?,
                path_id: row.get(1)?,
                title: row.get(2)?,
                sort_order: row.get(3)?,
                description: row.get(4)?,
                theory_markdown: row.get(5)?,
                reading_list_json: row.get(6)?,
                estimated_read_minutes: row.get(7)?,
                discussion_prompts: row.get(8)?,
                community_solutions: row.get(9)?,
                task_ids: row.get(10)?,
                status: row.get(11)?,
                completed_at: row.get(12)?,
                created_at: row.get(13)?,
                updated_at: row.get(14)?,
            })
        })?;
        match rows.next() {
            Some(Ok(m)) => Ok(Some(m)),
            Some(Err(e)) => Err(Box::new(e)),
            None => Ok(None),
        }
    }

    pub fn update_module(
        &self,
        m: &super::models::Module,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE modules SET title=?1, sort_order=?2, description=?3, theory_markdown=?4, reading_list_json=?5, estimated_read_minutes=?6, discussion_prompts=?7, community_solutions=?8, task_ids=?9, status=?10, completed_at=?11, updated_at=?12 WHERE id=?13",
            params![
                m.title, m.sort_order, m.description, m.theory_markdown,
                m.reading_list_json, m.estimated_read_minutes, m.discussion_prompts,
                m.community_solutions, m.task_ids, m.status, m.completed_at,
                m.updated_at, m.id,
            ],
        )?;
        Ok(())
    }

    pub fn delete_module(
        &self,
        id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute("DELETE FROM modules WHERE id = ?1", params![id])?;
        Ok(())
    }

    // ----- PracticeTask -----

    pub fn save_practice_task(
        &self,
        task: &super::models::PracticeTask,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "INSERT INTO practice_tasks (id, module_id, title, description, difficulty, estimated_minutes, prerequisites, hint_content, reference_links, status, started_at, completed_at, attempt_count, is_starred, reflection, code_snippets, screenshots_json, created_wiki_pages, related_wiki_pages, tags, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22)",
            params![
                task.id, task.module_id, task.title, task.description, task.difficulty,
                task.estimated_minutes, task.prerequisites, task.hint_content,
                task.reference_links, task.status, task.started_at, task.completed_at,
                task.attempt_count, task.is_starred, task.reflection, task.code_snippets,
                task.screenshots_json, task.created_wiki_pages, task.related_wiki_pages,
                task.tags, task.created_at, task.updated_at,
            ],
        )?;
        Ok(())
    }

    pub fn update_practice_task(
        &self,
        task: &super::models::PracticeTask,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE practice_tasks SET title=?1, description=?2, difficulty=?3, estimated_minutes=?4, prerequisites=?5, hint_content=?6, reference_links=?7, status=?8, started_at=?9, completed_at=?10, attempt_count=?11, is_starred=?12, reflection=?13, code_snippets=?14, screenshots_json=?15, created_wiki_pages=?16, related_wiki_pages=?17, tags=?18, updated_at=?19 WHERE id=?20",
            params![
                task.title, task.description, task.difficulty, task.estimated_minutes,
                task.prerequisites, task.hint_content, task.reference_links, task.status,
                task.started_at, task.completed_at, task.attempt_count, task.is_starred,
                task.reflection, task.code_snippets, task.screenshots_json,
                task.created_wiki_pages, task.related_wiki_pages, task.tags,
                task.updated_at, task.id,
            ],
        )?;
        Ok(())
    }

    pub fn update_task_status(
        &self,
        id: &str,
        status: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE practice_tasks SET status=?1, updated_at=?2 WHERE id=?3",
            params![status, now, id],
        )?;
        // Update started_at or completed_at as appropriate
        if status == "in_progress" {
            conn.execute(
                "UPDATE practice_tasks SET started_at=COALESCE(started_at, ?1) WHERE id=?2",
                params![now, id],
            )?;
        } else if status == "completed" || status == "reviewed" {
            conn.execute(
                "UPDATE practice_tasks SET completed_at=COALESCE(completed_at, ?1) WHERE id=?2",
                params![now, id],
            )?;
        }
        Ok(())
    }

    pub fn get_tasks_by_module_id(
        &self,
        module_id: &str,
    ) -> Result<Vec<super::models::PracticeTask>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, module_id, title, description, difficulty, estimated_minutes, prerequisites, hint_content, reference_links, status, started_at, completed_at, attempt_count, is_starred, reflection, code_snippets, screenshots_json, created_wiki_pages, related_wiki_pages, tags, created_at, updated_at
             FROM practice_tasks WHERE module_id = ?1 ORDER BY created_at"
        )?;
        let rows = stmt.query_map(params![module_id], |row| {
            Ok(super::models::PracticeTask {
                id: row.get(0)?,
                module_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                difficulty: row.get(4)?,
                estimated_minutes: row.get(5)?,
                prerequisites: row.get(6)?,
                hint_content: row.get(7)?,
                reference_links: row.get(8)?,
                status: row.get(9)?,
                started_at: row.get(10)?,
                completed_at: row.get(11)?,
                attempt_count: row.get(12)?,
                is_starred: row.get::<_, i32>(13)? != 0,
                reflection: row.get(14)?,
                code_snippets: row.get(15)?,
                screenshots_json: row.get(16)?,
                created_wiki_pages: row.get(17)?,
                related_wiki_pages: row.get(18)?,
                tags: row.get(19)?,
                created_at: row.get(20)?,
                updated_at: row.get(21)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn get_all_practice_tasks(
        &self,
    ) -> Result<Vec<super::models::PracticeTask>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, module_id, title, description, difficulty, estimated_minutes, prerequisites, hint_content, reference_links, status, started_at, completed_at, attempt_count, is_starred, reflection, code_snippets, screenshots_json, created_wiki_pages, related_wiki_pages, tags, created_at, updated_at
             FROM practice_tasks ORDER BY created_at DESC"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(super::models::PracticeTask {
                id: row.get(0)?,
                module_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                difficulty: row.get(4)?,
                estimated_minutes: row.get(5)?,
                prerequisites: row.get(6)?,
                hint_content: row.get(7)?,
                reference_links: row.get(8)?,
                status: row.get(9)?,
                started_at: row.get(10)?,
                completed_at: row.get(11)?,
                attempt_count: row.get(12)?,
                is_starred: row.get::<_, i32>(13)? != 0,
                reflection: row.get(14)?,
                code_snippets: row.get(15)?,
                screenshots_json: row.get(16)?,
                created_wiki_pages: row.get(17)?,
                related_wiki_pages: row.get(18)?,
                tags: row.get(19)?,
                created_at: row.get(20)?,
                updated_at: row.get(21)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn get_task_by_id(
        &self,
        id: &str,
    ) -> Result<Option<super::models::PracticeTask>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, module_id, title, description, difficulty, estimated_minutes, prerequisites, hint_content, reference_links, status, started_at, completed_at, attempt_count, is_starred, reflection, code_snippets, screenshots_json, created_wiki_pages, related_wiki_pages, tags, created_at, updated_at
             FROM practice_tasks WHERE id = ?1"
        )?;
        let mut rows = stmt.query_map(params![id], |row| {
            Ok(super::models::PracticeTask {
                id: row.get(0)?,
                module_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                difficulty: row.get(4)?,
                estimated_minutes: row.get(5)?,
                prerequisites: row.get(6)?,
                hint_content: row.get(7)?,
                reference_links: row.get(8)?,
                status: row.get(9)?,
                started_at: row.get(10)?,
                completed_at: row.get(11)?,
                attempt_count: row.get(12)?,
                is_starred: row.get::<_, i32>(13)? != 0,
                reflection: row.get(14)?,
                code_snippets: row.get(15)?,
                screenshots_json: row.get(16)?,
                created_wiki_pages: row.get(17)?,
                related_wiki_pages: row.get(18)?,
                tags: row.get(19)?,
                created_at: row.get(20)?,
                updated_at: row.get(21)?,
            })
        })?;
        match rows.next() {
            Some(Ok(task)) => Ok(Some(task)),
            Some(Err(e)) => Err(Box::new(e)),
            None => Ok(None),
        }
    }

    pub fn delete_practice_task(
        &self,
        id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute("DELETE FROM practice_tasks WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn count_tasks_by_status(&self) -> Result<(i32, i32, i32), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let total: i32 = conn
            .query_row("SELECT COUNT(*) FROM practice_tasks", [], |row| row.get(0))?;
        let completed: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM practice_tasks WHERE status IN ('completed', 'reviewed')",
                [],
                |row| row.get(0),
            )?;
        let in_progress: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM practice_tasks WHERE status = 'in_progress'",
                [],
                |row| row.get(0),
            )?;
        Ok((total, completed, in_progress))
    }

    // ----- TaskDailyLog -----

    pub fn save_or_update_today_log(
        &self,
        log: &super::models::TaskDailyLog,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "INSERT INTO task_daily_logs (id, date, total_minutes, tasks_completed, tasks_in_progress, streak_day, reflection, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
             ON CONFLICT(date) DO UPDATE SET
                total_minutes=excluded.total_minutes,
                tasks_completed=excluded.tasks_completed,
                tasks_in_progress=excluded.tasks_in_progress,
                streak_day=excluded.streak_day,
                reflection=excluded.reflection",
            params![
                log.id, log.date, log.total_minutes, log.tasks_completed,
                log.tasks_in_progress, log.streak_day, log.reflection, log.created_at,
            ],
        )?;
        Ok(())
    }

    pub fn get_log_by_date(
        &self,
        date: &str,
    ) -> Result<Option<super::models::TaskDailyLog>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, date, total_minutes, tasks_completed, tasks_in_progress, streak_day, reflection, created_at
             FROM task_daily_logs WHERE date = ?1"
        )?;
        let mut rows = stmt.query_map(params![date], |row| {
            Ok(super::models::TaskDailyLog {
                id: row.get(0)?,
                date: row.get(1)?,
                total_minutes: row.get(2)?,
                tasks_completed: row.get(3)?,
                tasks_in_progress: row.get(4)?,
                streak_day: row.get(5)?,
                reflection: row.get(6)?,
                created_at: row.get(7)?,
            })
        })?;
        match rows.next() {
            Some(Ok(log)) => Ok(Some(log)),
            Some(Err(e)) => Err(Box::new(e)),
            None => Ok(None),
        }
    }

    pub fn get_recent_daily_logs(
        &self,
        days: i32,
    ) -> Result<Vec<super::models::TaskDailyLog>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, date, total_minutes, tasks_completed, tasks_in_progress, streak_day, reflection, created_at
             FROM task_daily_logs ORDER BY date DESC LIMIT ?1"
        )?;
        let rows = stmt.query_map(params![days], |row| {
            Ok(super::models::TaskDailyLog {
                id: row.get(0)?,
                date: row.get(1)?,
                total_minutes: row.get(2)?,
                tasks_completed: row.get(3)?,
                tasks_in_progress: row.get(4)?,
                streak_day: row.get(5)?,
                reflection: row.get(6)?,
                created_at: row.get(7)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        // Reverse to chronological order
        results.reverse();
        Ok(results)
    }

    pub fn calculate_streak(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let mut streak = 0;
        let mut cursor = today.clone();
        loop {
            let has_record: bool = conn
                .query_row(
                    "SELECT COUNT(*) FROM task_daily_logs WHERE date = ?1 AND tasks_completed > 0",
                    params![&cursor],
                    |row| row.get::<_, i32>(0),
                )
                .map(|c| c > 0)
                .unwrap_or(false);
            if has_record {
                streak += 1;
                // Move to previous day
                let date =
                    chrono::NaiveDate::parse_from_str(&cursor, "%Y-%m-%d").unwrap_or_default();
                cursor = (date - chrono::Duration::days(1))
                    .format("%Y-%m-%d")
                    .to_string();
            } else {
                break;
            }
        }
        Ok(streak)
    }

    pub fn get_total_study_minutes(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let total: i32 = conn
            .query_row(
                "SELECT COALESCE(SUM(total_minutes), 0) FROM task_daily_logs",
                [],
                |row| row.get(0),
            )?;
        Ok(total)
    }

    pub fn get_topic_distribution(
        &self,
    ) -> Result<Vec<super::models::TopicCount>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT COALESCE(tags, '未分类') as topic, COUNT(*) as count
             FROM practice_tasks GROUP BY topic ORDER BY count DESC"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(super::models::TopicCount {
                topic: row.get(0)?,
                count: row.get(1)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    // ----- TaskWikiLink (E-2-3) -----

    pub fn add_task_wiki_link(
        &self,
        link: &super::models::TaskWikiLink,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "INSERT OR IGNORE INTO task_wiki_links (id, task_id, wiki_id, relevance_score, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![link.id, link.task_id, link.wiki_id, link.relevance_score, link.created_at],
        )?;
        Ok(())
    }

    pub fn remove_task_wiki_link(
        &self,
        task_id: &str,
        wiki_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "DELETE FROM task_wiki_links WHERE task_id = ?1 AND wiki_id = ?2",
            params![task_id, wiki_id],
        )?;
        Ok(())
    }

    pub fn get_wiki_pages_by_task(
        &self,
        task_id: &str,
    ) -> Result<Vec<super::models::WikiPage>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT wp.id, wp.title, wp.slug, wp.page_type, wp.body_markdown, wp.summary, wp.tags, wp.status, wp.confidence, wp.created_at, wp.updated_at, wp.last_compiled_at, wp.source_message_id, wp.author_name, wp.author_url, wp.source_type, wp.source_task_id
             FROM wiki_pages wp
             JOIN task_wiki_links twl ON twl.wiki_id = wp.id
             WHERE twl.task_id = ?1
             ORDER BY twl.relevance_score DESC"
        )?;
        let rows = stmt.query_map(params![task_id], |row| {
            Ok(super::models::WikiPage {
                id: row.get(0)?,
                title: row.get(1)?,
                slug: row.get(2)?,
                page_type: row.get(3)?,
                body_markdown: row.get(4)?,
                summary: row.get(5)?,
                tags: row.get(6)?,
                status: row.get(7)?,
                confidence: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
                last_compiled_at: row.get(11)?,
                source_message_id: row.get(12).unwrap_or(None),
                author_name: row.get(13).unwrap_or(None),
                author_url: row.get(14).unwrap_or(None),
                source_type: row.get(15).unwrap_or(None),
                source_task_id: row.get(16).unwrap_or(None),
                monitor_enabled: row.get::<_, Option<i32>>(17).unwrap_or(Some(0)) != Some(0),
                monitor_query: row.get(18).unwrap_or(None),
                monitor_sources: row.get::<_, Option<String>>(19).unwrap_or(Some("[]".to_string())).unwrap_or_else(|| "[]".to_string()),
                last_discovered_at: row.get(20).unwrap_or(None),
                pending_count: row.get::<_, Option<i32>>(21).unwrap_or(Some(0)).unwrap_or(0),
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn get_tasks_by_wiki(
        &self,
        wiki_id: &str,
    ) -> Result<Vec<super::models::PracticeTask>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT pt.id, pt.module_id, pt.title, pt.description, pt.difficulty, pt.estimated_minutes, pt.prerequisites, pt.hint_content, pt.reference_links, pt.status, pt.started_at, pt.completed_at, pt.attempt_count, pt.is_starred, pt.reflection, pt.code_snippets, pt.screenshots_json, pt.created_wiki_pages, pt.related_wiki_pages, pt.tags, pt.created_at, pt.updated_at
             FROM practice_tasks pt
             JOIN task_wiki_links twl ON twl.task_id = pt.id
             WHERE twl.wiki_id = ?1
             ORDER BY twl.relevance_score DESC"
        )?;
        let rows = stmt.query_map(params![wiki_id], |row| {
            Ok(super::models::PracticeTask {
                id: row.get(0)?,
                module_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                difficulty: row.get(4)?,
                estimated_minutes: row.get(5)?,
                prerequisites: row.get(6)?,
                hint_content: row.get(7)?,
                reference_links: row.get(8)?,
                status: row.get(9)?,
                started_at: row.get(10)?,
                completed_at: row.get(11)?,
                attempt_count: row.get(12)?,
                is_starred: row.get::<_, i32>(13)? != 0,
                reflection: row.get(14)?,
                code_snippets: row.get(15)?,
                screenshots_json: row.get(16)?,
                created_wiki_pages: row.get(17)?,
                related_wiki_pages: row.get(18)?,
                tags: row.get(19)?,
                created_at: row.get(20)?,
                updated_at: row.get(21)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn search_wiki_by_keyword(
        &self,
        query: &str,
        limit: i64,
    ) -> Result<Vec<super::models::TaskWikiMatch>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let pattern = format!("%{}%", query);
        let mut stmt = conn.prepare(
            "SELECT id, title FROM wiki_pages
             WHERE (title LIKE ?1 OR body_markdown LIKE ?1 OR COALESCE(summary, '') LIKE ?1 OR COALESCE(tags, '') LIKE ?1)
             AND status = 'active'
             ORDER BY created_at DESC LIMIT ?2"
        )?;
        let rows = stmt.query_map(params![pattern, limit], |row| {
            let _id: String = row.get(0)?;
            let title: String = row.get(1)?;
            Ok(super::models::TaskWikiMatch {
                wiki_id: _id,
                title,
                score: 1.0,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    // ----- TaskSolution (E-2-6) -----

    pub fn create_task_solution(
        &self,
        sol: &super::models::TaskSolution,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "INSERT INTO task_solutions (id, task_id, title, author, source_url, content, solution_type, difficulty_rating, quality_rating, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                sol.id, sol.task_id, sol.title, sol.author, sol.source_url,
                sol.content, sol.solution_type, sol.difficulty_rating, sol.quality_rating,
                sol.created_at,
            ],
        )?;
        Ok(())
    }

    pub fn get_solutions_for_task(
        &self,
        task_id: &str,
    ) -> Result<Vec<super::models::TaskSolution>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, task_id, title, author, source_url, content, solution_type, difficulty_rating, quality_rating, created_at
             FROM task_solutions WHERE task_id = ?1 ORDER BY created_at DESC"
        )?;
        let rows = stmt.query_map(params![task_id], |row| {
            Ok(super::models::TaskSolution {
                id: row.get(0)?,
                task_id: row.get(1)?,
                title: row.get(2)?,
                author: row.get(3)?,
                source_url: row.get(4)?,
                content: row.get(5)?,
                solution_type: row.get(6)?,
                difficulty_rating: row.get(7)?,
                quality_rating: row.get(8)?,
                created_at: row.get(9)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn get_solutions_for_task_for_update(
        &self,
        id: &str,
    ) -> Result<Option<super::models::TaskSolution>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, task_id, title, author, source_url, content, solution_type, difficulty_rating, quality_rating, created_at
             FROM task_solutions WHERE id = ?1"
        )?;
        let mut rows = stmt.query_map(params![id], |row| {
            Ok(super::models::TaskSolution {
                id: row.get(0)?,
                task_id: row.get(1)?,
                title: row.get(2)?,
                author: row.get(3)?,
                source_url: row.get(4)?,
                content: row.get(5)?,
                solution_type: row.get(6)?,
                difficulty_rating: row.get(7)?,
                quality_rating: row.get(8)?,
                created_at: row.get(9)?,
            })
        })?;
        match rows.next() {
            Some(Ok(sol)) => Ok(Some(sol)),
            Some(Err(e)) => Err(Box::new(e)),
            None => Ok(None),
        }
    }

    pub fn update_task_solution(
        &self,
        sol: &super::models::TaskSolution,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE task_solutions SET title=?1, author=?2, source_url=?3, content=?4, solution_type=?5, difficulty_rating=?6, quality_rating=?7 WHERE id=?8",
            params![
                sol.title, sol.author, sol.source_url, sol.content,
                sol.solution_type, sol.difficulty_rating, sol.quality_rating, sol.id,
            ],
        )?;
        Ok(())
    }

    pub fn delete_task_solution(
        &self,
        id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute("DELETE FROM task_solutions WHERE id = ?1", params![id])?;
        Ok(())
    }

    // ----- Wiki Page Author & Content update (E-2-6, E-3-2) -----

    pub fn update_wiki_page_author(
        &self,
        page_id: &str,
        author_name: Option<&str>,
        author_url: Option<&str>,
        source_type: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE wiki_pages SET author_name=?1, author_url=?2, source_type=COALESCE(?3, source_type), updated_at=datetime('now') WHERE id=?4",
            params![author_name, author_url, source_type, page_id],
        )?;
        Ok(())
    }

    pub fn update_wiki_page_content(
        &self,
        page_id: &str,
        body_markdown: &str,
        slug: &str,
        summary: Option<&str>,
        source_task_id: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE wiki_pages SET body_markdown=?1, slug=?2, summary=?3, source_task_id=COALESCE(?4, source_task_id), updated_at=datetime('now') WHERE id=?5",
            params![body_markdown, slug, summary, source_task_id, page_id],
        )?;
        Ok(())
    }

    /// Fetch wiki pages for analysis (title, body, summary, tags) — used by
    /// recommendation engine and reflection analysis.
    pub fn get_wiki_analysis_pages(
        &self,
        limit: i64,
    ) -> Result<Vec<super::models::WikiPage>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, title, slug, page_type, body_markdown, summary, tags, status, confidence, created_at, updated_at, last_compiled_at, source_message_id, author_name, author_url, source_type, source_task_id, monitor_enabled, monitor_query, monitor_sources, last_discovered_at, pending_count
             FROM wiki_pages WHERE status = 'active' ORDER BY updated_at DESC LIMIT ?1"
        )?;
        let rows = stmt.query_map(params![limit], |row| {
            Ok(super::models::WikiPage {
                id: row.get(0)?,
                title: row.get(1)?,
                slug: row.get(2)?,
                page_type: row.get(3)?,
                body_markdown: row.get(4)?,
                summary: row.get(5)?,
                tags: row.get(6)?,
                status: row.get(7)?,
                confidence: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
                last_compiled_at: row.get(11)?,
                source_message_id: row.get(12).unwrap_or(None),
                author_name: row.get(13).unwrap_or(None),
                author_url: row.get(14).unwrap_or(None),
                source_type: row.get(15).unwrap_or(None),
                source_task_id: row.get(16).unwrap_or(None),
                monitor_enabled: row.get::<_, Option<i32>>(17).unwrap_or(Some(0)) != Some(0),
                monitor_query: row.get(18).unwrap_or(None),
                monitor_sources: row.get::<_, Option<String>>(19).unwrap_or(Some("[]".to_string())).unwrap_or_else(|| "[]".to_string()),
                last_discovered_at: row.get(20).unwrap_or(None),
                pending_count: row.get::<_, Option<i32>>(21).unwrap_or(Some(0)).unwrap_or(0),
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    // ========== TaskRecommendations (E-3-1) ==========

    pub fn save_recommendation(
        &self,
        rec: &super::models::TaskRecommendation,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "INSERT INTO task_recommendations (id, wiki_page_id, task_template_title, task_template_description, task_template_difficulty, task_template_tags, score, matched_keywords, status, ignore_count, created_task_id, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                rec.id, rec.wiki_page_id, rec.task_template_title,
                rec.task_template_description, rec.task_template_difficulty,
                rec.task_template_tags, rec.score, rec.matched_keywords, rec.status,
                rec.ignore_count, rec.created_task_id, rec.created_at, rec.updated_at,
            ],
        )?;
        Ok(())
    }

    pub fn get_all_recommendations(
        &self,
    ) -> Result<Vec<super::models::TaskRecommendation>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, wiki_page_id, task_template_title, task_template_description, task_template_difficulty, task_template_tags, score, matched_keywords, status, ignore_count, created_task_id, created_at, updated_at
             FROM task_recommendations ORDER BY score DESC, updated_at DESC"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(super::models::TaskRecommendation {
                id: row.get(0)?,
                wiki_page_id: row.get(1)?,
                task_template_title: row.get(2)?,
                task_template_description: row.get(3)?,
                task_template_difficulty: row.get(4)?,
                task_template_tags: row.get(5)?,
                score: row.get(6)?,
                matched_keywords: row.get(7)?,
                status: row.get(8)?,
                ignore_count: row.get(9)?,
                created_task_id: row.get(10)?,
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn get_recommendations_by_status(
        &self,
        status: &str,
    ) -> Result<Vec<super::models::TaskRecommendation>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, wiki_page_id, task_template_title, task_template_description, task_template_difficulty, task_template_tags, score, matched_keywords, status, ignore_count, created_task_id, created_at, updated_at
             FROM task_recommendations WHERE status = ?1 ORDER BY score DESC"
        )?;
        let rows = stmt.query_map(params![status], |row| {
            Ok(super::models::TaskRecommendation {
                id: row.get(0)?,
                wiki_page_id: row.get(1)?,
                task_template_title: row.get(2)?,
                task_template_description: row.get(3)?,
                task_template_difficulty: row.get(4)?,
                task_template_tags: row.get(5)?,
                score: row.get(6)?,
                matched_keywords: row.get(7)?,
                status: row.get(8)?,
                ignore_count: row.get(9)?,
                created_task_id: row.get(10)?,
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn get_recommendation_for_wiki_page(
        &self,
        wiki_page_id: &str,
    ) -> Result<Vec<super::models::TaskRecommendation>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, wiki_page_id, task_template_title, task_template_description, task_template_difficulty, task_template_tags, score, matched_keywords, status, ignore_count, created_task_id, created_at, updated_at
             FROM task_recommendations WHERE wiki_page_id = ?1 ORDER BY score DESC"
        )?;
        let rows = stmt.query_map(params![wiki_page_id], |row| {
            Ok(super::models::TaskRecommendation {
                id: row.get(0)?,
                wiki_page_id: row.get(1)?,
                task_template_title: row.get(2)?,
                task_template_description: row.get(3)?,
                task_template_difficulty: row.get(4)?,
                task_template_tags: row.get(5)?,
                score: row.get(6)?,
                matched_keywords: row.get(7)?,
                status: row.get(8)?,
                ignore_count: row.get(9)?,
                created_task_id: row.get(10)?,
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn increment_recommendation_ignore(
        &self,
        id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE task_recommendations SET ignore_count = ignore_count + 1, updated_at = ?1 WHERE id = ?2",
            params![chrono::Utc::now().to_rfc3339(), id],
        )?;
        Ok(())
    }

    pub fn update_recommendation_status(
        &self,
        id: &str,
        status: &str,
        created_task_id: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE task_recommendations SET status=?1, created_task_id=COALESCE(?2, created_task_id), updated_at=?3 WHERE id=?4",
            params![status, created_task_id, chrono::Utc::now().to_rfc3339(), id],
        )?;
        Ok(())
    }

    pub fn delete_recommendation(
        &self,
        id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {e}"))?;
        conn.execute(
            "DELETE FROM task_recommendations WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    // ========== Review Format Rotation (Sprint 5A, E-5-5) ==========

    /// All available review formats (can be extended).
    const ALL_FORMATS: &[&str] = &["choice", "judgment", "essay"];

    /// Get the last N review format records for a schedule.
    pub fn get_review_format_history(
        &self,
        schedule_id: &str,
        limit: i32,
    ) -> Result<Vec<super::models::ReviewFormatHistory>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {e}"))?;
        let mut stmt = conn.prepare(
            "SELECT id, schedule_id, format, used_at
             FROM review_format_history
             WHERE schedule_id = ?1
             ORDER BY used_at DESC
             LIMIT ?2"
        )?;
        let rows = stmt.query_map(params![schedule_id, limit], |row| {
            Ok(super::models::ReviewFormatHistory {
                id: row.get(0)?,
                schedule_id: row.get(1)?,
                format: row.get(2)?,
                used_at: row.get(3)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    /// Record a review format usage.
    pub fn record_review_format(
        &self,
        schedule_id: &str,
        format: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {e}"))?;
        let id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO review_format_history (id, schedule_id, format, used_at)
             VALUES (?1, ?2, ?3, datetime('now'))",
            params![id, schedule_id, format],
        )?;
        // Also update last_format on the schedule
        conn.execute(
            "UPDATE review_schedule SET last_format = ?1, updated_at = datetime('now') WHERE id = ?2",
            params![format, schedule_id],
        )?;
        Ok(())
    }

    /// Determine the next review format using the rotation algorithm.
    ///
    /// Algorithm:
    /// - 1st review (review_count == 0): random from ['choice', 'judgment']
    /// - 2nd-3rd review (review_count <= 2): avoid any previously used format
    /// - 4th+ review (review_count >= 3): avoid the last 2 used formats
    /// - "essay" only appears when review_count >= 2 (needs basic knowledge first)
    pub fn get_next_review_format(
        &self,
        schedule_id: &str,
        review_count: i32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let history = self.get_review_format_history(schedule_id, 5)?;
        let used_formats: Vec<&str> = history.iter().map(|h| h.format.as_str()).collect();

        // Filter out "essay" for users with less than 2 reviews
        let effective_formats: Vec<&str> = if review_count < 2 {
            Self::ALL_FORMATS.iter().filter(|f| **f != "essay").copied().collect()
        } else {
            Self::ALL_FORMATS.to_vec()
        };

        let available: Vec<&str> = if review_count == 0 {
            // First review: choice or judgment
            vec!["choice", "judgment"]
        } else if review_count <= 2 {
            // 2nd-3rd review: try unused formats
            let unused: Vec<&&str> = effective_formats
                .iter()
                .filter(|f| !used_formats.contains(f))
                .collect();
            if unused.is_empty() {
                effective_formats.to_vec()
            } else {
                unused.into_iter().copied().collect()
            }
        } else {
            // 4th+ review: avoid last 2 used formats
            let exclude: Vec<&str> = used_formats.iter().take(2).copied().collect();
            let avail: Vec<&str> = effective_formats
                .iter()
                .filter(|f| !exclude.contains(f))
                .copied()
                .collect();
            if avail.is_empty() {
                effective_formats.to_vec()
            } else {
                avail
            }
        };

        // Simple random selection
        let idx = rand::random::<usize>() % available.len();
        Ok(available[idx].to_string())
    }

    // ========== Review System (Sprint 4, E-4-1) ==========

    pub fn create_review_schedule(
        &self,
        wiki_page_id: &str,
    ) -> Result<super::models::ReviewSchedule, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {e}"))?;
        let now = chrono::Utc::now().to_rfc3339();
        let id = uuid::Uuid::new_v4().to_string();
        let schedule = super::models::ReviewSchedule {
            id: id.clone(),
            wiki_page_id: wiki_page_id.to_string(),
            ease_factor: 2.5,
            interval_days: 0,
            next_review_at: now.clone(),
            review_count: 0,
            last_reviewed_at: None,
            mastery: 0.0,
            is_archived: false,
            created_at: now.clone(),
            updated_at: now.clone(),
            last_format: None,
            variant_streak: 0,
            variant_mode: 0,
        };
        conn.execute(
            "INSERT INTO review_schedule (id, wiki_page_id, ease_factor, interval_days, next_review_at, review_count, last_reviewed_at, mastery, is_archived, created_at, updated_at, last_format)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                schedule.id, schedule.wiki_page_id, schedule.ease_factor,
                schedule.interval_days, schedule.next_review_at, schedule.review_count,
                schedule.last_reviewed_at, schedule.mastery, schedule.is_archived,
                schedule.created_at, schedule.updated_at, schedule.last_format,
            ],
        )?;
        Ok(schedule)
    }

    pub fn get_review_schedule(
        &self,
        wiki_page_id: &str,
    ) -> Result<Option<super::models::ReviewSchedule>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {e}"))?;
        let mut stmt = conn.prepare(
            "SELECT id, wiki_page_id, ease_factor, interval_days, next_review_at, review_count, last_reviewed_at, mastery, is_archived, created_at, updated_at, last_format, variant_streak, variant_mode
             FROM review_schedule WHERE wiki_page_id = ?1"
        )?;
        let mut rows = stmt.query_map(params![wiki_page_id], |row| {
            Ok(super::models::ReviewSchedule {
                id: row.get(0)?,
                wiki_page_id: row.get(1)?,
                ease_factor: row.get(2)?,
                interval_days: row.get(3)?,
                next_review_at: row.get(4)?,
                review_count: row.get(5)?,
                last_reviewed_at: row.get(6)?,
                mastery: row.get(7)?,
                is_archived: row.get::<_, i32>(8)? != 0,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
                last_format: row.get(11)?,
                variant_streak: row.get::<_, i32>(12)?,
                variant_mode: row.get::<_, i32>(13)?,
            })
        })?;
        match rows.next() {
            Some(Ok(s)) => Ok(Some(s)),
            Some(Err(e)) => Err(Box::new(e)),
            None => Ok(None),
        }
    }

    pub fn get_due_reviews(
        &self,
        limit: i64,
    ) -> Result<Vec<super::models::DueReviewItem>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {e}"))?;
        let now = chrono::Utc::now().to_rfc3339();
        let mut results = {
            let mut stmt = conn.prepare(
            "SELECT rs.id, rs.wiki_page_id, rs.ease_factor, rs.interval_days, rs.next_review_at, rs.review_count, rs.last_reviewed_at, rs.mastery, rs.is_archived, rs.created_at, rs.updated_at, rs.last_format, rs.variant_streak, rs.variant_mode, wp.title, wp.summary, wp.tags
             FROM review_schedule rs
             JOIN wiki_pages wp ON wp.id = rs.wiki_page_id
             WHERE rs.next_review_at <= ?1 AND rs.is_archived = 0
             ORDER BY rs.next_review_at ASC
             LIMIT ?2"
        )?;
        let rows = stmt.query_map(params![now, limit], |row| {
            let schedule = super::models::ReviewSchedule {
                id: row.get(0)?,
                wiki_page_id: row.get(1)?,
                ease_factor: row.get(2)?,
                interval_days: row.get(3)?,
                next_review_at: row.get(4)?,
                review_count: row.get(5)?,
                last_reviewed_at: row.get(6)?,
                mastery: row.get(7)?,
                is_archived: row.get::<_, i32>(8)? != 0,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
                last_format: row.get(11)?,
                variant_streak: row.get::<_, i32>(12).unwrap_or(0),
                variant_mode: row.get::<_, i32>(13).unwrap_or(0),
            };
            let wiki_title: String = row.get(14)?;
            let wiki_summary: Option<String> = row.get(15)?;
            let wiki_tags: Option<String> = row.get(16)?;
            Ok(super::models::DueReviewItem {
                schedule,
                wiki_title,
                wiki_summary,
                wiki_tags,
                next_format: None, // Will be filled below
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
            results
        }; // stmt drops here, conn borrow released
        drop(conn); // Release DB lock before calling back into Repository
        for item in &mut results {
            match self.get_next_review_format(&item.schedule.id, item.schedule.review_count) {
                Ok(fmt) => item.next_format = Some(fmt),
                Err(e) => {
                    log::warn!("Failed to compute next format for {}: {}", item.schedule.id, e);
                    item.next_format = Some("choice".to_string());
                }
            }
        }
        Ok(results)
    }

    pub fn get_due_review_for_page(
        &self,
        wiki_page_id: &str,
    ) -> Result<Option<super::models::DueReviewItem>, Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        let mut result = {
            let mut stmt = conn.prepare(
                "SELECT rs.id, rs.wiki_page_id, rs.ease_factor, rs.interval_days, rs.next_review_at, rs.review_count, rs.last_reviewed_at, rs.mastery, rs.is_archived, rs.created_at, rs.updated_at, rs.last_format, rs.variant_streak, rs.variant_mode, wp.title, wp.summary, wp.tags
                 FROM review_schedule rs
                 JOIN wiki_pages wp ON wp.id = rs.wiki_page_id
                 WHERE rs.wiki_page_id = ?1 AND rs.is_archived = 0"
            )?;
            let mut rows = stmt.query_map(params![wiki_page_id], |row| {
                let schedule = super::models::ReviewSchedule {
                    id: row.get(0)?,
                    wiki_page_id: row.get(1)?,
                    ease_factor: row.get(2)?,
                    interval_days: row.get(3)?,
                    next_review_at: row.get(4)?,
                    review_count: row.get(5)?,
                    last_reviewed_at: row.get(6)?,
                    mastery: row.get(7)?,
                    is_archived: row.get::<_, i32>(8)? != 0,
                    created_at: row.get(9)?,
                    updated_at: row.get(10)?,
                    last_format: row.get(11)?,
                    variant_streak: row.get::<_, i32>(12).unwrap_or(0),
                    variant_mode: row.get::<_, i32>(13).unwrap_or(0),
                };
                let wiki_title: String = row.get(14)?;
                let wiki_summary: Option<String> = row.get(15)?;
                let wiki_tags: Option<String> = row.get(16)?;
                Ok(super::models::DueReviewItem {
                    schedule,
                    wiki_title,
                    wiki_summary,
                    wiki_tags,
                    next_format: None,
                })
            })?;
            match rows.next() {
                Some(Ok(item)) => Some(item),
                _ => None,
            }
        }; // stmt drops here
        drop(conn);
        if let Some(ref mut item) = result {
            match self.get_next_review_format(&item.schedule.id, item.schedule.review_count) {
                Ok(fmt) => item.next_format = Some(fmt),
                Err(e) => {
                    log::warn!("Failed to compute next format for {}: {}", item.schedule.id, e);
                    item.next_format = Some("choice".to_string());
                }
            }
        }
        Ok(result)
    }

    pub fn submit_review_feedback(
        &self,
        schedule_id: &str,
        quality: i32,
        session_id: Option<&str>,
        question_snapshot: Option<&str>,
    ) -> Result<super::models::ReviewSchedule, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {e}"))?;

        // Fetch existing schedule
        let mut stmt = conn.prepare(
            "SELECT id, wiki_page_id, ease_factor, interval_days, next_review_at, review_count, last_reviewed_at, mastery, is_archived, created_at, updated_at, last_format, variant_streak, variant_mode
             FROM review_schedule WHERE id = ?1"
        )?;
        let schedule = stmt.query_row(params![schedule_id], |row| {
            Ok(super::models::ReviewSchedule {
                id: row.get(0)?,
                wiki_page_id: row.get(1)?,
                ease_factor: row.get(2)?,
                interval_days: row.get(3)?,
                next_review_at: row.get(4)?,
                review_count: row.get(5)?,
                last_reviewed_at: row.get(6)?,
                mastery: row.get(7)?,
                is_archived: row.get::<_, i32>(8)? != 0,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
                last_format: row.get(11)?,
                variant_streak: row.get::<_, i32>(12)?,
                variant_mode: row.get::<_, i32>(13)?,
            })
        })?;

        let interval_before = schedule.interval_days;
        let ef_before = schedule.ease_factor;

        // Apply SM-2 algorithm
        let (new_interval, new_ef) = sm2_calculate(quality, schedule.ease_factor, schedule.interval_days);
        let now = chrono::Utc::now().to_rfc3339();
        let next_review = (chrono::Utc::now() + chrono::Duration::days(new_interval as i64)).to_rfc3339();
        let new_review_count = schedule.review_count + 1;

        // Calculate recent accuracy: for simplicity, use quality >= 1 as "correct"
        let recent_accuracy = if quality >= 1 { 1.0 } else { 0.0 };
        let mastery = calculate_mastery(new_review_count, recent_accuracy, new_interval);

        // ========== Variant Tracking (Sprint 6A, E-6-3) ==========
        let (new_variant_mode, new_variant_streak) = if schedule.variant_mode == 1 {
            if quality >= 1 {
                let streak = schedule.variant_streak + 1;
                if streak >= 3 { (0, 0) } else { (1, streak) }
            } else {
                (1, 0)
            }
        } else {
            if quality == 0 && schedule.review_count > 0 {
                (1, 0)
            } else {
                (schedule.variant_mode, schedule.variant_streak)
            }
        };  
        let is_variant_val: i32 = if new_variant_mode == 1 || schedule.variant_mode == 1 { 1 } else { 0 };
        let variant_gen_val: i32 = if is_variant_val == 1 { schedule.variant_streak + 1 } else { 0 };

        // Update schedule (including variant fields)
        conn.execute(
            "UPDATE review_schedule SET ease_factor=?1, interval_days=?2, next_review_at=?3, review_count=?4, last_reviewed_at=?5, mastery=?6, updated_at=?7, variant_streak=?8, variant_mode=?9 WHERE id=?10",
            params![new_ef, new_interval, next_review, new_review_count, now, mastery, now, new_variant_streak, new_variant_mode, schedule_id],
        )?;

        // Create review log with variant info
        let log_id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO review_logs (id, schedule_id, session_id, quality, interval_before, interval_after, ease_factor_before, ease_factor_after, reviewed_at, is_variant, variant_generation, question_snapshot)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![log_id, schedule_id, session_id, quality, interval_before, new_interval, ef_before, new_ef, now, is_variant_val, variant_gen_val, question_snapshot],
        )?;

        // Update daily_review_summary
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let summary_id = format!("review-{today}");
        let is_correct = if quality >= 1 { 1 } else { 0 };

        // Upsert daily summary
        let existing_summary: Option<(i32, i32, i32)> = conn.query_row(
            "SELECT total_reviewed, correct_count, streak_day FROM daily_review_summary WHERE date = ?1",
            params![today],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        ).ok();

        let (new_total, new_correct, new_streak) = if let Some((total, correct, streak)) = existing_summary {
            let new_total_val = total + 1;
            let new_correct_val = correct + is_correct;
            let new_streak_val = if is_correct > 0 { streak } else { 0 };
            (new_total_val, new_correct_val, new_streak_val)
        } else {
            let yesterday = (chrono::Utc::now() - chrono::Duration::days(1)).format("%Y-%m-%d").to_string();
            let prev_streak: i32 = conn.query_row(
                "SELECT streak_day FROM daily_review_summary WHERE date = ?1",
                params![yesterday],
                |row| row.get(0),
            ).unwrap_or(0);
            (1, is_correct, if is_correct > 0 { prev_streak + 1 } else { 0 })
        };

        conn.execute(
            "INSERT OR REPLACE INTO daily_review_summary (id, date, total_reviewed, correct_count, streak_day, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![summary_id, today, new_total, new_correct, new_streak, now],
        )?;

        // Return updated schedule
        Ok(super::models::ReviewSchedule {
            id: schedule.id,
            wiki_page_id: schedule.wiki_page_id,
            ease_factor: new_ef,
            interval_days: new_interval,
            next_review_at: next_review,
            review_count: new_review_count,
            last_reviewed_at: Some(now.clone()),
            mastery,
            is_archived: schedule.is_archived,
            created_at: schedule.created_at,
            updated_at: now,
            last_format: None,
            variant_streak: new_variant_streak,
            variant_mode: new_variant_mode,
        })
    }

    /// Submit review feedback with review format tracking (Sprint 5A).
    /// Records the format used and response time for analytics.
    pub fn submit_review_feedback_with_format(
        &self,
        schedule_id: &str,
        quality: i32,
        review_format: Option<&str>,
        response_time_seconds: Option<i32>,
        session_id: Option<&str>,
        question_snapshot: Option<&str>,
    ) -> Result<super::models::ReviewSchedule, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {e}"))?;

        // Fetch existing schedule (including variant fields)
        let mut stmt = conn.prepare(
            "SELECT id, wiki_page_id, ease_factor, interval_days, next_review_at, review_count, last_reviewed_at, mastery, is_archived, created_at, updated_at, last_format, variant_streak, variant_mode
             FROM review_schedule WHERE id = ?1"
        )?;
        let schedule = stmt.query_row(params![schedule_id], |row| {
            Ok(super::models::ReviewSchedule {
                id: row.get(0)?,
                wiki_page_id: row.get(1)?,
                ease_factor: row.get(2)?,
                interval_days: row.get(3)?,
                next_review_at: row.get(4)?,
                review_count: row.get(5)?,
                last_reviewed_at: row.get(6)?,
                mastery: row.get(7)?,
                is_archived: row.get::<_, i32>(8)? != 0,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
                last_format: row.get(11)?,
                variant_streak: row.get::<_, i32>(12).unwrap_or(0),
                variant_mode: row.get::<_, i32>(13).unwrap_or(0),
            })
        })?;

        let interval_before = schedule.interval_days;
        let ef_before = schedule.ease_factor;

        // Apply SM-2 algorithm
        let (new_interval, new_ef) = sm2_calculate(quality, schedule.ease_factor, schedule.interval_days);
        let now = chrono::Utc::now().to_rfc3339();
        let next_review = (chrono::Utc::now() + chrono::Duration::days(new_interval as i64)).to_rfc3339();
        let new_review_count = schedule.review_count + 1;

        // Calculate recent accuracy: for simplicity, use quality >= 1 as "correct"
        let recent_accuracy = if quality >= 1 { 1.0 } else { 0.0 };
        let mastery = calculate_mastery(new_review_count, recent_accuracy, new_interval);

        // ========== Variant Tracking (Sprint 6A, E-6-3) ==========
        let (new_variant_mode, new_variant_streak) = if schedule.variant_mode == 1 {
            // Already in variant mode
            if quality >= 1 {
                // Answered correctly — increment streak
                let streak = schedule.variant_streak + 1;
                if streak >= 3 {
                    // 3 consecutive correct → exit variant mode
                    (0, 0)
                } else {
                    (1, streak)
                }
            } else {
                // Wrong again in variant mode — reset streak
                (1, 0)
            }
        } else {
            // Normal mode
            if quality == 0 && schedule.review_count > 0 {
                // Wrong answer + not first review → enter variant mode
                (1, 0)
            } else {
                (schedule.variant_mode, schedule.variant_streak)
            }
        };

        // Determine variant log fields
        let is_variant_val: i32 = if new_variant_mode == 1 || schedule.variant_mode == 1 { 1 } else { 0 };
        let variant_gen_val: i32 = if is_variant_val == 1 { schedule.variant_streak + 1 } else { 0 };

        // Update schedule (including variant fields)
        conn.execute(
            "UPDATE review_schedule SET ease_factor=?1, interval_days=?2, next_review_at=?3, review_count=?4, last_reviewed_at=?5, mastery=?6, updated_at=?7, variant_streak=?8, variant_mode=?9 WHERE id=?10",
            params![new_ef, new_interval, next_review, new_review_count, now, mastery, now, new_variant_streak, new_variant_mode, schedule_id],
        )?;

        // Create review log with format, response time, and variant info
        let log_id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO review_logs (id, schedule_id, session_id, quality, interval_before, interval_after, ease_factor_before, ease_factor_after, reviewed_at, review_format, response_time_seconds, is_variant, variant_generation, question_snapshot)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            params![log_id, schedule_id, session_id, quality, interval_before, new_interval, ef_before, new_ef, now, review_format, response_time_seconds, is_variant_val, variant_gen_val, question_snapshot],
        )?;

        // Record format in review_format_history if provided
        if let Some(fmt) = review_format {
            let fmt_id = uuid::Uuid::new_v4().to_string();
            conn.execute(
                "INSERT INTO review_format_history (id, schedule_id, format, used_at) VALUES (?1, ?2, ?3, ?4)",
                params![fmt_id, schedule_id, fmt, now],
            )?;
            // Update last_format on schedule
            conn.execute(
                "UPDATE review_schedule SET last_format = ?1, updated_at = ?2 WHERE id = ?3",
                params![fmt, now, schedule_id],
            )?;
        }

        // Update daily_review_summary
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let summary_id = format!("review-{today}");
        let is_correct = if quality >= 1 { 1 } else { 0 };

        let existing_summary: Option<(i32, i32, i32)> = conn.query_row(
            "SELECT total_reviewed, correct_count, streak_day FROM daily_review_summary WHERE date = ?1",
            params![today],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        ).ok();

        let (new_total, new_correct, new_streak) = if let Some((total, correct, streak)) = existing_summary {
            (total + 1, correct + is_correct, if is_correct > 0 { streak } else { 0 })
        } else {
            let yesterday = (chrono::Utc::now() - chrono::Duration::days(1)).format("%Y-%m-%d").to_string();
            let prev_streak: i32 = conn.query_row(
                "SELECT streak_day FROM daily_review_summary WHERE date = ?1",
                params![yesterday],
                |row| row.get(0),
            ).unwrap_or(0);
            (1, is_correct, if is_correct > 0 { prev_streak + 1 } else { 0 })
        };

        conn.execute(
            "INSERT OR REPLACE INTO daily_review_summary (id, date, total_reviewed, correct_count, streak_day, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![summary_id, today, new_total, new_correct, new_streak, now],
        )?;

        // Return updated schedule
        Ok(super::models::ReviewSchedule {
            id: schedule.id,
            wiki_page_id: schedule.wiki_page_id,
            ease_factor: new_ef,
            interval_days: new_interval,
            next_review_at: next_review,
            review_count: new_review_count,
            last_reviewed_at: Some(now.clone()),
            mastery,
            is_archived: schedule.is_archived,
            created_at: schedule.created_at,
            updated_at: now,
            last_format: schedule.last_format.clone(),
            variant_streak: new_variant_streak,
            variant_mode: new_variant_mode,
        })
    }

    pub fn get_daily_review_summary(
        &self,
        date: &str,
    ) -> Result<Option<super::models::DailyReviewSummary>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {e}"))?;
        let mut stmt = conn.prepare(
            "SELECT id, date, total_reviewed, correct_count, streak_day, created_at
             FROM daily_review_summary WHERE date = ?1"
        )?;
        let mut rows = stmt.query_map(params![date], |row| {
            Ok(super::models::DailyReviewSummary {
                id: row.get(0)?,
                date: row.get(1)?,
                total_reviewed: row.get(2)?,
                correct_count: row.get(3)?,
                streak_day: row.get(4)?,
                created_at: row.get(5)?,
            })
        })?;
        match rows.next() {
            Some(Ok(s)) => Ok(Some(s)),
            Some(Err(e)) => Err(Box::new(e)),
            None => Ok(None),
        }
    }

    pub fn get_review_stats(&self) -> Result<(i32, i32, i32), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {e}"))?;
        let now = chrono::Utc::now().to_rfc3339();

        // Total due
        let total_due: i32 = conn.query_row(
            "SELECT COUNT(*) FROM review_schedule WHERE next_review_at <= ?1 AND is_archived = 0",
            params![now],
            |row| row.get(0),
        )?;

        // Today's reviewed count
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let total_reviewed_today: i32 = conn.query_row(
            "SELECT COALESCE(total_reviewed, 0) FROM daily_review_summary WHERE date = ?1",
            params![today],
            |row| row.get(0),
        ).unwrap_or(0);

        // Streak
        let streak: i32 = conn.query_row(
            "SELECT COALESCE(streak_day, 0) FROM daily_review_summary WHERE date = ?1",
            params![today],
            |row| row.get(0),
        ).unwrap_or(0);

        Ok((total_due, total_reviewed_today, streak))
    }

    // ========== Knowledge Health Panel (E-4-3) ==========

    pub fn get_all_review_schedules(&self) -> Result<Vec<(super::models::ReviewSchedule, super::models::WikiPage)>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {e}"))?;
        let mut stmt = conn.prepare(
            "SELECT rs.id, rs.wiki_page_id, rs.ease_factor, rs.interval_days, rs.next_review_at, rs.review_count, rs.last_reviewed_at, rs.mastery, rs.is_archived, rs.created_at, rs.updated_at, rs.last_format, rs.variant_streak, rs.variant_mode, wp.id, wp.title, wp.slug, wp.page_type, wp.body_markdown, wp.summary, wp.tags, wp.status, wp.confidence, wp.created_at, wp.updated_at, wp.last_compiled_at, wp.source_message_id, wp.author_name, wp.author_url, wp.source_type, wp.source_task_id, wp.monitor_enabled, wp.monitor_query, wp.monitor_sources, wp.last_discovered_at, wp.pending_count
             FROM review_schedule rs
             JOIN wiki_pages wp ON wp.id = rs.wiki_page_id
             WHERE rs.is_archived = 0
             ORDER BY rs.next_review_at ASC"
        )?;
        let rows = stmt.query_map([], |row| {
            let schedule = super::models::ReviewSchedule {
                id: row.get(0)?,
                wiki_page_id: row.get(1)?,
                ease_factor: row.get(2)?,
                interval_days: row.get(3)?,
                next_review_at: row.get(4)?,
                review_count: row.get(5)?,
                last_reviewed_at: row.get(6)?,
                mastery: row.get(7)?,
                is_archived: row.get::<_, i32>(8)? != 0,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
                last_format: row.get(11)?,
                variant_streak: row.get::<_, i32>(12).unwrap_or(0),
                variant_mode: row.get::<_, i32>(13).unwrap_or(0),
            };
            let page = super::models::WikiPage {
                id: row.get(14)?,
                title: row.get(15)?,
                slug: row.get(16)?,
                page_type: row.get(17)?,
                body_markdown: row.get(18)?,
                summary: row.get(19)?,
                tags: row.get(20)?,
                status: row.get(21)?,
                confidence: row.get(22)?,
                created_at: row.get(23)?,
                updated_at: row.get(24)?,
                last_compiled_at: row.get(25)?,
                source_message_id: row.get(26)?,
                author_name: row.get(27)?,
                author_url: row.get(28)?,
                source_type: row.get(29)?,
                source_task_id: row.get(30)?,
                monitor_enabled: row.get::<_, Option<i32>>(31).unwrap_or(Some(0)) != Some(0),
                monitor_query: row.get(32).unwrap_or(None),
                monitor_sources: row.get::<_, Option<String>>(33).unwrap_or(Some("[]".to_string())).unwrap_or_else(|| "[]".to_string()),
                last_discovered_at: row.get(34).unwrap_or(None),
                pending_count: row.get::<_, Option<i32>>(35).unwrap_or(Some(0)).unwrap_or(0),
            };
            Ok((schedule, page))
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn get_review_health_stats(&self) -> Result<super::models::ReviewHealthStats, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {e}"))?;
        let now = chrono::Utc::now().to_rfc3339();
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let week_ago = (chrono::Utc::now() - chrono::Duration::days(7)).to_rfc3339();

        // Total pages with body content
        let total_pages: i32 = conn.query_row(
            "SELECT COUNT(*) FROM wiki_pages WHERE body_markdown != '' AND status = 'active'",
            [],
            |row| row.get(0),
        )?;

        // Pages with non-archived review schedules
        let pages_with_reviews: i32 = conn.query_row(
            "SELECT COUNT(*) FROM review_schedule WHERE is_archived = 0",
            [],
            |row| row.get(0),
        )?;

        // Total review logs all time
        let total_reviews_all_time: i32 = conn.query_row(
            "SELECT COUNT(*) FROM review_logs",
            [],
            |row| row.get(0),
        )?;

        // Average quality across all logs (0.0-2.0 mapped to 0-100%)
        let avg_accuracy: f64 = if total_reviews_all_time > 0 {
            let total_quality: f64 = conn.query_row(
                "SELECT COALESCE(SUM(quality), 0) FROM review_logs",
                [],
                |row| row.get(0),
            )?;
            (total_quality / (total_reviews_all_time as f64 * 2.0)) * 100.0
        } else {
            0.0
        };

        // Streak from today's daily_review_summary
        let streak_day: i32 = conn.query_row(
            "SELECT COALESCE(streak_day, 0) FROM daily_review_summary WHERE date = ?1",
            params![today],
            |row| row.get(0),
        ).unwrap_or(0);

        // Review logs in last 7 days
        let weekly_review_count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM review_logs WHERE reviewed_at >= ?1",
            params![week_ago],
            |row| row.get(0),
        )?;

        // Overdue schedules (next_review_at < now and not archived)
        let overdue_count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM review_schedule WHERE next_review_at < ?1 AND is_archived = 0",
            params![now],
            |row| row.get(0),
        )?;

        // Mastered schedules (mastery >= 0.9 and not archived)
        let mastered_count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM review_schedule WHERE mastery >= 0.9 AND is_archived = 0",
            [],
            |row| row.get(0),
        )?;

        Ok(super::models::ReviewHealthStats {
            total_pages,
            pages_with_reviews,
            total_reviews_all_time,
            avg_accuracy,
            streak_day,
            weekly_review_count,
            overdue_count,
            mastered_count,
        })
    }

    // ========== Wiki Learning Trail (E-4-4) ==========

    pub fn get_wiki_learning_trail(&self, wiki_page_id: &str) -> Result<super::models::HealthTrailResult, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {e}"))?;
        let now = chrono::Utc::now().to_rfc3339();

        // Get review schedule for this wiki page
        let mut stmt = conn.prepare(
            "SELECT id, wiki_page_id, ease_factor, interval_days, next_review_at, review_count, last_reviewed_at, mastery, is_archived, created_at, updated_at
             FROM review_schedule WHERE wiki_page_id = ?1"
        )?;
        let schedule: Option<super::models::ReviewSchedule> = {
            let mut rows = stmt.query_map(params![wiki_page_id], |row| {
                Ok(super::models::ReviewSchedule {
                    id: row.get(0)?,
                    wiki_page_id: row.get(1)?,
                    ease_factor: row.get(2)?,
                    interval_days: row.get(3)?,
                    next_review_at: row.get(4)?,
                    review_count: row.get(5)?,
                    last_reviewed_at: row.get(6)?,
                    mastery: row.get(7)?,
                    is_archived: row.get::<_, i32>(8)? != 0,
                    created_at: row.get(9)?,
                    updated_at: row.get(10)?,
                    last_format: None,
                    variant_streak: 0,
                    variant_mode: 0,
                })
            })?;
            match rows.next() {
                Some(Ok(s)) => Some(s),
                _ => None,
            }
        };

        // Get last 10 review logs for this schedule
        let recent_logs: Vec<super::models::ReviewLog> = if let Some(ref sched) = schedule {
            let mut stmt2 = conn.prepare(
                "SELECT id, schedule_id, session_id, quality, interval_before, interval_after, ease_factor_before, ease_factor_after, reviewed_at, review_format, response_time_seconds, is_variant, variant_generation, question_snapshot
                 FROM review_logs WHERE schedule_id = ?1 ORDER BY reviewed_at DESC LIMIT 10"
            )?;
            let rows = stmt2.query_map(params![sched.id], |row| {
                Ok(super::models::ReviewLog {
                    id: row.get(0)?,
                    schedule_id: row.get(1)?,
                    session_id: row.get(2)?,
                    quality: row.get(3)?,
                    interval_before: row.get(4)?,
                    interval_after: row.get(5)?,
                    ease_factor_before: row.get(6)?,
                    ease_factor_after: row.get(7)?,
                    reviewed_at: row.get(8)?,
                    review_format: row.get(9)?,
                    response_time_seconds: row.get(10)?,
                    is_variant: Some(row.get::<_, i32>(11).unwrap_or(0) != 0),
                    variant_generation: row.get::<_, i32>(12).ok(),
                    question_snapshot: row.get(13)?,
                })
            })?;
            let mut logs = Vec::new();
            for row in rows {
                logs.push(row?);
            }
            logs
        } else {
            Vec::new()
        };

        let is_due = if let Some(ref sched) = schedule {
            sched.next_review_at <= now && !sched.is_archived
        } else {
            false
        };

        // Exam stats for this wiki page
        let exam_stats = {
            let mut stmt = conn.prepare(
                "SELECT COUNT(*),
                        SUM(CASE WHEN is_correct = 1 THEN 1 ELSE 0 END),
                        SUM(CASE WHEN is_correct = 0 THEN 1 ELSE 0 END)
                 FROM exam_questions WHERE wiki_page_id = ?1"
            )?;
            let mut rows = stmt.query_map(params![wiki_page_id], |row| {
                Ok((
                    row.get::<_, i32>(0).unwrap_or(0),
                    row.get::<_, i32>(1).unwrap_or(0),
                    row.get::<_, i32>(2).unwrap_or(0),
                ))
            })?;
            rows.next()
                .and_then(|r| r.ok())
                .and_then(|(total, correct, wrong)| {
                    if total > 0 {
                        Some(super::models::ExamStats { total, correct, wrong })
                    } else {
                        None
                    }
                })
        };

        // Goals linked to this wiki page
        let linked_goals = {
            let mut stmt = conn.prepare(
                "SELECT g.id, g.title FROM goals g
                 JOIN goal_wiki_links gw ON g.id = gw.goal_id
                 WHERE gw.wiki_page_id = ?1"
            )?;
            let rows = stmt.query_map(params![wiki_page_id], |row| {
                Ok(super::models::LinkedGoal {
                    goal_id: row.get(0)?,
                    goal_title: row.get(1)?,
                })
            })?;
            let mut out = Vec::new();
            for r in rows {
                out.push(r?);
            }
            out
        };

        Ok(super::models::HealthTrailResult {
            schedule,
            recent_logs,
            is_due,
            exam_stats,
            linked_goals,
        })
    }
}

/// SM-2 algorithm implementation.
/// quality: 0=forgot, 1=remembered, 2=easy
/// Returns (new_interval_days, new_ease_factor)
pub fn sm2_calculate(quality: i32, ease_factor: f64, interval_days: i32) -> (i32, f64) {
    let new_ef = if quality >= 1 {
        // Easy/remembered: increase ease factor
        let ef = ease_factor + (0.1 - (2 - quality) as f64 * (0.08 + (2 - quality) as f64 * 0.02));
        ef.max(1.3)
    } else {
        // Forgot: decrease
        (ease_factor - 0.2).max(1.3)
    };

    let new_interval = if quality == 0 {
        1 // Reset to 1 day
    } else if interval_days == 0 {
        1 // First review
    } else if interval_days == 1 {
        3 // Second review: day 1 -> day 3
    } else {
        ((interval_days as f64) * new_ef).round() as i32
    };

    (new_interval.min(180), new_ef)
}

/// Calculate mastery score [0.0, 1.0]
pub fn calculate_mastery(review_count: i32, recent_accuracy: f64, interval_days: i32) -> f64 {
    let count_factor = (review_count as f64 / 10.0).min(1.0);
    let accuracy_factor = recent_accuracy; // 0.0 to 1.0
    let interval_factor = (interval_days as f64 / 90.0).min(1.0);
    (count_factor * accuracy_factor * interval_factor).min(1.0)
}

// ========== Phase 7: Knowledge Discovery (E-7-1) ==========

impl Repository {
    // ----- PendingContent -----

    pub fn create_pending_content(
        &self,
        content: &PendingContent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "INSERT INTO pending_content (id, title, source_url, source_name, content_summary, source_page_id, source_page_title, match_reason, match_keywords, relevance_score, full_content, content_hash, status, read_at, imported_content_id, discovered_at, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
            params![
                content.id, content.title, content.source_url, content.source_name,
                content.content_summary, content.source_page_id, content.source_page_title,
                content.match_reason, content.match_keywords, content.relevance_score,
                content.full_content, content.content_hash, content.status, content.read_at,
                content.imported_content_id, content.discovered_at, content.created_at,
            ],
        )?;
        Ok(())
    }

    pub fn get_pending_content(
        &self,
        id: &str,
    ) -> Result<Option<PendingContent>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, title, source_url, source_name, content_summary, source_page_id, source_page_title, match_reason, match_keywords, relevance_score, full_content, content_hash, status, read_at, imported_content_id, discovered_at, created_at
             FROM pending_content WHERE id = ?1"
        )?;
        let mut rows = stmt.query_map(params![id], |row| {
            Ok(PendingContent {
                id: row.get(0)?,
                title: row.get(1)?,
                source_url: row.get(2)?,
                source_name: row.get(3)?,
                content_summary: row.get(4)?,
                source_page_id: row.get(5)?,
                source_page_title: row.get(6)?,
                match_reason: row.get(7)?,
                match_keywords: row.get(8)?,
                relevance_score: row.get(9)?,
                full_content: row.get(10)?,
                content_hash: row.get(11)?,
                status: row.get(12)?,
                read_at: row.get(13)?,
                imported_content_id: row.get(14)?,
                discovered_at: row.get(15)?,
                created_at: row.get(16)?,
            })
        })?;
        match rows.next() {
            Some(Ok(c)) => Ok(Some(c)),
            Some(Err(e)) => Err(Box::new(e)),
            None => Ok(None),
        }
    }

    pub fn list_pending_content(
        &self,
        status_filter: Option<&str>,
        limit: i64,
    ) -> Result<Vec<PendingContent>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let (sql, params_vec): (String, Vec<Box<dyn rusqlite::types::ToSql>>) = if let Some(status) = status_filter {
            (
                "SELECT id, title, source_url, source_name, content_summary, source_page_id, source_page_title, match_reason, match_keywords, relevance_score, full_content, content_hash, status, read_at, imported_content_id, discovered_at, created_at
                 FROM pending_content WHERE status = ?1 ORDER BY discovered_at DESC LIMIT ?2".to_string(),
                vec![Box::new(status.to_string()), Box::new(limit)],
            )
        } else {
            (
                "SELECT id, title, source_url, source_name, content_summary, source_page_id, source_page_title, match_reason, match_keywords, relevance_score, full_content, content_hash, status, read_at, imported_content_id, discovered_at, created_at
                 FROM pending_content ORDER BY discovered_at DESC LIMIT ?1".to_string(),
                vec![Box::new(limit)],
            )
        };
        let mut stmt = conn.prepare(&sql)?;
        let param_refs: Vec<&dyn rusqlite::types::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
        let rows = stmt.query_map(param_refs.as_slice(), |row| {
            Ok(PendingContent {
                id: row.get(0)?,
                title: row.get(1)?,
                source_url: row.get(2)?,
                source_name: row.get(3)?,
                content_summary: row.get(4)?,
                source_page_id: row.get(5)?,
                source_page_title: row.get(6)?,
                match_reason: row.get(7)?,
                match_keywords: row.get(8)?,
                relevance_score: row.get(9)?,
                full_content: row.get(10)?,
                content_hash: row.get(11)?,
                status: row.get(12)?,
                read_at: row.get(13)?,
                imported_content_id: row.get(14)?,
                discovered_at: row.get(15)?,
                created_at: row.get(16)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn update_pending_content_status(
        &self,
        id: &str,
        status: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE pending_content SET status = ?1, read_at = COALESCE(read_at, ?2) WHERE id = ?3",
            params![status, now, id],
        )?;
        Ok(())
    }

    pub fn get_pending_content_by_page(
        &self,
        page_id: &str,
    ) -> Result<Vec<PendingContent>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, title, source_url, source_name, content_summary, source_page_id, source_page_title, match_reason, match_keywords, relevance_score, full_content, content_hash, status, read_at, imported_content_id, discovered_at, created_at
             FROM pending_content WHERE source_page_id = ?1 ORDER BY discovered_at DESC"
        )?;
        let rows = stmt.query_map(params![page_id], |row| {
            Ok(PendingContent {
                id: row.get(0)?,
                title: row.get(1)?,
                source_url: row.get(2)?,
                source_name: row.get(3)?,
                content_summary: row.get(4)?,
                source_page_id: row.get(5)?,
                source_page_title: row.get(6)?,
                match_reason: row.get(7)?,
                match_keywords: row.get(8)?,
                relevance_score: row.get(9)?,
                full_content: row.get(10)?,
                content_hash: row.get(11)?,
                status: row.get(12)?,
                read_at: row.get(13)?,
                imported_content_id: row.get(14)?,
                discovered_at: row.get(15)?,
                created_at: row.get(16)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn count_pending_by_status(
        &self,
        status: &str,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM pending_content WHERE status = ?1",
            params![status],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    pub fn pending_content_exists_by_url(
        &self,
        url: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM pending_content WHERE source_url = ?1",
            params![url],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    // ----- KnowledgeMonitorSource -----

    pub fn create_monitor_source(
        &self,
        source: &KnowledgeMonitorSource,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "INSERT INTO knowledge_monitor_source (id, page_id, search_query, source_type, rss_url, is_active, last_checked_at, last_found_count, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                source.id, source.page_id, source.search_query, source.source_type,
                source.rss_url, source.is_active, source.last_checked_at,
                source.last_found_count, source.created_at,
            ],
        )?;
        Ok(())
    }

    pub fn get_monitor_sources_for_page(
        &self,
        page_id: &str,
    ) -> Result<Vec<KnowledgeMonitorSource>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, page_id, search_query, source_type, rss_url, is_active, last_checked_at, last_found_count, created_at
             FROM knowledge_monitor_source WHERE page_id = ?1 ORDER BY created_at DESC"
        )?;
        let rows = stmt.query_map(params![page_id], |row| {
            Ok(KnowledgeMonitorSource {
                id: row.get(0)?,
                page_id: row.get(1)?,
                search_query: row.get(2)?,
                source_type: row.get(3)?,
                rss_url: row.get(4)?,
                is_active: row.get::<_, i32>(5)? != 0,
                last_checked_at: row.get(6)?,
                last_found_count: row.get(7)?,
                created_at: row.get(8)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn update_monitor_source(
        &self,
        source: &KnowledgeMonitorSource,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE knowledge_monitor_source SET page_id=?1, search_query=?2, source_type=?3, rss_url=?4, is_active=?5, last_checked_at=?6, last_found_count=?7 WHERE id=?8",
            params![
                source.page_id, source.search_query, source.source_type, source.rss_url,
                source.is_active, source.last_checked_at, source.last_found_count, source.id,
            ],
        )?;
        Ok(())
    }

    pub fn list_active_monitor_sources(
        &self,
    ) -> Result<Vec<KnowledgeMonitorSource>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, page_id, search_query, source_type, rss_url, is_active, last_checked_at, last_found_count, created_at
             FROM knowledge_monitor_source WHERE is_active = 1 ORDER BY created_at DESC"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(KnowledgeMonitorSource {
                id: row.get(0)?,
                page_id: row.get(1)?,
                search_query: row.get(2)?,
                source_type: row.get(3)?,
                rss_url: row.get(4)?,
                is_active: row.get::<_, i32>(5)? != 0,
                last_checked_at: row.get(6)?,
                last_found_count: row.get(7)?,
                created_at: row.get(8)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn update_monitor_last_checked(
        &self,
        id: &str,
        last_checked_at: &str,
        last_found_count: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE knowledge_monitor_source SET last_checked_at = ?1, last_found_count = ?2 WHERE id = ?3",
            params![last_checked_at, last_found_count, id],
        )?;
        Ok(())
    }

    // ----- WikiPage discovery helpers -----

    pub fn update_wiki_page_pending_count(
        &self,
        page_id: &str,
        pending_count: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE wiki_pages SET pending_count = ?1, updated_at = datetime('now') WHERE id = ?2",
            params![pending_count, page_id],
        )?;
        Ok(())
    }

    pub fn update_wiki_page_last_discovered(
        &self,
        page_id: &str,
        last_discovered_at: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE wiki_pages SET last_discovered_at = ?1, updated_at = datetime('now') WHERE id = ?2",
            params![last_discovered_at, page_id],
        )?;
        Ok(())
    }

    pub fn update_wiki_page_monitor(
        &self,
        page_id: &str,
        monitor_enabled: bool,
        monitor_query: Option<&str>,
        monitor_sources: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE wiki_pages SET monitor_enabled = ?1, monitor_query = ?2, monitor_sources = ?3, updated_at = datetime('now') WHERE id = ?4",
            params![monitor_enabled, monitor_query, monitor_sources, page_id],
        )?;
        Ok(())
    }

    // ----- Discovery Suppression (E-7-7) -----

    pub fn record_dismissal(
        &self,
        source_page_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "INSERT INTO discovery_suppression (id, source_page_id, dismiss_count, is_auto_ignored, created_at, updated_at)
             VALUES (?1, ?2, 1, 0, datetime('now'), datetime('now'))
             ON CONFLICT(source_page_id) DO UPDATE SET
                dismiss_count = dismiss_count + 1,
                is_auto_ignored = CASE WHEN dismiss_count + 1 >= 3 THEN 1 ELSE is_auto_ignored END,
                updated_at = datetime('now')",
            params![uuid::Uuid::new_v4().to_string(), source_page_id],
        )?;
        Ok(())
    }

    pub fn should_auto_ignore(
        &self,
        source_page_id: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM discovery_suppression WHERE source_page_id = ?1 AND is_auto_ignored = 1",
            params![source_page_id],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    pub fn get_ignored_sources(
        &self,
    ) -> Result<Vec<(String, String, i32)>, Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT ds.source_page_id, wp.title, ds.dismiss_count
             FROM discovery_suppression ds
             LEFT JOIN wiki_pages wp ON wp.id = ds.source_page_id
             WHERE ds.is_auto_ignored = 1
             ORDER BY ds.updated_at DESC"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, Option<String>>(1)?.unwrap_or_else(|| "Unknown".to_string()),
                row.get::<_, i32>(2)?,
            ))
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn unignore_source(
        &self,
        source_page_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .db
            .conn
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "UPDATE discovery_suppression SET dismiss_count = 0, is_auto_ignored = 0, updated_at = datetime('now') WHERE source_page_id = ?1",
            params![source_page_id],
        )?;
        Ok(())
    }

    // ========== Folder Sync ==========

    pub fn save_sync_folder(&self, folder: &super::models::SyncFolder) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "INSERT INTO sync_folders (id, path, enabled, last_synced_at, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![folder.id, folder.path, folder.enabled as i32, folder.last_synced_at, folder.created_at],
        )?;
        Ok(())
    }

    pub fn get_sync_folders(&self) -> Result<Vec<super::models::SyncFolder>, Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, path, enabled, last_synced_at, created_at FROM sync_folders ORDER BY created_at"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(super::models::SyncFolder {
                id: row.get(0)?,
                path: row.get(1)?,
                enabled: row.get::<_, i32>(2)? != 0,
                last_synced_at: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows { results.push(row?); }
        Ok(results)
    }

    pub fn update_sync_folder_enabled(&self, id: &str, enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        conn.execute("UPDATE sync_folders SET enabled = ?1 WHERE id = ?2", params![enabled as i32, id])?;
        Ok(())
    }

    pub fn update_sync_folder_last_synced(&self, id: &str, time: &str) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        conn.execute("UPDATE sync_folders SET last_synced_at = ?1 WHERE id = ?2", params![time, id])?;
        Ok(())
    }

    pub fn delete_sync_folder(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        conn.execute("DELETE FROM sync_folders WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn get_sync_record_by_path(&self, folder_id: &str, file_path: &str) -> Result<Option<super::models::SyncRecord>, Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, folder_id, file_path, file_name, file_size, file_mtime, file_type, content_id, status, synced_at FROM sync_records WHERE folder_id = ?1 AND file_path = ?2"
        )?;
        let mut rows = stmt.query_map(params![folder_id, file_path], |row| {
            Ok(super::models::SyncRecord {
                id: row.get(0)?,
                folder_id: row.get(1)?,
                file_path: row.get(2)?,
                file_name: row.get(3)?,
                file_size: row.get(4)?,
                file_mtime: row.get(5)?,
                file_type: row.get(6)?,
                content_id: row.get(7)?,
                status: row.get(8)?,
                synced_at: row.get(9)?,
            })
        })?;
        match rows.next() {
            Some(Ok(r)) => Ok(Some(r)),
            Some(Err(e)) => Err(Box::new(e)),
            None => Ok(None),
        }
    }

    pub fn upsert_sync_record(&self, record: &super::models::SyncRecord) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        conn.execute(
            "INSERT INTO sync_records (id, folder_id, file_path, file_name, file_size, file_mtime, file_type, content_id, status, synced_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
             ON CONFLICT(folder_id, file_path) DO UPDATE SET file_name=?4, file_size=?5, file_mtime=?6, content_id=?8, status=?9, synced_at=?10",
            params![record.id, record.folder_id, record.file_path, record.file_name, record.file_size, record.file_mtime, record.file_type, record.content_id, record.status, record.synced_at],
        )?;
        Ok(())
    }

    // ----- WikiMasteryFlag -----

    pub fn create_mastery_flag(&self, flag: &super::models::WikiMasteryFlag) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {e}"))?;
        conn.execute(
            "INSERT INTO wiki_mastery_flags (id, wiki_page_id, goal_id, exam_id, is_resolved, created_at, resolved_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![flag.id, flag.wiki_page_id, flag.goal_id, flag.exam_id, flag.is_resolved as i32, flag.created_at, flag.resolved_at],
        )?;
        Ok(())
    }

    pub fn get_unresolved_mastery_flags(&self, goal_id: &str) -> Result<Vec<super::models::WikiMasteryFlag>, Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {e}"))?;
        let mut stmt = conn.prepare(
            "SELECT id, wiki_page_id, goal_id, exam_id, is_resolved, created_at, resolved_at
             FROM wiki_mastery_flags WHERE goal_id = ?1 AND is_resolved = 0"
        )?;
        let rows = stmt.query_map(params![goal_id], |row| {
            Ok(super::models::WikiMasteryFlag {
                id: row.get(0)?, wiki_page_id: row.get(1)?, goal_id: row.get(2)?,
                exam_id: row.get(3)?, is_resolved: row.get::<_, i32>(4)? != 0,
                created_at: row.get(5)?, resolved_at: row.get(6)?,
            })
        })?;
        let mut out = Vec::new();
        for r in rows { out.push(r?); }
        Ok(out)
    }

    pub fn resolve_mastery_flags(&self, wiki_page_id: &str, goal_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {e}"))?;
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE wiki_mastery_flags SET is_resolved = 1, resolved_at = ?1
             WHERE wiki_page_id = ?2 AND goal_id = ?3 AND is_resolved = 0",
            params![now, wiki_page_id, goal_id],
        )?;
        Ok(())
    }

    pub fn has_unresolved_mastery_flags(&self, goal_id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {e}"))?;
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM wiki_mastery_flags WHERE goal_id = ?1 AND is_resolved = 0",
            params![goal_id],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    // ----- Review count -----

    pub fn get_review_count_for_wiki(&self, wiki_page_id: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {e}"))?;
        let count: i32 = conn.query_row(
            "SELECT COALESCE(review_count, 0) FROM review_schedule WHERE wiki_page_id = ?1",
            params![wiki_page_id],
            |row| row.get(0),
        ).unwrap_or(0);
        Ok(count)
    }

    // ----- Exam queries -----

    pub fn get_exams_for_goal(&self, goal_id: &str) -> Result<Vec<super::models::Exam>, Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {e}"))?;
        let mut stmt = conn.prepare(
            "SELECT id, goal_id, title, total_questions, score, grade, status, started_at, completed_at, diagnosis_json, created_at, version, question_config
             FROM exams WHERE goal_id = ?1 ORDER BY version DESC"
        )?;
        let rows = stmt.query_map(params![goal_id], |row| {
            Ok(super::models::Exam {
                id: row.get(0)?, goal_id: row.get(1)?, title: row.get(2)?,
                total_questions: row.get(3)?, score: row.get(4)?, grade: row.get(5)?,
                status: row.get(6)?, started_at: row.get(7)?, completed_at: row.get(8)?,
                diagnosis_json: row.get(9)?, created_at: row.get(10)?,
                version: row.get(11)?, question_config: row.get(12)?,
            })
        })?;
        let mut out = Vec::new();
        for r in rows { out.push(r?); }
        Ok(out)
    }

    pub fn get_exams_for_wiki(&self, wiki_page_id: &str) -> Result<Vec<super::models::Exam>, Box<dyn std::error::Error>> {
        let conn = self.db.conn.lock().map_err(|e| format!("Lock error: {e}"))?;
        let mut stmt = conn.prepare(
            "SELECT DISTINCT e.id, e.goal_id, e.title, e.total_questions, e.score, e.grade, e.status, e.started_at, e.completed_at, e.diagnosis_json, e.created_at, e.version, e.question_config
             FROM exams e JOIN exam_questions eq ON eq.exam_id = e.id
             WHERE eq.wiki_page_id = ?1 ORDER BY e.created_at DESC"
        )?;
        let rows = stmt.query_map(params![wiki_page_id], |row| {
            Ok(super::models::Exam {
                id: row.get(0)?, goal_id: row.get(1)?, title: row.get(2)?,
                total_questions: row.get(3)?, score: row.get(4)?, grade: row.get(5)?,
                status: row.get(6)?, started_at: row.get(7)?, completed_at: row.get(8)?,
                diagnosis_json: row.get(9)?, created_at: row.get(10)?,
                version: row.get(11)?, question_config: row.get(12)?,
            })
        })?;
        let mut out = Vec::new();
        for r in rows { out.push(r?); }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::database::Database;
    use crate::storage::models::{CapturedContent, ContentType};

    /// Create an in-memory database with all migrations applied.
    fn test_db() -> Arc<Database> {
        let db = Database::new_in_memory().expect("Failed to create test DB");
        Arc::new(db)
    }

    fn make_content(id: &str, captured_at: &str) -> CapturedContent {
        CapturedContent {
            id: id.to_string(),
            content_type: ContentType::Text,
            raw_text: Some(format!("Test content {}", id)),
            image_path: None,
            thumbnail_path: None,
            source_app: "TestApp".to_string(),
            source_bundle_id: None,
            source_url: None,
            user_note: None,
            captured_at: captured_at.to_string(),
            content_hash: format!("hash_{}", id),
            byte_size: 100,
            is_deleted: false,
            created_at: captured_at.to_string(),
            updated_at: captured_at.to_string(),
            digested_at: None,
            digest_action: None,
            summary: None,
            tags: None,
            digest: None,
            wiki_compile_hash: None,
            wiki_assessed_hash: None,
            clean_content: None,
        }
    }

    #[test]
    fn test_get_content_stats_counts_total_days_inclusively() {
        let items = vec![
            ContentForAnalysis {
                id: "1".to_string(),
                raw_text: Some("a".to_string()),
                source_url: None,
                captured_at: "2026-03-21T10:00:00Z".to_string(),
                summary: None,
                tags: None,
                user_note: Some("note".to_string()),
                source_app: "WeChat".to_string(),
                content_type: "text".to_string(),
            },
            ContentForAnalysis {
                id: "2".to_string(),
                raw_text: Some("b".to_string()),
                source_url: None,
                captured_at: "2026-04-05T09:00:00Z".to_string(),
                summary: None,
                tags: Some("tag".to_string()),
                user_note: None,
                source_app: "Chrome".to_string(),
                content_type: "url".to_string(),
            },
        ];

        let stats = Repository::get_content_stats(&items);

        assert_eq!(stats["total_days"], 16);
        assert_eq!(stats["source_count"], 2);
        assert_eq!(stats["annotation_rate"], "100%");
    }

    #[test]
    fn test_get_undigested_returns_oldest_first() {
        let db = test_db();
        let repo = Repository::new(db);

        // Insert 5 items with different timestamps
        for i in 1..=5 {
            let content = make_content(
                &format!("item_{}", i),
                &format!("2025-01-{:02}T10:00:00", i),
            );
            repo.save_content(&content).unwrap();
        }

        let items = repo.get_undigested_content(3).unwrap();
        assert_eq!(items.len(), 3);
        // Should be oldest first
        assert_eq!(items[0].id, "item_1");
        assert_eq!(items[1].id, "item_2");
        assert_eq!(items[2].id, "item_3");
    }

    #[test]
    fn test_get_undigested_skips_digested() {
        let db = test_db();
        let repo = Repository::new(db);

        for i in 1..=3 {
            let content = make_content(
                &format!("item_{}", i),
                &format!("2025-01-{:02}T10:00:00", i),
            );
            repo.save_content(&content).unwrap();
        }

        // Digest item_1
        repo.update_digest_action("item_1", "keep").unwrap();

        let items = repo.get_undigested_content(5).unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].id, "item_2");
        assert_eq!(items[1].id, "item_3");
    }

    #[test]
    fn test_get_undigested_empty_when_all_digested() {
        let db = test_db();
        let repo = Repository::new(db);

        let content = make_content("item_1", "2025-01-01T10:00:00");
        repo.save_content(&content).unwrap();
        repo.update_digest_action("item_1", "archive").unwrap();

        let items = repo.get_undigested_content(5).unwrap();
        assert!(items.is_empty());
    }

    #[test]
    fn test_update_digest_keep() {
        let db = test_db();
        let repo = Repository::new(db);

        let content = make_content("item_1", "2025-01-01T10:00:00");
        repo.save_content(&content).unwrap();

        repo.update_digest_action("item_1", "keep").unwrap();

        // Verify by fetching — it should no longer be undigested
        let undigested = repo.get_undigested_content(5).unwrap();
        assert!(undigested.is_empty());

        // Verify the action was set correctly
        let item = repo.get_content_by_id("item_1").unwrap().unwrap();
        assert_eq!(item.digest_action.as_deref(), Some("keep"));
        assert!(item.digested_at.is_some());
    }

    #[test]
    fn test_update_digest_archive() {
        let db = test_db();
        let repo = Repository::new(db);

        let content = make_content("item_1", "2025-01-01T10:00:00");
        repo.save_content(&content).unwrap();

        repo.update_digest_action("item_1", "archive").unwrap();

        let item = repo.get_content_by_id("item_1").unwrap().unwrap();
        assert_eq!(item.digest_action.as_deref(), Some("archive"));
        assert!(item.digested_at.is_some());
    }

    #[test]
    fn test_update_digest_pin() {
        let db = test_db();
        let repo = Repository::new(db);

        let content = make_content("item_1", "2025-01-01T10:00:00");
        repo.save_content(&content).unwrap();

        repo.update_digest_action("item_1", "pin").unwrap();

        let item = repo.get_content_by_id("item_1").unwrap().unwrap();
        assert_eq!(item.digest_action.as_deref(), Some("pin"));
        assert!(item.digested_at.is_some());
    }

    #[test]
    fn test_update_digest_invalid_id() {
        let db = test_db();
        let repo = Repository::new(db);

        let result = repo.update_digest_action("nonexistent", "keep");
        assert!(result.is_err());
    }

    #[test]
    fn test_count_undigested() {
        let db = test_db();
        let repo = Repository::new(db);

        for i in 1..=5 {
            let content = make_content(
                &format!("item_{}", i),
                &format!("2025-01-{:02}T10:00:00", i),
            );
            repo.save_content(&content).unwrap();
        }

        assert_eq!(repo.count_undigested().unwrap(), 5);

        repo.update_digest_action("item_1", "archive").unwrap();
        repo.update_digest_action("item_2", "keep").unwrap();

        assert_eq!(repo.count_undigested().unwrap(), 3);
    }

    // ========== FTS / candidate retrieval ==========

    fn make_wiki_page(
        id: &str,
        title: &str,
        summary: &str,
        body: &str,
        created_at: &str,
        page_type: &str,
    ) -> super::super::models::WikiPage {
        super::super::models::WikiPage {
            id: id.to_string(),
            title: title.to_string(),
            slug: format!("slug-{}", id),
            page_type: page_type.to_string(),
            body_markdown: body.to_string(),
            summary: Some(summary.to_string()),
            tags: None,
            status: "active".to_string(),
            confidence: 1.0,
            created_at: created_at.to_string(),
            updated_at: created_at.to_string(),
            last_compiled_at: None,
            source_message_id: None,
            author_name: None,
            author_url: None,
            source_type: None,
            source_task_id: None,
            monitor_enabled: false,
            monitor_query: None,
            monitor_sources: "[]".to_string(),
            last_discovered_at: None,
            pending_count: 0,
        }
    }

    #[test]
    fn fts_table_is_available_after_migration() {
        let db = test_db();
        let repo = Repository::new(db);
        assert!(
            repo.fts_available(),
            "FTS table should exist after migrations"
        );
    }

    #[test]
    fn fts_indexes_inserted_pages_via_trigger() {
        let db = test_db();
        let repo = Repository::new(db);
        repo.save_wiki_page(&make_wiki_page(
            "p1",
            "RAG technology",
            "Retrieval-augmented generation overview",
            "RAG combines retrieval with LLMs.",
            "2026-04-26T10:00:00Z",
            "concept",
        ))
        .unwrap();

        let candidates = repo
            .get_wiki_page_candidates(Some("RAG"), None, None, true, 10)
            .unwrap();
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].0, "p1");
    }

    #[test]
    fn fts_delete_trigger_removes_from_index() {
        let db = test_db();
        let repo = Repository::new(db);
        repo.save_wiki_page(&make_wiki_page(
            "p1",
            "DeepSeek model",
            "DeepSeek V4 release notes",
            "DeepSeek is...",
            "2026-04-26T10:00:00Z",
            "concept",
        ))
        .unwrap();
        repo.delete_wiki_page("p1").unwrap();

        let candidates = repo
            .get_wiki_page_candidates(Some("DeepSeek"), None, None, true, 10)
            .unwrap();
        assert_eq!(candidates.len(), 0);
    }

    #[test]
    fn date_range_filter_narrows_results() {
        let db = test_db();
        let repo = Repository::new(db);
        repo.save_wiki_page(&make_wiki_page(
            "old",
            "Old page",
            "old",
            "old body",
            "2026-01-01T10:00:00Z",
            "concept",
        ))
        .unwrap();
        repo.save_wiki_page(&make_wiki_page(
            "new",
            "New page",
            "new",
            "new body",
            "2026-04-25T10:00:00Z",
            "concept",
        ))
        .unwrap();

        let recent = repo
            .get_wiki_page_candidates(None, Some("2026-04-01T00:00:00Z"), None, true, 10)
            .unwrap();
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].0, "new");
    }

    #[test]
    fn excludes_qa_pages_when_requested() {
        let db = test_db();
        let repo = Repository::new(db);
        repo.save_wiki_page(&make_wiki_page(
            "concept-1",
            "Buffett",
            "Investment philosophy",
            "Value investing.",
            "2026-04-25T10:00:00Z",
            "concept",
        ))
        .unwrap();
        repo.save_wiki_page(&make_wiki_page(
            "qa-1",
            "Buffett FAQ",
            "Past Q&A about Buffett",
            "What did Buffett say...",
            "2026-04-25T10:00:00Z",
            "qa",
        ))
        .unwrap();

        let qa_excluded = repo
            .get_wiki_page_candidates(Some("Buffett"), None, None, true, 10)
            .unwrap();
        assert_eq!(qa_excluded.len(), 1);
        assert_eq!(qa_excluded[0].0, "concept-1");

        let qa_included = repo
            .get_wiki_page_candidates(Some("Buffett"), None, None, false, 10)
            .unwrap();
        assert_eq!(qa_included.len(), 2);
    }

    #[test]
    fn empty_fts_query_falls_back_to_no_filter() {
        let db = test_db();
        let repo = Repository::new(db);
        repo.save_wiki_page(&make_wiki_page(
            "p1",
            "Anything",
            "any",
            "body",
            "2026-04-25T10:00:00Z",
            "concept",
        ))
        .unwrap();

        // Query with only FTS-syntax chars resolves to empty match expr
        // → falls back to non-FTS query, returns the page
        let r = repo
            .get_wiki_page_candidates(Some("*-+:"), None, None, true, 10)
            .unwrap();
        assert_eq!(r.len(), 1);
    }

    #[test]
    fn fts_query_sanitizes_special_chars() {
        let db = test_db();
        let repo = Repository::new(db);
        repo.save_wiki_page(&make_wiki_page(
            "p1",
            "Hello world",
            "summary",
            "body",
            "2026-04-25T10:00:00Z",
            "concept",
        ))
        .unwrap();

        // Quotes and other FTS5 syntax chars should not crash — they
        // get stripped before the MATCH expression is built.
        let r = repo
            .get_wiki_page_candidates(Some("\"hello\" -world :extra"), None, None, true, 10)
            .unwrap();
        assert_eq!(r.len(), 1);
    }

    #[test]
    fn fts_finds_cjk_substring_in_continuous_chinese() {
        // The bug this guards against: unicode61 indexed continuous CJK
        // sequences as a single token, so "整理设计风格" was one token
        // and a search for "设计" missed it. Migration 015 + cjk_seg()
        // forces character-level tokenization.
        let db = test_db();
        let repo = Repository::new(db);
        repo.save_wiki_page(&make_wiki_page(
            "design-md",
            "awesome-design-md",
            "整理设计风格并支持开源共建",
            "项目整理各大网站的设计风格用于喂给 AI",
            "2026-04-25T10:00:00Z",
            "source",
        ))
        .unwrap();

        // Query "设计" must hit the page even though "设计" is buried
        // in the middle of a continuous Chinese string.
        let r = repo
            .get_wiki_page_candidates(Some("设计"), None, None, true, 10)
            .unwrap();
        assert_eq!(r.len(), 1, "expected to find page with 设计 in summary");
        assert_eq!(r[0].0, "design-md");
    }

    #[test]
    fn fts_mixed_cjk_and_english_query() {
        // Real-world Q&A query shape: extracted keywords mix Chinese
        // topic words with English technical terms.
        let db = test_db();
        let repo = Repository::new(db);
        repo.save_wiki_page(&make_wiki_page(
            "p1",
            "awesome-design-md",
            "整理设计风格并支持开源共建",
            "body",
            "2026-04-25T10:00:00Z",
            "source",
        ))
        .unwrap();
        repo.save_wiki_page(&make_wiki_page(
            "p2",
            "NanoBanana-PPT-Skills",
            "AI 生成 PPT 的 Skill",
            "body",
            "2026-04-25T10:00:00Z",
            "concept",
        ))
        .unwrap();
        repo.save_wiki_page(&make_wiki_page(
            "p3",
            "无关页面",
            "完全不相关",
            "body",
            "2026-04-25T10:00:00Z",
            "concept",
        ))
        .unwrap();

        // Query "设计 skill" → should pull p1 (matches 设计) AND p2
        // (matches skill) but not p3.
        let r = repo
            .get_wiki_page_candidates(Some("设计 skill"), None, None, true, 10)
            .unwrap();
        let ids: Vec<&str> = r.iter().map(|t| t.0.as_str()).collect();
        assert!(ids.contains(&"p1"), "expected p1 (设计 match)");
        assert!(ids.contains(&"p2"), "expected p2 (skill match)");
        assert!(!ids.contains(&"p3"), "p3 should not match");
    }

    #[test]
    fn cjk_segment_inserts_spaces_between_ideographs() {
        use crate::storage::database::cjk_segment;
        assert_eq!(cjk_segment("整理设计风格"), "整 理 设 计 风 格");
        // English passes through unchanged
        assert_eq!(cjk_segment("hello world"), "hello world");
        // Mixed: only CJK gets spaces, English stays intact
        assert_eq!(cjk_segment("AI 设计 skill"), "AI 设 计 skill");
        // Idempotent
        assert_eq!(cjk_segment("整 理"), "整 理");
        // Empty
        assert_eq!(cjk_segment(""), "");
    }

    #[test]
    fn extract_project_url_prefers_github() {
        let body = "这个项目的源码：\n\n## 项目地址\n- GitHub: https://github.com/foo/bar\n";
        let url = Repository::extract_project_url_from_body(body).unwrap();
        assert_eq!(url, "https://github.com/foo/bar");
    }

    #[test]
    fn extract_project_url_skips_social_in_favor_of_github() {
        // Even if a tweet appears earlier in the body, GitHub wins.
        let body =
            "原文 https://x.com/some/status/123\n更多介绍\n## 项目地址\nhttps://github.com/foo/bar";
        let url = Repository::extract_project_url_from_body(body).unwrap();
        assert_eq!(url, "https://github.com/foo/bar");
    }

    #[test]
    fn extract_project_url_falls_back_to_non_social_link() {
        // No GitHub but has a non-social URL → use that.
        let body = "项目主页 https://example.com/project\n推文 https://x.com/some/status/123";
        let url = Repository::extract_project_url_from_body(body).unwrap();
        assert_eq!(url, "https://example.com/project");
    }

    #[test]
    fn extract_project_url_returns_none_when_only_social_links() {
        // Only social links and no project page → None (caller falls back to source_url)
        let body =
            "看到这个推文 https://x.com/foo/status/1 还有微信文章 https://mp.weixin.qq.com/s/abc";
        assert!(Repository::extract_project_url_from_body(body).is_none());
    }

    #[test]
    fn extract_project_url_strips_trailing_punctuation() {
        let body = "看 https://github.com/foo/bar，挺好用的";
        let url = Repository::extract_project_url_from_body(body).unwrap();
        assert_eq!(url, "https://github.com/foo/bar");
    }

    #[test]
    fn extract_project_url_returns_none_for_empty_body() {
        assert!(Repository::extract_project_url_from_body("").is_none());
    }
}
