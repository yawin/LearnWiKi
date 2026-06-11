interface ExamConfirmModalProps {
  examVersion: number;
  questionConfig: { choice: number; judgment: number; essay: number };
  estimatedMinutes: number;
  onConfirm: () => void;
  onClose: () => void;
}

export function ExamConfirmModal({
  examVersion,
  questionConfig,
  estimatedMinutes,
  onConfirm,
  onClose,
}: ExamConfirmModalProps) {
  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/30"
      onClick={onClose}
    >
      <div
        className="rounded-xl p-6 max-w-sm w-full mx-4 text-center"
        style={{
          backgroundColor: "var(--color-surface)",
          border: "1px solid var(--color-border)",
        }}
        onClick={(e) => e.stopPropagation()}
      >
        <h3
          className="text-lg font-bold mb-2"
          style={{ color: "var(--color-text-primary)" }}
        >
          试卷 v{examVersion}
        </h3>
        <p className="text-sm mb-4" style={{ color: "var(--color-text-secondary)" }}>
          选择题 {questionConfig.choice || 0} · 判断题 {questionConfig.judgment || 0} · 论述题{" "}
          {questionConfig.essay || 0}
          <br />
          预计 {estimatedMinutes} 分钟
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
            开始考试
          </button>
        </div>
      </div>
    </div>
  );
}
