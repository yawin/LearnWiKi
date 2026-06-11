import { useState } from "react";
import { useTranslation } from "react-i18next";

export function BackupSection() {
  const { t } = useTranslation("settings");
  const [status, setStatus] = useState<"idle" | "busy" | "done">("idle");
  const [message, setMessage] = useState("");
  const [isError, setIsError] = useState(false);
  const [exportPath, setExportPath] = useState("");
  const [importPath, setImportPath] = useState("");

  const handleAutoBackup = async () => {
    setStatus("busy");
    setIsError(false);
    try {
      const { autoBackup } = await import("../../services/backupService");
      const result = await autoBackup();
      if (result) {
        setMessage(t("backup.backupDone", { path: result }));
      } else {
        setMessage(t("backup.backupSkipped"));
      }
    } catch (e) {
      setMessage(t("backup.error", { message: String(e) }));
      setIsError(true);
    } finally {
      setStatus("done");
      setTimeout(() => setStatus("idle"), 4000);
    }
  };

  const handleExport = async () => {
    if (!exportPath.trim()) {
      setMessage(t("backup.noFileSelected"));
      setIsError(true);
      setTimeout(() => { setMessage(""); setIsError(false); }, 3000);
      return;
    }
    setStatus("busy");
    setIsError(false);
    try {
      const { exportBackup } = await import("../../services/backupService");
      const result = await exportBackup(exportPath.trim());
      setMessage(t("backup.exportSuccess", { path: result }));
      setExportPath("");
    } catch (e) {
      setMessage(t("backup.error", { message: String(e) }));
      setIsError(true);
    } finally {
      setStatus("done");
      setTimeout(() => setStatus("idle"), 4000);
    }
  };

  const handleImport = async (mode: "replace" | "merge") => {
    if (!importPath.trim()) {
      setMessage(t("backup.noFileSelected"));
      setIsError(true);
      setTimeout(() => { setMessage(""); setIsError(false); }, 3000);
      return;
    }
    if (mode === "replace") {
      const confirmed = window.confirm(t("backup.replaceConfirm"));
      if (!confirmed) return;
    }
    setStatus("busy");
    setIsError(false);
    try {
      const { importBackup } = await import("../../services/backupService");
      const result = await importBackup(importPath.trim(), mode);
      setMessage(t("backup.importSuccess", { path: result }));
      setImportPath("");
    } catch (e) {
      setMessage(t("backup.error", { message: String(e) }));
      setIsError(true);
    } finally {
      setStatus("done");
      setTimeout(() => setStatus("idle"), 4000);
    }
  };

  return (
    <div className="space-y-1">
      <h2 className="text-lg font-semibold text-gray-800 dark:text-gray-100 mb-4">{t("backup.title")}</h2>

      {/* Auto Backup */}
      <div className="glass rounded-2xl mb-4">
        <div className="p-4">
          <div className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">{t("backup.autoBackup")}</div>
          <div className="text-xs text-gray-400 dark:text-slate-500 mb-3">{t("backup.autoBackupDesc")}</div>
          <button
            onClick={handleAutoBackup}
            disabled={status === "busy"}
            className="w-full py-2 text-sm font-medium rounded-lg border text-orange-600 dark:text-orange-400 border-orange-200/50 dark:border-orange-500/20 hover:bg-orange-50 dark:hover:bg-orange-500/10 disabled:opacity-50"
          >
            {t("backup.runAutoBackup")}
          </button>
        </div>
      </div>

      {/* Manual Export */}
      <div className="glass rounded-2xl mb-4">
        <div className="p-4">
          <div className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">{t("backup.manualExport")}</div>
          <div className="text-xs text-gray-400 dark:text-slate-500 mb-3">{t("backup.manualExportDesc")}</div>
          <div className="flex gap-2">
            <input
              type="text"
              value={exportPath}
              onChange={e => setExportPath(e.target.value)}
              placeholder="/path/to/export.db"
              className="flex-1 px-3 py-1.5 text-xs rounded-lg border border-gray-200 dark:border-slate-600 bg-white dark:bg-slate-800 text-gray-700 dark:text-gray-300"
              aria-label={t("backup.exportPathLabel")}
            />
            <button
              onClick={handleExport}
              disabled={status === "busy"}
              className="px-4 py-1.5 text-xs font-medium rounded-lg border border-orange-200/50 dark:border-orange-500/20 text-orange-600 dark:text-orange-400 hover:bg-orange-50 dark:hover:bg-orange-500/10 disabled:opacity-50"
            >
              {t("backup.exportButton")}
            </button>
          </div>
        </div>
      </div>

      {/* Import */}
      <div className="glass rounded-2xl">
        <div className="p-4">
          <div className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">{t("backup.importTitle")}</div>
          <div className="text-xs text-gray-400 dark:text-slate-500 mb-3">{t("backup.importDesc")}</div>
          <div className="flex gap-2 mb-3">
            <input
              type="text"
              value={importPath}
              onChange={e => setImportPath(e.target.value)}
              placeholder="/path/to/backup.db"
              className="flex-1 px-3 py-1.5 text-xs rounded-lg border border-gray-200 dark:border-slate-600 bg-white dark:bg-slate-800 text-gray-700 dark:text-gray-300"
              aria-label={t("backup.importPathLabel")}
            />
          </div>
          <div className="flex gap-2">
            <button
              onClick={() => handleImport("merge")}
              disabled={status === "busy"}
              className="flex-1 py-2 text-sm font-medium rounded-lg border border-gray-200/50 dark:border-slate-600/50 text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-slate-700/30 disabled:opacity-50"
            >
              {t("backup.mergeButton")}
            </button>
            <button
              onClick={() => handleImport("replace")}
              disabled={status === "busy"}
              className="flex-1 py-2 text-sm font-medium rounded-lg border text-red-600 dark:text-red-400 border-red-200/50 dark:border-red-500/20 hover:bg-red-50 dark:hover:bg-red-500/10 disabled:opacity-50"
            >
              {t("backup.replaceButton")}
            </button>
          </div>
        </div>
      </div>

      {message && (
        <p className={`text-xs mt-2 ${isError ? "text-red-500" : "text-green-600"}`}>
          {message}
        </p>
      )}
    </div>
  );
}
