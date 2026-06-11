import { BookOpen, ExternalLink, Star, Trash2, Download } from "lucide-react";
import type { PendingContent } from "../../types/learning";

interface PendingContentCardProps {
  item: PendingContent;
  selected?: boolean;
  onToggleSelect?: (id: string) => void;
  onRead: (item: PendingContent) => void;
  onImport: (item: PendingContent) => void;
  onDismiss: (item: PendingContent) => void;
}

function formatDate(dateStr: string): string {
  try {
    const d = new Date(dateStr);
    return d.toLocaleDateString("zh-CN", {
      year: "numeric",
      month: "2-digit",
      day: "2-digit",
    });
  } catch {
    return dateStr?.slice(0, 10) ?? "";
  }
}

export default function PendingContentCard({
  item,
  selected,
  onToggleSelect,
  onRead,
  onImport,
  onDismiss,
}: PendingContentCardProps) {
  const isUnread = item.status === "unread";
  const statusLabel: Record<string, string> = {
    unread: "🆕 未读",
    reading: "📖 阅读中",
    imported: "✅ 已导入",
    dismissed: "🗑️ 已忽略",
  };

  return (
    <div
      className="rounded-xl p-4 transition-all hover:shadow-sm"
      style={{
        backgroundColor: "var(--color-surface)",
        border: selected
          ? "1px solid #F97316"
          : "1px solid var(--color-border)",
        opacity: item.status === "dismissed" ? 0.55 : 1,
      }}
    >
      {/* Top row: checkbox + title + status badge */}
      <div className="flex items-start gap-3 mb-2">
        {onToggleSelect && (
          <input
            type="checkbox"
            checked={!!selected}
            onChange={() => onToggleSelect(item.id)}
            className="mt-0.5 accent-orange-500"
            style={{ width: 16, height: 16 }}
          />
        )}
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2 mb-0.5">
            {isUnread && (
              <span
                className="flex-shrink-0 inline-flex items-center justify-center rounded-full"
                style={{
                  width: 8,
                  height: 8,
                  backgroundColor: "#F97316",
                }}
              />
            )}
            <h4
              style={{
                fontSize: 14,
                fontWeight: 600,
                color: "var(--color-text-primary)",
                lineHeight: 1.3,
              }}
              className="truncate"
            >
              {item.title}
            </h4>
          </div>

          {/* Source info */}
          <div
            className="flex items-center gap-2 flex-wrap"
            style={{ fontSize: 11, color: "var(--color-text-muted)" }}
          >
            {item.source_page_title && (
              <span className="flex items-center gap-1">
                <BookOpen size={11} />
                {item.source_page_title}
              </span>
            )}
            {item.source_name && (
              <>
                <span>·</span>
                <span>{item.source_name}</span>
              </>
            )}
            <span>·</span>
            <span>{formatDate(item.discovered_at)}</span>
          </div>
        </div>

        {/* Status badge */}
        <span
          className="flex-shrink-0 text-[10px] font-medium px-2 py-0.5 rounded"
          style={{
            backgroundColor:
              item.status === "unread"
                ? "rgba(249, 115, 22, 0.08)"
                : item.status === "reading"
                  ? "rgba(59, 130, 246, 0.08)"
                  : item.status === "imported"
                    ? "rgba(16, 185, 129, 0.08)"
                    : "rgba(107, 114, 128, 0.08)",
            color:
              item.status === "unread"
                ? "#F97316"
                : item.status === "reading"
                  ? "#3B82F6"
                  : item.status === "imported"
                    ? "#10B981"
                    : "#6B7280",
          }}
        >
          {statusLabel[item.status] ?? item.status}
        </span>
      </div>

      {/* AI Summary */}
      {item.content_summary && (
        <p
          style={{
            fontSize: 12,
            color: "var(--color-text-secondary)",
            lineHeight: 1.5,
            marginBottom: 8,
          }}
          className="line-clamp-2"
        >
          {item.content_summary}
        </p>
      )}

      {/* Match info bottom row */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          {/* Relevance stars */}
          {item.relevance_score > 0 && (
            <div className="flex items-center gap-1">
              <Star size={11} className="text-amber-500" />
              <span style={{ fontSize: 10, fontFamily: "'JetBrains Mono', monospace", color: "var(--color-text-muted)" }}>
                {Math.round(item.relevance_score * 20)}%
              </span>
            </div>
          )}

          {/* Match keywords */}
          {item.match_keywords && item.match_keywords !== "[]" && (
            <div className="flex items-center gap-1">
              <span style={{ fontSize: 10, color: "var(--color-text-muted)" }}>·</span>
              {(() => {
                try {
                  const keywords: string[] = JSON.parse(item.match_keywords!);
                  return keywords.slice(0, 3).map((kw, i) => (
                    <span
                      key={i}
                      className="inline-block px-1.5 py-0.5 rounded"
                      style={{
                        fontSize: 9,
                        backgroundColor: "rgba(249, 115, 22, 0.06)",
                        color: "#F97316",
                      }}
                    >
                      {kw}
                    </span>
                  ));
                } catch {
                  return null;
                }
              })()}
            </div>
          )}
        </div>

        {/* Action buttons */}
        {item.status !== "dismissed" && (
          <div className="flex items-center gap-1.5">
            <button
              onClick={() => onRead(item)}
              className="inline-flex items-center gap-1 px-2.5 py-1 rounded-md text-[10px] font-medium transition-all hover:bg-orange-100 dark:hover:bg-orange-900/30"
              style={{
                color: "#F97316",
                backgroundColor: "rgba(249, 115, 22, 0.08)",
              }}
              title="阅读"
            >
              <ExternalLink size={11} />
              阅读
            </button>
            <button
              onClick={() => onImport(item)}
              className="inline-flex items-center gap-1 px-2.5 py-1 rounded-md text-[10px] font-medium transition-all hover:bg-emerald-100 dark:hover:bg-emerald-900/30"
              style={{
                color: "#10B981",
                backgroundColor: "rgba(16, 185, 129, 0.08)",
              }}
              title="导入为 wiki 页面"
            >
              <Download size={11} />
              导入
            </button>
            <button
              onClick={() => onDismiss(item)}
              className="inline-flex items-center gap-1 px-2.5 py-1 rounded-md text-[10px] font-medium transition-all hover:bg-red-100 dark:hover:bg-red-900/30"
              style={{
                color: "#EF4444",
                backgroundColor: "rgba(239, 68, 68, 0.06)",
              }}
              title="忽略"
            >
              <Trash2 size={11} />
            </button>
          </div>
        )}
      </div>
    </div>
  );
}
