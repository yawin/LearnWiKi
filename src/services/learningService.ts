import { invoke } from "@tauri-apps/api/core";
import type {
  ClozeQuestion,
  DueReviewItem,
  ErrorHuntQuestion,
  Exam,
  ExamDetail,
  ExamQuestion,
  ExamSummary,
  ExplainFeedback,
  ExplainQuestion,
  Goal,
  GoalRecommendation,
  GoalWikiLink,
  HealthTrailResult,
  OrderingSteps,
  QuizQuestion,
  ReviewHealthItem,
  ReviewHealthStats,
  ReviewLogDetail,
  ReviewLogEntry,
  ReviewSessionRecord,
  ReviewSchedule,
  ReviewStats,
  VariantQuestion,
} from "../types/learning";

// ===== Review System (Sprint 4, E-4-2) =====

export async function createReviewSchedule(
  wikiPageId: string
): Promise<ReviewSchedule> {
  return invoke("create_review_schedule", { wikiPageId });
}

export async function getDueReviews(
  limit?: number
): Promise<DueReviewItem[]> {
  return invoke("get_due_reviews", { limit: limit ?? null });
}

export async function getDueReviewForPage(
  wikiPageId: string
): Promise<DueReviewItem | null> {
  return invoke("get_due_review_for_page", { wikiPageId });
}

export async function submitReviewFeedback(
  scheduleId: string,
  quality: number,
  reviewFormat?: string,
  responseTimeSeconds?: number,
  sessionId?: string,
  questionSnapshot?: string
): Promise<ReviewSchedule> {
  return invoke("submit_review_feedback", {
    scheduleId,
    quality,
    reviewFormat: reviewFormat ?? null,
    responseTimeSeconds: responseTimeSeconds ?? null,
    sessionId: sessionId ?? null,
    questionSnapshot: questionSnapshot ?? null,
  });
}

export async function getReviewStats(): Promise<ReviewStats> {
  return invoke("get_review_stats");
}

export async function autoCreateReviewSchedule(
  wikiPageId: string
): Promise<ReviewSchedule> {
  return invoke("auto_create_review_schedule", { wikiPageId });
}

// ===== E-4-3: Knowledge Health Panel =====

export async function getHealthSchedules(): Promise<ReviewHealthItem[]> {
  return invoke("get_health_schedules");
}

export async function getReviewHealthStats(): Promise<ReviewHealthStats> {
  return invoke("get_review_health_stats");
}

// ===== E-4-4: Wiki Learning Trail =====

export async function getWikiLearningTrail(wikiPageId: string): Promise<HealthTrailResult> {
  return invoke("get_wiki_learning_trail", { wikiPageId });
}

// ===== Sprint 5A: Diverse Review Formats =====

export async function generateQuizQuestions(
  wikiPageId: string,
  count?: number
): Promise<QuizQuestion[]> {
  return invoke("generate_quiz_questions", {
    wikiPageId,
    count: count ?? null,
  });
}

export async function generateOrderingSteps(
  wikiPageId: string
): Promise<OrderingSteps> {
  return invoke("generate_ordering_steps", { wikiPageId });
}

export async function getAvailableFormats(): Promise<string[]> {
  return invoke("get_available_formats");
}

// ===== Sprint 5B: Error Hunt & Cloze =====

export async function generateErrorHunt(
  wikiPageId: string
): Promise<ErrorHuntQuestion> {
  return invoke("generate_error_hunt", { wikiPageId });
}

export async function generateCloze(
  wikiPageId: string
): Promise<ClozeQuestion> {
  return invoke("generate_cloze", { wikiPageId });
}

// ===== Sprint 6A: Explain (E-6-1) =====

export async function generateExplainReview(
  wikiPageId: string
): Promise<ExplainQuestion> {
  return invoke("generate_explain_review", { wikiPageId });
}

export async function submitExplainAnswer(
  wikiPageId: string,
  userExplanation: string
): Promise<ExplainFeedback> {
  return invoke("submit_explain_answer", {
    wikiPageId,
    userExplanation,
  });
}

// ===== Sprint 6A: Variant Question (E-6-3) =====

export async function generateVariantQuestion(
  wikiPageId: string,
  currentFormat: string,
  variantGeneration: number
): Promise<VariantQuestion> {
  return invoke("generate_variant_question", {
    wikiPageId,
    currentFormat,
    variantGeneration,
  });
}

// ===== Goal =====

export async function createGoal(
  title: string,
  description?: string
): Promise<Goal> {
  return invoke("create_goal", { title, description: description ?? null });
}

export async function getGoals(status?: string): Promise<Goal[]> {
  return invoke("get_goals", { status: status ?? null });
}

