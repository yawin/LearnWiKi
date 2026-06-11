import { useState, useEffect } from "react";
import { Target } from "lucide-react";
import type { Goal } from "../../types/learning";

interface GoalListProps {
  goals: Goal[];
  onSelect: (goalId: string) => void;
}

export function GoalList({ goals, onSelect }: GoalListProps) {
  const [progressMap, setProgressMap] = useState<Record<string, number>>({});

  useEffect(() => {
    const loadProgress = async () => {
      const { getGoalWikiPages, getWikiReadStatus } = await import("../../services/learningService");
      const map: Record<string, number> = {};
      for (const goal of goals) {
        try {
          const links = await getGoalWikiPages(goal.id);
          let readAndReviewed = 0;
          for (const link of links) {
            let isRead = false;
            try { isRead = await getWikiReadStatus(link.wiki_page_id); } catch { /* ignore */ }
            if (isRead && (link.review_count || 0) >= 1) readAndReviewed++;
          }
          map[goal.id] = links.length > 0 ? Math.round((readAndReviewed / links.length) * 100) : 0;
        } catch { map[goal.id] = 0; }
      }
      setProgressMap(map);
    };
    if (goals.length > 0) loadProgress();
  }, [goals]);

  if (goals.length === 0) return null;

  return (
    <div className="grid grid-cols-2 gap-3">
      {goals.map((goal) => {
        const progress = progressMap[goal.id] ?? Math.round(goal.progress);
        return (
          <button
            key={goal.id}
            onClick={() => onSelect(goal.id)}
            className="rounded-xl p-4 text-left transition-all hover:shadow-sm"
            style={{
              backgroundColor: "var(--color-surface)",
              border: "1px solid var(--color-border)",
            }}
          >
            <div
              className="p-2 rounded-lg inline-block mb-3"
              style={{ backgroundColor: "#FFF7ED" }}
            >
              <Target size={20} className="text-orange-500" />
            </div>
            <span
              className="block truncate mb-2"
              style={{ fontSize: 14, fontWeight: 600, color: "var(--color-text-primary)" }}
            >
              {goal.title}
            </span>
            <div
              className="h-1.5 rounded-full overflow-hidden mb-1"
              style={{ backgroundColor: "var(--color-border)" }}
            >
              <div
                className="h-full rounded-full"
                style={{ width: `${progress}%`, backgroundColor: "#F97316" }}
              />
            </div>
            <span style={{ fontSize: 11, color: "var(--color-text-muted)" }}>
              {progress}% 掌握
            </span>
          </button>
        );
      })}
    </div>
  );
}
