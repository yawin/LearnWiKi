import { useState, useMemo } from "react";
import { FileQuestion, CheckCircle2, XCircle, HelpCircle, ChevronRight } from "lucide-react";
import type { ClozeQuestion } from "../../types/learning";

interface ClozeReviewProps {
  question: ClozeQuestion;
  wikiTitle?: string;
  onComplete: (results: { blankResults: boolean[]; allCorrect: boolean }) => void;
  onClose: () => void;
}

// Levenshtein distance for fuzzy matching
function levenshteinDistance(a: string, b: string): number {
  const matrix: number[][] = [];
  for (let i = 0; i <= b.length; i++) {
    matrix[i] = [i];
  }
  for (let j = 0; j <= a.length; j++) {
    matrix[0][j] = j;
  }
  for (let i = 1; i <= b.length; i++) {
    for (let j = 1; j <= a.length; j++) {
      if (b[i - 1] === a[j - 1]) {
        matrix[i][j] = matrix[i - 1][j - 1];
      } else {
        matrix[i][j] = Math.min(
          matrix[i - 1][j - 1] + 1, // substitution
          matrix[i][j - 1] + 1,     // insertion
          matrix[i - 1][j] + 1      // deletion
        );
      }
    }
  }
  return matrix[b.length][a.length];
}

// Fuzzy matching: exact match OR contains/has-contained OR edit distance ≤ 2
function isAnswerCorrect(userInput: string, correctAnswers: string[]): boolean {
  const input = userInput.trim().toLowerCase();
  if (!input) return false;
  return correctAnswers.some((answer) => {
    const normalized = answer.toLowerCase();
    // Exact match
    if (input === normalized) return true;
    // Contains relationship (user input contains the answer or vice versa)
    if (input.includes(normalized) || normalized.includes(input)) return true;
    // Edit distance ≤ 2 (typo tolerance)
    return levenshteinDistance(input, normalized) <= 2;
  });
}

