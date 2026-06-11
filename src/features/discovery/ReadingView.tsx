import { useState, useCallback } from "react";
import {
  ArrowLeft,
  ExternalLink,
  Check,
  Trash2,
  Download,
  Loader2,
  Star,
  BookOpen,
  Globe,
  Calendar,
  CheckCheck,
} from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import type { PendingContent } from "../../types/learning";

interface ReadingViewProps {
  item: PendingContent;
  onBack: () => void;
  onImport: () => void;
  onDismiss: () => void;
}

function formatDate(dateStr: string): string {
  try {
    const d = new Date(dateStr);
    return d.toLocaleDateString("zh-CN", {
      year: "numeric",
      month: "2-digit",
      day: "2-digit",
      hour: "2-digit",
      minute: "2-digit",
    });
  } catch {
    return dateStr?.slice(0, 10) ?? "";
  }
}

export default function ReadingView({
  item,
  onBack,
  onImport,
  onDismiss,
}: ReadingViewProps) {
  const [actionLoading, setActionLoading] = useState<string | null>(null);
  const [actionError, setActionError] = useState<string | null>(null);
  const [importSuccess, setImportSuccess] = useState(false);

  const handleMarkRead = useCallback(async () => {
    setActionLoading("read");
    setActionError(null);
    try {
      await invoke("update_pending_status", {
        id: item.id,
        status: "read",
      });
      onBack();
    } catch (err) {
      setActionError((err as Error).message);
    } finally {
      setActionLoading(null);
    }
  }, [item.id, onBack]);

  const handleDismiss = useCallback(async () => {
    setActionLoading("dismiss");
    setActionError(null);
    try {
      // Update status to dismissed
      await invoke("update_pending_status", {
        id: item.id,
        status: "dismissed",
      });

      // Record feedback if source_page_id exists
      if (item.source_page_id) {
        try {
          await invoke("record_dismissal_feedback", {
            sourcePageId: item.source_page_id,
          });
        } catch {
          // Non-critical: dismissal recording is best-effort
        }
      }

      onDismiss();
    } catch (err) {
      setActionError((err as Error).message);
    } finally {
      setActionLoading(null);
    }
  }, [item.id, item.source_page_id, onDismiss]);

  const handleImport = useCallback(async () => {
    if (importSuccess) return;

    setActionLoading("import");
    setActionError(null);
    try {
      // Step 1: Mark as importing
      await invoke("update_pending_status", {
        id: item.id,
        status: "importing",
      });

      // Step 2: Save to captured content via save_content_auto
      // The save_captured_content / save_content_auto expects a CaptureEvent.
      // We use the internal function by invoking save_captured_content.
      const contentToImport = item.full_content ?? item.content_summary ?? item.title;

      await invoke("save_captured_content", {
        event: {
          content_type: "text",
          preview: item.title.slice(0, 100),
          source_app: item.source_name ?? "web",
          raw_text: contentToImport,
          image_path: null,
        },
      });

      // Step 3: Mark as imported with the content ID
      await invoke("update_pending_status", {
        id: item.id,
        status: "imported",
      });

      setImportSuccess(true);
      onImport();
    } catch (err) {
      setActionError((err as Error).message);
      // Reset status to unread on failure
      try {
        await invoke("update_pending_status", {
          id: item.id,
          status: "unread",
        });
      } catch {
        // ignore
      }
    } finally {
      setActionLoading(null);
    }
  }, [item, importSuccess, onImport]);

  const handleOpenOriginal = useCallback(() => {
    if (item.source_url) {
      window.open(item.source_url, "_blank");
    }
  }, [item.source_url]);

  // Parse match keywords for display
  const matchKeywords: string[] = (() => {
    if (!item.match_keywords) return [];
    try {
      return JSON.parse(item.match_keywords);
    } catch {
      return item.match_keywords.split(",").map((k) => k.trim());
    }
  })();

  return (
    <div>
      {/* Header with back button */}
      <div className="flex items-center justify-between mb-4">
        <button
          onClick={onBack}
          className="inline-flex items-center justify-center w-8 h-8 rounded-lg transition-all hover:bg-gray-100 dark:hover:bg-gray-800"
          style={{ color: "var(--color-text-muted)" }}
          title="返回收件箱"
        >
          <ArrowLeft size={18} />
        </button>
        <span
          className="text-[11px] font-medium px-2 py-1 rounded"
          style={{
            backgroundColor:
              item.status === "unread"
                ? "rgba(249, 115, 22, 0.08)"
                : item.status === "importing"
                  ? "rgba(59, 130, 246, 0.08)"
                  : item.status === "imported"
                    ? "rgba(16, 185, 129, 0.08)"
                    : "rgba(107, 114, 128, 0.08)",
            color:
              item.status === "unread"
                ? "#F97316"
                : item.status === "importing"
                  ? "#3B82F6"
                  : item.status === "imported"
                    ? "#10B981"
                    : "#6B7280",
          }}
        >
          {item.status === "unread"
            ? "🆕 未读"
            : item.status === "importing"
              ? "⏳ 导入中"
              : item.status === "imported"
                ? "✅ 已导入"
                : "🗑️ 已忽略"}
        </span>
      </div>

      {/* Main content card */}
      <div
        className="rounded-xl overflow-hidden"
        style={{
          backgroundColor: "var(--color-surface)",
          border: "1px solid var(--color-border)",
        }}
      >
        {/* Article header */}
        <div className="p-5 pb-4 border-b" style={{ borderColor: "var(--color-border)" }}>
          <h1
            style={{
              fontSize: 20,
              fontWeight: 700,
              color: "var(--color-text-primary)",
              lineHeight: 1.4,
              fontFamily: "'Cabinet Grotesk', sans-serif",
              letterSpacing: "-0.3px",
              marginBottom: 12,
            }}
          >
            {item.title}
          </h1>

          {/* Meta info row */}
          <div className="flex flex-wrap items-center gap-x-4 gap-y-1.5">
            {item.source_page_title && (
              <div className="flex items-center gap-1.5" style={{ fontSize: 12, color: "var(--color-text-muted)" }}>
                <BookOpen size={13} className="text-orange-500" />
                <span>{item.source_page_title}</span>
              </div>
            )}
            {item.source_name && (
              <div className="flex items-center gap-1.5" style={{ fontSize: 12, color: "var(--color-text-muted)" }}>
                <Globe size={13} />
                <span>{item.source_name}</span>
              </div>
            )}
            <div className="flex items-center gap-1.5" style={{ fontSize: 12, color: "var(--color-text-muted)" }}>
              <Calendar size={13} />
              <span>{formatDate(item.discovered_at)}</span>
            </div>
            {item.relevance_score > 0 && (
              <div className="flex items-center gap-1">
                <Star size={12} className="text-amber-500" />
                <span style={{ fontSize: 11, fontFamily: "'JetBrains Mono', monospace", color: "var(--color-text-muted)" }}>
                  {Math.round(item.relevance_score * 20)}%
                </span>
              </div>
            )}
          </div>

          {/* Keywords */}
          {matchKeywords.length > 0 && (
            <div className="flex items-center gap-1.5 mt-3 flex-wrap">
              {matchKeywords.slice(0, 5).map((kw, i) => (
                <span
                  key={i}
                  className="inline-block px-2 py-0.5 rounded text-[10px] font-medium"
                  style={{
                    backgroundColor: "rgba(249, 115, 22, 0.06)",
                    color: "#F97316",
                  }}
                >
                  {kw}
                </span>
              ))}
            </div>
          )}

          {/* Match reason */}
          {item.match_reason && (
            <p
              className="mt-3"
              style={{ fontSize: 11, color: "var(--color-text-muted)", fontStyle: "italic", lineHeight: 1.4 }}
            >
              {item.match_reason}
            </p>
          )}
        </div>

        {/* Content body */}
        <div className="p-5">
          <div
            style={{
              fontSize: 14,
              color: "var(--color-text-primary)",
              lineHeight: 1.7,
              whiteSpace: "pre-wrap",
              wordBreak: "break-word",
            }}
          >
            {item.full_content || item.content_summary || (
              <span style={{ color: "var(--color-text-muted)", fontStyle: "italic" }}>
                暂无详细内容
              </span>
            )}
          </div>
        </div>
      </div>

      {/* Error message */}
      {actionError && (
        <div
          className="rounded-xl p-3 mt-4"
          style={{
            fontSize: 13,
            backgroundColor: "rgba(239, 68, 68, 0.08)",
            border: "1px solid rgba(239, 68, 68, 0.2)",
            color: "var(--color-text-secondary)",
          }}
        >
          <p className="text-red-700 dark:text-red-400">{actionError}</p>
        </div>
      )}

      {/* Import success message */}
      {importSuccess && (
        <div
          className="rounded-xl p-3 mt-4 flex items-center gap-2"
          style={{
            fontSize: 13,
            backgroundColor: "rgba(16, 185, 129, 0.08)",
            border: "1px solid rgba(16, 185, 129, 0.2)",
            color: "#10B981",
          }}
        >
          <CheckCheck size={16} />
          <span>已成功导入知识库！</span>
        </div>
      )}

      {/* Action buttons */}
      <div className="flex items-center gap-2 mt-4 flex-wrap">
        {item.status !== "imported" && item.status !== "dismissed" && (
          <>
            <button
              onClick={handleImport}
              disabled={actionLoading === "import" || importSuccess}
              className="inline-flex items-center gap-1.5 px-4 py-2 rounded-lg text-[12px] font-medium transition-all hover:bg-emerald-100 dark:hover:bg-emerald-900/30 disabled:opacity-50"
              style={{
                color: "#10B981",
                backgroundColor: importSuccess
                  ? "rgba(16, 185, 129, 0.15)"
                  : "rgba(16, 185, 129, 0.08)",
              }}
            >
              {actionLoading === "import" ? (
                <Loader2 size={14} className="animate-spin" />
              ) : (
                <Download size={14} />
              )}
              {importSuccess ? "已导入" : "📥 导入知识库"}
            </button>

            <button
              onClick={handleMarkRead}
              disabled={actionLoading === "read"}
              className="inline-flex items-center gap-1.5 px-4 py-2 rounded-lg text-[12px] font-medium transition-all hover:bg-blue-100 dark:hover:bg-blue-900/30 disabled:opacity-50"
              style={{
                color: "#3B82F6",
                backgroundColor: "rgba(59, 130, 246, 0.08)",
              }}
            >
              {actionLoading === "read" ? (
                <Loader2 size={14} className="animate-spin" />
              ) : (
                <Check size={14} />
              )}
              ✅ 标记已读
            </button>

            <button
              onClick={handleDismiss}
              disabled={actionLoading === "dismiss"}
              className="inline-flex items-center gap-1.5 px-4 py-2 rounded-lg text-[12px] font-medium transition-all hover:bg-red-100 dark:hover:bg-red-900/30 disabled:opacity-50"
              style={{
                color: "#EF4444",
                backgroundColor: "rgba(239, 68, 68, 0.06)",
              }}
            >
              {actionLoading === "dismiss" ? (
                <Loader2 size={14} className="animate-spin" />
              ) : (
                <Trash2 size={14} />
              )}
              🗑️ 不感兴趣
            </button>
          </>
        )}

        {item.source_url && (
          <button
            onClick={handleOpenOriginal}
            className="inline-flex items-center gap-1.5 px-4 py-2 rounded-lg text-[12px] font-medium transition-all hover:bg-gray-100 dark:hover:bg-gray-800"
            style={{
              color: "var(--color-text-secondary)",
              backgroundColor: "var(--color-surface)",
              border: "1px solid var(--color-border)",
            }}
          >
            <ExternalLink size={14} />
            🔗 打开原文
          </button>
        )}
      </div>
    </div>
  );
}
