import { useState, useEffect } from "react";
import { LearningDashboard } from "./LearningDashboard";
import { GoalDetail } from "./GoalDetail";
import { ReviewLogDetail } from "./ReviewLogDetail";
import { ReviewQuiz } from "./ReviewQuiz";
import ReviewSession from "./ReviewSession";
import { ExamSession } from "./ExamSession";

type ViewState =
  | { view: "dashboard" }
  | { view: "goal"; goalId: string }
  | { view: "review" }
  | { view: "exam"; examId: string }
  | { view: "resumeExam"; examId: string }
  | { view: "reviewLog"; logId: string; goalId: string }
  | { view: "reviewQuiz"; wikiPageId: string; wikiTitle: string };

export default function LearningView() {
  const [state, setState] = useState<ViewState>({ view: "dashboard" });

  // Check for unfinished exams on mount (interrupt recovery)
  useEffect(() => {
    const checkUnfinished = async () => {
      try {
        const { getGoals, getExamHistory } = await import("../../services/learningService");
        const activeGoals = await getGoals("active");
        for (const g of activeGoals) {
          const exams = await getExamHistory(g.id);
          const unfinished = exams.find(
            (e) => e.status === "in_progress" &&
              Date.now() - new Date(e.created_at).getTime() < 24 * 60 * 60 * 1000
          );
          if (unfinished) {
            setState({ view: "resumeExam", examId: unfinished.id });
            return;
          }
        }
      } catch {}
    };
    checkUnfinished();
  }, []);

  // Listen for navigate-to-review-log from GoalDetail
  useEffect(() => {
    const handler = (e: Event) => {
      const detail = (e as CustomEvent<{ logId: string; goalId: string }>).detail;
      if (detail?.logId) setState({ view: "reviewLog", logId: detail.logId, goalId: detail.goalId ?? "" });
    };
    window.addEventListener("navigate-to-review-log", handler);
    return () => window.removeEventListener("navigate-to-review-log", handler);
  }, []);

  // Listen for navigate-to-goal from App.tsx preview overlay
  useEffect(() => {
    const handler = (e: Event) => {
      const goalId = (e as CustomEvent<{ goalId: string }>).detail?.goalId;
      if (goalId) setState({ view: "goal", goalId });
    };
    window.addEventListener("navigate-to-goal", handler);
    return () => window.removeEventListener("navigate-to-goal", handler);
  }, []);

  // Listen for start-single-review from GoalDetail
  useEffect(() => {
    const handler = (e: Event) => {
      const detail = (e as CustomEvent<{ wikiPageId: string; wikiTitle: string }>).detail;
      if (detail?.wikiPageId) {
        setState({ view: "reviewQuiz", wikiPageId: detail.wikiPageId, wikiTitle: detail.wikiTitle ?? "" });
      }
    };
    window.addEventListener("start-single-review", handler);
    return () => window.removeEventListener("start-single-review", handler);
  }, []);

  if (state.view === "resumeExam") {
    return (
      <div className="flex items-center justify-center" style={{ height: "calc(100vh - 44px)" }}>
        <div
          className="rounded-xl p-8 text-center max-w-sm mx-4"
          style={{ backgroundColor: "var(--color-surface)", border: "1px solid var(--color-border)" }}
        >
          <p style={{ fontSize: 15, fontWeight: 600, color: "var(--color-text-primary)", marginBottom: 8 }}>
            上次考试未完成
          </p>
          <p style={{ fontSize: 13, color: "var(--color-text-muted)", marginBottom: 20 }}>
            是否继续？如重新开始将丢失当前答题进度。
          </p>
          <div className="flex gap-3 justify-center">
            <button
              onClick={() => setState({ view: "dashboard" })}
              className="px-4 py-2 rounded-md text-sm font-medium"
              style={{ color: "var(--color-text-secondary)", border: "1px solid var(--color-border)" }}
            >
              重新开始
            </button>
            <button
              onClick={() => setState({ view: "exam", examId: state.examId })}
              className="px-4 py-2 rounded-md text-sm font-medium text-white"
              style={{ backgroundColor: "#F97316" }}
            >
              继续考试
            </button>
          </div>
        </div>
      </div>
    );
  }

  if (state.view === "exam") {
    return <ExamSession examId={state.examId} onClose={() => setState({ view: "dashboard" })} />;
  }

  if (state.view === "review") {
    return <ReviewSession onClose={() => setState({ view: "dashboard" })} />;
  }

  if (state.view === "reviewQuiz") {
    return (
      <ReviewQuiz
        wikiPageId={state.wikiPageId}
        wikiTitle={state.wikiTitle}
        onClose={() => setState({ view: "dashboard" })}
      />
    );
  }

  if (state.view === "reviewLog") {
    return (
      <ReviewLogDetail
        logId={state.logId}
        onBack={() => setState({ view: "goal", goalId: state.goalId })}
      />
    );
  }

  if (state.view === "goal") {
    return (
      <div className="overflow-y-auto" style={{ height: "calc(100vh - 44px)", color: "var(--color-text-primary)" }}>
        <GoalDetail
          goalId={state.goalId}
          onBack={() => setState({ view: "dashboard" })}
        />
      </div>
    );
  }

  return (
    <div className="overflow-y-auto" style={{ height: "calc(100vh - 44px)", color: "var(--color-text-primary)" }}>
      <LearningDashboard
        onSelectGoal={(id) => setState({ view: "goal", goalId: id })}
      />
    </div>
  );
}
