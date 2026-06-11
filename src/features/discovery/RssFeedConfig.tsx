import { useState, useEffect, useCallback } from "react";
import { Rss, Plus, Trash2, Loader2, ExternalLink } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";

interface RssFeed {
  id: string;
  search_query: string;
  source_type: string;
  rss_url: string | null;
  is_active: boolean;
  last_checked_at: string | null;
  last_found_count: number;
}

interface RssFeedConfigProps {
  pageId: string;
}

export default function RssFeedConfig({ pageId }: RssFeedConfigProps) {
  const [feeds, setFeeds] = useState<RssFeed[]>([]);
  const [loading, setLoading] = useState(true);
  const [feedUrl, setFeedUrl] = useState("");
  const [feedName, setFeedName] = useState("");
  const [adding, setAdding] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [testing, setTesting] = useState<string | null>(null);
  const [testResult, setTestResult] = useState<{ title: string; url: string }[] | null>(null);

  const loadFeeds = useCallback(async () => {
    try {
      setLoading(true);
      const data = await invoke<RssFeed[]>("get_monitor_sources_for_page", {
        pageId,
      });
      setFeeds(data.filter((f) => f.source_type === "rss"));
    } catch (err) {
      console.error("Failed to load RSS feeds:", err);
    } finally {
      setLoading(false);
    }
  }, [pageId]);

  useEffect(() => {
    loadFeeds();
  }, [loadFeeds]);

  const handleAddFeed = async () => {
    if (!feedUrl.trim() || !feedName.trim()) {
      setError("请填写 RSS 链接和名称");
      return;
    }
    setAdding(true);
    setError(null);
    try {
      await invoke("create_monitor_source", {
        pageId,
        searchQuery: feedName.trim(),
        sourceType: "rss",
        rssUrl: feedUrl.trim(),
      });
      setFeedUrl("");
      setFeedName("");
      await loadFeeds();
    } catch (err) {
      setError((err as Error).message);
    } finally {
      setAdding(false);
    }
  };

  const handleTestFeed = async (url: string) => {
    setTesting(url);
    setTestResult(null);
    try {
      const result = await invoke<{ title: string; url: string }[]>("test_rss_feed", {
        url,
      });
      setTestResult(result.slice(0, 3));
    } catch (err) {
      setError((err as Error).message);
    } finally {
      setTesting(null);
    }
  };

  const handleRemoveFeed = async (id: string) => {
    try {
      await invoke("update_monitor_source", {
        id,
        pageId,
        searchQuery: "",
        sourceType: "rss",
        rssUrl: null,
        isActive: false,
      });
      await loadFeeds();
    } catch (err) {
      setError((err as Error).message);
    }
  };

  return (
    <div
      className="rounded-xl p-4"
      style={{
        backgroundColor: "var(--color-surface)",
        border: "1px solid var(--color-border)",
      }}
    >
      <div className="flex items-center gap-2 mb-3">
        <Rss size={14} style={{ color: "#F97316" }} />
        <span
          style={{
            fontSize: 12,
            fontWeight: 600,
            color: "var(--color-text-primary)",
          }}
        >
          RSS 订阅源配置
        </span>
      </div>

      {/* Error */}
      {error && (
        <div
          className="rounded-lg p-2 mb-3"
          style={{
            fontSize: 11,
            backgroundColor: "rgba(239, 68, 68, 0.08)",
            color: "#DC2626",
          }}
        >
          {error}
        </div>
      )}

      {/* Existing feeds */}
      {loading ? (
        <div className="flex items-center gap-2 py-2">
          <Loader2 size={12} className="animate-spin" style={{ color: "var(--color-text-muted)" }} />
          <span style={{ fontSize: 11, color: "var(--color-text-muted)" }}>加载中...</span>
        </div>
      ) : feeds.length > 0 ? (
        <div className="space-y-2 mb-4">
          {feeds.map((feed) => (
            <div
              key={feed.id}
              className="flex items-center justify-between gap-2 px-3 py-2 rounded-lg"
              style={{
                backgroundColor: "rgba(249, 115, 22, 0.04)",
                border: "1px solid rgba(249, 115, 22, 0.1)",
              }}
            >
              <div className="flex-1 min-w-0">
                <p
                  style={{
                    fontSize: 12,
                    fontWeight: 500,
                    color: "var(--color-text-primary)",
                  }}
                  className="truncate"
                >
                  {feed.search_query}
                </p>
                <p
                  style={{
                    fontSize: 10,
                    color: "var(--color-text-muted)",
                  }}
                  className="truncate"
                >
                  {feed.rss_url}
                </p>
              </div>
              <div className="flex items-center gap-1">
                <button
                  onClick={() => feed.rss_url && handleTestFeed(feed.rss_url)}
                  disabled={testing === feed.rss_url}
                  className="p-1 rounded hover:bg-white/50 transition-colors"
                  title="测试订阅源"
                >
                  {testing === feed.rss_url ? (
                    <Loader2 size={12} className="animate-spin" style={{ color: "#F97316" }} />
                  ) : (
                    <ExternalLink size={12} style={{ color: "var(--color-text-muted)" }} />
                  )}
                </button>
                <button
                  onClick={() => handleRemoveFeed(feed.id)}
                  className="p-1 rounded hover:bg-red-50 transition-colors"
                  title="删除"
                >
                  <Trash2 size={12} style={{ color: "#DC2626" }} />
                </button>
              </div>
            </div>
          ))}
        </div>
      ) : (
        <p
          style={{
            fontSize: 11,
            color: "var(--color-text-muted)",
            marginBottom: 12,
          }}
        >
          暂无 RSS 订阅源
        </p>
      )}

      {/* Test result */}
      {testResult && testResult.length > 0 && (
        <div
          className="rounded-lg p-2 mb-3"
          style={{
            backgroundColor: "rgba(22, 163, 74, 0.06)",
            border: "1px solid rgba(22, 163, 74, 0.15)",
          }}
        >
          <p style={{ fontSize: 11, fontWeight: 500, color: "#16A34A", marginBottom: 4 }}>
            ✓ 测试成功，最近条目:
          </p>
          {testResult.map((item, i) => (
            <p
              key={i}
              style={{
                fontSize: 10,
                color: "var(--color-text-secondary)",
                paddingLeft: 8,
              }}
              className="truncate"
            >
              {i + 1}. {item.title}
            </p>
          ))}
        </div>
      )}

      {/* Add new feed */}
      <div className="space-y-2">
        <input
          type="text"
          value={feedUrl}
          onChange={(e) => setFeedUrl(e.target.value)}
          placeholder="RSS 链接 (URL)"
          style={{
            width: "100%",
            padding: "6px 10px",
            fontSize: 12,
            borderRadius: 8,
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
        <input
          type="text"
          value={feedName}
          onChange={(e) => setFeedName(e.target.value)}
          placeholder="订阅源名称 (如: Rust Blog)"
          style={{
            width: "100%",
            padding: "6px 10px",
            fontSize: 12,
            borderRadius: 8,
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
        <button
          onClick={handleAddFeed}
          disabled={adding}
          className="w-full inline-flex items-center justify-center gap-1 px-3 py-1.5 rounded-lg text-[11px] font-medium transition-all"
          style={{
            backgroundColor: "rgba(249, 115, 22, 0.1)",
            color: "#F97316",
          }}
        >
          {adding ? (
            <Loader2 size={12} className="animate-spin" />
          ) : (
            <Plus size={12} />
          )}
          添加
        </button>
      </div>
    </div>
  );
}
