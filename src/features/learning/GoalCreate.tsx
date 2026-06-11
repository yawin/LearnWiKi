import { useState } from "react";
import { Target, X } from "lucide-react";

interface GoalCreateProps {
  onCreated: () => void;
  onClose: () => void;
}

export function GoalCreate({ onCreated, onClose }: GoalCreateProps) {
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");
  const [loading, setLoading] = useState(false);

  const handleSubmit = async () => {
    if (!title.trim()) return;
    setLoading(true);
    try {
      const { createGoal } = await import("../../services/learningService");
      await createGoal(title.trim(), description.trim() || undefined);
      onCreated();
    } catch (err) {
      console.error("Failed to create goal:", err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div
      className="rounded-xl p-5"
      style={{
        backgroundColor: "var(--color-surface)",
        border: "1px solid var(--color-border)",
      }}
    >
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-2">
          <Target size={20} className="text-orange-500" />
          <h3
            style={{
              fontSize: 18,
              fontFamily: "'Cabinet Grotesk', sans-serif",
              fontWeight: 700,
              color: "var(--color-text-primary)",
            }}
          >
            设定学习目标
          </h3>
        </div>
        <button
          onClick={onClose}
          className="p-1 rounded-md hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
        >
          <X size={16} style={{ color: "var(--color-text-muted)" }} />
        </button>
      </div>

      <div className="space-y-3">
        <div>
          <label
            className="block mb-1"
            style={{ fontSize: 13, color: "var(--color-text-secondary)" }}
          >
            我想搞懂什么？
          </label>
          <input
            type="text"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            placeholder="例如：掌握 Rust 的所有权机制"
            className="w-full px-3 py-2 rounded-md outline-none transition-colors"
            style={{
              fontSize: 15,
              backgroundColor: "var(--color-bg-primary)",
              border: "1px solid var(--color-border)",
              color: "var(--color-text-primary)",
            }}
            onFocus={(e) => (e.target.style.borderColor = "#F97316")}
            onBlur={(e) => (e.target.style.borderColor = "var(--color-border)")}
            autoFocus
          />
        </div>

        <div>
          <label
            className="block mb-1"
            style={{ fontSize: 13, color: "var(--color-text-secondary)" }}
          >
            补充说明（选填）
          </label>
          <textarea
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            placeholder="更具体的描述，比如想达到什么程度、有什么具体方向..."
            rows={3}
            className="w-full px-3 py-2 rounded-md outline-none resize-none transition-colors"
            style={{
              fontSize: 13,
              backgroundColor: "var(--color-bg-primary)",
              border: "1px solid var(--color-border)",
              color: "var(--color-text-primary)",
            }}
            onFocus={(e) => (e.target.style.borderColor = "#F97316")}
            onBlur={(e) => (e.target.style.borderColor = "var(--color-border)")}
          />
        </div>

        <button
          onClick={handleSubmit}
          disabled={!title.trim() || loading}
          className="w-full py-2 rounded-md text-white font-medium transition-colors disabled:opacity-50"
          style={{
            fontSize: 14,
            backgroundColor: title.trim() && !loading ? "#F97316" : "#D6D3D1",
          }}
        >
          {loading ? "创建中..." : "开始学习"}
        </button>
      </div>
    </div>
  );
}
