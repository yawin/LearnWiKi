import { useState, useRef, useEffect, useCallback } from "react";
import { MessageSquareText, Lightbulb, Send, Sparkles, ChevronLeft, X } from "lucide-react";
import { useLearningStore } from "../../stores/learningStore";

interface ExplainReviewProps {
  wikiPageId: string;
  onComplete: (quality: number, responseTimeSeconds: number) => void;
  onClose: () => void;
}
export default function ExplainReview({
  wikiPageId,
  onComplete,
  onClose,
}: ExplainReviewProps) {
  const {
    explainQuestion,
    explainFeedback,
    explainLoading,
    explainFeedbackLoading,
    explainError,
    fetchExplainQuestion,
    fetchExplainFeedback,
    clearExplain,
  } = useLearningStore();

  const [userExplanation, setUserExplanation] = useState("");
  const [hasSubmitted, setHasSubmitted] = useState(false);
  const [responseTimeSeconds, setResponseTimeSeconds] = useState(0);
  const startTimeRef = useRef(Date.now());
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  useEffect(() => {
    clearExplain();
    fetchExplainQuestion(wikiPageId);
    startTimeRef.current = Date.now();
  }, [wikiPageId, fetchExplainQuestion, clearExplain]);

  // Timer for response time tracking
  useEffect(() => {
    const interval = setInterval(() => {
      setResponseTimeSeconds(Math.floor((Date.now() - startTimeRef.current) / 1000));
    }, 1000);
    return () => clearInterval(interval);
  }, []);

  // Auto-focus textarea when question loads
  useEffect(() => {
    if (!explainLoading && explainQuestion && textareaRef.current) {
      textareaRef.current.focus();
    }
  }, [explainLoading, explainQuestion]);

  // Submit explanation
  const handleSubmit = useCallback(async () => {
    if (!userExplanation.trim() || explainFeedbackLoading) return;
    setHasSubmitted(true);
    try {
      await fetchExplainFeedback(wikiPageId, userExplanation.trim());
    } catch {
      setHasSubmitted(false);
    }
  }, [userExplanation, explainFeedbackLoading, wikiPageId, fetchExplainFeedback]);

  // Continue to next review
  const handleContinue = useCallback(() => {
    const quality = (explainFeedback?.score ?? 3) >= 3 ? 1 : 0;
    clearExplain();
    onComplete(quality, responseTimeSeconds);
  }, [explainFeedback, responseTimeSeconds, clearExplain, onComplete]);

  // Render stars
  const renderStars = (score: number) => {
    return Array.from({ length: 5 }, (_, i) => (
      <span
        key={i}
        style={{
          fontSize: 22,
          color: i < score ? "#F97316" : "var(--color-border)",
        }}
      >
        {i < score ? "★" : "☆"}
      </span>
    ));
  };

  return (
    <div
      className="flex flex-col h-full"
      style={{
        fontFamily: "'Plus Jakarta Sans', sans-serif",
      }}
    >
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3">
        <button
          onClick={onClose}
          className="inline-flex items-center gap-1 text-sm"
          style={{ color: "var(--color-text-muted)" }}
        >
          <ChevronLeft size={16} />
          返回
        </button>
        <div className="flex items-center gap-2">
          <span
            className="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-[10px] font-medium"
            style={{
              backgroundColor: "rgba(249, 115, 22, 0.08)",
              color: "#F97316",
            }}
          >
            <MessageSquareText size={12} />
            解释给新手
          </span>
          <button
            onClick={onClose}
            className="p-1 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
          >
            <X size={16} style={{ color: "var(--color-text-muted)" }} />
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto px-6 py-4">
        <div className="max-w-lg mx-auto">
          {/* Loading state */}
          {explainLoading && (
            <div className="flex flex-col items-center justify-center py-12">
              <div
                className="w-8 h-8 rounded-full animate-spin mb-3"
                style={{
                  border: "2px solid var(--color-border)",
                  borderTopColor: "#F97316",
                }}
              />
              <p style={{ fontSize: 13, color: "var(--color-text-muted)" }}>
                正在生成解释题...
              </p>
            </div>
          )}

          {/* Error state */}
          {explainError && !explainLoading && (
            <div
              className="rounded-xl p-4 mb-4"
              style={{
                backgroundColor: "rgba(239, 68, 68, 0.08)",
                border: "1px solid rgba(239, 68, 68, 0.2)",
              }}
            >
              <p style={{ fontSize: 13, color: "var(--color-text-secondary)" }}>
                {explainError}
              </p>
            </div>
          )}

          {/* Explain question */}
          {!explainLoading && explainQuestion && !hasSubmitted && (
            <>
              {/* Concept title */}
              <div className="flex items-start gap-3 mb-4">
                <div
                  className="flex items-center justify-center w-10 h-10 rounded-full shrink-0"
                  style={{ backgroundColor: "rgba(249, 115, 22, 0.1)" }}
                >
                  <Sparkles size={18} style={{ color: "#F97316" }} />
                </div>
                <div>
                  <h2
                    style={{
                      fontSize: 20,
                      fontWeight: 700,
                      fontFamily: "'Cabinet Grotesk', sans-serif",
                      color: "var(--color-text-primary)",
                      lineHeight: 1.3,
                      marginBottom: 2,
                    }}
                  >
                    {explainQuestion.wiki_title}
                  </h2>
                  <p style={{ fontSize: 12, color: "var(--color-text-muted)" }}>
                    用你自己的话解释这个概念
                  </p>
                </div>
              </div>

              {/* Summary */}
              {explainQuestion.wiki_summary && (
                <div
                  className="rounded-xl p-4 mb-4"
                  style={{
                    backgroundColor: "var(--color-surface)",
                    border: "1px solid var(--color-border)",
                  }}
                >
                  <p
                    style={{
                      fontSize: 13,
                      color: "var(--color-text-secondary)",
                      lineHeight: 1.7,
                    }}
                  >
                    {explainQuestion.wiki_summary}
                  </p>
                </div>
              )}

              {/* Prompt */}
              <div
                className="rounded-xl p-4 mb-4"
                style={{
                  backgroundColor: "rgba(249, 115, 22, 0.04)",
                  border: "1px solid rgba(249, 115, 22, 0.12)",
                }}
              >
                <div className="flex items-start gap-2">
                  <MessageSquareText
                    size={16}
                    className="mt-0.5 shrink-0"
                    style={{ color: "#F97316" }}
                  />
                  <p
                    style={{
                      fontSize: 15,
                      color: "var(--color-text-primary)",
                      lineHeight: 1.7,
                      fontWeight: 500,
                    }}
                  >
                    {explainQuestion.prompt}
                  </p>
                </div>
              </div>

              {/* Hint */}
              {explainQuestion.hint && (
                <div className="flex items-start gap-2 mb-5">
                  <Lightbulb
                    size={14}
                    className="mt-0.5 shrink-0"
                    style={{ color: "#F97316" }}
                  />
                  <p
                    style={{
                      fontSize: 12,
                      color: "var(--color-text-muted)",
                      lineHeight: 1.6,
                      fontStyle: "italic",
                    }}
                  >
                    💡 {explainQuestion.hint}
                  </p>
                </div>
              )}

              {/* Text input */}
              <textarea
                ref={textareaRef}
                value={userExplanation}
                onChange={(e) => setUserExplanation(e.target.value)}
                placeholder="在这里输入你的解释..."
                rows={5}
                disabled={explainFeedbackLoading}
                style={{
                  width: "100%",
                  padding: 14,
                  fontSize: 14,
                  fontFamily: "'Plus Jakarta Sans', sans-serif",
                  color: "var(--color-text-primary)",
                  backgroundColor: "var(--color-surface)",
                  border: "1px solid var(--color-border)",
                  borderRadius: 12,
                  outline: "none",
                  resize: "vertical",
                  lineHeight: 1.7,
                }}
                onFocus={(e) => {
                  e.currentTarget.style.borderColor = "#F97316";
                }}
                onBlur={(e) => {
                  e.currentTarget.style.borderColor = "var(--color-border)";
                }}
              />

              {/* Error */}
              {explainError && (
                <div
                  className="rounded-lg p-2 mt-3"
                  style={{
                    fontSize: 12,
                    backgroundColor: "rgba(239, 68, 68, 0.08)",
                    color: "var(--color-text-secondary)",
                  }}
                >
                  <span className="text-red-600 dark:text-red-400">{explainError}</span>
                </div>
              )}
            </>
          )}

          {/* Feedback after submission */}
          {hasSubmitted && explainFeedback && (
            <div className="animate-fadeIn">
              {/* Score */}
              <div className="text-center mb-6">
                <div className="flex justify-center gap-0.5 mb-2">
                  {renderStars(explainFeedback.score)}
                </div>
                <p
                  style={{
                    fontSize: 16,
                    fontWeight: 700,
                    fontFamily: "'Cabinet Grotesk', sans-serif",
                    color: "var(--color-text-primary)",
                  }}
                >
                  {explainFeedback.score}/5 - {explainFeedback.score_label}
                </p>
              </div>

              {/* Strength Points */}
              {explainFeedback.strength_points.length > 0 && (
                <div className="mb-5">
                  <p
                    style={{
                      fontSize: 13,
                      fontWeight: 600,
                      color: "#16A34A",
                      marginBottom: 8,
                    }}
                  >
                    ✅ 做得好的地方：
                  </p>
                  <ul style={{ paddingLeft: 0, listStyle: "none" }}>
                    {explainFeedback.strength_points.map((point, i) => (
                      <li
                        key={i}
                        style={{
                          fontSize: 13,
                          color: "var(--color-text-secondary)",
                          lineHeight: 1.6,
                          marginBottom: 4,
                          paddingLeft: 16,
                          position: "relative",
                        }}
                      >
                        <span
                          style={{
                            position: "absolute",
                            left: 0,
                            color: "#16A34A",
                          }}
                        >
                          •
                        </span>
                        {point}
                      </li>
                    ))}
                  </ul>
                </div>
              )}

              {/* Weakness Points */}
              {explainFeedback.weakness_points.length > 0 && (
                <div className="mb-5">
                  <p
                    style={{
                      fontSize: 13,
                      fontWeight: 600,
                      color: "#F97316",
                      marginBottom: 8,
                    }}
                  >
                    💪 可以改进的地方：
                  </p>
                  <ul style={{ paddingLeft: 0, listStyle: "none" }}>
                    {explainFeedback.weakness_points.map((point, i) => (
                      <li
                        key={i}
                        style={{
                          fontSize: 13,
                          color: "var(--color-text-secondary)",
                          lineHeight: 1.6,
                          marginBottom: 4,
                          paddingLeft: 16,
                          position: "relative",
                        }}
                      >
                        <span
                          style={{
                            position: "absolute",
                            left: 0,
                            color: "#F97316",
                          }}
                        >
                          •
                        </span>
                        {point}
                      </li>
                    ))}
                  </ul>
                </div>
              )}

              {/* Improvement Suggestions */}
              {explainFeedback.improvement_suggestions.length > 0 && (
                <div className="mb-5">
                  <p
                    style={{
                      fontSize: 13,
                      fontWeight: 600,
                      color: "var(--color-text-primary)",
                      marginBottom: 8,
                    }}
                  >
                    📝 改进建议：
                  </p>
                  <ul style={{ paddingLeft: 0, listStyle: "none" }}>
                    {explainFeedback.improvement_suggestions.map((suggestion, i) => (
                      <li
                        key={i}
                        style={{
                          fontSize: 13,
                          color: "var(--color-text-secondary)",
                          lineHeight: 1.6,
                          marginBottom: 4,
                          paddingLeft: 16,
                          position: "relative",
                        }}
                      >
                        <span
                          style={{
                            position: "absolute",
                            left: 0,
                            color: "var(--color-text-muted)",
                          }}
                        >
                          •
                        </span>
                        {suggestion}
                      </li>
                    ))}
                  </ul>
                </div>
              )}

              {/* Better Example */}
              {explainFeedback.better_example && (
                <div
                  className="rounded-xl p-4 mb-4"
                  style={{
                    backgroundColor: "rgba(249, 115, 22, 0.06)",
                    border: "1px solid rgba(249, 115, 22, 0.12)",
                  }}
                >
                  <p
                    style={{
                      fontSize: 12,
                      fontWeight: 600,
                      color: "#F97316",
                      marginBottom: 6,
                    }}
                  >
                    🌟 更好的示例：
                  </p>
                  <p
                    style={{
                      fontSize: 13,
                      color: "var(--color-text-secondary)",
                      lineHeight: 1.7,
                    }}
                  >
                    {explainFeedback.better_example}
                  </p>
                </div>
              )}

              {/* Response time */}
              <p
                style={{
                  fontSize: 11,
                  color: "var(--color-text-muted)",
                  textAlign: "center",
                  marginBottom: 16,
                }}
              >
                用时 {Math.floor(responseTimeSeconds / 60)}:
                {(responseTimeSeconds % 60).toString().padStart(2, "0")}
              </p>
            </div>
          )}

          {/* Feedback loading */}
          {hasSubmitted && explainFeedbackLoading && (
            <div className="flex flex-col items-center justify-center py-8">
              <div
                className="w-8 h-8 rounded-full animate-spin mb-3"
                style={{
                  border: "2px solid var(--color-border)",
                  borderTopColor: "#F97316",
                }}
              />
              <p style={{ fontSize: 13, color: "var(--color-text-muted)" }}>
                AI 正在评估你的解释...
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
          {!hasSubmitted ? (
            <button
              onClick={handleSubmit}
              disabled={
                !userExplanation.trim() || explainFeedbackLoading || explainLoading
              }
              className="w-full inline-flex items-center justify-center gap-2 px-6 py-3 rounded-xl text-sm font-medium transition-all disabled:opacity-50"
              style={{
                backgroundColor: "#F97316",
                color: "white",
              }}
              onMouseEnter={(e) => {
                e.currentTarget.style.backgroundColor = "#EA580C";
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.backgroundColor = "#F97316";
              }}
            >
              {explainFeedbackLoading ? (
                <>评估中...</>
              ) : (
                <>
                  <Send size={16} />
                  提交解释
                </>
              )}
            </button>
          ) : (
            <button
              onClick={handleContinue}
              disabled={explainFeedbackLoading}
              className="w-full inline-flex items-center justify-center gap-2 px-6 py-3 rounded-xl text-sm font-medium transition-all disabled:opacity-50"
              style={{
                backgroundColor: "#F97316",
                color: "white",
              }}
              onMouseEnter={(e) => {
                e.currentTarget.style.backgroundColor = "#EA580C";
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.backgroundColor = "#F97316";
              }}
            >
              继续复习
            </button>
          )}
        </div>
      </div>
    </div>
  );
}
