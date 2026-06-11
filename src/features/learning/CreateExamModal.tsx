interface CreateExamModalProps {
  examConfig: { choice: number; judgment: number; essay: number };
  onChangeConfig: (config: { choice: number; judgment: number; essay: number }) => void;
  onConfirm: () => void;
  onClose: () => void;
}

export function CreateExamModal({
  examConfig,
  onChangeConfig,
  onConfirm,
  onClose,
}: CreateExamModalProps) {
  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/30"
      onClick={onClose}
    >
      <div
        className="rounded-xl p-6 max-w-sm w-full mx-4"
        style={{
          backgroundColor: "var(--color-surface)",
          border: "1px solid var(--color-border)",
        }}
        onClick={(e) => e.stopPropagation()}
      >
        <h3
          className="text-lg font-bold mb-4"
          style={{ color: "var(--color-text-primary)" }}
        >
          创建新的考试
        </h3>
        <div className="space-y-3 mb-4">
          <label className="flex items-center justify-between">
            <span style={{ fontSize: 13, color: "var(--color-text-secondary)" }}>
              选择题
            </span>
            <input
              type="number"
              min={0}
              max={20}
              value={examConfig.choice}
              onChange={(e) =>
                onChangeConfig({ ...examConfig, choice: parseInt(e.target.value) || 0 })
              }
              className="w-16 px-2 py-1 rounded border text-center text-sm"
              style={{
                borderColor: "var(--color-border)",
                color: "var(--color-text-primary)",
                backgroundColor: "var(--color-surface)",
              }}
            />
          </label>
          <label className="flex items-center justify-between">
            <span style={{ fontSize: 13, color: "var(--color-text-secondary)" }}>
              判断题
            </span>
            <input
              type="number"
              min={0}
              max={20}
              value={examConfig.judgment}
              onChange={(e) =>
                onChangeConfig({ ...examConfig, judgment: parseInt(e.target.value) || 0 })
              }
              className="w-16 px-2 py-1 rounded border text-center text-sm"
              style={{
                borderColor: "var(--color-border)",
                color: "var(--color-text-primary)",
                backgroundColor: "var(--color-surface)",
              }}
            />
          </label>
          <label className="flex items-center justify-between">
            <span style={{ fontSize: 13, color: "var(--color-text-secondary)" }}>
              论述题
            </span>
            <input
              type="number"
              min={0}
              max={10}
              value={examConfig.essay}
              onChange={(e) =>
                onChangeConfig({ ...examConfig, essay: parseInt(e.target.value) || 0 })
              }
              className="w-16 px-2 py-1 rounded border text-center text-sm"
              style={{
                borderColor: "var(--color-border)",
                color: "var(--color-text-primary)",
                backgroundColor: "var(--color-surface)",
              }}
            />
          </label>
        </div>
        <p className="text-xs mb-4" style={{ color: "var(--color-text-muted)" }}>
          共 {examConfig.choice + examConfig.judgment + examConfig.essay} 题 · 预计{" "}
          {Math.round(examConfig.choice + examConfig.judgment * 0.5 + examConfig.essay * 2)} 分钟
        </p>
        <div className="flex gap-2">
          <button
            onClick={onClose}
            className="flex-1 py-2 rounded-md text-sm border"
            style={{
              borderColor: "var(--color-border)",
              color: "var(--color-text-secondary)",
            }}
          >
            取消
          </button>
          <button
            onClick={onConfirm}
            className="flex-1 py-2 rounded-md text-sm font-medium text-white"
            style={{ backgroundColor: "#F97316" }}
          >
            开始出题
          </button>
        </div>
      </div>
    </div>
  );
}
