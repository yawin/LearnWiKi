import { useState, useEffect, useRef } from "react";
import { FolderSync, Trash2, Loader2, FolderOpen, Keyboard } from "lucide-react";
import { getSyncFolders, addSyncFolder, removeSyncFolder, updateSyncFolder } from "../../services/syncService";
import type { SyncFolder } from "../../types/sync";

export function SyncFoldersSection() {
  const [folders, setFolders] = useState<SyncFolder[]>([]);
  const [loading, setLoading] = useState(true);
  const [adding, setAdding] = useState(false);
  const [showInput, setShowInput] = useState(false);
  const [inputPath, setInputPath] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    loadFolders();
  }, []);

  const loadFolders = async () => {
    try {
      const result = await getSyncFolders();
      setFolders(result);
    } catch (err) {
      console.error("Failed to load sync folders:", err);
    } finally {
      setLoading(false);
    }
  };

  const handleBrowse = async () => {
    setAdding(true);
    try {
      const { open } = await import("@tauri-apps/plugin-dialog");
      const path = await open({ directory: true, multiple: false, title: "选择要同步的文件夹" });
      if (path && typeof path === "string") {
        await addSyncFolder(path);
        await loadFolders();
      }
    } catch (err) {
      console.error("Failed to add folder:", err);
    } finally {
      setAdding(false);
    }
  };

  const handleManualAdd = () => {
    setShowInput(true);
    setTimeout(() => inputRef.current?.focus(), 50);
  };

  const handleSubmitPath = async () => {
    const path = inputPath.trim();
    if (!path) return;
    try {
      await addSyncFolder(path);
      setInputPath("");
      setShowInput(false);
      await loadFolders();
    } catch (err) {
      console.error("Failed to add folder:", err);
    }
  };

  const handleRemove = async (id: string) => {
    try {
      await removeSyncFolder(id);
      setFolders(prev => prev.filter(f => f.id !== id));
    } catch (err) {
      console.error("Failed to remove folder:", err);
    }
  };

  const handleToggle = async (id: string, enabled: boolean) => {
    try {
      await updateSyncFolder(id, enabled);
      setFolders(prev => prev.map(f => f.id === id ? { ...f, enabled } : f));
    } catch (err) {
      console.error("Failed to toggle folder:", err);
    }
  };

  return (
    <div className="mb-8">
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-2">
          <FolderSync size={18} className="text-orange-500" />
          <h3 className="text-[15px] font-semibold text-gray-800 dark:text-gray-100">
            同步文件夹
          </h3>
        </div>
        <div className="flex items-center gap-1">
          <button
            onClick={handleBrowse}
            disabled={adding}
            className="inline-flex items-center gap-1 px-2 py-1 rounded-md text-orange-500 hover:bg-orange-50 dark:hover:bg-orange-950/30 transition-colors text-xs disabled:opacity-50"
          >
            {adding ? <Loader2 size={14} className="animate-spin" /> : <FolderOpen size={14} />}
            {adding ? "选择中..." : "浏览"}
          </button>
          <button
            onClick={handleManualAdd}
            className="inline-flex items-center gap-1 px-2 py-1 rounded-md text-gray-400 hover:text-orange-500 hover:bg-orange-50 dark:hover:bg-orange-950/30 transition-colors text-xs"
            title="手动输入路径"
          >
            <Keyboard size={14} />
          </button>
        </div>
      </div>

      <p className="text-xs text-gray-400 dark:text-slate-500 mb-3">
        选择本地文件夹，其中的 md / txt / pdf / docx / epub / 图片文件会被自动扫描导入。
        导入后的内容可编译为 Wiki 页面并自动关联到学习目标。
      </p>

      {/* Manual path input */}
      {showInput && (
        <div className="flex gap-2 mb-3">
          <input
            ref={inputRef}
            type="text"
            value={inputPath}
            onChange={(e) => setInputPath(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") handleSubmitPath();
              if (e.key === "Escape") { setShowInput(false); setInputPath(""); }
            }}
            placeholder="/Users/macbook/Documents/notes"
            className="flex-1 px-3 py-2 text-sm rounded-lg bg-white/50 dark:bg-white/[0.04] border border-gray-200/50 dark:border-white/[0.08] text-gray-800 dark:text-gray-200 placeholder-gray-400 dark:placeholder-slate-600 focus:outline-none focus:ring-1 focus:ring-orange-400/50"
          />
          <button
            onClick={handleSubmitPath}
            className="px-3 py-1.5 text-xs font-medium rounded-lg bg-orange-500 text-white hover:bg-orange-600 transition-colors"
          >
            确定
          </button>
          <button
            onClick={() => { setShowInput(false); setInputPath(""); }}
            className="px-3 py-1.5 text-xs font-medium rounded-lg border border-gray-200/50 dark:border-white/[0.08] text-gray-500 dark:text-slate-400 hover:bg-gray-100/50 dark:hover:bg-white/[0.04] transition-colors"
          >
            取消
          </button>
        </div>
      )}

      {loading && (
        <div className="flex justify-center py-4">
          <Loader2 size={16} className="animate-spin text-orange-500" />
        </div>
      )}

      {!loading && folders.length === 0 && !showInput && (
        <div
          className="rounded-lg p-4 text-center glass"
          style={{ border: "1px solid var(--color-border, #e5e5e5)" }}
        >
          <p className="text-[13px] text-gray-400 dark:text-slate-500">
            还没有配置同步文件夹，点击「浏览」选择文件夹或「键盘图标」手动输入路径
          </p>
        </div>
      )}

      {!loading && folders.length > 0 && (
        <div className="space-y-2">
          {folders.map((folder) => (
            <div
              key={folder.id}
              className="flex items-center gap-3 rounded-lg p-3 glass"
              style={{
                border: "1px solid var(--color-border, #e5e5e5)",
                opacity: folder.enabled ? 1 : 0.5,
              }}
            >
              <div className="flex-1 min-w-0">
                <p className="truncate text-[13px] text-gray-800 dark:text-gray-200">
                  {folder.path}
                </p>
                {folder.last_synced_at && (
                  <p className="text-[11px] text-gray-400 dark:text-slate-500 mt-0.5">
                    上次同步: {new Date(folder.last_synced_at).toLocaleString("zh-CN")}
                  </p>
                )}
              </div>
              <label className="relative inline-flex items-center cursor-pointer">
                <input
                  type="checkbox"
                  checked={folder.enabled}
                  onChange={(e) => handleToggle(folder.id, e.target.checked)}
                  className="sr-only peer"
                />
                <div className="w-8 h-4 bg-gray-200 rounded-full peer dark:bg-gray-700 peer-checked:bg-orange-500 transition-colors after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:rounded-full after:h-3 after:w-3 after:transition-all peer-checked:after:translate-x-full" />
              </label>
              <button
                onClick={() => handleRemove(folder.id)}
                className="p-1 rounded hover:bg-red-50 dark:hover:bg-red-950/30 transition-colors"
              >
                <Trash2 size={14} className="text-red-400" />
              </button>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
