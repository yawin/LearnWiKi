import { useEffect } from "react";
import {
  BookOpen,
  CheckCircle2,
  Flame,
  Loader2,
  Play,
} from "lucide-react";
import { useLearningStore } from "../../stores/learningStore";

interface ReviewListProps {
  onStartReview: () => void;
}

export default function ReviewList({ onStartReview }: ReviewListProps) {
  const {
    dueReviews,
    reviewStats,
    reviewLoading,
    reviewError,
    fetchDueReviews,
    fetchReviewStats,
  } = useLearningStore();

  useEffect(() => {
    fetchDueReviews();
    fetchReviewStats();
  }, [fetchDueReviews, fetchReviewStats]);

  if (reviewLoading) {
    return (
      <div
        className="rounded-xl p-4 mb-6"
        style={{
          backgroundColor: "var(--color-surface)",
          border: "1px solid var(--color-border)",
        }}
      >
        <div className="flex items-center gap-2 mb-3">
          <BookOpen size={16} className="text-orange-500" />
          <h3
            style={{
              fontSize: 13,
              fontWeight: 600,
              color: "var(--color-text-primary)",
            }}
          >
            今日复习
          </h3>
        </div>
        <div className="flex items-center gap-2 py-2">
          <Loader2 size={14} className="animate-spin text-orange-500" />
          <span style={{ fontSize: 12, color: "var(--color-text-muted)" }}>
            加载复习列表...
          </span>
        </div>
      </div>
    );
  }

  const totalDue = reviewStats?.total_due ?? dueReviews.length;
  const completedToday = reviewStats?.total_reviewed_today ?? 0;
  const streak = reviewStats?.streak ?? 0;

  return (
    <div
      className="rounded-xl p-4 mb-6"
      style={{
        backgroundColor: "var(--color-surface)",
        border: "1px solid var(--color-border)",
      }}
    >
      {/* Header */}
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-2">
          <BookOpen size={16} className="text-orange-500" />
          <h3
            style={{
              fontSize: 13,
              fontWeight: 600,
              color: "var(--color-text-primary)",
            }}
          >
            每日打卡
          </h3>
        </div>
        {/* Mini stats */}
        <div className="flex items-center gap-3">
          <div className="flex items-center gap-1" style={{ fontSize: 11, color: "var(--color-text-muted)" }}>
            <Flame size={12} className="text-orange-500" />
            <span>{streak} 天</span>
          </div>
          <div className="flex items-center gap-1" style={{ fontSize: 11, color: "var(--color-text-muted)" }}>
            <CheckCircle2 size={12} className="text-emerald-500" />
            <span>今日 {completedToday}</span>
          </div>
        </div>
      </div>

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

      {/* Empty state: all done */}
      {dueReviews.length === 0 && !reviewError && (
        <div className="text-center py-4">
          <CheckCircle2
            size={32}
            className="mx-auto mb-2 text-emerald-500"
            strokeWidth={1.5}
          />
          <p style={{ fontSize: 13, fontWeight: 600, color: "var(--color-text-primary)" }}>
            今日已完成 ✅
          </p>
          <p
            style={{
              fontSize: 11,
              color: "var(--color-text-muted)",
              marginTop: 2,
            }}
          >
            太棒了！今天没有需要复习的内容。
          </p>
          {completedToday > 0 && (
            <p
              style={{
                fontSize: 11,
                color: "var(--color-text-muted)",
                marginTop: 4,
              }}
            >
              今天已复习 {completedToday} 个知识点
            </p>
          )}
        </div>
      )}

      {/* Due reviews list */}
      {dueReviews.length > 0 && (
        <>
          <div className="flex items-center justify-between mb-2">
            <span
              style={{
                fontSize: 11,
                color: "var(--color-text-muted)",
              }}
            >
              待复习 {totalDue} 个知识点
            </span>
            <button
              onClick={onStartReview}
              className="inline-flex items-center gap-1 px-3 py-1.5 rounded-lg text-[11px] font-medium transition-all"
              style={{
                backgroundColor: "rgba(249, 115, 22, 0.1)",
                color: "#F97316",
              }}
            >
              <Play size={12} />
              开始复习
            </button>
          </div>

          <div className="space-y-1.5 max-h-48 overflow-y-auto">
            {dueReviews.map((item, idx) => (
              <div
                key={item.schedule.id}
                className="flex items-center gap-3 rounded-lg px-3 py-2"
                style={{
                  backgroundColor: "rgba(249, 115, 22, 0.03)",
                  border: "1px solid rgba(249, 115, 22, 0.08)",
                }}
              >
                {/* Index */}
                <span
                  style={{
                    fontSize: 10,
                    fontFamily: "'JetBrains Mono', monospace",
                    color: "var(--color-text-muted)",
                    minWidth: 16,
                  }}
                >
                  {idx + 1}
                </span>

                {/* Title + tags */}
                <div className="flex-1 min-w-0">
                  <p
                    style={{
                      fontSize: 12,
                      fontWeight: 500,
                      color: "var(--color-text-primary)",
                      lineHeight: 1.3,
                    }}
                    className="truncate"
                  >
                    {item.wiki_title}
                  </p>
                  {item.wiki_tags && item.wiki_tags !== "[]" && (
                    <span
                      style={{
                        fontSize: 9,
                        color: "var(--color-text-muted)",
                      }}
                    >
                      {(() => {
                        try {
                          const tags: string[] = JSON.parse(item.wiki_tags!);
                          return tags.slice(0, 2).join(", ");
                        } catch {
                          return item.wiki_tags;
                        }
                      })()}
                    </span>
                  )}
                </div>

                {/* Mastery bar */}
                <div className="flex items-center gap-1.5">
                  <div
                    className="rounded-full overflow-hidden"
                    style={{
                      width: 40,
                      height: 4,
                      backgroundColor: "var(--color-border)",
                    }}
                  >
                    <div
                      className="h-full rounded-full"
                      style={{
                        width: `${Math.round(item.schedule.mastery * 100)}%`,
                        background:
                          item.schedule.mastery > 0.5
                            ? "linear-gradient(90deg, #10B981, #34D399)"
                            : item.schedule.mastery > 0.2
                              ? "linear-gradient(90deg, #F59E0B, #FBBF24)"
                              : "linear-gradient(90deg, #EF4444, #F87171)",
                      }}
                    />
                  </div>
                  <span
                    style={{
                      fontSize: 9,
                      fontFamily: "'JetBrains Mono', monospace",
                      color: "var(--color-text-muted)",
                      minWidth: 24,
                    }}
                  >
                    {Math.round(item.schedule.mastery * 100)}%
                  </span>
                </div>
              </div>
            ))}
          </div>
        </>
      )}
    </div>
  );
}
