import { useEffect, useState } from "react";
import { useDataHubStore } from "../../stores/dataHubStore";
import { DateSidebar } from "./DateSidebar";
import { DayDetail } from "./DayDetail";
import { ExportPanel } from "./ExportPanel";

export function DataHubView() {
  const totalItems = useDataHubStore((s) => s.totalItems);
  const totalDates = useDataHubStore((s) => s.totalDates);
  const loadDateList = useDataHubStore((s) => s.loadDateList);
  const [showExportPanel, setShowExportPanel] = useState(false);
  const [showDateSidebar, setShowDateSidebar] = useState(false);

  useEffect(() => {
    loadDateList();
  }, [loadDateList]);

  return (
    <>
      <div className="flex" style={{ height: "calc(100vh - 44px)" }}>
        {/* DateSidebar: visible on md+, hidden on mobile unless toggled */}
        <div className="hidden md:block">
          <DateSidebar
            totalItems={totalItems}
            totalDates={totalDates}
            onOpenExportPanel={() => setShowExportPanel(true)}
          />
        </div>
        <div className="flex-1 min-w-0">
          {/* Mobile: compact date toggle bar */}
          <div className="md:hidden flex items-center gap-2 px-4 py-2 border-b border-white/30 dark:border-white/[0.06]">
            <button
              onClick={() => setShowDateSidebar(!showDateSidebar)}
              className={`text-xs font-medium px-3 py-1.5 rounded-lg transition-all ${
                showDateSidebar
                  ? "bg-orange-500 text-white"
                  : "text-gray-500 dark:text-slate-400 bg-gray-100/50 dark:bg-white/[0.06]"
              }`}
            >
              📅 {showDateSidebar ? "隐藏日期" : "选择日期"}
            </button>
            <span className="text-xs text-gray-400 dark:text-slate-500 ml-auto">
              {totalItems} 项 · {totalDates} 天
            </span>
          </div>
          {/* Mobile: collapsible horizontal date sidebar */}
          {showDateSidebar && (
            <div className="md:hidden border-b border-white/30 dark:border-white/[0.06]">
              <div className="max-h-48 overflow-y-auto">
                <DateSidebar
                  totalItems={totalItems}
                  totalDates={totalDates}
                  onOpenExportPanel={() => setShowExportPanel(true)}
                />
              </div>
            </div>
          )}
          <DayDetail />
        </div>
      </div>

      {/* Export panel modal */}
      {showExportPanel && (
        <ExportPanel onClose={() => setShowExportPanel(false)} />
      )}
    </>
  );
}
