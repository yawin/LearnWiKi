import { useTranslation } from "react-i18next";
import { Brain, CheckCircle2, Clock } from "lucide-react";

interface ReviewSummaryProps {
  total: number;
  correctCount: number;
  elapsedSeconds: number;
  onClose: () => void;
}

export function ReviewSummary({
  total,
  correctCount,
  elapsedSeconds,
  onClose,
}: ReviewSummaryProps) {
  const { t } = useTranslation("learning");
  const minutes = Math.floor(elapsedSeconds / 60);
  const seconds = elapsedSeconds % 60;

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center"
      style={{
        backgroundColor: "var(--color-bg)",
      }}
    >
      <div className="text-center px-6 max-w-sm">
        <div
          className="flex items-center justify-center w-20 h-20 rounded-full mx-auto mb-4"
          style={{ backgroundColor: "rgba(16, 185, 129, 0.1)" }}
        >
          <CheckCircle2 size={40} className="text-emerald-500" strokeWidth={1.5} />
        </div>
        <h2
          style={{
            fontSize: 22,
            fontWeight: 700,
            fontFamily: "'Cabinet Grotesk', sans-serif",
            color: "var(--color-text-primary)",
            marginBottom: 4,
          }}
        >
          复习完成！🎉
        </h2>
        <p
          style={{
            fontSize: 13,
            color: "var(--color-text-muted)",
            marginBottom: 24,
          }}
        >
          {t("reviewSession.completeSubtitle")}
        </p>

        {/* Stats grid */}
        <div className="grid grid-cols-3 gap-3 mb-6">
          <div
            className="rounded-xl p-3 text-center"
            style={{
              backgroundColor: "var(--color-surface)",
              border: "1px solid var(--color-border)",
            }}
          >
            <Brain size={18} className="mx-auto mb-1 text-orange-500" />
            <div
              style={{
                fontSize: 18,
                fontWeight: 700,
                fontFamily: "'JetBrains Mono', monospace",
                color: "var(--color-text-primary)",
              }}
            >
              {total}
            </div>
            <div style={{ fontSize: 10, color: "var(--color-text-muted)" }}>
              {t("reviewSession.totalReviews")}
            </div>
          </div>
          <div
            className="rounded-xl p-3 text-center"
            style={{
              backgroundColor: "var(--color-surface)",
              border: "1px solid var(--color-border)",
            }}
          >
            <CheckCircle2 size={18} className="mx-auto mb-1 text-emerald-500" />
            <div
              style={{
                fontSize: 18,
                fontWeight: 700,
                fontFamily: "'JetBrains Mono', monospace",
                color: "var(--color-text-primary)",
              }}
            >
              {correctCount}
            </div>
            <div style={{ fontSize: 10, color: "var(--color-text-muted)" }}>
              {t("reviewSession.correct")}
            </div>
          </div>
          <div
            className="rounded-xl p-3 text-center"
            style={{
              backgroundColor: "var(--color-surface)",
              border: "1px solid var(--color-border)",
            }}
          >
            <Clock size={18} className="mx-auto mb-1 text-blue-500" />
            <div
              style={{
                fontSize: 18,
                fontWeight: 700,
                fontFamily: "'JetBrains Mono', monospace",
                color: "var(--color-text-primary)",
              }}
            >
              {minutes}:{seconds.toString().padStart(2, "0")}
            </div>
            <div style={{ fontSize: 10, color: "var(--color-text-muted)" }}>
              {t("reviewSession.timeSpent")}
            </div>
          </div>
        </div>

        <button
          onClick={onClose}
          className="inline-flex items-center gap-2 px-6 py-2.5 rounded-xl text-sm font-medium transition-all"
          style={{
            backgroundColor: "#F97316",
            color: "white",
          }}
        >
          {t("reviewSession.finishReview")}
        </button>
      </div>
    </div>
  );
}
