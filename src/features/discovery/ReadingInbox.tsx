import { useEffect, useState, useCallback } from "react";
import {
  Inbox,
  ArrowLeft,
  RefreshCw,
  CheckCheck,
  Download,
  Loader2,
} from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import type { PendingContent } from "../../types/learning";
import PendingContentCard from "./PendingContentCard";
import ReadingView from "./ReadingView";

interface ReadingInboxProps {
  onBack: () => void;
}

type FilterTab = "all" | "unread" | "reading" | "imported";

const FILTER_TABS: { key: FilterTab; label: string }[] = [
  { key: "all", label: "全部" },
  { key: "unread", label: "🆕 未读" },
  { key: "reading", label: "📖 阅读中" },
  { key: "imported", label: "✅ 已导入" },
];

export default function ReadingInbox({ onBack }: ReadingInboxProps) {
  const [items, setItems] = useState<PendingContent[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<FilterTab>("unread");
  const [selectedIds, setSelectedIds] = useState<Set<string>>(new Set());
  const [batchActionLoading, setBatchActionLoading] = useState(false);
  const [selectedItem, setSelectedItem] = useState<PendingContent | null>(null);

  const fetchItems = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const statusFilter = activeTab === "all" ? undefined : activeTab;
      const data = await invoke<PendingContent[]>("get_pending_content", {
        statusFilter: statusFilter ?? null,
        limit: 200,
      });
      setItems(data);
    } catch (err) {
      setError((err as Error).message);
    } finally {
      setLoading(false);
    }
  }, [activeTab]);

  useEffect(() => {
    fetchItems();
  }, [fetchItems]);

  // Reset selection when items change
  useEffect(() => {
    setSelectedIds(new Set());
  }, [items]);

  const handleToggleSelect = (id: string) => {
    setSelectedIds((prev) => {
      const next = new Set(prev);
      if (next.has(id)) {
        next.delete(id);
      } else {
        next.add(id);
      }
      return next;
    });
  };

  const handleSelectAll = () => {
    if (selectedIds.size === items.length) {
      setSelectedIds(new Set());
    } else {
      setSelectedIds(new Set(items.map((i) => i.id)));
    }
  };

  const handleRead = async (item: PendingContent) => {
    try {
      await invoke("update_pending_status", {
        id: item.id,
        status: "reading",
      });
      // Update local state
      setItems((prev) =>
        prev.map((i) =>
          i.id === item.id ? { ...i, status: "reading" } : i
        )
      );
      // Open reading view
      setSelectedItem(item);
    } catch (err) {
      console.error("Failed to mark as reading:", err);
    }
  };

  const handleImport = async (item: PendingContent) => {
    try {
      // Mark as imported — in a full implementation this would also
      // create a wiki page from the content
      await invoke("update_pending_status", {
        id: item.id,
        status: "imported",
      });
      setItems((prev) =>
        prev.map((i) =>
          i.id === item.id ? { ...i, status: "imported" } : i
        )
      );
    } catch (err) {
      console.error("Failed to mark as imported:", err);
    }
  };

  const handleDismiss = async (item: PendingContent) => {
    try {
      await invoke("update_pending_status", {
        id: item.id,
        status: "dismissed",
      });
      setItems((prev) => prev.filter((i) => i.id !== item.id));
    } catch (err) {
      console.error("Failed to dismiss:", err);
    }
  };

  const handleBatchMarkRead = async () => {
    if (selectedIds.size === 0) return;
    setBatchActionLoading(true);
    try {
      for (const id of selectedIds) {
        await invoke("update_pending_status", {
          id,
          status: "reading",
        });
      }
      setItems((prev) =>
        prev.map((i) =>
          selectedIds.has(i.id) ? { ...i, status: "reading" } : i
        )
      );
      setSelectedIds(new Set());
    } catch (err) {
      console.error("Batch mark read failed:", err);
    } finally {
      setBatchActionLoading(false);
    }
  };

  const handleBatchImport = async () => {
    if (selectedIds.size === 0) return;
    setBatchActionLoading(true);
    try {
      for (const id of selectedIds) {
        await invoke("update_pending_status", {
          id,
          status: "imported",
        });
      }
      setItems((prev) =>
        prev.map((i) =>
          selectedIds.has(i.id) ? { ...i, status: "imported" } : i
        )
      );
      setSelectedIds(new Set());
    } catch (err) {
      console.error("Batch import failed:", err);
    } finally {
      setBatchActionLoading(false);
    }
  };

  // Handle returning from reading view — refresh the list
  const handleReadingViewBack = useCallback(() => {
    setSelectedItem(null);
    fetchItems();
  }, [fetchItems]);

  // Handle import from reading view
  const handleReadingViewImport = useCallback(() => {
    setSelectedItem(null);
    fetchItems();
  }, [fetchItems]);

  // Handle dismiss from reading view
  const handleReadingViewDismiss = useCallback(() => {
    setSelectedItem(null);
    fetchItems();
  }, [fetchItems]);

  // Compute counts for each tab
  const counts = {
    all: items.length,
    unread: items.filter((i) => i.status === "unread").length,
    reading: items.filter((i) => i.status === "reading").length,
    imported: items.filter((i) => i.status === "imported").length,
  };

  // Filter items based on active tab (server-side filtered, but client-side for computed tab counts)
  const filteredItems = activeTab === "all" ? items : items.filter((i) => i.status === activeTab);

  return (
    <div>
      {/* Reading View Mode */}
      {selectedItem ? (
        <ReadingView
          item={selectedItem}
          onBack={handleReadingViewBack}
          onImport={handleReadingViewImport}
          onDismiss={handleReadingViewDismiss}
        />
      ) : (
        <div>
      {/* Header */}
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-3">
          <button
            onClick={onBack}
            className="inline-flex items-center justify-center w-8 h-8 rounded-lg transition-all hover:bg-gray-100 dark:hover:bg-gray-800"
            style={{ color: "var(--color-text-muted)" }}
            title="返回"
          >
            <ArrowLeft size={18} />
          </button>
          <div className="flex items-center gap-2">
            <Inbox size={20} className="text-orange-500" />
            <h2
              style={{
                fontSize: 20,
                fontFamily: "'Cabinet Grotesk', sans-serif",
                fontWeight: 700,
                color: "var(--color-text-primary)",
                letterSpacing: "-0.3px",
              }}
            >
              阅读收件箱
            </h2>
          </div>
        </div>
        <button
          onClick={fetchItems}
          disabled={loading}
          className="inline-flex items-center gap-1 px-3 py-1.5 rounded-lg text-[11px] font-medium transition-all"
          style={{
            backgroundColor: "rgba(249, 115, 22, 0.1)",
            color: loading ? "var(--color-text-muted)" : "#F97316",
          }}
        >
          <RefreshCw size={13} className={loading ? "animate-spin" : ""} />
          刷新
        </button>
      </div>

      {/* Filter tabs */}
      <div className="flex items-center gap-1 mb-4 flex-wrap">
        {FILTER_TABS.map((tab) => (
          <button
            key={tab.key}
            onClick={() => setActiveTab(tab.key)}
            className={`inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-[12px] font-medium transition-all ${
              activeTab === tab.key
                ? "bg-orange-500 text-white shadow-sm"
                : "text-gray-500 dark:text-slate-400 hover:text-gray-700 dark:hover:text-slate-300"
            }`}
          >
            {tab.label}
            <span
              className="inline-flex items-center justify-center rounded-full text-[10px] font-medium px-1.5"
              style={{
                backgroundColor:
                  activeTab === tab.key
                    ? "rgba(255,255,255,0.2)"
                    : "var(--color-border)",
                color:
                  activeTab === tab.key
                    ? "rgba(255,255,255,0.9)"
                    : "var(--color-text-muted)",
                minWidth: 18,
                height: 16,
              }}
            >
              {counts[tab.key]}
            </span>
          </button>
        ))}
      </div>

      {/* Error */}
      {error && (
        <div
          className="rounded-xl p-3 mb-4"
          style={{
            fontSize: 13,
            backgroundColor: "rgba(239, 68, 68, 0.08)",
            border: "1px solid rgba(239, 68, 68, 0.2)",
            color: "var(--color-text-secondary)",
          }}
        >
          <p className="text-red-700 dark:text-red-400">{error}</p>
        </div>
      )}

      {/* Loading */}
      {loading && (
        <div className="flex items-center gap-2 mb-4 p-4 rounded-xl" style={{ backgroundColor: "var(--color-surface)", border: "1px solid var(--color-border)" }}>
          <Loader2 size={16} className="animate-spin text-orange-500" />
          <span style={{ fontSize: 13, color: "var(--color-text-muted)" }}>加载收件箱...</span>
        </div>
      )}

      {/* Batch operations bar */}
      {!loading && selectedIds.size > 0 && (
        <div
          className="rounded-xl p-3 mb-4 flex items-center gap-3"
          style={{
            backgroundColor: "rgba(249, 115, 22, 0.06)",
            border: "1px solid rgba(249, 115, 22, 0.2)",
          }}
        >
          <span style={{ fontSize: 12, color: "var(--color-text-primary)", fontWeight: 500 }}>
            已选 {selectedIds.size} 项
          </span>
          <div className="flex items-center gap-2">
            <button
              onClick={handleBatchMarkRead}
              disabled={batchActionLoading}
              className="inline-flex items-center gap-1 px-3 py-1.5 rounded-lg text-[11px] font-medium transition-all hover:bg-blue-100 dark:hover:bg-blue-900/30"
              style={{
                color: "#3B82F6",
                backgroundColor: "rgba(59, 130, 246, 0.08)",
              }}
            >
              {batchActionLoading ? (
                <Loader2 size={12} className="animate-spin" />
              ) : (
                <CheckCheck size={12} />
              )}
              全部标记已读
            </button>
            <button
              onClick={handleBatchImport}
              disabled={batchActionLoading}
              className="inline-flex items-center gap-1 px-3 py-1.5 rounded-lg text-[11px] font-medium transition-all hover:bg-emerald-100 dark:hover:bg-emerald-900/30"
              style={{
                color: "#10B981",
                backgroundColor: "rgba(16, 185, 129, 0.08)",
              }}
            >
              {batchActionLoading ? (
                <Loader2 size={12} className="animate-spin" />
              ) : (
                <Download size={12} />
              )}
              批量导入
            </button>
          </div>
        </div>
      )}

      {/* Select all toggle */}
      {!loading && filteredItems.length > 0 && (
        <div className="flex items-center gap-2 mb-2 px-1">
          <input
            type="checkbox"
            checked={selectedIds.size === filteredItems.length && filteredItems.length > 0}
            onChange={handleSelectAll}
            className="accent-orange-500"
            style={{ width: 15, height: 15 }}
          />
          <span style={{ fontSize: 11, color: "var(--color-text-muted)" }}>
            全选
          </span>
        </div>
      )}

      {/* Empty state */}
      {!loading && !error && filteredItems.length === 0 && (
        <div className="flex flex-col items-center justify-center py-12 text-center">
          <div
            className="flex items-center justify-center w-16 h-16 rounded-2xl mb-4"
            style={{ backgroundColor: "rgba(249, 115, 22, 0.06)" }}
          >
            <Inbox size={30} className="text-orange-300" strokeWidth={1.5} />
          </div>
          <h4
            style={{
              fontSize: 15,
              fontWeight: 600,
              color: "var(--color-text-primary)",
              marginBottom: 4,
            }}
          >
            收件箱空空如也
          </h4>
          <p
            style={{
              fontSize: 12,
              color: "var(--color-text-muted)",
              lineHeight: 1.6,
            }}
          >
            {activeTab === "unread"
              ? "所有内容已阅读，继续保持！"
              : activeTab === "reading"
                ? "没有正在阅读的内容"
                : activeTab === "imported"
                  ? "还没有已导入的内容"
                  : "知识发现会自动从监控源收集相关内容"}
          </p>
        </div>
      )}

      {/* Items list */}
      {!loading && filteredItems.length > 0 && (
        <div className="space-y-3">
          {filteredItems.map((item) => (
            <PendingContentCard
              key={item.id}
              item={item}
              selected={selectedIds.has(item.id)}
              onToggleSelect={handleToggleSelect}
              onRead={handleRead}
              onImport={handleImport}
              onDismiss={handleDismiss}
            />
          ))}
        </div>
      )}
      </div>
      )}
    </div>
  );
}
