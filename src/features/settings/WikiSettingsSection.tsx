import { useState, useEffect } from "react";
import { useTranslation } from "react-i18next";

export function WikiSettingsSection() {
  const { t } = useTranslation("settings");
  const [stats, setStats] = useState<{ total_pages: number; total_edges: number; total_sources: number } | null>(null);
  const [autoCompile, setAutoCompile] = useState(true);

  useEffect(() => {
    import("../../services/wikiService").then(async (ws) => {
      try {
        const s = await ws.getWikiStats();
        setStats(s);
      } catch {}
    });
    import("../../services/settingsService").then(async (ss) => {
      try {
        const settings = await ss.getSettings();
        setAutoCompile(settings.wiki_auto_compile !== "false");
      } catch {}
    });
  }, []);

  const handleToggle = async () => {
    const newVal = !autoCompile;
    setAutoCompile(newVal);
    try {
      const { updateSetting } = await import("../../services/settingsService");
      await updateSetting("wiki_auto_compile", newVal ? "true" : "false");
    } catch (e) {
      console.error("Failed to update wiki setting:", e);
    }
  };

  return (
    <div className="px-5 py-4 border-t" style={{ borderColor: "var(--color-border, #E7E5E4)" }}>
      <h3 style={{ fontSize: 14, fontWeight: 700, color: "var(--color-text-primary, #1C1917)", marginBottom: 12 }}>
        {t("wiki.title")}
      </h3>

      {stats && (
        <div className="flex gap-4 mb-4">
          <div className="text-center">
            <div style={{ fontSize: 18, fontWeight: 700, color: "#F97316", fontFamily: "'Cabinet Grotesk', sans-serif" }}>
              {stats.total_pages}
            </div>
            <div style={{ fontSize: 11, color: "var(--color-text-muted)" }}>{t("wiki.pages")}</div>
          </div>
          <div className="text-center">
            <div style={{ fontSize: 18, fontWeight: 700, color: "#F97316", fontFamily: "'Cabinet Grotesk', sans-serif" }}>
              {stats.total_edges}
            </div>
            <div style={{ fontSize: 11, color: "var(--color-text-muted)" }}>{t("wiki.edges")}</div>
          </div>
          <div className="text-center">
            <div style={{ fontSize: 18, fontWeight: 700, color: "#F97316", fontFamily: "'Cabinet Grotesk', sans-serif" }}>
              {stats.total_sources}
            </div>
            <div style={{ fontSize: 11, color: "var(--color-text-muted)" }}>{t("wiki.sources")}</div>
          </div>
        </div>
      )}

      <div className="flex items-center justify-between py-2">
        <div>
          <p style={{ fontSize: 13, fontWeight: 500, color: "var(--color-text-primary)" }}>{t("wiki.autoCompile")}</p>
          <p style={{ fontSize: 11, color: "var(--color-text-muted)" }}>{t("wiki.autoCompileDesc")}</p>
        </div>
        <button
          onClick={handleToggle}
          role="switch"
          aria-checked={autoCompile}
          className="relative w-10 h-5 rounded-full transition-colors"
          style={{ backgroundColor: autoCompile ? "#F97316" : "var(--color-border, #E7E5E4)" }}
        >
          <div
            className="absolute top-0.5 w-4 h-4 rounded-full bg-white shadow transition-transform"
            style={{ left: autoCompile ? 22 : 2 }}
          />
        </button>
      </div>

    </div>
  );
}
