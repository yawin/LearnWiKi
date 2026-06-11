import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useTranslation } from "react-i18next";

export function ExportSection({ totalItems }: { totalItems: number }) {
  const { t } = useTranslation("settings");
  const { t: tc } = useTranslation("common");
  const [exportStatus, setExportStatus] = useState<"idle" | "exporting" | "done">("idle");
  const [resultMsg, setResultMsg] = useState("");
  const [rangeOpen, setRangeOpen] = useState(false);
  const [startDate, setStartDate] = useState("");
  const [endDate, setEndDate] = useState("");
  const [copyPathStatus, setCopyPathStatus] = useState<"idle" | "copying" | "done">("idle");
  const [copiedPath, setCopiedPath] = useState("");

  const handleExportAll = async () => {
    setExportStatus("exporting");
    try {
      await invoke("export_all_single");
      setResultMsg(t("export.exportedAll", { count: totalItems }));
      setExportStatus("done");
      setTimeout(() => setExportStatus("idle"), 3000);
    } catch (e) {
      console.error(e);
      setExportStatus("idle");
    }
  };

  const handleCopyPath = async () => {
    setCopyPathStatus("copying");
    try {
      const path = await invoke<string>("export_all_single_quiet");
      await navigator.clipboard.writeText(path);
      setCopiedPath(path);
      setCopyPathStatus("done");
      setTimeout(() => setCopyPathStatus("idle"), 4000);
    } catch (e) {
      console.error(e);
      setCopyPathStatus("idle");
    }
  };

  const handleExportRange = async () => {
    if (!startDate || !endDate) return;
    setExportStatus("exporting");
    try {
      await invoke("export_range_single", { start: startDate, end: endDate });
      setResultMsg(t("export.exportedRange", { start: startDate, end: endDate }));
      setExportStatus("done");
      setRangeOpen(false);
      setTimeout(() => setExportStatus("idle"), 3000);
    } catch (e) {
      console.error(e);
      setExportStatus("idle");
    }
  };

  return (
    <div className="glass rounded-2xl divide-y divide-gray-100/50 dark:divide-white/[0.06]">
      {/* Export all */}
      <div className="p-4">
        <div className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">{t("export.exportAll")}</div>
        <div className="text-xs text-gray-400 dark:text-slate-500 mb-3">
          {t("export.exportAllDesc", { count: totalItems })}
        </div>
        <button
          onClick={handleExportAll}
          disabled={exportStatus === "exporting"}
          className="w-full py-2 text-sm font-medium rounded-lg border text-orange-600 dark:text-orange-400 border-orange-200/50 dark:border-orange-500/20 hover:bg-orange-50 dark:hover:bg-orange-500/10 disabled:opacity-50"
        >
          {exportStatus === "exporting" ? tc("loading") : t("export.exportButton")}
        </button>
        {exportStatus === "done" && (
          <p className="text-xs text-green-600 mt-2">{resultMsg}</p>
        )}
      </div>

      {/* Copy export path */}
      <div className="p-4">
        <div className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">{t("export.copyPath")}</div>
        <div className="text-xs text-gray-400 dark:text-slate-500 mb-3">{t("export.copyPathDesc")}</div>
        <button
          onClick={handleCopyPath}
          disabled={copyPathStatus === "copying"}
          className="w-full py-2 text-sm font-medium rounded-lg border border-gray-200/50 dark:border-slate-600/50 text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-slate-700/30 disabled:opacity-50"
        >
          {copyPathStatus === "done" ? tc("done") : t("export.copyPathButton")}
        </button>
        {copyPathStatus === "done" && (
          <p className="text-xs text-green-600 mt-2 break-all">{copiedPath}</p>
        )}
      </div>

      {/* Export date range */}
      <div className="p-4">
        <button
          onClick={() => setRangeOpen(!rangeOpen)}
          className="flex items-center gap-1.5 text-sm font-medium text-orange-600 dark:text-orange-400"
        >
          {rangeOpen ? "−" : "+"} {t("export.dateRange")}
        </button>
        {rangeOpen && (
          <div className="mt-3 space-y-3">
            <div className="flex gap-2">
              <input
                type="date"
                value={startDate}
                onChange={e => setStartDate(e.target.value)}
                className="flex-1 px-3 py-1.5 text-xs rounded-lg border border-gray-200 dark:border-slate-600 bg-white dark:bg-slate-800 text-gray-700 dark:text-gray-300"
                aria-label={t("export.startDate")}
              />
              <input
                type="date"
                value={endDate}
                onChange={e => setEndDate(e.target.value)}
                className="flex-1 px-3 py-1.5 text-xs rounded-lg border border-gray-200 dark:border-slate-600 bg-white dark:bg-slate-800 text-gray-700 dark:text-gray-300"
                aria-label={t("export.endDate")}
              />
            </div>
            <button
              onClick={handleExportRange}
              disabled={exportStatus === "exporting" || !startDate || !endDate}
              className="w-full py-2 text-sm font-medium rounded-lg border border-orange-200/50 dark:border-orange-500/20 text-orange-600 dark:text-orange-400 hover:bg-orange-50 dark:hover:bg-orange-500/10 disabled:opacity-50"
            >
              {exportStatus === "exporting" ? tc("loading") : t("export.exportButton")}
            </button>
            {exportStatus === "done" && (
              <p className="text-xs text-green-600 mt-2">{resultMsg}</p>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
