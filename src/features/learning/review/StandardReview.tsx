import { useTranslation } from "react-i18next";
import {
  X,
  ChevronLeft,
  Brain,
  AlertCircle,
  MessageSquareText,
  Shuffle,
} from "lucide-react";
import type { DueReviewItem } from "../../../types/learning";

interface StandardReviewProps {
  current: DueReviewItem;
  currentReviewIndex: number;
  total: number;
  nextFormat: string;
  isVariantMode: boolean;
  variantData: {
    format: string;
    question_data: any;
    variant_generation: number;
    twist_description: string;
  } | null;
  showFeedback: boolean;
  lastQuality: number | null;
  reviewError: string | null;
  reviewStarted: boolean;
  onQuality: (quality: number) => void;
  onStartReview: () => void;
  onClose: () => void;
  loading: boolean;
}

export function StandardReview({
  current,
  currentReviewIndex,
  total,
  nextFormat,
  isVariantMode,
  variantData,
  showFeedback,
  lastQuality,
  reviewError,
  reviewStarted,
  onQuality,
  onStartReview,
  onClose,
  loading,
}: StandardReviewProps) {
  const { t } = useTranslation("learning");

  const progressPercent = total > 0 ? ((currentReviewIndex) / total) * 100 : 0;

  // Variant badge
  const variantBadge =
    isVariantMode && variantData ? (
      <div
        className="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-[10px] font-medium mb-2"
        style={{
          backgroundColor: "rgba(139, 92, 246, 0.08)",
          color: "#8B5CF6",
        }}
      >
        <Shuffle size={12} />
        变体题 ·{" "}
        {t("reviewSession.variantBadge", {
          generation: variantData.variant_generation,
        })}
      </div>
    ) : null;

  // Format indicator
  const formatLabel =
    nextFormat === "judgment"
      ? { icon: <AlertCircle size={12} />, text: "判断题" }
      : nextFormat === "essay"
        ? { icon: <MessageSquareText size={12} />, text: "论述题" }
        : { icon: <Brain size={12} />, text: "选择题" };

  return (
    <div
      className="fixed inset-0 z-50 flex flex-col"
      style={{
        backgroundColor: "var(--color-bg)",
      }}
    >
      {/* Top bar */}
      <div className="flex items-center justify-between px-4 py-3">
        <button
          onClick={onClose}
          className="inline-flex items-center gap-1 text-sm"
          style={{ color: "var(--color-text-muted)" }}
        >
          <ChevronLeft size={16} />
          {t("reviewSession.back")}
        </button>
        <div className="flex items-center gap-2">
          <span
            style={{
              fontSize: 12,
              fontFamily: "'JetBrains Mono', monospace",
              color: "var(--color-text-muted)",
            }}
          >
            {currentReviewIndex + 1} / {total}
          </span>
          <button
            onClick={onClose}
            className="p-1 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
          >
            <X size={16} style={{ color: "var(--color-text-muted)" }} />
          </button>
        </div>
      </div>

      {/* Progress bar */}
      <div className="w-full" style={{ height: 3, backgroundColor: "var(--color-border)" }}>
        <div
          className="h-full transition-all duration-300"
          style={{
            width: `${progressPercent}%`,
            background: "linear-gradient(90deg, #F97316, #FB923C)",
          }}
        />
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto px-6 py-6">
        <div className="max-w-lg mx-auto">
          {/* Format indicator */}
          <div
            className="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-[10px] font-medium mb-4"
            style={{
              backgroundColor: "rgba(249, 115, 22, 0.08)",
              color: "#F97316",
            }}
          >
            {formatLabel.icon}
            {formatLabel.text}
          </div>

          {/* Variant badge */}
          {variantBadge}

          {/* Twist description in variant mode */}
          {isVariantMode && variantData?.twist_description && (
            <div
              className="rounded-xl p-3 mb-4"
              style={{
                backgroundColor: "rgba(139, 92, 246, 0.06)",
                border: "1px solid rgba(139, 92, 246, 0.12)",
              }}
            >
              <p
                style={{
                  fontSize: 12,
                  color: "var(--color-text-secondary)",
                  lineHeight: 1.6,
                  fontStyle: "italic",
                }}
              >
                🔀 {variantData.twist_description}
              </p>
            </div>
          )}

          {/* Title */}
          <h2
            style={{
              fontSize: 20,
              fontWeight: 700,
              fontFamily: "'Cabinet Grotesk', sans-serif",
              color: "var(--color-text-primary)",
              lineHeight: 1.4,
              marginBottom: 12,
            }}
          >
            {current.wiki_title}
          </h2>

          {/* Summary / body as the "question" */}
          {current.wiki_summary && (
            <div
              className="rounded-xl p-4 mb-6"
              style={{
                backgroundColor: "var(--color-surface)",
                border: "1px solid var(--color-border)",
              }}
            >
              <p
                style={{
                  fontSize: 14,
                  color: "var(--color-text-secondary)",
                  lineHeight: 1.7,
                  whiteSpace: "pre-wrap",
                }}
              >
                {current.wiki_summary}
              </p>
            </div>
          )}

          {/* Tags */}
          {current.wiki_tags && current.wiki_tags !== "[]" && (
            <div className="flex flex-wrap gap-1.5 mb-6">
              {(() => {
                try {
                  const tags: string[] = JSON.parse(current.wiki_tags!);
                  return tags.map((tag, i) => (
                    <span
                      key={i}
                      className="inline-block px-2 py-0.5 rounded"
                      style={{
                        fontSize: 10,
                        backgroundColor: "rgba(249, 115, 22, 0.06)",
                        color: "#F97316",
                      }}
                    >
                      {tag}
                    </span>
                  ));
                } catch {
                  return null;
                }
              })()}
            </div>
          )}

          {/* Action prompt based on format */}
          {nextFormat === "judgment" && (
            <div
              className="rounded-xl p-4 mb-4"
              style={{
                backgroundColor: "rgba(249, 115, 22, 0.06)",
                border: "1px solid rgba(249, 115, 22, 0.12)",
              }}
            >
              <p style={{ fontSize: 13, color: "var(--color-text-secondary)", lineHeight: 1.6 }}>
                阅读上述知识点，判断自己对内容的掌握程度
              </p>
            </div>
          )}

          {nextFormat === "choice" && (
            <p style={{ fontSize: 12, color: "var(--color-text-muted)", marginBottom: 16 }}>
              阅读上述知识点，回顾并评估自己的掌握程度
            </p>
          )}

          {/* Feedback after answering */}
          {showFeedback && lastQuality !== null && (
            <div
              className="rounded-xl p-3 mb-4 text-center"
              style={{
                backgroundColor:
                  lastQuality >= 1
                    ? "rgba(16, 185, 129, 0.08)"
                    : "rgba(239, 68, 68, 0.08)",
                border: `1px solid ${
                  lastQuality >= 1
                    ? "rgba(16, 185, 129, 0.2)"
                    : "rgba(239, 68, 68, 0.2)"
                }`,
              }}
            >
              <span
                style={{
                  fontSize: 14,
                  fontWeight: 600,
                  color: "var(--color-text-primary)",
                }}
              >
                {lastQuality >= 1
                  ? t("reviewSession.correctAnswer")
                  : t("reviewSession.needReview")}
              </span>
            </div>
          )}

          {/* Error */}
          {reviewError && (
            <div
              className="rounded-lg p-2 mb-3"
              style={{
                fontSize: 12,
                backgroundColor: "rgba(239, 68, 68, 0.08)",
                color: "var(--color-text-secondary)",
              }}
            >
              <span className="text-red-600 dark:text-red-400">{reviewError}</span>
            </div>
          )}
        </div>
      </div>

      {/* Bottom bar — different buttons based on format */}
      <div
        className="px-4 py-4 border-t"
        style={{
          backgroundColor: "var(--color-surface)",
          borderColor: "var(--color-border)",
        }}
      >
        <div className="max-w-lg mx-auto">
          {nextFormat === "essay" ? (
            // Essay routes to ExplainReview
            <button
              onClick={onStartReview}
              disabled={loading}
              className="w-full inline-flex items-center justify-center gap-2 px-6 py-3 rounded-xl text-sm font-medium transition-all disabled:opacity-50"
              style={{
                backgroundColor: "#F97316",
                color: "white",
              }}
              onMouseEnter={(e) => {
                e.currentTarget.style.backgroundColor = "#EA580C";
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.backgroundColor = "#F97316";
              }}
            >
              {loading ? (
                "加载中..."
              ) : (
                <>
                  <MessageSquareText size={16} /> 开始论述
                </>
              )}
            </button>
          ) : !reviewStarted ? (
            <button
              onClick={onStartReview}
              disabled={loading}
              className="w-full inline-flex items-center justify-center gap-2 px-6 py-3 rounded-xl text-sm font-medium transition-all disabled:opacity-50"
              style={{
                backgroundColor: "#F97316",
                color: "white",
              }}
              onMouseEnter={(e) => {
                e.currentTarget.style.backgroundColor = "#EA580C";
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.backgroundColor = "#F97316";
              }}
            >
              {loading ? "加载中..." : nextFormat === "judgment" ? (
                <>
                  <AlertCircle size={16} /> 开始判断题
                </>
              ) : (
                <>
                  <Brain size={16} /> 开始选择题
                </>
              )}
            </button>
          ) : (
            <div className="grid grid-cols-3 gap-3">
              <button
                onClick={() => onQuality(0)}
                disabled={showFeedback}
                className="inline-flex flex-col items-center gap-1 py-3 rounded-xl text-sm font-medium transition-all disabled:opacity-40"
                style={{
                  backgroundColor: showFeedback
                    ? "var(--color-border)"
                    : "rgba(239, 68, 68, 0.08)",
                  color: showFeedback ? "var(--color-text-muted)" : "#EF4444",
                }}
              >
                <span style={{ fontSize: 20 }}>🔴</span>
                <span>{t("reviewSession.forgot")}</span>
              </button>
              <button
                onClick={() => onQuality(1)}
                disabled={showFeedback}
                className="inline-flex flex-col items-center gap-1 py-3 rounded-xl text-sm font-medium transition-all disabled:opacity-40"
                style={{
                  backgroundColor: showFeedback
                    ? "var(--color-border)"
                    : "rgba(245, 158, 11, 0.08)",
                  color: showFeedback ? "var(--color-text-muted)" : "#F59E0B",
                }}
              >
                <span style={{ fontSize: 20 }}>🟡</span>
                <span>{t("reviewSession.remember")}</span>
              </button>
              <button
                onClick={() => onQuality(2)}
                disabled={showFeedback}
                className="inline-flex flex-col items-center gap-1 py-3 rounded-xl text-sm font-medium transition-all disabled:opacity-40"
                style={{
                  backgroundColor: showFeedback
                    ? "var(--color-border)"
                    : "rgba(16, 185, 129, 0.08)",
                  color: showFeedback ? "var(--color-text-muted)" : "#10B981",
                }}
              >
                <span style={{ fontSize: 20 }}>🟢</span>
                <span>{t("reviewSession.easy")}</span>
              </button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
