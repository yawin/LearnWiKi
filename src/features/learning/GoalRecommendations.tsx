import { useState, useEffect } from "react";
import { Search, BookOpen, ExternalLink, X, Loader2 } from "lucide-react";
import { searchGoalResources, getGoalRecommendations, dismissGoalRecommendation } from "../../services/learningService";
import type { GoalRecommendation } from "../../types/learning";

interface GoalRecommendationsProps {
  goalId: string;
}

export function GoalRecommendations({ goalId }: GoalRecommendationsProps) {
  const [recommendations, setRecommendations] = useState<GoalRecommendation[]>([]);
  const [loading, setLoading] = useState(false);
  const [searching, setSearching] = useState(false);

  useEffect(() => {
    loadRecommendations();
  }, [goalId]);

  const loadRecommendations = async () => {
    setLoading(true);
    try {
      const recs = await getGoalRecommendations(goalId);
      setRecommendations(recs);
    } catch (err) {
      console.error("Failed to load recommendations:", err);
    } finally {
      setLoading(false);
    }
  };

  const handleSearch = async () => {
    setSearching(true);
    try {
      const recs = await searchGoalResources(goalId);
      setRecommendations(recs);
    } catch (err) {
      console.error("Failed to search resources:", err);
    } finally {
      setSearching(false);
    }
  };

  const handleDismiss = async (id: string) => {
    try {
      await dismissGoalRecommendation(id);
      setRecommendations(prev => prev.filter(r => r.id !== id));
    } catch (err) {
      console.error("Failed to dismiss:", err);
    }
  };

  const difficultyBadge = (difficulty: string) => {
    const map: Record<string, { label: string; bg: string; color: string }> = {
      beginner: { label: "入门", bg: "#DCFCE7", color: "#166534" },
      intermediate: { label: "进阶", bg: "#FEF3C7", color: "#92400E" },
      advanced: { label: "深入", bg: "#FEE2E2", color: "#991B1B" },
    };
    const info = map[difficulty] ?? { label: difficulty, bg: "#F3F4F6", color: "#374151" };
    return (
      <span
        className="inline-flex px-1.5 py-0.5 rounded text-xs font-medium"
        style={{ backgroundColor: info.bg, color: info.color }}
      >
        {info.label}
      </span>
    );
  };

  const pendingRecs = recommendations.filter(r => r.status === "pending");

  return (
    <div className="mt-6">
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-2">
          <BookOpen size={16} style={{ color: "var(--color-text-secondary)" }} />
          <span style={{ fontSize: 14, fontWeight: 600, color: "var(--color-text-primary)" }}>
            推荐学习资源
          </span>
          {pendingRecs.length > 0 && (
            <span
              className="px-1.5 py-0.5 rounded text-xs font-medium"
              style={{ backgroundColor: "var(--color-border)", color: "var(--color-text-secondary)" }}
            >
              {pendingRecs.length}
            </span>
          )}
        </div>
        <button
          onClick={handleSearch}
          disabled={searching}
          className="inline-flex items-center gap-1 px-2 py-1 rounded-md text-orange-500 hover:bg-orange-50 dark:hover:bg-orange-950/30 transition-colors disabled:opacity-50"
          style={{ fontSize: 12 }}
        >
          {searching ? <Loader2 size={12} className="animate-spin" /> : <Search size={12} />}
          {searching ? "搜索中..." : recommendations.length > 0 ? "重新搜索" : "AI 搜索"}
        </button>
      </div>

      {loading && (
        <div className="flex items-center justify-center py-6">
          <Loader2 size={16} className="animate-spin text-orange-500" />
        </div>
      )}

      {!loading && pendingRecs.length === 0 && !searching && (
        <div
          className="rounded-lg p-4 text-center"
          style={{ backgroundColor: "var(--color-surface)", border: "1px solid var(--color-border)" }}
        >
          <p style={{ fontSize: 13, color: "var(--color-text-muted)" }}>
            点击"AI 搜索"生成由浅入深的学习资源推荐
          </p>
        </div>
      )}

      {!loading && pendingRecs.length > 0 && (
        <div className="space-y-2">
          {pendingRecs.map((rec, i) => (
            <div
              key={rec.id}
              className="rounded-lg p-3 transition-colors"
              style={{
                backgroundColor: "var(--color-surface)",
                border: "1px solid var(--color-border)",
              }}
            >
              <div className="flex items-start gap-2">
                <span
                  className="mt-0.5 w-5 h-5 rounded-full flex items-center justify-center shrink-0 text-xs font-medium"
                  style={{ backgroundColor: "#FFF7ED", color: "#F97316" }}
                >
                  {i + 1}
                </span>
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2 mb-1">
                    <span
                      className="truncate"
                      style={{ fontSize: 13, fontWeight: 500, color: "var(--color-text-primary)" }}
                    >
                      {rec.title}
                    </span>
                    {difficultyBadge(rec.difficulty)}
                  </div>
                  {rec.summary && (
                    <p
                      className="line-clamp-2"
                      style={{ fontSize: 12, color: "var(--color-text-muted)", lineHeight: 1.5 }}
                    >
                      {rec.summary}
                    </p>
                  )}
                  {rec.url && (
                    <a
                      href={rec.url}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="inline-flex items-center gap-1 mt-1 text-orange-500 hover:underline"
                      style={{ fontSize: 11 }}
                    >
                      <ExternalLink size={10} />
                      查看资源
                    </a>
                  )}
                </div>
                <button
                  onClick={() => handleDismiss(rec.id)}
                  className="p-1 rounded hover:bg-gray-100 dark:hover:bg-gray-800 shrink-0"
                >
                  <X size={12} style={{ color: "var(--color-text-muted)" }} />
                </button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
