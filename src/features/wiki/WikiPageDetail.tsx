import { useState, useEffect } from "react";
import { X, BookOpen, User, FileText, GitCompare, Layers, Trash2, RotateCcw, Loader2, Search } from "lucide-react";
import { useTranslation } from "react-i18next";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import type { WikiPage, WikiPageSource } from "../../types/wiki";
import type { CapturedContent } from "../../types/content";
import type { HealthTrailResult, Exam } from "../../types/learning";
import { REVIEW_FORMAT_LABELS } from "../../types/learning";
import { getPageSources } from "../../services/wikiService";
import { getWikiLearningTrail, setWikiReadStatus, getWikiReadStatus, createReviewSchedule, getWikiExamHistory } from "../../services/learningService";
import { invoke } from "@tauri-apps/api/core";
import MonitorConfig from "../discovery/MonitorConfig";

const TYPE_ICONS: Record<string, React.ComponentType<{ className?: string; size?: number; style?: React.CSSProperties }>> = {
  concept: BookOpen,
  entity: User,
  source: FileText,
  comparison: GitCompare,
  overview: Layers,
};

const TYPE_LABEL_KEYS: Record<string, string> = {
  concept: "browse.pageType.concept",
  entity: "browse.pageType.entity",
  source: "browse.pageType.source",
  comparison: "browse.pageType.comparison",
  overview: "browse.pageType.overview",
};

const SOURCE_STATUS_ICON: Record<string, string> = {
  active: "✓",
  stale: "⚠",
  deleted: "✗",
};

const SOURCE_STATUS_COLOR: Record<string, string> = {
  active: "#16A34A",
  stale: "#CA8A04",
  deleted: "#DC2626",
};

interface WikiPageDetailProps {
  page: WikiPage;
  onClose: () => void;
  onDelete: (id: string) => void;
  onNavigateToContent?: (contentId: string) => void;
  onNavigateToGoal?: (goalId: string) => void;
}

function getRelativeTime(dateStr: string): string {
  const now = new Date();
  const date = new Date(dateStr);
  const diffMs = date.getTime() - now.getTime();
  const diffDays = Math.round(diffMs / (1000 * 60 * 60 * 24));

  if (diffDays < 0) {
    const absDays = Math.abs(diffDays);
    return absDays === 0 ? "刚刚" : `${absDays}天前`;
  } else if (diffDays === 0) {
    return "今天";
  } else if (diffDays === 1) {
    return "昨天";
  } else {
    return `${diffDays}天前`;
  }
}

function getMasteryColor(mastery: number): string {
  if (mastery >= 0.9) return "#22c55e";
  if (mastery >= 0.5) return "#eab308";
  return "#ef4444";
}

function parseSnapshot(snapshot: string | null): any[] {
  if (!snapshot) return [];
  try { return JSON.parse(snapshot); } catch { return []; }
}

