import { useSettingsStore } from "../../stores/settingsStore";

type Level = "loose" | "balanced" | "strict";

const OPTIONS: Array<{ value: Level; label: string; emoji: string; desc: string }> = [
  { value: "loose", emoji: "🟢", label: "宽松",
    desc: "多关联,可能有不太相关的内容混进目标" },
  { value: "balanced", emoji: "🟡", label: "平衡(默认)",
    desc: "推荐。漏关联和误关联都比较少" },
  { value: "strict", emoji: "🔴", label: "严格",
    desc: "少关联,只关联系统认为很相关的" },
];

export function LearningSection() {
  const current = useSettingsStore((s) => s.autoLinkSensitivity);
  const setLevel = useSettingsStore((s) => s.setAutoLinkSensitivity);
  const apiKey = useSettingsStore((s) => s.apiKey);
  const aiConfigured = apiKey.trim().length > 0;

  return (
    <section className="space-y-3">
      <h3 className="text-sm font-semibold text-gray-800 dark:text-gray-100">
        学习
      </h3>
      <div className="space-y-2">
        <div className="text-xs text-gray-500 dark:text-slate-400">
          自动关联敏感度
        </div>
        {OPTIONS.map((opt) => (
          <label
            key={opt.value}
            className="flex items-start gap-3 cursor-pointer rounded-lg border border-gray-200 dark:border-white/[0.08] p-3 hover:border-orange-300"
          >
            <input
              type="radio"
              name="auto-link-sensitivity"
              value={opt.value}
              checked={current === opt.value}
              onChange={() => setLevel(opt.value)}
              className="mt-1"
            />
            <div>
              <div className="text-sm font-medium">
                {opt.emoji} {opt.label}
              </div>
              <div className="text-xs text-gray-500 dark:text-slate-400 mt-0.5">
                {opt.desc}
              </div>
            </div>
          </label>
        ))}
        {!aiConfigured && (
          <p className="text-xs text-gray-400 dark:text-slate-500 italic">
            当前未配置 AI,使用关键词匹配兜底,匹配质量可能较低
          </p>
        )}
      </div>
    </section>
  );
}
