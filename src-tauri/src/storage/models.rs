use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Text,
    Image,
    Url,
    Mixed,
}

impl ContentType {
    pub fn as_str(&self) -> &str {
        match self {
            ContentType::Text => "text",
            ContentType::Image => "image",
            ContentType::Url => "url",
            ContentType::Mixed => "mixed",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "image" => ContentType::Image,
            "url" => ContentType::Url,
            "mixed" => ContentType::Mixed,
            _ => ContentType::Text,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturedContent {
    pub id: String,
    pub content_type: ContentType,
    pub raw_text: Option<String>,
    pub image_path: Option<String>,
    pub thumbnail_path: Option<String>,
    pub source_app: String,
    pub source_bundle_id: Option<String>,
    pub source_url: Option<String>,
    pub user_note: Option<String>,
    pub captured_at: String,
    pub content_hash: String,
    pub byte_size: i64,
    pub is_deleted: bool,
    pub created_at: String,
    pub updated_at: String,
    pub digested_at: Option<String>,
    pub digest_action: Option<String>,
    pub summary: Option<String>,
    pub tags: Option<String>,
    pub digest: Option<String>,
    pub wiki_compile_hash: Option<String>,
    pub wiki_assessed_hash: Option<String>,
    pub clean_content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyReport {
    pub id: String,
    pub week_start: String,
    pub week_end: String,
    pub summary_text: String,
    pub report_json: serde_json::Value,
    pub content_count: i32,
    pub model_used: String,
    pub tokens_used: Option<i32>,
    pub generated_at: String,
    pub sections: Vec<ReportSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSection {
    pub id: String,
    pub report_id: String,
    pub section_type: String,
    pub title: String,
    pub body: String,
    pub relevance_score: Option<f64>,
    pub sort_order: i32,
    pub content_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeedbackType {
    Interested,
    Dismissed,
    Bookmarked,
}

impl FeedbackType {
    pub fn as_str(&self) -> &str {
        match self {
            FeedbackType::Interested => "interested",
            FeedbackType::Dismissed => "dismissed",
            FeedbackType::Bookmarked => "bookmarked",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "dismissed" => FeedbackType::Dismissed,
            "bookmarked" => FeedbackType::Bookmarked,
            _ => FeedbackType::Interested,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFeedback {
    pub id: String,
    pub content_id: Option<String>,
    pub section_id: Option<String>,
    pub feedback_type: FeedbackType,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreference {
    pub id: String,
    pub topic: String,
    pub weight: f64,
    pub occurrence_count: i32,
    pub last_updated: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureEvent {
    pub content_type: String,
    pub preview: String,
    pub source_app: String,
    pub raw_text: Option<String>,
    pub image_path: Option<String>,
}

/// Rich content data for radar v2 analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentForAnalysis {
    pub id: String,
    pub raw_text: Option<String>,
    pub source_url: Option<String>,
    pub captured_at: String,
    pub summary: Option<String>,
    pub tags: Option<String>,
    pub user_note: Option<String>,
    pub source_app: String,
    pub content_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionInsight {
    pub id: i64,
    pub analysis_json: Option<String>,
    pub status: String,
    pub error_message: Option<String>,
    pub analyzed_at: String,
    pub window_start: String,
    pub window_end: String,
    pub content_count: i32,
    pub model_used: String,
    pub is_current: bool,
}

// ========== Wiki Knowledge Base ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiPage {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub page_type: String,
    pub body_markdown: String,
    pub summary: Option<String>,
    pub tags: Option<String>,
    pub status: String,
    pub confidence: f64,
    pub created_at: String,
    pub updated_at: String,
    pub last_compiled_at: Option<String>,
    pub source_message_id: Option<String>,
    pub author_name: Option<String>,
    pub author_url: Option<String>,
    pub source_type: Option<String>,
    pub source_task_id: Option<String>,
    // Phase 7: Knowledge Discovery fields
    pub monitor_enabled: bool,
    pub monitor_query: Option<String>,
    pub monitor_sources: String,
    pub last_discovered_at: Option<String>,
    pub pending_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiPageSource {
    pub id: i64,
    pub page_id: String,
    pub content_id: String,
    pub compile_hash: String,
    pub source_status: String,
    pub contributed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiEdge {
    pub id: i64,
    pub source_page_id: String,
    pub target_page_id: String,
    pub relation: String,
    pub weight: f64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiCompileLog {
    pub id: i64,
    pub content_id: String,
    pub content_hash: String,
    pub status: String,
    pub knowledge_score: Option<f64>,
    pub pages_touched: Option<String>,
    pub model_used: Option<String>,
    pub error_message: Option<String>,
    pub compiled_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiConversation {
    pub id: String,
    pub question: String,
    pub answer: String,
    pub pages_used: String,
    pub saved_as_page: Option<String>,
    pub model_used: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiLintResult {
    pub id: i64,
    pub lint_type: String,
    pub severity: String,
    pub title: String,
    pub description: String,
    pub page_ids: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiChatSession {
    pub id: String,
    pub title: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiChatMessage {
    pub id: String,
    pub session_id: String,
    pub role: String,
    pub content: String,
    pub pages_used: Option<String>,
    pub source_mode: Option<String>,
    pub turn_index: i32,
    pub created_at: String,
}

// ========== Learning Data Models (E-2, E-3) ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewSchedule {
    pub id: String,
    pub wiki_page_id: String,
    pub ease_factor: f64,
    pub interval_days: i32,
    pub next_review_at: String,
    pub review_count: i32,
    pub last_reviewed_at: Option<String>,
    pub mastery: f64,
    pub is_archived: bool,
    pub created_at: String,
    pub updated_at: String,
    pub last_format: Option<String>,
    pub variant_streak: i32,
    pub variant_mode: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewLog {
    pub id: String,
    pub schedule_id: String,
    pub session_id: Option<String>,
    pub quality: i32,
    pub interval_before: i32,
    pub interval_after: i32,
    pub ease_factor_before: f64,
    pub ease_factor_after: f64,
    pub reviewed_at: String,
    pub review_format: Option<String>,
    pub response_time_seconds: Option<i32>,
    pub is_variant: Option<bool>,
    pub variant_generation: Option<i32>,
    pub question_snapshot: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiMasteryFlag {
    pub id: String,
    pub wiki_page_id: String,
    pub goal_id: String,
    pub exam_id: String,
    pub is_resolved: bool,
    pub created_at: String,
    pub resolved_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyReviewSummary {
    pub id: String,
    pub date: String,
    pub total_reviewed: i32,
    pub correct_count: i32,
    pub streak_day: i32,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewFormatHistory {
    pub id: String,
    pub schedule_id: String,
    pub format: String,
    pub used_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DueReviewItem {
    pub schedule: ReviewSchedule,
    pub wiki_title: String,
    pub wiki_summary: Option<String>,
    pub wiki_tags: Option<String>,
    pub next_format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPath {
    pub id: String,
    pub title: String,
    pub description: String,
    pub topic: String,
    pub difficulty: String,
    pub estimated_days: i32,
    pub module_count: i32,
    pub completion_rate: f64,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: String,
    pub path_id: String,
    pub title: String,
    pub sort_order: i32,
    pub description: String,
    pub theory_markdown: String,
    pub reading_list_json: String,
    pub estimated_read_minutes: i32,
    pub discussion_prompts: String,
    pub community_solutions: String,
    pub task_ids: String,
    pub status: String,
    pub completed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PracticeTask {
    pub id: String,
    pub module_id: String,
    pub title: String,
    pub description: String,
    pub difficulty: String,
    pub estimated_minutes: i32,
    pub prerequisites: String,
    pub hint_content: Option<String>,
    pub reference_links: Option<String>,
    pub status: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub attempt_count: i32,
    pub is_starred: bool,
    pub reflection: Option<String>,
    pub code_snippets: Option<String>,
    pub screenshots_json: Option<String>,
    pub created_wiki_pages: String,
    pub related_wiki_pages: String,
    pub tags: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDailyLog {
    pub id: String,
    pub date: String,
    pub total_minutes: i32,
    pub tasks_completed: i32,
    pub tasks_in_progress: i32,
    pub streak_day: i32,
    pub reflection: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningStats {
    pub streak_day: i32,
    pub completion_rate: f64,
    pub total_minutes: i32,
    pub total_tasks: i32,
    pub completed_tasks: i32,
    pub weekly_data: Vec<TaskDailyLog>,
    pub topic_distribution: Vec<TopicCount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicCount {
    pub topic: String,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskWikiLink {
    pub id: String,
    pub task_id: String,
    pub wiki_id: String,
    pub relevance_score: f64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskSolution {
    pub id: String,
    pub task_id: String,
    pub title: String,
    pub author: Option<String>,
    pub source_url: Option<String>,
    pub content: String,
    pub solution_type: String,
    pub difficulty_rating: Option<f64>,
    pub quality_rating: Option<f64>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskWikiMatch {
    pub wiki_id: String,
    pub title: String,
    pub score: f64,
}

// ========== Knowledge Health Panel (E-4-3) ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewHealthItem {
    pub schedule: ReviewSchedule,
    pub wiki_title: String,
    pub wiki_summary: Option<String>,
    pub tags: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewHealthStats {
    pub total_pages: i32,
    pub pages_with_reviews: i32,
    pub total_reviews_all_time: i32,
    pub avg_accuracy: f64,
    pub streak_day: i32,
    pub weekly_review_count: i32,
    pub overdue_count: i32,
    pub mastered_count: i32,
}

// ========== Wiki Learning Trail (E-4-4) ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamStats {
    pub total: i32,
    pub correct: i32,
    pub wrong: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedGoal {
    pub goal_id: String,
    pub goal_title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiReadStatus {
    pub wiki_page_id: String,
    pub is_read: bool,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalWikiLinkWithTitle {
    pub id: String,
    pub goal_id: String,
    pub wiki_page_id: String,
    pub relevance_score: f64,
    pub source: String,
    pub is_new: bool,
    pub created_at: String,
    pub wiki_title: String,
    pub review_count: i32,
    pub next_review_at: Option<String>,
    pub last_reviewed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewSessionItem {
    pub log_id: String,
    pub wiki_page_id: String,
    pub wiki_title: String,
    pub quality: i32,
    pub review_format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewSessionRecord {
    pub session_id: String,
    pub reviewed_at: String,
    pub total_count: i32,
    pub correct_count: i32,
    pub items: Vec<ReviewSessionItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalReviewLogItem {
    pub id: String,
    pub schedule_id: String,
    pub wiki_page_id: String,
    pub wiki_title: String,
    pub quality: i32,
    pub reviewed_at: String,
    pub review_format: Option<String>,
    pub response_time_seconds: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewLogDetail {
    pub id: String,
    pub schedule_id: String,
    pub wiki_page_id: String,
    pub wiki_title: String,
    pub wiki_summary: Option<String>,
    pub wiki_tags: Option<String>,
    pub quality: i32,
    pub interval_before: i32,
    pub interval_after: i32,
    pub ease_factor_before: f64,
    pub ease_factor_after: f64,
    pub reviewed_at: String,
    pub review_format: Option<String>,
    pub response_time_seconds: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthTrailResult {
    pub schedule: Option<ReviewSchedule>,
    pub recent_logs: Vec<ReviewLog>,
    pub is_due: bool,
    pub exam_stats: Option<ExamStats>,
    pub linked_goals: Vec<LinkedGoal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRecommendation {
    pub id: String,
    pub wiki_page_id: String,
    pub task_template_title: String,
    pub task_template_description: String,
    pub task_template_difficulty: String,
    pub task_template_tags: String,
    pub score: f64,
    pub matched_keywords: String,
    pub status: String,
    pub ignore_count: i32,
    pub created_task_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// ========== Sprint 5A: Quiz & Ordering Models ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizQuestion {
    pub stem: String,
    pub options: Vec<String>,
    pub correct_index: i32,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderingSteps {
    pub title: String,
    pub correct_order: Vec<String>,
}

// ========== Sprint 5B: Error Hunt & Cloze Models ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorHuntQuestion {
    pub title: String,
    pub content: String,
    pub error_index: i32,
    pub options: Vec<String>,
    pub explanation: String,
    pub correct_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClozeQuestion {
    pub template: String,
    pub blanks: Vec<ClozeBlank>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClozeBlank {
    pub index: usize,
    pub correct_answers: Vec<String>,
    pub hint: Option<String>,
}

// ========== Sprint 6A: Explain (E-6-1) & Variant (E-6-3) ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplainQuestion {
    pub wiki_title: String,
    pub wiki_summary: String,
    pub wiki_tags: Vec<String>,
    pub prompt: String,
    pub hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplainFeedback {
    pub score: i32,
    pub score_label: String,
    pub improvement_suggestions: Vec<String>,
    pub better_example: Option<String>,
    pub strength_points: Vec<String>,
    pub weakness_points: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantQuestion {
    pub format: String,
    pub question_data: serde_json::Value,
    pub variant_generation: i32,
    pub twist_description: String,
}

// ========== Sprint 6B: Comprehensive Quiz (E-6-2) ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveQuizQuestion {
    pub id: i32,
    pub stem: String,
    pub question_type: String, // "choice", "true_false", "short_answer"
    pub options: Vec<String>,
    pub correct_index: i32,
    pub explanation: String,
    pub source_page_title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveQuiz {
    pub id: String,
    pub title: String,
    pub questions: Vec<ComprehensiveQuizQuestion>,
    pub source_page_ids: Vec<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizSubmission {
    pub answers: Vec<QuizAnswer>,
    pub quiz_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizAnswer {
    pub question_id: i32,
    pub selected_index: i32,
    pub short_answer_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizResult {
    pub total: i32,
    pub correct: i32,
    pub score_percent: f64,
    pub answers: Vec<QuestionResult>,
    pub weak_pages: Vec<WeakPageInfo>,
    pub review_suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionResult {
    pub question_id: i32,
    pub stem: String,
    pub correct: bool,
    pub correct_index: i32,
    pub selected_index: i32,
    pub explanation: String,
    pub source_page_id: Option<String>,
    pub source_page_title: Option<String>,
    pub short_answer_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeakPageInfo {
    pub page_id: String,
    pub page_title: String,
    pub wrong_count: i32,
    pub total_related: i32,
}

// ========== Sprint 6B: Adaptive Recommendations (E-6-4) ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveRecommendation {
    pub recommendation_type: String, // "strengthen", "advance", "reengage"
    pub title: String,
    pub description: String,
    pub reason: String,
    pub target_id: Option<String>,
    pub target_type: String, // "learning_path", "module", "practice_task", "wiki_page"
    pub priority: i32,
    pub learning_path_id: Option<String>,
    pub learning_path_name: Option<String>,
}

// ========== Phase 7: Knowledge Discovery (E-7-1) ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingContent {
    pub id: String,
    pub title: String,
    pub source_url: Option<String>,
    pub source_name: Option<String>,
    pub content_summary: Option<String>,
    pub source_page_id: Option<String>,
    pub source_page_title: Option<String>,
    pub match_reason: Option<String>,
    pub match_keywords: Option<String>,
    pub relevance_score: f64,
    pub full_content: Option<String>,
    pub content_hash: Option<String>,
    pub status: String,
    pub read_at: Option<String>,
    pub imported_content_id: Option<String>,
    pub discovered_at: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeMonitorSource {
    pub id: String,
    pub page_id: Option<String>,
    pub search_query: String,
    pub source_type: String,
    pub rss_url: Option<String>,
    pub is_active: bool,
    pub last_checked_at: Option<String>,
    pub last_found_count: i32,
    pub created_at: String,
}

// ========== Ranking System (Phase 8) ==========

#[derive(Debug, Clone, Serialize)]
pub struct KnowledgeRanking {
    pub total_score: f64,
    pub level: String,
    pub level_color: String,
    pub breadth: CategoryScore,
    pub depth: CategoryScore,
    pub mastery: CategoryScore,
    pub discovery: CategoryScore,
    pub connections: CategoryScore,
    pub tag_distribution: Vec<TagScore>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LearningRanking {
    pub total_score: f64,
    pub level: String,
    pub level_color: String,
    pub consistency: CategoryScore,
    pub completion: CategoryScore,
    pub quality: CategoryScore,
    pub dedication: CategoryScore,
    pub stats_summary: StatsSummary,
}

#[derive(Debug, Clone, Serialize)]
pub struct CategoryScore {
    pub score: f64,
    pub max_score: f64,
    pub percentage: f64,
    pub label: String,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TagScore {
    pub tag: String,
    pub page_count: i32,
    pub avg_mastery: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct StatsSummary {
    pub streak_day: i32,
    pub completed_tasks: i32,
    pub total_tasks: i32,
    pub total_reviews: i32,
    pub avg_quality: f64,
    pub total_minutes: i32,
}

// ========== Goal System ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub keywords: String,  // JSON array
    pub status: String,    // "active" | "achieved" | "archived"
    pub progress: f64,     // 0.0 - 100.0
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalWikiLink {
    pub id: String,
    pub goal_id: String,
    pub wiki_page_id: String,
    pub relevance_score: f64,
    pub source: String,    // "auto" | "manual"
    pub is_new: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exam {
    pub id: String,
    pub goal_id: String,
    pub title: Option<String>,
    pub total_questions: i32,
    pub score: Option<f64>,
    pub grade: Option<String>,
    pub status: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub diagnosis_json: Option<String>,
    pub created_at: String,
    pub version: i32,
    pub question_config: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamQuestion {
    pub id: String,
    pub exam_id: String,
    pub wiki_page_id: String,
    pub question_type: String,
    pub question_json: String,
    pub user_answer: Option<String>,
    pub correct_answer: Option<String>,
    pub is_correct: Option<bool>,
    pub score: Option<f64>,
    pub ai_feedback: Option<String>,
    pub sort_order: i32,
    pub answered_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamDetail {
    pub exam: Exam,
    pub questions: Vec<ExamQuestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamSummary {
    pub id: String,
    pub title: Option<String>,
    pub score: Option<f64>,
    pub grade: Option<String>,
    pub total_questions: i32,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalRecommendation {
    pub id: String,
    pub goal_id: String,
    pub title: String,
    pub url: Option<String>,
    pub summary: Option<String>,
    pub difficulty: String,
    pub sort_order: i32,
    pub status: String,
    pub imported_content_id: Option<String>,
    pub created_at: String,
}

// ========== Folder Sync ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncFolder {
    pub id: String,
    pub path: String,
    pub enabled: bool,
    pub last_synced_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRecord {
    pub id: String,
    pub folder_id: String,
    pub file_path: String,
    pub file_name: String,
    pub file_size: Option<i64>,
    pub file_mtime: String,
    pub file_type: String,
    pub content_id: Option<String>,
    pub status: String,
    pub synced_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub imported: Vec<String>,
    pub updated: Vec<String>,
    pub skipped: i32,
    pub errors: Vec<String>,
}