export function WikiPageDetail({ page, onClose, onDelete, onNavigateToContent, onNavigateToGoal }: WikiPageDetailProps) {
  const { t } = useTranslation("wiki");
  const [sources, setSources] = useState<(WikiPageSource & { content?: CapturedContent })[]>([]);
  const [loadingSources, setLoadingSources] = useState(true);
  const [deleteConfirm, setDeleteConfirm] = useState(false);
  const [trail, setTrail] = useState<HealthTrailResult | null>(null);
  const [trailLoading, setTrailLoading] = useState(true);
  const [isRead, setIsRead] = useState(false);
  const [readLoading, setReadLoading] = useState(true);
  const [monitorConfigOpen, setMonitorConfigOpen] = useState(false);
  const [linkedExams, setLinkedExams] = useState<Exam[]>([]);
  const IconComponent = TYPE_ICONS[page.page_type] || BookOpen;

  useEffect(() => {
    loadSources();
    loadTrail();
    loadReadStatus();
    loadLinkedExams();
  }, [page.id]);

  async function loadSources() {
    setLoadingSources(true);
    try {
      const pageSources = await getPageSources(page.id);
      const enriched = await Promise.all(
        pageSources.map(async (src) => {
          try {
            const content = await invoke<CapturedContent | null>("get_contents_by_ids", {
              ids: [src.content_id],
            });
            return { ...src, content: Array.isArray(content) ? content[0] : undefined };
          } catch {
            return { ...src, content: undefined };
          }
        })
      );
      setSources(enriched);
    } catch (e) {
      console.error("Failed to load sources:", e);
    }
    setLoadingSources(false);
  }

  async function loadTrail() {
    setTrailLoading(true);
    try {
      const result = await getWikiLearningTrail(page.id);
      setTrail(result);
    } catch (e) {
      // Silently fail — don't show errors for trail
      console.warn("Failed to load learning trail:", e);
      setTrail(null);
    }
    setTrailLoading(false);
  }

  async function loadReadStatus() {
    try {
      const status = await getWikiReadStatus(page.id);
      setIsRead(status);
    } catch { setIsRead(false); }
    setReadLoading(false);
  }

  async function loadLinkedExams() {
    try {
      const exams = await getWikiExamHistory(page.id);
      setLinkedExams(exams);
    } catch { setLinkedExams([]); }
  }

  const toggleRead = async () => {
    const next = !isRead;
    setIsRead(next);
    try {
      await setWikiReadStatus(page.id, next);
      if (next) {
        try { await createReviewSchedule(page.id); }
        catch { /* schedule may already exist */ }
      }
      window.dispatchEvent(new CustomEvent("wiki-read-status-changed", {
        detail: { wikiPageId: page.id, isRead: next },
      }));
    } catch { setIsRead(!next); }
  };

  const isStale = page.status === "needs_recompile";

  return (
    <div className="fixed inset-0 z-50 flex items-start justify-center pt-10 pb-10">
      {/* Backdrop */}
      <div className="absolute inset-0 bg-black/30 dark:bg-black/50" onClick={onClose} />

      {/* Panel */}
      <div
        className="relative w-full max-w-2xl max-h-[85vh] overflow-y-auto rounded-2xl shadow-2xl"
        style={{
          backgroundColor: "var(--color-surface, #FFFFFF)",
          border: "1px solid var(--color-border, #E7E5E4)",
        }}
      >
        {/* Header */}
        <div className="sticky top-0 z-10 flex items-center justify-between px-6 py-4 border-b"
          style={{ borderColor: "var(--color-border, #E7E5E4)", backgroundColor: "var(--color-surface, #FFFFFF)" }}
        >
          <div className="flex items-center gap-2">
            <IconComponent size={18} style={{ color: "#F97316" }} />
            <span className="text-[11px] font-semibold px-2 py-0.5 rounded"
              style={{ color: "#F97316", backgroundColor: "#F9731615" }}
            >
              {t(TYPE_LABEL_KEYS[page.page_type]) || page.page_type}
            </span>
            {isStale && (
              <span className="text-[11px] font-medium px-2 py-0.5 rounded bg-amber-50 dark:bg-amber-500/10 text-amber-600 dark:text-amber-400">
                {t("detail.staleWarning")}
              </span>
            )}
            {!readLoading && (
              <button
                onClick={toggleRead}
                className={`inline-flex items-center gap-1 px-2 py-0.5 rounded text-[11px] font-medium transition-colors ${
                  isRead
                    ? "text-orange-500 bg-orange-50 dark:bg-orange-500/10"
                    : "text-gray-400 bg-gray-50 dark:bg-white/[0.04] hover:text-orange-500"
                }`}
              >
                {isRead ? "📖 已阅读" : "📖 标记已读"}
              </button>
            )}
          </div>
          <div className="flex items-center gap-1">
            {deleteConfirm ? (
              <div className="flex items-center gap-1">
                <button
                  onClick={() => { onDelete(page.id); setDeleteConfirm(false); }}
                  className="px-2 py-1 rounded-md text-[11px] font-medium text-white bg-red-500 hover:bg-red-600 transition-colors"
                >
                  {t("detail.confirmDelete")}
                </button>
                <button
                  onClick={() => setDeleteConfirm(false)}
                  className="px-2 py-1 rounded-md text-[11px] text-stone-400 hover:text-stone-600 transition-colors"
                >
                  {t("detail.cancel")}
                </button>
              </div>
            ) : (
              <>
              <button
                onClick={() => setMonitorConfigOpen(true)}
                className="p-1.5 rounded-lg hover:bg-orange-50 dark:hover:bg-orange-500/10 text-stone-400 hover:text-orange-500 transition-colors"
                title="知识发现设置"
              >
                <Search size={16} />
              </button>
              <button
                onClick={() => setDeleteConfirm(true)}
                className="p-1.5 rounded-lg hover:bg-red-50 dark:hover:bg-red-500/10 text-stone-400 hover:text-red-500 transition-colors"
                title={t("detail.deleteTooltip")}
              >
                <Trash2 size={16} />
              </button>
              </>
            )}
            <button
              onClick={onClose}
              className="p-1.5 rounded-lg hover:bg-stone-100 dark:hover:bg-white/[0.08] text-stone-400 transition-colors"
            >
              <X size={16} />
            </button>
          </div>
        </div>

        {/* Body */}
        <div className="px-6 py-5">
          {/* Title */}
          <h1
            className="font-bold mb-2"
            style={{ fontSize: 22, fontFamily: "'Cabinet Grotesk', sans-serif", color: "var(--color-text-primary, #1C1917)" }}
          >
            {page.title}
          </h1>

          {/* Summary */}
          {page.summary && (
            <p className="mb-4" style={{ fontSize: 14, color: "var(--color-text-secondary, #57534E)" }}>
              {page.summary}
            </p>
          )}

          {/* Markdown content */}
          <article
            className="prose prose-sm prose-stone dark:prose-invert max-w-none mb-6
                       prose-headings:font-bold prose-headings:text-stone-800 dark:prose-headings:text-stone-200
                       prose-p:text-stone-600 dark:prose-p:text-stone-300
                       prose-a:text-orange-500 prose-a:no-underline hover:prose-a:underline
                       prose-strong:text-stone-700 dark:prose-strong:text-stone-200
                       prose-code:text-orange-600 dark:prose-code:text-orange-400
                       prose-code:bg-orange-50 dark:prose-code:bg-orange-500/10
                       prose-code:px-1 prose-code:py-0.5 prose-code:rounded
                       prose-code:before:content-none prose-code:after:content-none"
            style={{ fontSize: 14, lineHeight: 1.8 }}
          >
            <ReactMarkdown remarkPlugins={[remarkGfm]}>
              {page.body_markdown}
            </ReactMarkdown>
          </article>

          {/* E-4-4: Learning Trail Section */}
          <div className="border-t pt-4 mb-4" style={{ borderColor: "var(--color-border, #E7E5E4)" }}>
            <h3 className="flex items-center gap-1.5 mb-3" style={{ fontSize: 13, fontWeight: 600, color: "var(--color-text-primary)" }}>
              <RotateCcw size={14} className="text-orange-500" />
              学习轨迹
            </h3>

            {trailLoading ? (
              <div className="flex items-center gap-2 py-3" style={{ fontSize: 12, color: "var(--color-text-muted)" }}>
                <Loader2 size={14} className="animate-spin" />
                加载学习轨迹...
              </div>
            ) : trail ? (
              <div className="space-y-4">
                {/* Block A: Mastery overview */}
                {trail.schedule ? (
                  <div className="space-y-2">
                    <div>
                      <div className="flex items-center justify-between mb-1">
                        <span style={{ fontSize: 11, color: "var(--color-text-muted)" }}>🎯 掌握度</span>
                        <span style={{ fontSize: 12, fontWeight: 600, fontFamily: "'JetBrains Mono', monospace", color: getMasteryColor(trail.schedule.mastery) }}>
                          {Math.round(trail.schedule.mastery * 100)}%
                        </span>
                      </div>
                      <div className="rounded-full overflow-hidden" style={{ height: 6, backgroundColor: "var(--color-border)" }}>
                        <div className="h-full rounded-full" style={{ width: `${Math.min(Math.round(trail.schedule.mastery * 100), 100)}%`, backgroundColor: getMasteryColor(trail.schedule.mastery) }} />
                      </div>
                    </div>
                    <div style={{ fontSize: 12, color: "var(--color-text-secondary)" }}>
                      🔄 已复习 {trail.schedule.review_count} 次
                      {trail.schedule.last_reviewed_at && (
                        <span style={{ color: "var(--color-text-muted)", marginLeft: 8 }}>
                          📅 上次: {getRelativeTime(trail.schedule.last_reviewed_at)}
                        </span>
                      )}
                    </div>
                    <div style={{ fontSize: 12, color: "var(--color-text-secondary)" }}>
                      下次复习: <span style={{ fontWeight: 600, color: trail.is_due ? "#ef4444" : getMasteryColor(trail.schedule.mastery) }}>
                        {getRelativeTime(trail.schedule.next_review_at)}
                      </span>
                    </div>
                  </div>
                ) : (
                  <p style={{ fontSize: 12, color: "var(--color-text-muted)" }}>暂无复习数据</p>
                )}

                {/* Block B: Exam stats */}
                {trail.exam_stats ? (
                  <div style={{ fontSize: 12, color: "var(--color-text-secondary)" }}>
                    📝 考试表现: 共 {trail.exam_stats.total} 题 ·
                    答对 <span style={{ fontWeight: 600, color: "#10b981" }}>{trail.exam_stats.correct}</span> ·
                    答错 <span style={{ fontWeight: 600, color: "#ef4444" }}>{trail.exam_stats.wrong}</span>
                    {trail.exam_stats.total > 0 && (
                      <span> (正确率 {Math.round((trail.exam_stats.correct / trail.exam_stats.total) * 100)}%)</span>
                    )}
                  </div>
                ) : (
                  <p style={{ fontSize: 12, color: "var(--color-text-muted)" }}>暂无考试数据</p>
                )}

                {/* Block C: Recent review logs */}
                <div>
                  <div className="flex items-center gap-1.5 mb-2" style={{ fontSize: 12, color: "var(--color-text-secondary)" }}>
                    📋 最近记录
                  </div>
                  {trail.recent_logs.length > 0 ? (
                    <div className="space-y-2">
                      {trail.recent_logs.slice(0, 10).map((log) => {
                        const isCorrect = log.quality >= 1;
                        const snapshot = parseSnapshot(log.question_snapshot);
                        return (
                          <div
                            key={log.id}
                            className="rounded px-2 py-1.5"
                            style={{ fontSize: 11, backgroundColor: "var(--color-bg)", borderLeft: `3px solid ${isCorrect ? "#16A34A" : "#DC2626"}` }}
                          >
                            <div className="flex items-center justify-between mb-1">
                              <span style={{ color: "var(--color-text-muted)" }}>{getRelativeTime(log.reviewed_at)}</span>
                              <span style={{ color: isCorrect ? "#16A34A" : "#DC2626", fontWeight: 500 }}>
                                {isCorrect ? "正确" : "错误"}
                              </span>
                            </div>
                            {snapshot.length > 0 ? (
                              <div className="space-y-1 mt-1.5">
                                {snapshot.map((q: any, i: number) => (
                                  <div key={i} className="text-xs" style={{ color: "var(--color-text-secondary)" }}>
                                    <span style={{ color: "var(--color-text-muted)" }}>{i + 1}.</span>{" "}
                                    {q.stem}{" "}
                                    <span style={{
                                      color: q.selected_index === q.correct_index ? "#16A34A" : "#DC2626",
                                      fontWeight: 500,
                                    }}>
                                      {q.selected_index === q.correct_index ? "✓" : "✗"}
                                    </span>
                                  </div>
                                ))}
                              </div>
                            ) : (
                              <span style={{ color: "var(--color-text-secondary)" }}>
                                {log.review_format ? (REVIEW_FORMAT_LABELS[log.review_format] ?? log.review_format) : "复习记录"}
                              </span>
                            )}
                          </div>
                        );
                      })}
                    </div>
                  ) : (
                    <p style={{ fontSize: 12, color: "var(--color-text-muted)" }}>暂无复习记录</p>
                  )}
                </div>

                {/* Block D: Linked goals */}
                <div>
                  <div className="flex items-center gap-1.5 mb-2" style={{ fontSize: 12, color: "var(--color-text-secondary)" }}>
                    🎯 关联目标
                  </div>
                  {trail.linked_goals && trail.linked_goals.length > 0 ? (
                    <div className="space-y-1">
                      {trail.linked_goals.map((g) => (
                        <button
                          key={g.goal_id}
                          onClick={() => {
                            onClose();
                            onNavigateToGoal?.(g.goal_id);
                          }}
                          className="w-full text-left px-2 py-1 rounded text-sm hover:bg-orange-50 dark:hover:bg-orange-950/20 transition-colors"
                          style={{ fontSize: 12, color: "var(--color-text-secondary)" }}
                        >
                          {g.goal_title}
                        </button>
                      ))}
                    </div>
                  ) : (
                    <p style={{ fontSize: 12, color: "var(--color-text-muted)" }}>暂无关联目标</p>
                  )}
                </div>

                {/* Block E: Linked exams */}
                <div>
                  <div className="flex items-center gap-1.5 mb-2" style={{ fontSize: 12, color: "var(--color-text-secondary)" }}>
                    📝 关联考试
                  </div>
                  {linkedExams.length > 0 ? (
                    <div className="space-y-1">
                      {linkedExams.map((exam) => (
                        <div key={exam.id} className="flex items-center justify-between" style={{ fontSize: 12 }}>
                          <span style={{ color: "var(--color-text-secondary)" }}>
                            {exam.title || "考试"} v{exam.version}
                          </span>
                          <span style={{ color: exam.grade === "A" ? "#16A34A" : exam.grade === "B" ? "#F97316" : "var(--color-text-muted)" }}>
                            {exam.score != null ? `${Math.round(exam.score)}分 ${exam.grade}` : "未评分"}
                          </span>
                        </div>
                      ))}
                    </div>
                  ) : (
                    <p style={{ fontSize: 12, color: "var(--color-text-muted)" }}>暂无关联考试</p>
                  )}
                </div>
              </div>
            ) : (
              <p style={{ fontSize: 12, color: "var(--color-text-muted)" }}>学习轨迹数据不可用</p>
            )}
          </div>

          {/* Sources section */}
          <div className="border-t pt-4" style={{ borderColor: "var(--color-border, #E7E5E4)" }}>
            <h3 className="flex items-center gap-1.5 mb-3" style={{ fontSize: 13, fontWeight: 600, color: "var(--color-text-primary)" }}>
              <span className="w-1 h-1 rounded-full" style={{ backgroundColor: "#F97316" }} />
              {t("detail.compiledFrom")}
            </h3>

            {loadingSources ? (
              <div className="text-xs" style={{ color: "var(--color-text-muted)" }}>{t("detail.loading")}</div>
            ) : sources.length === 0 ? (
              <div className="text-xs" style={{ color: "var(--color-text-muted)" }}>{t("detail.noSources")}</div>
            ) : (
              <div className="space-y-2">
                {sources.map((src) => (
                  <button
                    key={src.id}
                    onClick={() => src.content && onNavigateToContent?.(src.content_id)}
                    className="w-full text-left flex items-center gap-3 p-3 rounded-lg transition-colors hover:bg-stone-50 dark:hover:bg-white/[0.04]"
                    style={{ border: "1px solid var(--color-border, #E7E5E4)" }}
                  >
                    <span style={{ color: SOURCE_STATUS_COLOR[src.source_status], fontSize: 14, fontWeight: 700 }}>
                      {SOURCE_STATUS_ICON[src.source_status]}
                    </span>
                    <div className="flex-1 min-w-0">
                      <p className="text-xs truncate" style={{ color: "var(--color-text-primary)" }}>
                        {src.content?.raw_text?.slice(0, 80) || src.content?.source_url || t("detail.contentDeleted")}
                      </p>
                      <p className="text-[10px] mt-0.5" style={{ color: "var(--color-text-muted)" }}>
                        {src.content?.source_app || t("detail.unknownApp")} · {src.contributed_at?.slice(0, 10)}
                      </p>
                    </div>
                  </button>
                ))}
              </div>
            )}
          </div>

          {/* Confidence footer */}
          <div className="mt-4 pt-3 flex items-center justify-between border-t" style={{ borderColor: "var(--color-border)" }}>
            <div className="flex items-center gap-2">
              <span style={{ fontSize: 11, color: "var(--color-text-muted)" }}>{t("detail.confidence")}</span>
              <div className="w-20 h-1.5 rounded-full" style={{ backgroundColor: "var(--color-border)" }}>
                <div
                  className="h-1.5 rounded-full"
                  style={{
                    width: `${page.confidence * 100}%`,
                    backgroundColor: page.confidence >= 0.8 ? "#16A34A" : page.confidence >= 0.5 ? "#CA8A04" : "#DC2626",
                  }}
                />
              </div>
              <span style={{ fontSize: 11, fontFamily: "'JetBrains Mono', monospace", color: "var(--color-text-muted)" }}>
                {Math.round(page.confidence * 100)}%
              </span>
            </div>
            <span style={{ fontSize: 11, color: "var(--color-text-muted)" }}>
              {page.last_compiled_at ? t("detail.compiledAt", { date: page.last_compiled_at.slice(0, 10) }) : t("detail.notCompiled")} · {t("detail.sourceCount", { count: sources.length })}
            </span>
          </div>
        </div>
      </div>

      {/* Knowledge Discovery Monitor Config */}
      {monitorConfigOpen && (
        <MonitorConfig
          pageId={page.id}
          pageTitle={page.title}
          config={{
            monitor_enabled: page.monitor_enabled ?? false,
            monitor_query: page.monitor_query ?? "",
            monitor_sources: parseMonitorSources(page.monitor_sources),
            last_discovered_at: page.last_discovered_at ?? null,
            pending_count: page.pending_count ?? 0,
          }}
          onSave={() => {
            setMonitorConfigOpen(false);
          }}
          onSearchNow={async () => {
            try {
              await invoke("run_discovery_for_page", { pageId: page.id });
            } catch (err) {
              console.error("Discovery failed:", err);
            }
          }}
          onClose={() => setMonitorConfigOpen(false)}
        />
      )}

    </div>
  );
}

/** Parse the monitor_sources JSON string into a string array */
function parseMonitorSources(raw: string | undefined | null): string[] {
  if (!raw || raw === "[]" || raw === "null" || raw === "") {
    return [];
  }
  try {
    return JSON.parse(raw);
  } catch {
    return [];
  }
}