export async function getGoal(id: string): Promise<Goal> {
  return invoke("get_goal", { id });
}

export async function updateGoal(
  id: string,
  title?: string,
  description?: string,
  keywords?: string,
  status?: string
): Promise<Goal> {
  return invoke("update_goal", {
    id,
    title: title ?? null,
    description: description ?? null,
    keywords: keywords ?? null,
    status: status ?? null,
  });
}

export async function deleteGoal(id: string): Promise<void> {
  return invoke("delete_goal", { id });
}

export async function linkWikiToGoal(
  goalId: string,
  wikiPageId: string,
  relevanceScore?: number,
  source?: string
): Promise<void> {
  return invoke("link_wiki_to_goal", {
    goalId,
    wikiPageId,
    relevanceScore: relevanceScore ?? null,
    source: source ?? null,
  });
}

export async function unlinkWikiFromGoal(
  goalId: string,
  wikiPageId: string
): Promise<void> {
  return invoke("unlink_wiki_from_goal", { goalId, wikiPageId });
}

export async function getGoalWikiPages(goalId: string): Promise<GoalWikiLink[]> {
  return invoke("get_goal_wiki_pages", { goalId });
}

export async function markGoalLinksSeen(goalId: string): Promise<void> {
  return invoke("mark_goal_links_seen", { goalId });
}

// ===== Learning Mode =====

export async function getLearningContent(wikiPageId: string): Promise<{ title: string; concept: string; detail: string; extend: string }> {
  return invoke("get_learning_content", { wikiPageId });
}

export async function markAsLearned(goalId: string, wikiPageId: string): Promise<void> {
  return invoke("mark_as_learned", { goalId, wikiPageId });
}

export async function generateInstantQuiz(wikiPageId: string): Promise<QuizQuestion[]> {
  return invoke("generate_instant_quiz", { wikiPageId });
}

// ===== Exam =====

export async function createExam(
  goalId: string,
  questionCount?: number | null,
  questionConfig?: string | null
): Promise<ExamDetail> {
  return invoke("create_exam", { goalId, questionCount: questionCount ?? null, questionConfig: questionConfig ?? null });
}

export async function getExam(examId: string): Promise<ExamDetail> {
  return invoke("get_exam", { examId });
}

export async function submitExamAnswer(
  questionId: string,
  answer: string
): Promise<ExamQuestion> {
  return invoke("submit_exam_answer", { questionId, answer });
}

export async function completeExam(examId: string): Promise<ExamDetail> {
  return invoke("complete_exam", { examId });
}

export async function getExamHistory(goalId: string): Promise<ExamSummary[]> {
  return invoke("get_exam_history", { goalId });
}

// ===== Goal Recommendations =====

export async function searchGoalResources(goalId: string): Promise<GoalRecommendation[]> {
  return invoke("search_goal_resources", { goalId });
}

export async function getGoalRecommendations(goalId: string): Promise<GoalRecommendation[]> {
  return invoke("get_goal_recommendations", { goalId });
}

export async function dismissGoalRecommendation(recommendationId: string): Promise<void> {
  return invoke("dismiss_goal_recommendation", { recommendationId });
}

export async function matchWikiToGoals(wikiPageId: string): Promise<GoalWikiLink[]> {
  return invoke("match_wiki_to_goals", { wikiPageId });
}

// ===== Wiki Read Status =====

export async function getWikiReadStatus(wikiPageId: string): Promise<boolean> {
  return invoke("get_wiki_read_status", { wikiPageId });
}

export async function setWikiReadStatus(
  wikiPageId: string,
  isRead: boolean
): Promise<void> {
  return invoke("set_wiki_read_status", { wikiPageId, isRead });
}

export async function getGoalExams(goalId: string): Promise<Exam[]> {
  return invoke("get_goal_exams", { goalId });
}

export async function getGoalReviewLogs(
  goalId: string,
  limit?: number
): Promise<ReviewLogEntry[]> {
  return invoke("get_goal_review_logs", {
    goalId,
    limit: limit ?? null,
  });
}

export async function getGoalReviewSessions(
  goalId: string,
  limit?: number
): Promise<ReviewSessionRecord[]> {
  return invoke("get_goal_review_sessions", { goalId, limit: limit ?? null });
}

export async function getReviewLog(
  logId: string
): Promise<ReviewLogDetail | null> {
  return invoke("get_review_log", { logId });
}

export async function getWikiExamHistory(wikiPageId: string): Promise<Exam[]> {
  return invoke("get_wiki_exam_history", { wikiPageId });
}

export async function checkMasteryFlags(goalId: string): Promise<boolean> {
  return invoke("check_mastery_flags", { goalId });
}
