import { useState, useEffect, useCallback } from "react";
import { ChevronLeft, BookOpen, Lightbulb, ArrowRight, CheckCircle2, Loader2 } from "lucide-react";
import { getLearningContent, markAsLearned, generateInstantQuiz } from "../../services/learningService";
import type { QuizQuestion } from "../../types/learning";

interface LearnModeProps {
  wikiPageId: string;
  goalId: string;
  onClose: () => void;
}

type Layer = "concept" | "detail" | "extend";

export function LearnMode({ wikiPageId, goalId, onClose }: LearnModeProps) {
  const [layer, setLayer] = useState<Layer>("concept");
  const [content, setContent] = useState<{ concept: string; detail: string; extend: string; title: string } | null>(null);
  const [loading, setLoading] = useState(true);
  const [quiz, setQuiz] = useState<QuizQuestion[]>([]);
  const [quizDone, setQuizDone] = useState(false);
  const [selectedAnswer, setSelectedAnswer] = useState<number | null>(null);
  const [answered, setAnswered] = useState(false);
  const [quizLoading, setQuizLoading] = useState(false);

  useEffect(() => {
    const load = async () => {
      try {
        const data = await getLearningContent(wikiPageId);
        setContent(data);
      } catch (err) {
        console.error("Failed to load learning content:", err);
      } finally {
        setLoading(false);
      }
    };
    load();
  }, [wikiPageId]);

  const handleStartQuiz = useCallback(async () => {
    setQuizLoading(true);
    try {
      const questions = await generateInstantQuiz(wikiPageId);
      setQuiz(questions.slice(0, 2));
    } catch {
      console.error("Failed to generate quiz");
    } finally {
      setQuizLoading(false);
    }
  }, [wikiPageId]);

  const handleAnswer = (_qIndex: number, answerIndex: number) => {
    setSelectedAnswer(answerIndex);
    setAnswered(true);
  };

  const handleCompleteQuiz = async () => {
    try {
      await markAsLearned(goalId, wikiPageId);
    } catch {}
    setQuizDone(true);
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center py-20">
        <Loader2 size={20} className="animate-spin text-orange-500" />
      </div>
    );
  }

  if (!content) {
    return (
      <div className="px-5 pt-20 text-center text-sm" style={{ color: "var(--color-text-muted)" }}>
        无法加载学习内容
      </div>
    );
  }

  return (
    <div className="px-5 pt-4 pb-8" style={{ color: "var(--color-text-primary)" }}>
      {/* Back */}
      <button
        onClick={onClose}
        className="inline-flex items-center gap-1 mb-4 text-sm hover:text-orange-500 transition-colors"
        style={{ color: "var(--color-text-muted)" }}
      >
        <ChevronLeft size={16} />
        返回
      </button>

      <h2 style={{ fontSize: 20, fontFamily: "'Cabinet Grotesk', sans-serif", fontWeight: 700, marginBottom: 16 }}>
        {content.title}
      </h2>

      {/* Layer tabs */}
      <div className="flex gap-1 mb-5 p-0.5 rounded-md bg-gray-100/60 dark:bg-white/[0.06]">
        {(["concept", "detail", "extend"] as const).map((l) => (
          <button
            key={l}
            onClick={() => setLayer(l)}
            className="flex-1 py-1.5 rounded text-xs font-medium transition-all"
            style={{
              backgroundColor: layer === l ? "var(--color-surface)" : "transparent",
              color: layer === l ? "var(--color-accent)" : "var(--color-text-muted)",
              boxShadow: layer === l ? "0 1px 2px rgba(0,0,0,0.05)" : "none",
            }}
          >
            {l === "concept" ? "核心概念" : l === "detail" ? "详细解释" : "延伸"}
          </button>
        ))}
      </div>

      {/* Content */}
      <div
        className="rounded-xl p-5 mb-6"
        style={{ backgroundColor: "var(--color-surface)", border: "1px solid var(--color-border)" }}
      >
        {layer === "concept" && (
          <div>
            <div className="flex items-center gap-2 mb-3">
              <Lightbulb size={16} className="text-orange-500" />
              <span style={{ fontSize: 14, fontWeight: 600 }}>30 秒核心概念</span>
            </div>
            <p style={{ fontSize: 15, lineHeight: 1.7 }}>{content.concept}</p>
            <button
              onClick={() => setLayer("detail")}
              className="inline-flex items-center gap-1 mt-4 text-orange-500 text-sm font-medium"
            >
              了解更多 <ArrowRight size={14} />
            </button>
          </div>
        )}

        {layer === "detail" && (
          <div>
            <div className="flex items-center gap-2 mb-3">
              <BookOpen size={16} className="text-orange-500" />
              <span style={{ fontSize: 14, fontWeight: 600 }}>详细解释</span>
            </div>
            <div
              className="prose prose-sm dark:prose-invert max-w-none"
              style={{ fontSize: 14, lineHeight: 1.8 }}
              dangerouslySetInnerHTML={{ __html: content.detail }}
            />
            <button
              onClick={() => setLayer("extend")}
              className="inline-flex items-center gap-1 mt-4 text-orange-500 text-sm font-medium"
            >
              查看延伸 <ArrowRight size={14} />
            </button>
          </div>
        )}

        {layer === "extend" && (
          <div>
            <p style={{ fontSize: 14, lineHeight: 1.8 }}>{content.extend}</p>
          </div>
        )}
      </div>

      {/* Instant Quiz */}
      {!quizDone && (
        <div
          className="rounded-xl p-5 mb-6"
          style={{ backgroundColor: "#FFF7ED", border: "1px solid #FDBA74" }}
        >
          {quiz.length === 0 ? (
            <div className="text-center">
              <p style={{ fontSize: 13, color: "var(--color-text-secondary)", marginBottom: 12 }}>
                知识需要即时巩固，来试试快速检测吧
              </p>
              <button
                onClick={handleStartQuiz}
                disabled={quizLoading}
                className="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-orange-500 text-white text-sm font-medium disabled:opacity-50"
              >
                {quizLoading ? <Loader2 size={14} className="animate-spin" /> : <Lightbulb size={14} />}
                {quizLoading ? "生成中..." : "开始检测"}
              </button>
            </div>
          ) : (
            <div>
              <p style={{ fontSize: 14, fontWeight: 600, marginBottom: 12 }}>即时检测</p>
              {quiz.map((q, qi) => (
                <div key={qi} className="mb-4 last:mb-0">
                  <p style={{ fontSize: 13, marginBottom: 8 }}>{q.stem}</p>
                  <div className="space-y-1.5">
                    {q.options.map((opt, oi) => {
                      const isCorrect = oi === q.correct_index;
                      const isSelected = oi === selectedAnswer;
                      const showResult = answered && (isCorrect || isSelected);
                      return (
                        <button
                          key={oi}
                          onClick={() => !answered && handleAnswer(qi, oi)}
                          disabled={answered}
                          className="w-full text-left px-3 py-2 rounded-md text-xs transition-colors"
                          style={{
                            backgroundColor: showResult
                              ? isCorrect ? "#DCFCE7" : isSelected ? "#FEE2E2" : "var(--color-surface)"
                              : "var(--color-surface)",
                            border: `1px solid ${showResult && isCorrect ? "#86EFAC" : showResult && isSelected ? "#FCA5A5" : "var(--color-border)"}`,
                            cursor: answered ? "default" : "pointer",
                          }}
                        >
                          {opt}
                        </button>
                      );
                    })}
                  </div>
                  {answered && (
                    <p style={{ fontSize: 11, color: "var(--color-text-muted)", marginTop: 6 }}>
                      {q.explanation}
                    </p>
                  )}
                </div>
              ))}
              {answered && (
                <button
                  onClick={handleCompleteQuiz}
                  className="inline-flex items-center gap-1.5 px-3 py-1.5 mt-4 rounded-md bg-orange-500 text-white text-sm font-medium"
                >
                  <CheckCircle2 size={14} />
                  完成学习
                </button>
              )}
            </div>
          )}
        </div>
      )}

      {quizDone && (
        <div
          className="rounded-xl p-5 text-center"
          style={{ backgroundColor: "#DCFCE7", border: "1px solid #86EFAC" }}
        >
          <CheckCircle2 size={24} className="mx-auto mb-2 text-green-600" />
          <p style={{ fontSize: 14, fontWeight: 600, color: "#166534" }}>
            学习完成，知识点已加入复习池
          </p>
        </div>
      )}
    </div>
  );
}