export default function ClozeReview({
  question,
  wikiTitle,
  onComplete,
  onClose,
}: ClozeReviewProps) {
  const [answers, setAnswers] = useState<Record<number, string>>({});
  const [submitted, setSubmitted] = useState(false);

  // Parse template to find ___ positions
  const segments = useMemo(() => {
    return question.template.split("___");
  }, [question.template]);

  const sortedBlanks = useMemo(() => {
    return [...question.blanks].sort((a, b) => a.index - b.index);
  }, [question.blanks]);

  const handleAnswerChange = (index: number, value: string) => {
    if (submitted) return;
    setAnswers((prev) => ({ ...prev, [index]: value }));
  };

  // Results per blank
  const blankResults = useMemo(() => {
    if (!submitted) return [];
    return sortedBlanks.map((blank) => {
      const userAnswer = answers[blank.index] || "";
      return isAnswerCorrect(userAnswer, blank.correct_answers);
    });
  }, [submitted, sortedBlanks, answers]);

  const allCorrect = blankResults.length > 0 && blankResults.every(Boolean);
  const allAnswered = sortedBlanks.every((b) => (answers[b.index] || "").trim().length > 0);

  const handleSubmit = () => {
    if (!allAnswered || submitted) return;
    setSubmitted(true);
  };

  const handleContinue = () => {
    onComplete({ blankResults, allCorrect });
  };

  return (
    <div
      className="fixed inset-0 z-50 flex flex-col"
      style={{ backgroundColor: "var(--color-bg)" }}
    >
      {/* Top bar */}
      <div className="flex items-center justify-between px-4 py-3">
        <button
          onClick={onClose}
          className="inline-flex items-center gap-1 text-sm"
          style={{ color: "var(--color-text-muted)" }}
        >
          <ChevronRight size={16} className="rotate-180" />
          返回
        </button>
        <div
          className="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-[10px] font-medium"
          style={{
            backgroundColor: "rgba(249, 115, 22, 0.08)",
            color: "#F97316",
          }}
        >
          <FileQuestion size={12} />
          填空
        </div>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto px-6 py-6">
        <div className="max-w-lg mx-auto">
          {/* Title */}
          <h2
            style={{
              fontSize: 20,
              fontWeight: 700,
              fontFamily: "'Cabinet Grotesk', sans-serif",
              color: "var(--color-text-primary)",
              lineHeight: 1.4,
              marginBottom: 16,
            }}
          >
            {wikiTitle || "填入缺失的关键词"}
          </h2>

          {/* Template with text and inputs */}
          <div
            className="rounded-xl p-5 mb-6"
            style={{
              backgroundColor: "var(--color-surface)",
              border: "1px solid var(--color-border)",
            }}
          >
            <p
              style={{
                fontSize: 14,
                color: "var(--color-text-primary)",
                lineHeight: 2,
                whiteSpace: "pre-wrap",
              }}
            >
              {segments.map((segment: string, i: number) => (
                <span key={i}>
                  {segment}
                  {i < sortedBlanks.length && (
                    <span className="inline-block mx-1">
                      {submitted ? (
                        <span className="inline-flex items-center gap-1">
                          <span
                            className="inline-block px-2 py-0.5 rounded text-sm font-medium"
                            style={{
                              backgroundColor: blankResults[i]
                                ? "rgba(16, 185, 129, 0.1)"
                                : "rgba(239, 68, 68, 0.1)",
                              color: blankResults[i] ? "#10B981" : "#EF4444",
                              borderBottom: `2px solid ${
                                blankResults[i] ? "#10B981" : "#EF4444"
                              }`,
                            }}
                          >
                            {answers[sortedBlanks[i].index] || "_____"}
                          </span>
                          {blankResults[i] ? (
                            <CheckCircle2 size={14} style={{ color: "#10B981" }} />
                          ) : (
                            <XCircle size={14} style={{ color: "#EF4444" }} />
                          )}
                        </span>
                      ) : (
                        <input
                          type="text"
                          value={answers[sortedBlanks[i].index] || ""}
                          onChange={(e) =>
                            handleAnswerChange(sortedBlanks[i].index, e.target.value)
                          }
                          placeholder="_____"
                          className="inline-block px-2 py-0.5 rounded text-sm outline-none"
                          style={{
                            backgroundColor: "rgba(249, 115, 22, 0.04)",
                            border: "none",
                            borderBottom: "2px solid #F97316",
                            color: "var(--color-text-primary)",
                            fontFamily: "'JetBrains Mono', monospace",
                            minWidth: 80,
                            maxWidth: 160,
                          }}
                          autoFocus={i === 0}
                        />
                      )}
                    </span>
                  )}
                </span>
              ))}
            </p>
          </div>

          {/* Hints */}
          <div className="space-y-2 mb-6">
            {sortedBlanks.map((blank, i) => (
              <div key={blank.index}>
                {blank.hint && (
                  <div
                    className="flex items-start gap-2 px-3 py-2 rounded-lg"
                    style={{
                      backgroundColor: "rgba(37, 99, 235, 0.04)",
                      border: "1px solid rgba(37, 99, 235, 0.1)",
                    }}
                  >
                    <HelpCircle
                      size={14}
                      style={{ color: "#2563EB", flexShrink: 0, marginTop: 2 }}
                    />
                    <span
                      style={{
                        fontSize: 12,
                        color: "var(--color-text-secondary)",
                        lineHeight: 1.5,
                      }}
                    >
                      <strong>第 {i + 1} 空提示：</strong>
                      {blank.hint}
                    </span>
                  </div>
                )}
                {submitted && (
                  <div
                    className="flex items-start gap-2 px-3 py-2 rounded-lg mt-1"
                    style={{
                      backgroundColor: blankResults[i]
                        ? "rgba(16, 185, 129, 0.04)"
                        : "rgba(239, 68, 68, 0.04)",
                      border: `1px solid ${
                        blankResults[i]
                          ? "rgba(16, 185, 129, 0.15)"
                          : "rgba(239, 68, 68, 0.15)"
                      }`,
                    }}
                  >
                    {blankResults[i] ? (
                      <CheckCircle2 size={14} style={{ color: "#10B981", flexShrink: 0, marginTop: 2 }} />
                    ) : (
                      <XCircle size={14} style={{ color: "#EF4444", flexShrink: 0, marginTop: 2 }} />
                    )}
                    <span
                      style={{
                        fontSize: 12,
                        color: "var(--color-text-secondary)",
                        lineHeight: 1.5,
                      }}
                    >
                      <strong>正确答案：</strong>
                      {blank.correct_answers.join("、")}
                      {!blankResults[i] && (
                        <span style={{ color: "#EF4444" }}>
                          {" "}（你的答案：{answers[blank.index] || "未填写"}）
                        </span>
                      )}
                    </span>
                  </div>
                )}
              </div>
            ))}
          </div>

          {/* Overall feedback */}
          {submitted && (
            <div
              className="rounded-xl p-4 mb-6"
              style={{
                backgroundColor: "var(--color-surface)",
                border: "1px solid var(--color-border)",
              }}
            >
              <div className="flex items-center gap-2 mb-2">
                {allCorrect ? (
                  <CheckCircle2 size={20} style={{ color: "#10B981" }} />
                ) : (
                  <XCircle size={20} style={{ color: "#EF4444" }} />
                )}
                <span
                  style={{
                    fontSize: 15,
                    fontWeight: 600,
                    color: allCorrect ? "#10B981" : "#EF4444",
                  }}
                >
                  {allCorrect
                    ? "🎉 全部正确！"
                    : blankResults.filter(Boolean).length > 0
                      ? "部分正确，继续加油！"
                      : "需要再复习"}
                </span>
              </div>
              <p
                style={{
                  fontSize: 12,
                  color: "var(--color-text-muted)",
                  lineHeight: 1.5,
                }}
              >
                正确 {blankResults.filter(Boolean).length}/{blankResults.length} 空
                {blankResults.some(Boolean) && !allCorrect && (
                  <span> — 支持同义词和近音词匹配</span>
                )}
              </p>
            </div>
          )}
        </div>
      </div>

      {/* Bottom bar */}
      <div
        className="px-4 py-4 border-t"
        style={{
          backgroundColor: "var(--color-surface)",
          borderColor: "var(--color-border)",
        }}
      >
        <div className="max-w-lg mx-auto">
          {!submitted ? (
            <button
              onClick={handleSubmit}
              disabled={!allAnswered}
              className="w-full inline-flex items-center justify-center gap-2 px-6 py-3 rounded-xl text-sm font-medium transition-all disabled:opacity-40"
              style={{
                backgroundColor: allAnswered ? "#F97316" : "var(--color-border)",
                color: allAnswered ? "white" : "var(--color-text-muted)",
              }}
              onMouseEnter={(e) => {
                if (allAnswered) e.currentTarget.style.backgroundColor = "#EA580C";
              }}
              onMouseLeave={(e) => {
                if (allAnswered) e.currentTarget.style.backgroundColor = "#F97316";
              }}
            >
              提交答案
            </button>
          ) : (
            <button
              onClick={handleContinue}
              className="w-full inline-flex items-center justify-center gap-2 px-6 py-3 rounded-xl text-sm font-medium transition-all"
              style={{
                backgroundColor: "#F97316",
                color: "white",
              }}
              onMouseEnter={(e) => { e.currentTarget.style.backgroundColor = "#EA580C"; }}
              onMouseLeave={(e) => { e.currentTarget.style.backgroundColor = "#F97316"; }}
            >
              继续复习
            </button>
          )}
        </div>
      </div>
    </div>
  );
}
