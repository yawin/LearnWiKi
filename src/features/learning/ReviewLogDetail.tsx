import { useState, useEffect } from "react";
import { ChevronLeft, FileText } from "lucide-react";
import { getReviewLog } from "../../services/learningService";
import type { ReviewLogDetail as ReviewLogDetailType } from "../../types/learning";
import { REVIEW_FORMAT_LABELS, QUALITY_LABELS } from "../../types/learning";

interface ReviewLogDetailProps {
  logId: string;
  onBack: () => void;
}

export function ReviewLogDetail({ logId, onBack }: ReviewLogDetailProps) {
  const [log, setLog] = useState<ReviewLogDetailType | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const load = async () => {
      try {
        const data = await getReviewLog(logId);
        setLog(data);
      } catch (e) {
        console.error("Failed to load review log:", e);
      } finally {
        setLoading(false);
      }
    };
    load();
  }, [logId]);

  if (loading) {
    return (
      <div className="flex items-center justify-center" style={{ height: "calc(100vh - 44px)" }}>
        <div className="animate-pulse" style={{ color: "var(--color-text-muted)" }}>加载中...</div>
      </div>
    );
  }

  if (!log) {
    return (
      <div className="flex flex-col items-center justify-center gap-4" style={{ height: "calc(100vh - 44px)" }}>
        <p style={{ color: "var(--color-text-muted)" }}>未找到复习记录</p>
        <button onClick={onBack} className="text-sm text-orange-500">返回</button>
      </div>
    );
  }

  const formatDate = (isoStr: string): string => {
    const d = new Date(isoStr);
    return `${d.getFullYear()}/${d.getMonth() + 1}/${d.getDate()} ${String(d.getHours()).padStart(2, "0")}:${String(d.getMinutes()).padStart(2, "0")}`;
  };

  return (
    <div className="overflow-y-auto px-5 pt-4 pb-8" style={{ height: "calc(100vh - 44px)", color: "var(--color-text-primary)" }}>
      {/* Back button */}
      <button
        onClick={onBack}
        className="inline-flex items-center gap-1 mb-4 text-sm hover:text-orange-500 transition-colors"
        style={{ color: "var(--color-text-muted)" }}
      >
        <ChevronLeft size={16} />
        返回目标
      </button>

      {/* Header */}
      <div className="flex items-start gap-3 mb-6">
        <div className="p-2 rounded-lg" style={{ backgroundColor: "#FFF7ED" }}>
          <FileText size={20} className="text-orange-500" />
        </div>
        <div>
          <h2
            style={{
              fontSize: 22,
              fontFamily: "'Cabinet Grotesk', sans-serif",
              fontWeight: 700,
              color: "var(--color-text-primary)",
              letterSpacing: "-0.3px",
            }}
          >
            复习记录
          </h2>
          <p style={{ fontSize: 13, color: "var(--color-text-secondary)", marginTop: 4 }}>
            {log.wiki_title}
          </p>
        </div>
      </div>

      {/* Wiki content */}
      {(log.wiki_summary || log.wiki_tags) && (
        <div
          className="rounded-xl p-4 mb-4"
          style={{ backgroundColor: "var(--color-surface)", border: "1px solid var(--color-border)" }}
        >
          <span style={{ fontSize: 12, fontWeight: 600, color: "var(--color-text-muted)", display: "block", marginBottom: 8 }}>
            复习内容
          </span>
          {log.wiki_summary && (
            <p style={{ fontSize: 13, color: "var(--color-text-secondary)", lineHeight: 1.7, whiteSpace: "pre-wrap" }}>
              {log.wiki_summary}
            </p>
          )}
          {log.wiki_tags && log.wiki_tags !== "[]" && (
            <div className="flex flex-wrap gap-1.5 mt-3">
              {(() => {
                try {
                  const tags: string[] = JSON.parse(log.wiki_tags);
                  return tags.map((tag, i) => (
                    <span key={i} className="px-2 py-0.5 rounded text-xs"
                      style={{ backgroundColor: "rgba(249, 115, 22, 0.06)", color: "#F97316" }}>
                      {tag}
                    </span>
                  ));
                } catch { return null; }
              })()}
            </div>
          )}
        </div>
      )}

      {/* Details card */}
      <div
        className="rounded-xl p-5 space-y-4"
        style={{ backgroundColor: "var(--color-surface)", border: "1px solid var(--color-border)" }}
      >
        <div className="flex justify-between items-center">
          <span style={{ fontSize: 12, color: "var(--color-text-muted)" }}>复习时间</span>
          <span style={{ fontSize: 13, color: "var(--color-text-primary)" }}>{formatDate(log.reviewed_at)}</span>
        </div>

        <div className="flex justify-between items-center">
          <span style={{ fontSize: 12, color: "var(--color-text-muted)" }}>题型</span>
          <span style={{ fontSize: 13, color: "var(--color-text-primary)" }}>
            {log.review_format ? (REVIEW_FORMAT_LABELS[log.review_format] ?? log.review_format) : "—"}
          </span>
        </div>

        <div className="flex justify-between items-center">
          <span style={{ fontSize: 12, color: "var(--color-text-muted)" }}>结果</span>
          <span
            className="px-2 py-0.5 rounded text-xs font-medium"
            style={{
              backgroundColor: log.quality === 2 ? "#DCFCE7" : log.quality === 1 ? "#FEF3C7" : "#FEE2E2",
              color: log.quality === 2 ? "#166534" : log.quality === 1 ? "#92400E" : "#991B1B",
            }}
          >
            {QUALITY_LABELS[log.quality] ?? log.quality}
          </span>
        </div>

        {log.response_time_seconds != null && (
          <div className="flex justify-between items-center">
            <span style={{ fontSize: 12, color: "var(--color-text-muted)" }}>用时</span>
            <span style={{ fontSize: 13, color: "var(--color-text-primary)" }}>{log.response_time_seconds} 秒</span>
          </div>
        )}

        <div
          className="my-1"
          style={{ borderTop: "1px solid var(--color-border)" }}
        />

        <div className="flex justify-between items-center">
          <span style={{ fontSize: 12, color: "var(--color-text-muted)" }}>复习间隔</span>
          <span style={{ fontSize: 13, color: "var(--color-text-primary)" }}>
            {log.interval_before}天 → {log.interval_after}天
          </span>
        </div>

        <div className="flex justify-between items-center">
          <span style={{ fontSize: 12, color: "var(--color-text-muted)" }}>难度系数</span>
          <span style={{ fontSize: 13, color: "var(--color-text-primary)" }}>
            {log.ease_factor_before.toFixed(1)} → {log.ease_factor_after.toFixed(1)}
          </span>
        </div>
      </div>
    </div>
  );
}
