// Types matching Rust backend structs (src-tauri/src/storage/models.rs)

export interface LearningPath {
  id: string;
  title: string;
  description: string;
  topic: string;
  difficulty: string; // "beginner" | "intermediate" | "advanced"
  estimated_days: number;
  module_count: number;
  completion_rate: number;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface Module {
  id: string;
  path_id: string;
  title: string;
  sort_order: number;
  description: string;
  theory_markdown: string;
  reading_list_json: string;
  estimated_read_minutes: number;
  discussion_prompts: string;
  community_solutions: string;
  task_ids: string;
  status: string; // "locked" | "available" | "completed"
  completed_at: string | null;
  created_at: string;
  updated_at: string;
}

export interface PracticeTask {
  id: string;
  module_id: string;
  title: string;
  description: string;
  difficulty: string; // "easy" | "medium" | "hard"
  estimated_minutes: number;
  prerequisites: string;
  hint_content: string | null;
  reference_links: string | null;
  status: string; // "not_started" | "in_progress" | "completed" | "reviewed"
  started_at: string | null;
  completed_at: string | null;
  attempt_count: number;
  is_starred: boolean;
  reflection: string | null;
  code_snippets: string | null;
  screenshots_json: string | null;
  created_wiki_pages: string;
  related_wiki_pages: string;
  tags: string | null;
  created_at: string;
  updated_at: string;
}

export interface TaskDailyLog {
  id: string;
  date: string;
  total_minutes: number;
  tasks_completed: number;
  tasks_in_progress: number;
  streak_day: number;
  reflection: string | null;
  created_at: string;
}

export interface LearningStats {
  streak_day: number;
  completion_rate: number;
  total_minutes: number;
  total_tasks: number;
  completed_tasks: number;
  weekly_data: TaskDailyLog[];
  topic_distribution: TopicCount[];
}

export interface TopicCount {
  topic: string;
  count: number;
}

export interface TaskWikiLink {
  id: string;
  task_id: string;
  wiki_id: string;
  relevance_score: number;
  created_at: string;
}

export interface TaskSolution {
  id: string;
  task_id: string;
  title: string;
  author: string | null;
  source_url: string | null;
  content: string;
  solution_type: string;
  difficulty_rating: number | null;
  quality_rating: number | null;
  created_at: string;
}

export interface TaskWikiMatch {
  wiki_id: string;
  title: string;
  score: number;
}

// Extended WikiPage with author fields
export interface WikiPageWithAuthor {
  id: string;
  title: string;
  slug: string;
  page_type: string;
  body_markdown: string;
  summary: string | null;
  tags: string | null;
  status: string;
  confidence: number;
  created_at: string;
  updated_at: string;
  last_compiled_at: string | null;
  source_message_id: string | null;
  author_name: string | null;
  author_url: string | null;
  source_type: string | null;
}

// ===== E-3-1: Task Recommendations =====

export interface TaskRecommendation {
  id: string;
  wiki_page_id: string;
  task_template_title: string;
  task_template_description: string;
  task_template_difficulty: string;
  task_template_tags: string;
  score: number;
  matched_keywords: string;
  status: string;
  ignore_count: number;
  created_task_id: string | null;
  created_at: string;
  updated_at: string;
}

// ===== Sprint 4: Review System (E-4-1, E-4-2) =====

export interface ReviewSchedule {
  id: string;
  wiki_page_id: string;
  ease_factor: number;
  interval_days: number;
  next_review_at: string;
  review_count: number;
  last_reviewed_at: string | null;
  mastery: number;
  is_archived: boolean;
  variant_streak: number;
  variant_mode: number;
  created_at: string;
  updated_at: string;
}

export interface DueReviewItem {
  schedule: ReviewSchedule;
  wiki_title: string;
  wiki_summary: string | null;
  wiki_tags: string | null;
  next_format?: string;
}

export interface ReviewStats {
  total_due: number;
  total_reviewed_today: number;
  streak: number;
}

// ===== E-4-3: Knowledge Health Panel =====

export interface ReviewHealthItem {
  schedule: ReviewSchedule;
  wiki_title: string;
  wiki_summary: string | null;
  tags: string | null;
}

export interface ReviewHealthStats {
  total_pages: number;
  pages_with_reviews: number;
  total_reviews_all_time: number;
  avg_accuracy: number;
  streak_day: number;
  weekly_review_count: number;
  overdue_count: number;
  mastered_count: number;
}

export interface ReviewLog {
  id: string;
  schedule_id: string;
  quality: number;
  interval_before: number;
  interval_after: number;
  ease_factor_before: number;
  ease_factor_after: number;
  reviewed_at: string;
  review_format: string | null;
  question_snapshot: string | null;
}

export interface ExamStats {
  total: number;
  correct: number;
  wrong: number;
}

export interface LinkedGoal {
  goal_id: string;
  goal_title: string;
}

// ===== E-4-4: Wiki Learning Trail =====

export interface HealthTrailResult {
  schedule: ReviewSchedule | null;
  recent_logs: ReviewLog[];
  is_due: boolean;
  exam_stats: ExamStats | null;
  linked_goals: LinkedGoal[];
}

// ===== Sprint 5A: Diverse Review Formats =====

export interface QuizQuestion {
  stem: string;
  options: string[];
  correct_index: number;
  explanation: string;
}

export interface OrderingSteps {
  title: string;
  steps: string[];
}

// ===== Sprint 5B: Error Hunt & Cloze =====

export interface ErrorHuntQuestion {
  title: string;
  content: string;         // 带错误的描述文本
  error_index: number;     // 错误选项索引
  options: string[];       // 4个选项
  explanation: string;     // 解析
  correct_version: string; // 正确版本
}

export interface ClozeQuestion {
  template: string;        // 带 ___ 占位符的文本
  blanks: ClozeBlank[];
}

export interface ClozeBlank {
  index: number;
  correct_answers: string[];
  hint?: string;
}

// ===== Sprint 6A: Explain (E-6-1) & Variant (E-6-3) =====

export interface ExplainQuestion {
  wiki_title: string;
  wiki_summary: string;
  wiki_tags: string[];
  prompt: string;
  hint?: string;
}

export interface ExplainFeedback {
  score: number;
  score_label: string;
  improvement_suggestions: string[];
  better_example: string | null;
  strength_points: string[];
  weakness_points: string[];
}

export interface VariantQuestion {
  format: string;
  question_data: any;
  variant_generation: number;
  twist_description: string;
}

// ALL_FORMATS constant — must stay in sync with backend
export const ALL_FORMATS = [
  "quiz",
  "cloze",
  "explain",
] as const;

export type ReviewFormat = (typeof ALL_FORMATS)[number];

// ===== Phase 7: Knowledge Discovery (E-7-1 / E-7-3) =====

export interface PendingContent {
  id: string;
  title: string;
  source_url: string | null;
  source_name: string | null;
  content_summary: string | null;
  source_page_id: string | null;
  source_page_title: string | null;
  match_reason: string | null;
  match_keywords: string | null;
  relevance_score: number;
  full_content: string | null;
  content_hash: string | null;
  status: string; // "unread" | "reading" | "imported" | "dismissed"
  read_at: string | null;
  imported_content_id: string | null;
  discovered_at: string;
  created_at: string;
}

// ===== Goal System (Phase 1 Refactor) =====

export interface Goal {
  id: string;
  title: string;
  description: string;
  keywords: string;      // JSON array string
  status: string;        // "active" | "achieved" | "archived"
  progress: number;      // 0-100
  created_at: string;
  updated_at: string;
}

export interface GoalWikiLink {
  id: string;
  goal_id: string;
  wiki_page_id: string;
  relevance_score: number;
  source: string;        // "auto" | "manual"
  is_new: boolean;
  created_at: string;
  wiki_title: string;    // joined from wiki_pages table
  review_count: number;
  next_review_at: string | null;
  last_reviewed_at: string | null;
}

export interface ReviewLogEntry {
  id: string;
  schedule_id: string;
  wiki_page_id: string;
  wiki_title: string;
  quality: number;       // 0=错误, 1=部分正确, 2=正确
  reviewed_at: string;
  review_format: string | null;
  response_time_seconds: number | null;
}

export interface ReviewSessionItem {
  log_id: string;
  wiki_page_id: string;
  wiki_title: string;
  quality: number;
  review_format: string | null;
}

export interface ReviewSessionRecord {
  session_id: string;
  reviewed_at: string;
  total_count: number;
  correct_count: number;
  items: ReviewSessionItem[];
}

export interface ReviewLogDetail {
  id: string;
  schedule_id: string;
  wiki_page_id: string;
  wiki_title: string;
  wiki_summary: string | null;
  wiki_tags: string | null;
  quality: number;
  interval_before: number;
  interval_after: number;
  ease_factor_before: number;
  ease_factor_after: number;
  reviewed_at: string;
  review_format: string | null;
  response_time_seconds: number | null;
}

export const REVIEW_FORMAT_LABELS: Record<string, string> = {
  choice: "选择题",
  judgment: "判断题",
  essay: "论述题",
  // Legacy format names for backward compatibility with old review_logs
  quiz: "选择题",
  matching: "判断题",
  rapid_fire: "选择题",
  ordering: "判断题",
  error_hunt: "选择题",
  cloze: "论述题",
  explain: "论述题",
};

export const QUALITY_LABELS: Record<number, string> = {
  0: "错误",
  1: "部分正确",
  2: "正确",
};

export interface GoalWikiItem {
  link: GoalWikiLink;
  wiki_title: string;
  wiki_summary: string | null;
  wiki_tags: string | null;
  mastery: number;       // from review_schedule
}

// ===== Exam System (Phase 2) =====

export interface Exam {
  id: string;
  goal_id: string;
  title: string | null;
  total_questions: number;
  score: number | null;
  grade: string | null;  // "A" | "B" | "C" | "D"
  status: string;        // "in_progress" | "completed"
  started_at: string;
  completed_at: string | null;
  diagnosis_json: string | null;
  version: number;
  question_config: string | null;
  created_at: string;
}

export interface ExamQuestion {
  id: string;
  exam_id: string;
  wiki_page_id: string;
  question_type: string;  // "choice" | "judgment" | "essay"
  question_json: string;  // JSON string containing stem, options, etc.
  user_answer: string | null;
  correct_answer: string | null;
  is_correct: boolean | null;
  score: number | null;
  ai_feedback: string | null;
  sort_order: number;
  answered_at: string | null;
}

export interface ExamDetail {
  exam: Exam;
  questions: ExamQuestion[];
}

export interface ExamSummary {
  id: string;
  title: string | null;
  score: number | null;
  grade: string | null;
  total_questions: number;
  status: string;
  created_at: string;
}

export interface ExamDiagnosis {
  weak_wiki_pages: string[];
  total_correct: number;
  total_wrong: number;
}

export interface WikiMasteryFlag {
  id: string;
  wiki_page_id: string;
  goal_id: string;
  exam_id: string;
  is_resolved: boolean;
  created_at: string;
  resolved_at: string | null;
}

// Parsed question content from question_json
export interface ParsedQuestion {
  id?: number;
  stem: string;
  question_type: string;
  options: string[];
  correct_answer: string;
  explanation: string;
  source_page_id?: string;
}

// ===== Goal Recommendations (Phase 3) =====

export interface GoalRecommendation {
  id: string;
  goal_id: string;
  title: string;
  url: string | null;
  summary: string | null;
  difficulty: string;  // "beginner" | "intermediate" | "advanced"
  sort_order: number;
  status: string;      // "pending" | "imported" | "dismissed"
  imported_content_id: string | null;
  created_at: string;
}
