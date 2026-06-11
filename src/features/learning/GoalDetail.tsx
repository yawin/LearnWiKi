import { useState, useEffect } from "react";
import { Target, BookOpen, ChevronLeft, ClipboardCheck, Loader2, Circle, CheckCircle2, ChevronRight, ChevronDown } from "lucide-react";
import { getGoal, getGoalWikiPages, markGoalLinksSeen, createExam, getGoalExams, getGoalReviewSessions } from "../../services/learningService";
import type { Goal, GoalWikiLink, Exam, ReviewSessionRecord } from "../../types/learning";
import { ExamSession } from "./ExamSession";
import { GoalRecommendations } from "./GoalRecommendations";
import { CreateExamModal } from "./CreateExamModal";
import { ExamConfirmModal } from "./ExamConfirmModal";

interface GoalDetailProps {
  goalId: string;
  onBack: () => void;
}

export function GoalDetail({ goalId, onBack }: GoalDetailProps) {
  const [goal, setGoal] = useState<Goal | null>(null);
  const [links, setLinks] = useState<GoalWikiLink[]>([]);
  const [loading, setLoading] = useState(true);
  const [examId, setExamId] = useState<string | null>(null);
  const [creatingExam, setCreatingExam] = useState(false);
  const [readStatuses, setReadStatuses] = useState<Record<string, boolean>>({});
  const [reviewSessions, setReviewSessions] = useState<ReviewSessionRecord[]>([]);
  const [showAllLogs, setShowAllLogs] = useState(false);
  const [examEligible, setExamEligible] = useState(false);
  const [showExamModal, setShowExamModal] = useState(false);
  const [examConfig, setExamConfig] = useState({ choice: 6, judgment: 2, essay: 2 });
  const [exams, setExams] = useState<Exam[]>([]);
  const [examConfirmId, setExamConfirmId] = useState<string | null>(null);
  const [examExpanded, setExamExpanded] = useState(false);
  const [reviewToast, setReviewToast] = useState<string | null>(null);

  useEffect(() => {
    loadData();
  }, [goalId]);

  // Live-refresh read status from WikiPageDetail toggle
  useEffect(() => {
    const handler = (e: Event) => {
      const detail = (e as CustomEvent<{ wikiPageId: string; isRead: boolean }>).detail;
      if (detail?.wikiPageId) {
        setReadStatuses((prev) => ({ ...prev, [detail.wikiPageId]: detail.isRead }));
      }
    };
    window.addEventListener("wiki-read-status-changed", handler);
    return () => window.removeEventListener("wiki-read-status-changed", handler);
  }, []);

  const loadData = async () => {
    setLoading(true);
    try {
      const [g, l, sessions] = await Promise.all([
        getGoal(goalId),
        getGoalWikiPages(goalId),
        getGoalReviewSessions(goalId, 20),
      ]);
      setGoal(g);
      setLinks(l);
      setReviewSessions(sessions);
      // Load read status for each linked wiki page
      const statuses: Record<string, boolean> = {};
      for (const link of l) {
        try {
          const { getWikiReadStatus } = await import("../../services/learningService");
          statuses[link.wiki_page_id] = await getWikiReadStatus(link.wiki_page_id);
        } catch {
          statuses[link.wiki_page_id] = false;
        }
      }
      setReadStatuses(statuses);

      // Load exams
      try {
        const examList = await getGoalExams(goalId);
        setExams(examList ?? []);
      } catch { setExams([]); }

      // Check exam eligibility: all pages read + reviewed at least once
      const allReady = l.every((link) =>
        statuses[link.wiki_page_id] &&
        (link.review_count || 0) >= 1
      );
      setExamEligible(allReady);

      // Mark links as seen
      if (l.some(link => link.is_new)) {
        await markGoalLinksSeen(goalId);
      }
    } catch (err) {
      console.error("Failed to load goal:", err);
    } finally {
      setLoading(false);
    }
  };

  const formatReviewTime = (nextReviewAt: string | null): { text: string; urgent: boolean } => {
    if (!nextReviewAt) return { text: "待复习", urgent: true };
    const next = new Date(nextReviewAt);
    const now = new Date();
    if (next <= now) return { text: "已到期", urgent: true };
    const diffMs = next.getTime() - now.getTime();
    const diffDays = Math.ceil(diffMs / (1000 * 60 * 60 * 24));
    if (diffDays === 0) return { text: "今天", urgent: true };
    if (diffDays === 1) return { text: "明天", urgent: false };
    if (diffDays <= 3) return { text: `${diffDays}天后 (${next.getMonth() + 1}/${next.getDate()})`, urgent: false };
    return { text: `${next.getMonth() + 1}/${next.getDate()}`, urgent: false };
  };

  const formatDate = (isoStr: string): string => {
    const d = new Date(isoStr);
    return `${d.getMonth() + 1}/${String(d.getDate()).padStart(2, "0")} ${String(d.getHours()).padStart(2, "0")}:${String(d.getMinutes()).padStart(2, "0")}`;
  };

  const startSingleReview = (wikiPageId: string, wikiTitle: string) => {
    window.dispatchEvent(new CustomEvent("start-single-review", {
      detail: { wikiPageId, wikiTitle },
    }));
  };

  const handleStartExam = async () => {
    setCreatingExam(true);
    try {
      const detail = await createExam(goalId);
      setExamId(detail.exam.id);
    } catch (err) {
      console.error("Failed to create exam:", err);
    } finally {
      setCreatingExam(false);
    }
  };

  const handleCreateCustomExam = async () => {
    setShowExamModal(false);
    setCreatingExam(true);
    try {
      const detail = await createExam(goalId, null, JSON.stringify(examConfig));
      setExamId(detail.exam.id);
    } catch (err) {
      console.error("Failed to create exam:", err);
    } finally {
      setCreatingExam(false);
    }
  };

  if (examId) {
    return <ExamSession examId={examId} onClose={() => setExamId(null)} />;
  }

  if (loading || !goal) {
    return (
      <div className="px-5 pt-5">
        <div className="animate-pulse space-y-3">
          <div className="h-6 bg-gray-200 dark:bg-gray-700 rounded w-1/2" />
          <div className="h-4 bg-gray-100 dark:bg-gray-800 rounded w-3/4" />
        </div>
      </div>
    );
  }

  const unreadLinks = links.filter((l) => !readStatuses[l.wiki_page_id]);
  const readLinks = links.filter((l) => readStatuses[l.wiki_page_id]);
  const dueLinks = readLinks.filter((l) => {
    const next = l.next_review_at;
    return !next || new Date(next) <= new Date();
  });
  const masteredLinks = readLinks.filter((l) => {
    const next = l.next_review_at;
    return next && new Date(next) > new Date();
  });
  const displaySessions = showAllLogs ? reviewSessions : reviewSessions.slice(0, 3);

  // Progress derived from actual read + review data (consistent with review overview)
  const readAndReviewedCount = links.filter(
    (l) => readStatuses[l.wiki_page_id] && (l.review_count || 0) >= 1
  ).length;
  const computedProgress = links.length > 0
    ? Math.round((readAndReviewedCount / links.length) * 100)
    : 0;

  return (
    <>
    <div className="px-5 pt-4 pb-8">
      {/* Back button */}
      <button
        onClick={onBack}
        className="inline-flex items-center gap-1 mb-3 text-sm hover:text-orange-500 transition-colors"
        style={{ color: "var(--color-text-muted)" }}
      >
        <ChevronLeft size={16} />
        返回
      </button>

      {/* Header */}
      <div className="flex items-start gap-3 mb-4">
        <div
          className="p-2 rounded-lg"
          style={{ backgroundColor: "#FFF7ED" }}
        >
          <Target size={20} className="text-orange-500" />
        </div>
        <div className="flex-1">
          <h2
            style={{
              fontSize: 22,
              fontFamily: "'Cabinet Grotesk', sans-serif",
              fontWeight: 700,
              color: "var(--color-text-primary)",
              letterSpacing: "-0.3px",
            }}
          >
            {goal.title}
          </h2>
          {goal.description && (
            <p style={{ fontSize: 13, color: "var(--color-text-muted)", marginTop: 4 }}>
              {goal.description}
            </p>
          )}
        </div>
      </div>

      {/* Progress bar */}
      <div className="mb-6">
        <div className="flex items-center justify-between mb-1">
          <span style={{ fontSize: 12, color: "var(--color-text-muted)" }}>掌握进度</span>
          <span style={{ fontSize: 12, fontWeight: 600, color: "var(--color-text-secondary)" }}>
            {computedProgress}%
          </span>
        </div>
        <div
          className="h-2 rounded-full overflow-hidden"
          style={{ backgroundColor: "var(--color-border)" }}
        >
          <div
            className="h-full rounded-full transition-all"
            style={{
              width: `${computedProgress}%`,
              backgroundColor: "#F97316",
            }}
          />
        </div>
        <p style={{ fontSize: 11, color: "var(--color-text-muted)", marginTop: 4 }}>
          基于阅读和复习完成度
        </p>
      </div>

      {/* Empty state */}
      {links.length === 0 && (
        <div
          className="mb-4 rounded-xl p-6 text-center"
          style={{
            backgroundColor: "var(--color-surface)",
            border: "1px solid var(--color-border)",
          }}
        >
          <p style={{ fontSize: 13, color: "var(--color-text-muted)" }}>
            还没有关联知识点。编译 Wiki 后相关内容会自动关联到这里。
          </p>
        </div>
      )}

      {/* 待阅读 section */}
      {unreadLinks.length > 0 && (
        <div className="mb-4">
          <div className="flex items-center gap-2 mb-2">
            <span style={{ fontSize: 13, fontWeight: 600, color: "var(--color-text-muted)" }}>
              ── 待阅读 ({unreadLinks.length}) ──
            </span>
          </div>
          <div className="space-y-1">
            {unreadLinks.map((link) => (
              <div
                key={link.id}
                className="w-full text-left rounded-lg p-2.5 flex items-center gap-2 cursor-pointer hover:bg-orange-50 dark:hover:bg-orange-950/20 transition-colors"
                style={{
                  backgroundColor: "var(--color-surface)",
                  borderLeft: link.is_new ? "2px solid #F97316" : "1px solid var(--color-border)",
                }}
                onClick={() => {
                  window.dispatchEvent(new CustomEvent("navigate-to-wiki-page", {
                    detail: { pageId: link.wiki_page_id },
                  }));
                }}
              >
                <Circle size={14} style={{ color: "var(--color-text-muted)" }} />
                <span style={{ fontSize: 13, color: "var(--color-text-primary)", flex: 1 }}>
                  {link.wiki_title}
                </span>
                {link.is_new && (
                  <span className="px-1 py-0.5 rounded text-xs font-medium"
                    style={{ backgroundColor: "#FFF7ED", color: "#F97316" }}>
                    新
                  </span>
                )}
              </div>
            ))}
          </div>
        </div>
      )}

      {/* 复习概览 card */}
      {readLinks.length > 0 && (
        <div className="mb-4 rounded-xl p-4" style={{ backgroundColor: "var(--color-surface)", border: "1px solid var(--color-border)" }}>
          {/* Overview header */}
          <div className="flex items-center gap-2 mb-3">
            <BookOpen size={16} style={{ color: "var(--color-text-secondary)" }} />
            <span style={{ fontSize: 14, fontWeight: 600, color: "var(--color-text-primary)" }}>复习概览</span>
            <span style={{ fontSize: 12, color: "var(--color-text-muted)" }}>
              待复习 {dueLinks.length} · 已阅读 {masteredLinks.length}
            </span>
          </div>

          {/* 待复习 links */}
          {dueLinks.length > 0 && (
            <div className="mb-3">
              <span style={{ fontSize: 12, fontWeight: 600, color: "var(--color-text-muted)" }}>
                ── 待复习 ({dueLinks.length}) ──
              </span>
              <div className="space-y-1 mt-1">
                {dueLinks.map((link) => {
                  const reviewTime = formatReviewTime(link.next_review_at);
                  return (
                    <div
                      key={link.id}
                      className="w-full text-left rounded-lg p-2.5 flex items-center gap-2 cursor-pointer hover:bg-orange-50 dark:hover:bg-orange-950/20 transition-colors"
                      style={{
                        backgroundColor: "var(--color-surface)",
                        borderLeft: reviewTime.urgent ? "2px solid #EF4444" : "1px solid var(--color-border)",
                      }}
                      onClick={() => {
                        window.dispatchEvent(new CustomEvent("navigate-to-wiki-page", {
                          detail: { pageId: link.wiki_page_id },
                        }));
                      }}
                    >
                      <CheckCircle2 size={14} style={{ color: "var(--color-success, #22C55E)" }} />
                      <span style={{ fontSize: 13, color: "var(--color-text-primary)", flex: 1 }}>
                        {link.wiki_title}
                      </span>
                      <span style={{ fontSize: 11, color: reviewTime.urgent ? "#EF4444" : "var(--color-text-muted)" }}>
                        {reviewTime.text}
                      </span>
                      <button
                        onClick={(e) => { e.stopPropagation(); startSingleReview(link.wiki_page_id, link.wiki_title); }}
                        className="px-2 py-0.5 rounded text-xs font-medium bg-orange-500 text-white shrink-0"
                      >
                        复习
                      </button>
                    </div>
                  );
                })}
              </div>
            </div>
          )}

          {/* 已阅读 links */}
          {masteredLinks.length > 0 && (
            <div className="mb-3">
              <span style={{ fontSize: 12, fontWeight: 600, color: "var(--color-text-muted)" }}>
                ── 已阅读 ({masteredLinks.length}) ──
              </span>
              <div className="space-y-1 mt-1">
                {masteredLinks.map((link) => {
                  const reviewTime = formatReviewTime(link.next_review_at);
                  return (
                    <div
                      key={link.id}
                      className="w-full text-left rounded-lg p-2.5 flex items-center gap-2 cursor-pointer hover:bg-orange-50 dark:hover:bg-orange-950/20 transition-colors"
                      style={{
                        backgroundColor: "var(--color-surface)",
                        borderLeft: "1px solid var(--color-border)",
                      }}
                      onClick={() => {
                        window.dispatchEvent(new CustomEvent("navigate-to-wiki-page", {
                          detail: { pageId: link.wiki_page_id },
                        }));
                      }}
                    >
                      <CheckCircle2 size={14} style={{ color: "var(--color-success, #22C55E)" }} />
                      <span style={{ fontSize: 13, color: "var(--color-text-primary)", flex: 1 }}>
                        {link.wiki_title}
                      </span>
                      <span style={{ fontSize: 11, color: "var(--color-text-muted)" }}>
                        {reviewTime.text}
                      </span>
                      <button
                        onClick={(e) => {
                          e.stopPropagation();
                          setReviewToast(`下次复习时间: ${reviewTime.text}`);
                          setTimeout(() => setReviewToast(null), 2500);
                        }}
                        className="px-2 py-0.5 rounded text-xs font-medium shrink-0 opacity-50 cursor-not-allowed"
                        style={{ backgroundColor: "var(--color-border)", color: "var(--color-text-secondary)" }}
                      >
                        复习
                      </button>
                    </div>
                  );
                })}
              </div>
            </div>
          )}

          {/* 最近复习记录 */}
          {reviewSessions.length > 0 && (
            <div>
              <div className="flex items-center justify-between mb-2">
                <span style={{ fontSize: 12, fontWeight: 600, color: "var(--color-text-muted)" }}>
                  📋 最近复习记录
                </span>
                {reviewSessions.length > 3 && (
                  <button
                    onClick={() => setShowAllLogs(!showAllLogs)}
                    className="inline-flex items-center gap-0.5 text-xs hover:text-orange-500 transition-colors"
                    style={{ color: "var(--color-text-muted)" }}
                  >
                    {showAllLogs ? "收起" : "查看全部"}
                    <ChevronRight size={12} />
                  </button>
                )}
              </div>
              <div className="space-y-1">
                {displaySessions.map((session) => (
                  <div
                    key={session.session_id || session.reviewed_at}
                    className="rounded py-1.5 px-2 text-xs"
                    style={{ color: "var(--color-text-secondary)" }}
                  >
                    <div className="flex items-center justify-between mb-1">
                      <span style={{ color: "var(--color-text-muted)" }}>{formatDate(session.reviewed_at)}</span>
                      <span style={{ color: "var(--color-text-muted)" }}>
                        {session.correct_count}/{session.total_count} 正确
                      </span>
                    </div>
                    <div className="flex flex-wrap gap-1">
                      {session.items.map((item, i) => (
                        <button
                          key={i}
                          className="px-1.5 py-0.5 rounded text-xs cursor-pointer hover:ring-1 hover:ring-orange-300 transition-all"
                          style={{
                            backgroundColor: item.quality === 2 ? "#DCFCE7" : item.quality === 1 ? "#FEF3C7" : "#FEE2E2",
                            color: item.quality === 2 ? "#166534" : item.quality === 1 ? "#92400E" : "#991B1B",
                          }}
                          onClick={() => {
                            window.dispatchEvent(new CustomEvent("navigate-to-review-log", {
                              detail: { logId: item.log_id, goalId },
                            }));
                          }}
                        >
                          {item.wiki_title}
                        </button>
                      ))}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      )}

      {/* Exam section */}
      <div className="mb-4 rounded-xl p-4" style={{ backgroundColor: "var(--color-surface)", border: "1px solid var(--color-border)" }}>
        <div className="flex items-center gap-2 mb-2">
          <ClipboardCheck size={16} style={{ color: "var(--color-text-secondary)" }} />
          <span style={{ fontSize: 14, fontWeight: 600, color: "var(--color-text-primary)" }}>考试</span>
          {!examEligible && (
            <span className="px-1.5 py-0.5 rounded text-xs font-medium"
              style={{ backgroundColor: "var(--color-border)", color: "var(--color-text-muted)" }}>
              未解锁
            </span>
          )}
          <span style={{ fontSize: 12, color: "var(--color-text-muted)" }}>
            {exams.length > 0 ? `${exams.length} 份试卷` : "暂无试卷"}
          </span>
        </div>

        {/* Exam records always visible */}
        {exams.length > 0 ? (
          <div className="space-y-1 mb-3">
            {exams.map((exam) => (
              <div
                key={exam.id}
                onClick={() => {
                  if (exam.status === "completed") setExamId(exam.id);
                  else if (exam.status === "in_progress") setExamConfirmId(exam.id);
                }}
                className="flex items-center justify-between rounded-lg p-2 cursor-pointer hover:bg-orange-50 dark:hover:bg-orange-950/20 transition-colors"
                style={{ backgroundColor: "var(--color-surface)", border: "1px solid var(--color-border)" }}
              >
                <span style={{ fontSize: 13, color: "var(--color-text-primary)" }}>
                  试卷 v{exam.version} {exam.status === "completed" ? `· ${Math.round(exam.score ?? 0)}分 ${exam.grade}` : exam.status === "in_progress" ? "· 未完成" : "· 已过期"}
                </span>
                <span style={{ fontSize: 12, color: "var(--color-text-secondary)" }}>
                  {exam.status === "completed" ? "查看结果 →" : exam.status === "in_progress" ? "继续 →" : "—"}
                </span>
              </div>
            ))}
          </div>
        ) : (
          <p style={{ fontSize: 12, color: "var(--color-text-muted)", marginBottom: 8 }}>暂无历史试卷</p>
        )}

        {/* Create controls collapsible */}
        <div
          className="flex items-center justify-between cursor-pointer select-none"
          onClick={() => setExamExpanded(!examExpanded)}
        >
          <span style={{ fontSize: 12, color: "var(--color-text-muted)" }}>
            {examExpanded ? "收起出题选项" : "展开出题选项"}
          </span>
          <ChevronDown
            size={14}
            style={{
              color: "var(--color-text-muted)",
              transform: examExpanded ? "rotate(180deg)" : "rotate(0deg)",
              transition: "transform 0.2s",
            }}
          />
        </div>

        {examExpanded && (
          <div className="mt-3 pt-3" style={{ borderTop: "1px solid var(--color-border)" }}>
            {!examEligible && (
              <div className="mb-3 p-3 rounded-lg" style={{ backgroundColor: "#FEF3C7", border: "1px solid #FCD34D" }}>
                <p style={{ fontSize: 12, color: "#92400E" }}>
                  需要所有知识点已阅读并至少复习一次后才能解锁考试
                </p>
                <p style={{ fontSize: 11, color: "#92400E", marginTop: 4 }}>
                  当前进度: 阅读 {readLinks.length}/{links.length} · 复习{" "}
                  {links.filter((l) => l.review_count >= 1).length}/{links.length}
                </p>
              </div>
            )}

            <div className="flex gap-2">
              <button
                onClick={handleStartExam}
                disabled={!examEligible || creatingExam}
                className="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md text-white font-medium transition-colors disabled:opacity-50"
                style={{ fontSize: 13, backgroundColor: "#F97316" }}
              >
                {creatingExam ? <Loader2 size={14} className="animate-spin" /> : <ClipboardCheck size={14} />}
                {creatingExam ? "出题中..." : "快速出题"}
              </button>
              <button
                onClick={() => setShowExamModal(true)}
                disabled={!examEligible}
                className="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md font-medium transition-colors disabled:opacity-50"
                style={{ fontSize: 13, backgroundColor: "var(--color-surface)", border: "1px solid var(--color-border)", color: "var(--color-text-primary)" }}
              >
                自定义题型
              </button>
            </div>
          </div>
        )}
      </div>

      {/* Goal Recommendations */}
      <GoalRecommendations goalId={goalId} />
    </div>

    {/* Review toast */}
    {reviewToast && (
      <div
        className="fixed bottom-6 left-1/2 -translate-x-1/2 z-50 px-4 py-2.5 rounded-lg text-sm font-medium shadow-lg"
        style={{
          backgroundColor: "var(--color-text-primary)",
          color: "var(--color-bg)",
        }}
      >
        {reviewToast}
      </div>
    )}

    {/* Create exam modal */}
    {showExamModal && (
      <CreateExamModal
        examConfig={examConfig}
        onChangeConfig={setExamConfig}
        onConfirm={handleCreateCustomExam}
        onClose={() => setShowExamModal(false)}
      />
    )}

    {/* Exam confirm modal */}
    {examConfirmId && (() => {
      const exam = exams.find(e => e.id === examConfirmId);
      if (!exam) return null;
      let config = { choice: 0, judgment: 0, essay: 0 };
      if (exam.question_config) {
        try { config = JSON.parse(exam.question_config); } catch { /* use defaults */ }
      }
      const estimatedMinutes = Math.round((config.choice || 0) * 1 + (config.judgment || 0) * 0.5 + (config.essay || 0) * 2);
      return (
        <ExamConfirmModal
          examVersion={exam.version}
          questionConfig={config}
          estimatedMinutes={estimatedMinutes}
          onConfirm={() => { setExamId(examConfirmId); setExamConfirmId(null); }}
          onClose={() => setExamConfirmId(null)}
        />
      );
    })()}
    </>
  );
}
