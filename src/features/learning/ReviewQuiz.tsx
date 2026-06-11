import { useState, useEffect, useCallback } from "react";
import { ChevronLeft, Loader2, CheckCircle2, XCircle } from "lucide-react";
import type { QuizQuestion } from "../../types/learning";
import { submitReviewFeedback, autoCreateReviewSchedule } from "../../services/learningService";

interface ReviewQuizProps {
  wikiPageId: string;
  wikiTitle: string;
  onClose: () => void;
}

export function ReviewQuiz({ wikiPageId, wikiTitle, onClose }: ReviewQuizProps) {
  const [questions, setQuestions] = useState<QuizQuestion[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [currentIndex, setCurrentIndex] = useState(0);
  const [selectedIndex, setSelectedIndex] = useState<number | null>(null);
  const [showResult, setShowResult] = useState(false);
  const [correctCount, setCorrectCount] = useState(0);
  const [answers, setAnswers] = useState<number[]>([]);
  const [submitted, setSubmitted] = useState(false);

  useEffect(() => {
    const load = async () => {
      try {
        const { generateQuizQuestions } = await import("../../services/learningService");
        const qs = await generateQuizQuestions(wikiPageId, 4);
        if (qs.length === 0) throw new Error("no questions");
        setQuestions(qs);
      } catch {
        setError("生成题目失败，请稍后重试");
      } finally {
        setLoading(false);
      }
    };
    load();
  }, [wikiPageId]);

  const handleSelect = useCallback((index: number) => {
    if (selectedIndex !== null) return; // already answered
    setSelectedIndex(index);
    setShowResult(true);
    setAnswers((a) => [...a, index]);
    if (index === questions[currentIndex].correct_index) {
      setCorrectCount((c) => c + 1);
    }
  }, [selectedIndex, currentIndex, questions]);

  const handleNext = useCallback(() => {
    if (currentIndex + 1 < questions.length) {
      setCurrentIndex((i) => i + 1);
      setSelectedIndex(null);
      setShowResult(false);
    } else {
      // All questions done — submit review
      finishReview();
    }
  }, [currentIndex, questions.length]);

  const finishReview = useCallback(async () => {
    if (submitted) return;
    setSubmitted(true);
    try {
      const quality = correctCount === questions.length ? 2 : correctCount >= questions.length / 2 ? 1 : 0;
      // Build question snapshot with user answers
      const snapshot = JSON.stringify(questions.map((q, i) => ({
        stem: q.stem,
        options: q.options,
        correct_index: q.correct_index,
        selected_index: answers[i] ?? -1,
        explanation: q.explanation,
      })));
      const schedule = await autoCreateReviewSchedule(wikiPageId);
      console.log("[ReviewQuiz] submitting review with schedule:", schedule?.id, "quality:", quality, "snapshot length:", snapshot.length);
      await submitReviewFeedback(schedule.id, quality, "choice", undefined, undefined, snapshot);
    } catch (e) {
      console.error("Failed to submit review:", e);
    }
  }, [correctCount, questions, answers, wikiPageId, submitted]);

  if (loading) {
    return (
      <div className="flex items-center justify-center" style={{ height: "calc(100vh - 44px)" }}>
        <Loader2 size={24} className="animate-spin text-orange-500" />
      </div>
    );
  }

  if (error || questions.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center gap-4" style={{ height: "calc(100vh - 44px)" }}>
        <p style={{ color: "var(--color-text-muted)" }}>{error || "暂无题目"}</p>
        <button onClick={onClose} className="px-4 py-2 rounded-md text-sm text-white" style={{ backgroundColor: "#F97316" }}>
          返回
        </button>
      </div>
    );
  }

  if (submitted) {
    const quality = correctCount === questions.length ? 2 : correctCount >= questions.length / 2 ? 1 : 0;
    return (
      <div className="flex flex-col items-center justify-center px-5" style={{ height: "calc(100vh - 44px)" }}>
        <div className="text-center max-w-sm">
          <div className="mb-4 text-5xl">{quality >= 1 ? "🎉" : "💪"}</div>
          <h2 className="text-xl font-bold mb-2" style={{ color: "var(--color-text-primary)" }}>复习完成</h2>
          <p style={{ fontSize: 13, color: "var(--color-text-muted)", marginBottom: 16 }}>
            {wikiTitle} · 正确 {correctCount}/{questions.length}
          </p>
          <button
            onClick={onClose}
            className="px-6 py-2.5 rounded-lg text-sm font-medium text-white"
            style={{ backgroundColor: "#F97316" }}
          >
            返回
          </button>
        </div>
      </div>
    );
  }

  const q = questions[currentIndex];

  return (
    <div className="flex flex-col" style={{ height: "calc(100vh - 44px)" }}>
      {/* Top bar */}
      <div className="flex items-center justify-between px-4 py-3">
        <button
          onClick={onClose}
          className="inline-flex items-center gap-1 text-sm"
          style={{ color: "var(--color-text-muted)" }}
        >
          <ChevronLeft size={16} />
          返回
        </button>
        <span style={{ fontSize: 12, color: "var(--color-text-muted)" }}>
          {currentIndex + 1} / {questions.length}
        </span>
      </div>

      {/* Progress */}
      <div className="w-full" style={{ height: 3, backgroundColor: "var(--color-border)" }}>
        <div
          className="h-full transition-all duration-300"
          style={{
            width: `${((currentIndex + (selectedIndex !== null ? 1 : 0)) / questions.length) * 100}%`,
            backgroundColor: "#F97316",
          }}
        />
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto px-5 py-6">
        <div className="max-w-lg mx-auto">
          <h2
            style={{
              fontSize: 18,
              fontWeight: 700,
              fontFamily: "'Cabinet Grotesk', sans-serif",
              color: "var(--color-text-primary)",
              lineHeight: 1.5,
              marginBottom: 24,
            }}
          >
            {q.stem}
          </h2>

          <div className="space-y-2.5">
            {q.options.map((opt, i) => {
              let bg = "var(--color-surface)";
              let border = "1px solid var(--color-border)";
              let textColor = "var(--color-text-primary)";

              if (showResult) {
                if (i === q.correct_index) {
                  bg = "#DCFCE7";
                  border = "1px solid #16A34A";
                  textColor = "#166534";
                } else if (i === selectedIndex && i !== q.correct_index) {
                  bg = "#FEE2E2";
                  border = "1px solid #DC2626";
                  textColor = "#991B1B";
                } else {
                  textColor = "var(--color-text-muted)";
                }
              }

              return (
                <button
                  key={i}
                  onClick={() => handleSelect(i)}
                  disabled={showResult}
                  className="w-full text-left px-4 py-3 rounded-lg transition-colors"
                  style={{ backgroundColor: bg, border, color: textColor, fontSize: 14 }}
                >
                  <div className="flex items-center gap-3">
                    <span
                      className="w-7 h-7 rounded-full flex items-center justify-center text-xs font-medium shrink-0"
                      style={{
                        backgroundColor: showResult && i === q.correct_index ? "#16A34A" : showResult && i === selectedIndex ? "#DC2626" : "var(--color-border)",
                        color: showResult && (i === q.correct_index || i === selectedIndex) ? "white" : "var(--color-text-secondary)",
                      }}
                    >
                      {showResult && i === q.correct_index ? <CheckCircle2 size={14} /> : showResult && i === selectedIndex ? <XCircle size={14} /> : String.fromCharCode(65 + i)}
                    </span>
                    <span className="flex-1">{opt}</span>
                  </div>
                </button>
              );
            })}
          </div>

          {/* Explanation after answering */}
          {showResult && q.explanation && (
            <div
              className="rounded-lg p-3 mt-4"
              style={{ backgroundColor: "rgba(249, 115, 22, 0.06)", border: "1px solid rgba(249, 115, 22, 0.12)" }}
            >
              <p style={{ fontSize: 12, color: "var(--color-text-secondary)", lineHeight: 1.6 }}>
                {q.explanation}
              </p>
            </div>
          )}
        </div>
      </div>

      {/* Bottom bar */}
      {showResult && (
        <div className="px-4 py-4 border-t" style={{ backgroundColor: "var(--color-surface)", borderColor: "var(--color-border)" }}>
          <button
            onClick={handleNext}
            className="w-full py-3 rounded-xl text-sm font-medium text-white"
            style={{ backgroundColor: "#F97316" }}
          >
            {currentIndex + 1 < questions.length ? "下一题" : "完成复习"}
          </button>
        </div>
      )}
    </div>
  );
}
