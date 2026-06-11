import { useState, useCallback } from "react";
import { X, Search, Loader2, BookOpen, ExternalLink, Globe, GraduationCap, Rss } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import RssFeedConfig from "./RssFeedConfig";

interface MonitorConfigProps {
  pageId: string;
  pageTitle: string;
  config: {
    monitor_enabled: boolean;
    monitor_query: string;
    monitor_sources: string[];
    last_discovered_at: string | null;
    pending_count: number;
  };
  onSave: (config: MonitorConfigProps["config"]) => void;
  onSearchNow: () => void;
  onClose: () => void;
}

const SOURCE_OPTIONS = [
  { value: "web_search", label: "Web Search (网络搜索)", icon: Globe },
  { value: "arxiv", label: "ArXiv (学术论文)", icon: GraduationCap },
  { value: "rss", label: "RSS Feed (需先配置)", icon: Rss },
];

export default function MonitorConfig({
  pageId,
  pageTitle,
  config,
  onSave,
  onSearchNow,
  onClose,
}: MonitorConfigProps) {
  const [enabled, setEnabled] = useState(config.monitor_enabled);
  const [query, setQuery] = useState(config.monitor_query);
  const [sources, setSources] = useState<string[]>(config.monitor_sources);
  const [saving, setSaving] = useState(false);
  const [searching, setSearching] = useState(false);
  const [showRssConfig, setShowRssConfig] = useState(false);

  const handleToggleSource = (value: string) => {
    setSources((prev) =>
      prev.includes(value)
        ? prev.filter((s) => s !== value)
        : [...prev, value]
    );
  };

  const handleSave = useCallback(async () => {
    setSaving(true);
    try {
      // Save/update the monitor source settings
      const sourcesJson = JSON.stringify(sources);
      // Update the wiki page monitor fields via an invoke call
      await invoke("update_wiki_page_monitor", {
        pageId,
        monitorEnabled: enabled,
        monitorQuery: query || "",
        monitorSources: sourcesJson,
      });
      onSave({
        monitor_enabled: enabled,
        monitor_query: query,
        monitor_sources: sources,
        last_discovered_at: config.last_discovered_at,
        pending_count: config.pending_count,
      });
    } catch (err) {
      console.error("Failed to save monitor config:", err);
    } finally {
      setSaving(false);
    }
  }, [enabled, query, sources, pageId, onSave, config]);

  const handleSearchNow = useCallback(async () => {
    setSearching(true);
    try {
      await onSearchNow();
    } finally {
      setSearching(false);
    }
  }, [onSearchNow]);

  return (
    <div className="fixed inset-0 z-50 flex items-start justify-center pt-10 pb-10">
      {/* Backdrop */}
      <div className="absolute inset-0 bg-black/30 dark:bg-black/50" onClick={onClose} />

      {/* Panel */}
      <div
        className="relative w-full max-w-lg max-h-[85vh] overflow-y-auto rounded-2xl shadow-2xl"
        style={{
          backgroundColor: "var(--color-surface, #FFFFFF)",
          border: "1px solid var(--color-border, #E7E5E4)",
        }}
      >
        {/* Header */}
        <div
          className="sticky top-0 z-10 flex items-center justify-between px-6 py-4 border-b"
          style={{
            borderColor: "var(--color-border, #E7E5E4)",
            backgroundColor: "var(--color-surface, #FFFFFF)",
          }}
        >
          <div className="flex items-center gap-2">
            <Search size={18} style={{ color: "#F97316" }} />
            <span
              style={{
                fontSize: 15,
                fontWeight: 600,
                color: "var(--color-text-primary, #1C1917)",
              }}
            >
              知识监视器设置
            </span>
          </div>
          <button
            onClick={onClose}
            className="p-1.5 rounded-lg hover:bg-stone-100 dark:hover:bg-white/[0.08] text-stone-400 transition-colors"
          >
            <X size={16} />
          </button>
        </div>

        {/* Body */}
        <div className="px-6 py-5 space-y-5">
          {/* Wiki page info */}
          <div className="flex items-center gap-2">
            <BookOpen size={14} style={{ color: "var(--color-text-muted)" }} />
            <span
              style={{
                fontSize: 13,
                fontWeight: 500,
                color: "var(--color-text-secondary, #57534E)",
              }}
            >
              Wiki: {pageTitle}
            </span>
          </div>

          {/* Divider */}
          <div style={{ height: 1, backgroundColor: "var(--color-border, #E7E5E4)" }} />

          {/* Enable toggle */}
          <label className="flex items-center gap-3 cursor-pointer">
            <div
              onClick={() => setEnabled(!enabled)}
              className="relative inline-flex h-5 w-9 items-center rounded-full transition-colors cursor-pointer"
              style={{
                backgroundColor: enabled ? "#F97316" : "var(--color-border, #E7E5E4)",
              }}
            >
              <span
                className="inline-block h-3.5 w-3.5 transform rounded-full bg-white shadow-sm transition-transform"
                style={{ transform: enabled ? "translateX(18px)" : "translateX(2px)" }}
              />
            </div>
            <span
              style={{
                fontSize: 13,
                fontWeight: 500,
                color: "var(--color-text-primary, #1C1917)",
              }}
            >
              启用知识监视器
            </span>
          </label>

          {/* Search query */}
          <div>
            <label
              style={{
                fontSize: 12,
                fontWeight: 500,
                color: "var(--color-text-secondary, #57534E)",
                display: "block",
                marginBottom: 6,
              }}
            >
              搜索关键词
            </label>
            <input
              type="text"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="留空则自动从标题生成"
              style={{
                width: "100%",
                padding: "8px 12px",
                fontSize: 13,
                borderRadius: 10,
                border: "1px solid var(--color-border, #E7E5E4)",
                backgroundColor: "var(--color-surface, #FFFFFF)",
                color: "var(--color-text-primary, #1C1917)",
                outline: "none",
              }}
              onFocus={(e) => {
                e.currentTarget.style.borderColor = "#F97316";
              }}
              onBlur={(e) => {
                e.currentTarget.style.borderColor = "var(--color-border, #E7E5E4)";
              }}
            />
            <p
              style={{
                fontSize: 11,
                color: "var(--color-text-muted, #A8A29E)",
                marginTop: 4,
              }}
            >
              留空则自动从标题生成
            </p>
          </div>

          {/* Source selection */}
          <div>
            <label
              style={{
                fontSize: 12,
                fontWeight: 500,
                color: "var(--color-text-secondary, #57534E)",
                display: "block",
                marginBottom: 6,
              }}
            >
              数据来源
            </label>
            <div className="space-y-2">
              {SOURCE_OPTIONS.map((opt) => {
                const Icon = opt.icon;
                const isSelected = sources.includes(opt.value);
                return (
                  <label
                    key={opt.value}
                    className="flex items-center gap-3 cursor-pointer px-3 py-2 rounded-lg transition-colors hover:bg-stone-50 dark:hover:bg-white/[0.04]"
                    style={{
                      border: `1px solid ${
                        isSelected
                          ? "rgba(249, 115, 22, 0.3)"
                          : "var(--color-border, #E7E5E4)"
                      }`,
                      backgroundColor: isSelected
                        ? "rgba(249, 115, 22, 0.04)"
                        : "transparent",
                    }}
                  >
                    <input
                      type="checkbox"
                      checked={isSelected}
                      onChange={() => handleToggleSource(opt.value)}
                      className="sr-only"
                    />
                    <div
                      className="flex items-center justify-center w-4 h-4 rounded border transition-colors"
                      style={{
                        borderColor: isSelected ? "#F97316" : "var(--color-border, #D6D3D1)",
                        backgroundColor: isSelected ? "#F97316" : "transparent",
                      }}
                    >
                      {isSelected && (
                        <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
                          <path d="M2 5L4 7L8 3" stroke="white" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
                        </svg>
                      )}
                    </div>
                    <Icon size={14} style={{ color: "var(--color-text-secondary, #57534E)" }} />
                    <span
                      style={{
                        fontSize: 13,
                        color: "var(--color-text-primary, #1C1917)",
                      }}
                    >
                      {opt.label}
                    </span>
                  </label>
                );
              })}
            </div>

            {/* RSS feed config (nested) */}
            {sources.includes("rss") && (
              <div className="mt-3">
                <button
                  onClick={() => setShowRssConfig(!showRssConfig)}
                  className="text-[11px] font-medium"
                  style={{
                    color: "#F97316",
                    backgroundColor: "transparent",
                    border: "none",
                    cursor: "pointer",
                    padding: "4px 0",
                  }}
                >
                  {showRssConfig ? "收起 RSS 配置" : "配置 RSS 订阅源 →"}
                </button>
                {showRssConfig && (
                  <div className="mt-2">
                    <RssFeedConfig pageId={pageId} />
                  </div>
                )}
              </div>
            )}
          </div>

          {/* Status info */}
          <div
            className="flex items-center justify-between px-3 py-2 rounded-lg"
            style={{
              backgroundColor: "rgba(249, 115, 22, 0.04)",
              border: "1px solid rgba(249, 115, 22, 0.1)",
            }}
          >
            <span style={{ fontSize: 12, color: "var(--color-text-secondary, #57534E)" }}>
              上次检查:{" "}
              {config.last_discovered_at
                ? new Date(config.last_discovered_at).toLocaleDateString("zh-CN")
                : "从未"}
            </span>
            <span style={{ fontSize: 12, color: "var(--color-text-secondary, #57534E)" }}>
              待阅读:{" "}
              <span style={{ fontWeight: 600, color: "#F97316" }}>
                {config.pending_count}
              </span>
            </span>
          </div>
        </div>

        {/* Footer actions */}
        <div
          className="sticky bottom-0 flex items-center justify-end gap-2 px-6 py-4 border-t"
          style={{
            borderColor: "var(--color-border, #E7E5E4)",
            backgroundColor: "var(--color-surface, #FFFFFF)",
          }}
        >
          <button
            onClick={handleSearchNow}
            disabled={searching}
            className="inline-flex items-center gap-1.5 px-4 py-2 rounded-lg text-[12px] font-medium transition-all"
            style={{
              backgroundColor: "rgba(249, 115, 22, 0.08)",
              color: "#F97316",
              opacity: searching ? 0.7 : 1,
            }}
          >
            {searching ? (
              <Loader2 size={13} className="animate-spin" />
            ) : (
              <ExternalLink size={13} />
            )}
            {searching ? "搜索中..." : "搜索最新内容"}
          </button>
          <button
            onClick={handleSave}
            disabled={saving}
            className="inline-flex items-center gap-1.5 px-4 py-2 rounded-lg text-[12px] font-medium text-white transition-all"
            style={{
              backgroundColor: saving ? "#D68A3C" : "#F97316",
              opacity: saving ? 0.7 : 1,
            }}
          >
            {saving ? (
              <Loader2 size={13} className="animate-spin" />
            ) : (
              <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z" />
                <polyline points="17 21 17 13 7 13 7 21" />
                <polyline points="7 3 7 8 15 8" />
              </svg>
            )}
            {saving ? "保存中..." : "保存设置"}
          </button>
        </div>
      </div>
    </div>
  );
}
