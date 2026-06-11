import { useEffect, useState, useCallback } from "react";
import { GraduationCap, Target, Plus, Loader2 } from "lucide-react";
import { GoalCreate } from "./GoalCreate";
import { GoalList } from "./GoalList";
import type { Goal } from "../../types/learning";

interface LearningDashboardProps {
  onSelectGoal: (goalId: string) => void;
}

export function LearningDashboard({ onSelectGoal }: LearningDashboardProps) {
  const [goals, setGoals] = useState<Goal[]>([]);
  const [loading, setLoading] = useState(true);
  const [showCreate, setShowCreate] = useState(false);

  const loadGoals = useCallback(async () => {
    try {
      const { getGoals } = await import("../../services/learningService");
      const result = await getGoals("active");
      setGoals(result);
    } catch (err) {
      console.error("Failed to load goals:", err);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    loadGoals();
  }, [loadGoals]);

  const handleGoalCreated = () => {
    setShowCreate(false);
    loadGoals();
  };

  return (
    <div className="px-5 pt-5 pb-8">
      {/* Header */}
      <div className="flex items-center gap-2 mb-1">
        <GraduationCap size={22} className="text-orange-500" strokeWidth={2} />
        <h2
          style={{
            fontSize: 22,
            fontFamily: "'Cabinet Grotesk', sans-serif",
            fontWeight: 700,
            color: "var(--color-text-primary)",
            letterSpacing: "-0.3px",
          }}
        >
          学习
        </h2>
      </div>
      <p style={{ fontSize: 13, color: "var(--color-text-muted)", marginBottom: 24 }}>
        设定目标，积累知识，检验成果
      </p>

      {/* Goals Section */}
      <div className="mb-6">
        <div className="flex items-center justify-between mb-3">
          <div className="flex items-center gap-2">
            <Target size={16} style={{ color: "var(--color-text-secondary)" }} />
            <span style={{ fontSize: 14, fontWeight: 600, color: "var(--color-text-primary)" }}>
              我的目标
            </span>
          </div>
          {!showCreate && (
            <button
              onClick={() => setShowCreate(true)}
              className="inline-flex items-center gap-1 px-2 py-1 rounded-md text-orange-500 hover:bg-orange-50 dark:hover:bg-orange-950/30 transition-colors"
              style={{ fontSize: 12 }}
            >
              <Plus size={14} />
              新目标
            </button>
          )}
        </div>

        {/* Create Goal Form */}
        {showCreate && (
          <div className="mb-4">
            <GoalCreate
              onCreated={handleGoalCreated}
              onClose={() => setShowCreate(false)}
            />
          </div>
        )}

        {/* Loading */}
        {loading && (
          <div className="flex items-center justify-center py-8">
            <Loader2 size={20} className="animate-spin text-orange-500" />
          </div>
        )}

        {/* Goals List */}
        {!loading && <GoalList goals={goals} onSelect={onSelectGoal} />}

        {/* Empty state */}
        {!loading && goals.length === 0 && !showCreate && (
          <div
            className="rounded-xl p-8 text-center"
            style={{
              backgroundColor: "var(--color-surface)",
              border: "1px solid var(--color-border)",
            }}
          >
            <Target size={32} className="mx-auto mb-3 text-orange-400" />
            <p style={{ fontSize: 15, fontWeight: 600, color: "var(--color-text-primary)", marginBottom: 4 }}>
              设定一个学习目标开始吧
            </p>
            <p style={{ fontSize: 13, color: "var(--color-text-muted)", marginBottom: 16 }}>
              告诉我你想搞懂什么，我来帮你规划学习路线
            </p>
            <button
              onClick={() => setShowCreate(true)}
              className="inline-flex items-center gap-1.5 px-4 py-2 rounded-md text-white font-medium transition-colors"
              style={{ fontSize: 13, backgroundColor: "#F97316" }}
            >
              <Plus size={14} />
              设定目标
            </button>
          </div>
        )}
      </div>
    </div>
  );
}
