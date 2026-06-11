import { useState, useEffect } from "react";
import { FolderSync, Loader2, CheckCircle2, XCircle, SkipForward, FileText } from "lucide-react";
import { startSync, getSyncFolders } from "../../services/syncService";
import type { SyncResult } from "../../types/sync";

export function SyncButton() {
  const [syncing, setSyncing] = useState(false);
  const [result, setResult] = useState<SyncResult | null>(null);
  const [folderCount, setFolderCount] = useState<number | null>(null);
  const [showNoFolderHint, setShowNoFolderHint] = useState(false);

  useEffect(() => {
    getSyncFolders().then(fs => setFolderCount(fs.length)).catch(() => {});
  }, [result]);

  const handleSync = async () => {
    // Check for configured folders first
    try {
      const folders = await getSyncFolders();
      if (folders.length === 0) {
        setShowNoFolderHint(true);
        return;
      }
    } catch { return; }

    setSyncing(true);
    setResult(null);
    setShowNoFolderHint(false);
    try {
      const res = await startSync();
      setResult(res);
      // Notify ContentList to auto-refresh
      if (res.imported.length > 0 || res.updated.length > 0) {
        window.dispatchEvent(new CustomEvent("content-synced"));
      }
    } catch (err) {
      setResult({ imported: [], updated: [], skipped: 0, errors: [String(err)] });
    } finally {
      setSyncing(false);
    }
  };

  const handleClose = () => setResult(null);

  const total = result ? result.imported.length + result.updated.length : 0;
  const newCount = result ? result.imported.length : 0;

  return (
    <>
      <button
        onClick={handleSync}
        disabled={syncing}
        className="inline-flex items-center gap-1.5 text-[11px] px-2.5 py-1 rounded-md border transition-all disabled:opacity-50 text-gray-400 dark:text-slate-500 border-gray-200/60 dark:border-white/[0.08] bg-white/60 dark:bg-white/[0.04] hover:border-orange-300 hover:text-orange-500"
      >
        {syncing ? <Loader2 size={13} className="animate-spin" /> : <FolderSync size={13} />}
        {syncing ? "同步中..." : folderCount ? `同步 (${folderCount} 个文件夹)` : "同步文件夹"}
      </button>

      {/* No folder configured hint */}
      {showNoFolderHint && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/30" onClick={() => setShowNoFolderHint(false)}>
          <div
            className="w-72 rounded-xl p-5 shadow-lg bg-white dark:bg-slate-800"
            style={{ border: "1px solid var(--color-border, #e5e5e5)" }}
            onClick={(e) => e.stopPropagation()}
          >
            <p className="text-sm font-semibold text-gray-800 dark:text-gray-100 mb-2">
              还没有设置同步文件夹
            </p>
            <p className="text-xs text-gray-500 dark:text-slate-400 mb-4">
              先在设置中选择要导入的本地文件夹，然后再来这里同步。
            </p>
            <div className="flex gap-2">
              <button
                onClick={() => setShowNoFolderHint(false)}
                className="flex-1 py-1.5 text-xs rounded-md border border-gray-200 dark:border-white/[0.1] text-gray-500"
              >
                以后再说
              </button>
              <button
                onClick={() => {
                  setShowNoFolderHint(false);
                }}
                className="flex-1 py-1.5 text-xs rounded-md bg-orange-500 text-white font-medium"
              >
                知道了
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Result Modal */}
      {result && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/30" onClick={handleClose}>
          <div
            className="w-80 rounded-xl p-5 shadow-lg bg-white dark:bg-slate-800"
            style={{ border: "1px solid var(--color-border, #e5e5e5)" }}
            onClick={(e) => e.stopPropagation()}
          >
            <h3 className="mb-3 text-base font-bold text-gray-800 dark:text-gray-100"
              style={{ fontFamily: "'Cabinet Grotesk', sans-serif" }}>
              同步完成
            </h3>

            <div className="space-y-2 mb-4">
              {result.imported.length > 0 && (
                <div className="flex items-start gap-2">
                  <CheckCircle2 size={14} className="text-green-500 mt-0.5 shrink-0" />
                  <div>
                    <span className="text-[13px] font-medium text-green-600">
                      新导入 {result.imported.length} 个
                    </span>
                    <div className="mt-1 space-y-0.5">
                      {result.imported.slice(0, 5).map((name, i) => (
                        <p key={i} className="text-[11px] text-gray-400 dark:text-slate-500">{name}</p>
                      ))}
                      {result.imported.length > 5 && (
                        <p className="text-[11px] text-gray-400 dark:text-slate-500">
                          ...还有 {result.imported.length - 5} 个
                        </p>
                      )}
                    </div>
                  </div>
                </div>
              )}

              {result.updated.length > 0 && (
                <div className="flex items-start gap-2">
                  <FolderSync size={14} className="text-orange-500 mt-0.5 shrink-0" />
                  <div>
                    <span className="text-[13px] font-medium text-orange-500">
                      更新 {result.updated.length} 个
                    </span>
                    <div className="mt-1 space-y-0.5">
                      {result.updated.slice(0, 5).map((name, i) => (
                        <p key={i} className="text-[11px] text-gray-400 dark:text-slate-500">{name}</p>
                      ))}
                    </div>
                  </div>
                </div>
              )}

              {result.skipped > 0 && (
                <div className="flex items-center gap-2">
                  <SkipForward size={14} className="text-gray-400 dark:text-slate-500" />
                  <span className="text-[13px] text-gray-400 dark:text-slate-500">
                    跳过 {result.skipped} 个（未变化）
                  </span>
                </div>
              )}

              {result.errors.length > 0 && (
                <div className="flex items-start gap-2">
                  <XCircle size={14} className="text-red-500 mt-0.5 shrink-0" />
                  <div>
                    <span className="text-[13px] font-medium text-red-500">
                      错误 {result.errors.length} 个
                    </span>
                    <div className="mt-1 space-y-0.5">
                      {result.errors.slice(0, 3).map((err, i) => (
                        <p key={i} className="text-[11px] text-gray-400 dark:text-slate-500">{err}</p>
                      ))}
                    </div>
                  </div>
                </div>
              )}

              {total === 0 && result.errors.length === 0 && (
                <p className="text-[13px] text-gray-400 dark:text-slate-500">
                  所有文件都已是最新，无需同步
                </p>
              )}
            </div>

            {/* Batch compile hint when new files imported */}
            {newCount > 0 && (
              <div className="mb-4 p-3 rounded-lg bg-orange-50 dark:bg-orange-950/20 border border-orange-200 dark:border-orange-800">
                <div className="flex items-center gap-2 mb-2">
                  <FileText size={14} className="text-orange-500" />
                  <p className="text-xs text-orange-700 dark:text-orange-300">
                    有 {newCount} 条新内容,编译为 Wiki 后会自动关联到目标
                  </p>
                </div>
                <button
                  onClick={() => {
                    window.dispatchEvent(new CustomEvent("highlight-recent-imports", {
                      detail: { ids: result?.imported ?? [] },
                    }));
                    handleClose();
                  }}
                  className="w-full py-1.5 text-xs rounded-md bg-white dark:bg-stone-800 border border-orange-300 text-orange-600 font-medium hover:bg-orange-50"
                >
                  去内容列表选择编译
                </button>
              </div>
            )}

            <button
              onClick={handleClose}
              className="w-full py-2 rounded-md text-[13px] font-medium bg-orange-500 text-white hover:bg-orange-600 transition-colors"
            >
              确定
            </button>
          </div>
        </div>
      )}
    </>
  );
}
