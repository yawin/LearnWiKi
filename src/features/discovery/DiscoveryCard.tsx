import { useEffect, useState, useCallback } from "react";
import { Inbox, ExternalLink, Loader2 } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import type { PendingContent } from "../../types/learning";

interface DiscoveryCardProps {
  onOpenInbox: () => void;
}

export default function DiscoveryCard({ onOpenInbox }: DiscoveryCardProps) {
  const [unreadItems, setUnreadItems] = useState<PendingContent[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [unreadCount, setUnreadCount] = useState(0);

  const fetchUnread = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await invoke<PendingContent[]>("get_pending_content", {
        statusFilter: "unread",
        limit: 100,
      });
      setUnreadItems(data.slice(0, 3));
      setUnreadCount(data.length);
    } catch (err) {
      setError((err as Error).message);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchUnread();
  }, [fetchUnread]);

  if (loading) {
    return (
      <div
        className="rounded-xl p-4 mb-6"
        style={{
          backgroundColor: "var(--color-surface)",
          border: "1px solid var(--color-border)",
        }}
      >
        <div className="flex items-center gap-2">
          <Loader2 size={14} className="animate-spin text-orange-500" />
          <span style={{ fontSize: 12, color: "var(--color-text-muted)" }}>
            加载知识发现...
          </span>
        </div>
      </div>
    );
  }

  return (
    <div
      className="rounded-xl p-4 mb-6 transition-all hover:shadow-sm"
      style={{
        backgroundColor: "var(--color-surface)",
        border: `1px solid ${
          unreadCount > 0
            ? "rgba(249, 115, 22, 0.3)"
            : "var(--color-border)"
        }`,
      }}
    >
      {/* Header */}
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-2">
          <Inbox size={16} className="text-orange-500" />
          <h3
            style={{
              fontSize: 13,
              fontWeight: 600,
              color: "var(--color-text-primary)",
            }}
          >
            知识发现
          </h3>
          {unreadCount > 0 && (
            <span
              className="inline-flex items-center justify-center rounded-full text-[10px] font-medium px-2 py-0.5"
              style={{
                backgroundColor: "rgba(249, 115, 22, 0.1)",
                color: "#F97316",
              }}
            >
              {unreadCount} 篇新内容待阅读
            </span>
          )}
        </div>
      </div>

      {/* Error */}
      {error && (
        <div
          className="rounded-lg p-2 mb-3"
          style={{
            fontSize: 12,
            backgroundColor: "rgba(239, 68, 68, 0.08)",
            color: "var(--color-text-secondary)",
          }}
        >
          <span className="text-red-600 dark:text-red-400">{error}</span>
        </div>
      )}

      {/* Empty state */}
      {!error && unreadItems.length === 0 && (
        <div className="text-center py-4">
          <Inbox
            size={28}
            className="mx-auto mb-2 text-orange-300"
            strokeWidth={1.5}
          />
          <p style={{ fontSize: 12, color: "var(--color-text-muted)" }}>
            暂无新发现内容
          </p>
          <p
            style={{
              fontSize: 11,
              color: "var(--color-text-muted)",
              marginTop: 2,
            }}
          >
            设置监控源后，相关内容会自动出现在这里
          </p>
        </div>
      )}

      {/* Recent unread items */}
      {unreadItems.length > 0 && (
        <>
          <p
            style={{
              fontSize: 11,
              color: "var(--color-text-muted)",
              marginBottom: 8,
            }}
          >
            今天发现的相关内容:
          </p>
          <div className="space-y-2 mb-3">
            {unreadItems.map((item) => (
              <div
                key={item.id}
                className="flex items-start gap-2 rounded-lg px-3 py-2"
                style={{
                  backgroundColor: "rgba(249, 115, 22, 0.03)",
                  border: "1px solid rgba(249, 115, 22, 0.08)",
                }}
              >
                <span style={{ fontSize: 11, flexShrink: 0 }}>🆕</span>
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
                    {item.title}
                  </p>
                  <div
                    className="flex items-center gap-1"
                    style={{ fontSize: 10, color: "var(--color-text-muted)" }}
                  >
                    <span>
                      匹配:{" "}
                      {"⭐".repeat(Math.min(Math.round(item.relevance_score), 5))}
                    </span>
                    <span>·</span>
                    <span>
                      {item.source_page_title ?? "未知来源"}
                    </span>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </>
      )}

      {/* Action buttons */}
      <div className="flex items-center gap-2">
        <button
          onClick={onOpenInbox}
          className="inline-flex items-center gap-1 px-3 py-1.5 rounded-lg text-[11px] font-medium transition-all"
          style={{
            backgroundColor: "rgba(249, 115, 22, 0.1)",
            color: "#F97316",
          }}
        >
          <ExternalLink size={12} />
          去阅读
        </button>
        <button
          onClick={() => {
            // Navigate to wiki settings — emit custom event or just open wiki
            window.dispatchEvent(
              new CustomEvent("navigate-to-discovery-settings")
            );
          }}
          className="inline-flex items-center gap-1 px-3 py-1.5 rounded-lg text-[11px] font-medium transition-all"
          style={{
            color: "var(--color-text-muted)",
            backgroundColor: "transparent",
            border: "1px solid var(--color-border)",
          }}
        >
          管理监控页面
        </button>
      </div>
    </div>
  );
}
