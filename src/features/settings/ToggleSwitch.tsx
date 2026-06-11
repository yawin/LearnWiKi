export function ToggleSwitch({ checked, onChange, color = "orange" }: { checked: boolean; onChange: (v: boolean) => void; color?: string }) {
  const bgColor = checked
    ? color === "amber" ? "bg-amber-500"
    : color === "green" ? "bg-green-500"
    : "bg-orange-500"
    : "bg-gray-300 dark:bg-slate-600";

  return (
    <button
      role="switch"
      aria-checked={checked}
      onClick={() => onChange(!checked)}
      className={`relative w-11 h-6 rounded-full transition-colors duration-200 shrink-0 ${bgColor}`}
    >
      <span className={`absolute top-0.5 left-0.5 w-5 h-5 rounded-full bg-white shadow-sm transition-transform duration-200 ${checked ? "translate-x-5" : "translate-x-0"}`} />
    </button>
  );
}
