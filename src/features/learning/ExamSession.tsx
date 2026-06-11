import { useState, useEffect, useRef } from "react";
import { FileText, ChevronRight, Loader2, CheckCircle2 } from "lucide-react";
import { getExam, submitExamAnswer, completeExam } from "../../services/learningService";
import type { ExamDetail, ExamQuestion, ParsedQuestion } from "../../types/learning";
import { ExamReport } from "./ExamReport";

interface ExamSessionProps {
  examId: string;
  onClose: () => void;
}

export function ExamSession({ examId, onClose }: ExamSessionProps) {
  const [examDetail, setExamDetail] = useState<ExamDetail | null>(null);
  const [currentIndex, setCurrentIndex] = useState(0);
  const [answers, setAnswers] = useState<Record<string, string>>({});
  const [loading, setLoading] = useState(true);
  const [submitting, setSubmitting] = useState(false);
  const [completed, setCompleted] = useState(false);
  const [completedDetail, setCompletedDetail] = useState<ExamDetail | null>(null);
  const [examConfig, setExamConfig] = useState<{choice:number;judgment:number;essay:number} | null>(null);
  const [confirmed, setConfirmed] = useState(false);
  const autoAdvanceRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    loadExam();
    return () => {
      if (autoAdvanceRef.current) clearTimeout(autoAdvanceRef.current);
    };
  }, [examId]);

  const loadExam = async () => {
    try {
      const detail = await getExam(examId);
      setExamDetail(detail);
      const config = detail.exam.question_config
        ? JSON.parse(detail.exam.question_config)
        : { choice: detail.questions.filter(q => q.question_type === "choice").length,
            judgment: detail.questions.filter(q => q.question_type === "judgment").length,
            essay: detail.questions.filter(q => q.question_type === "essay").length };
      setExamConfig(config);
      if (detail.exam.status === "completed") {
        setCompleted(true);
        setCompletedDetail(detail);
      }
    } catch (err) {
      console.error("Failed to load exam:", err);
    } finally {
      setLoading(false);
    }
  };

  const parseQuestion = (q: ExamQuestion): ParsedQuestion => {
    try {
      return JSON.parse(q.question_json);
    } catch {
      return { stem: "解析错误", question_type: q.question_type, options: [], correct_answer: "", explanation: "" };
    }
  };

  const handleAnswer = async (questionId: string, answer: string) => {
    setAnswers(prev => ({ ...prev, [questionId]: answer }));
    try {
      await submitExamAnswer(questionId, answer);
    } catch (err) {
      console.error("Failed to submit answer:", err);
    }
    // Auto advance to next question (cancelled if user manually navigates)
    if (examDetail && currentIndex < examDetail.questions.length - 1) {
      autoAdvanceRef.current = setTimeout(() => setCurrentIndex(prev => prev + 1), 300);
    }
  };

  const handleComplete = async () => {
    setSubmitting(true);
    try {
      const result = await completeExam(examId);
      setCompletedDetail(result);
      setCompleted(true);
    } catch (err) {
      console.error("Failed to complete exam:", err);
    } finally {
      setSubmitting(false);
    }
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center" style={{ height: "calc(100vh - 44px)" }}>
        <Loader2 size={24} className="animate-spin text-orange-500" />
      </div>
    );
  }

  if (completed && completedDetail) {
    return <ExamReport examDetail={completedDetail} onClose={onClose} />;
  }

  const goToQuestion = (index: number) => {
    if (autoAdvanceRef.current) { clearTimeout(autoAdvanceRef.current); autoAdvanceRef.current = null; }
    setCurrentIndex(Math.max(0, Math.min(index, questions.length - 1)));
  };

  if (!examDetail) return null;

  if (!confirmed && examConfig) {
    const totalMinutes = Math.round(
      (examConfig.choice || 0) * 1 +
      (examConfig.judgment || 0) * 0.5 +
      (examConfig.essay || 0) * 2
    );
    return (
      <div className="flex items-center justify-center" style={{ height: "calc(100vh - 44px)" }}>
        <div className="rounded-xl p-6 max-w-sm w-full mx-4 text-center"
          style={{ backgroundColor: "var(--color-surface)", border: "1px solid var(--color-border)" }}>
          <h3 className="text-lg font-bold mb-2" style={{ color: "var(--color-text-primary)" }}>
            {examDetail.exam.title || "考试"}
          </h3>
          <p className="text-sm mb-2" style={{ color: "var(--color-text-secondary)" }}>
            选择题 {examConfig.choice || 0} · 判断题 {examConfig.judgment || 0} · 论述题 {examConfig.essay || 0}
          </p>
          <p className="text-sm mb-1" style={{ color: "var(--color-text-secondary)" }}>
            共 {examDetail.questions.length} 题
          </p>
          <p className="text-sm mb-4" style={{ color: "var(--color-text-muted)" }}>
            预计 {totalMinutes} 分钟
          </p>
          <div className="flex gap-2">
            <button onClick={onClose}
              className="flex-1 py-2 rounded-md text-sm border"
              style={{ borderColor: "var(--color-border)", color: "var(--color-text-secondary)" }}>取消</button>
            <button onClick={() => setConfirmed(true)}
              className="flex-1 py-2 rounded-md text-sm font-medium text-white"
              style={{ backgroundColor: "#F97316" }}>开始考试</button>
          </div>
        </div>
      </div>
    );
  }

  const { questions } = examDetail;
  const safeIndex = Math.min(currentIndex, questions.length - 1);
  if (safeIndex !== currentIndex) setCurrentIndex(safeIndex);
  const currentQ = questions[safeIndex];
  if (!currentQ) return null;
  const parsed = parseQuestion(currentQ);
  const answeredCount = Object.keys(answers).length;
  const allAnswered = answeredCount === questions.length;

  return (
    <div
      className="overflow-y-auto"
      style={{ height: "calc(100vh - 44px)", color: "var(--color-text-primary)" }}
    >
      <div className="px-5 pt-5 pb-8 max-w-2xl mx-auto">
        {/* Header */}
        <div className="flex items-center justify-between mb-6">
          <div className="flex items-center gap-2">
            <FileText size={18} className="text-orange-500" />
            <span style={{ fontSize: 14, fontWeight: 600 }}>
              {examDetail.exam.title || "考试"}
            </span>
          </div>
          <span style={{ fontSize: 12, color: "var(--color-text-muted)" }}>
            {currentIndex + 1} / {questions.length}
          </span>
        </div>

        {/* Progress bar */}
        <div className="mb-6">
          <div
            className="h-1.5 rounded-full overflow-hidden"
            style={{ backgroundColor: "var(--color-border)" }}
          >
            <div
              className="h-full rounded-full transition-all"
              style={{
                width: `${((currentIndex + 1) / questions.length) * 100}%`,
                backgroundColor: "#F97316",
              }}
            />
          </div>
        </div>

        {/* Question type badge */}
        <div className="mb-3">
          <span
            className="inline-flex px-2 py-0.5 rounded text-xs font-medium"
            style={{
              backgroundColor: currentQ.question_type === "choice" ? "#DBEAFE" :
                currentQ.question_type === "judgment" ? "#FEF3C7" : "#E0E7FF",
              color: currentQ.question_type === "choice" ? "#1D4ED8" :
                currentQ.question_type === "judgment" ? "#92400E" : "#3730A3",
            }}
          >
            {currentQ.question_type === "choice" ? "选择题" :
             currentQ.question_type === "judgment" ? "判断题" : "论述题"}
          </span>
        </div>

        {/* Question stem */}
        <h3
          className="mb-5"
          style={{ fontSize: 16, fontWeight: 600, lineHeight: 1.6, color: "var(--color-text-primary)" }}
        >
          {parsed.stem}
        </h3>

        {/* Answer area */}
        {currentQ.question_type === "choice" && (
          <div className="space-y-2">
            {parsed.options.map((opt, i) => {
              const selected = answers[currentQ.id] === opt;
              return (
                <button
                  key={i}
                  onClick={() => handleAnswer(currentQ.id, opt)}
                  className="w-full text-left rounded-lg p-3 transition-all"
                  style={{
                    backgroundColor: selected ? "#FFF7ED" : "var(--color-surface)",
                    border: `1px solid ${selected ? "#F97316" : "var(--color-border)"}`,
                  }}
                >
                  <span style={{ fontSize: 14, color: "var(--color-text-primary)" }}>
                    {String.fromCharCode(65 + i)}. {opt}
                  </span>
                </button>
              );
            })}
          </div>
        )}

        {currentQ.question_type === "judgment" && (
          <div className="flex gap-3">
            {["对", "错"].map((opt) => {
              const selected = answers[currentQ.id] === opt;
              return (
                <button
                  key={opt}
                  onClick={() => handleAnswer(currentQ.id, opt)}
                  className="flex-1 py-4 rounded-lg text-center font-medium transition-all"
                  style={{
                    fontSize: 16,
                    backgroundColor: selected ? "#FFF7ED" : "var(--color-surface)",
                    border: `1px solid ${selected ? "#F97316" : "var(--color-border)"}`,
                    color: selected ? "#F97316" : "var(--color-text-primary)",
                  }}
                >
                  {opt}
                </button>
              );
            })}
          </div>
        )}

        {currentQ.question_type === "essay" && (
          <div>
            <textarea
              value={answers[currentQ.id] || ""}
              onChange={(e) => setAnswers(prev => ({ ...prev, [currentQ.id]: e.target.value }))}
              placeholder="请输入你的回答..."
              rows={6}
              className="w-full px-3 py-2 rounded-lg outline-none resize-none transition-colors"
              style={{
                fontSize: 14,
                backgroundColor: "var(--color-surface)",
                border: "1px solid var(--color-border)",
                color: "var(--color-text-primary)",
              }}
              onFocus={(e) => (e.target.style.borderColor = "#F97316")}
              onBlur={(e) => {
                e.target.style.borderColor = "var(--color-border)";
                // Submit essay on blur
                if (e.target.value.trim()) {
                  handleAnswer(currentQ.id, e.target.value.trim());
                }
              }}
            />
          </div>
        )}

        {/* Navigation */}
        <div className="flex items-center justify-between mt-6">
          <button
            onClick={() => goToQuestion(currentIndex - 1)}
            disabled={currentIndex === 0}
            className="px-3 py-1.5 rounded-md transition-colors disabled:opacity-30"
            style={{ fontSize: 13, color: "var(--color-text-secondary)" }}
          >
            上一题
          </button>

          {currentIndex < questions.length - 1 ? (
            <button
              onClick={() => goToQuestion(currentIndex + 1)}
              className="inline-flex items-center gap-1 px-3 py-1.5 rounded-md text-orange-500 font-medium transition-colors"
              style={{ fontSize: 13 }}
            >
              下一题
              <ChevronRight size={14} />
            </button>
          ) : allAnswered ? (
            <button
              onClick={handleComplete}
              disabled={submitting}
              className="inline-flex items-center gap-1.5 px-4 py-2 rounded-md text-white font-medium transition-colors disabled:opacity-50"
              style={{ fontSize: 13, backgroundColor: "#F97316" }}
            >
              {submitting ? <Loader2 size={14} className="animate-spin" /> : <CheckCircle2 size={14} />}
              {submitting ? "评分中..." : "提交考试"}
            </button>
          ) : (
            <span style={{ fontSize: 12, color: "var(--color-text-muted)" }}>
              已答 {answeredCount}/{questions.length}
            </span>
          )}
        </div>

        {/* Question dots navigation */}
        <div className="flex flex-wrap gap-1.5 mt-6 justify-center">
          {questions.map((q, i) => (
            <button
              key={q.id}
              onClick={() => goToQuestion(i)}
              className="w-6 h-6 rounded-full text-xs font-medium transition-all"
              style={{
                backgroundColor: i === currentIndex ? "#F97316" :
                  answers[q.id] ? "#FFF7ED" : "var(--color-surface)",
                border: `1px solid ${i === currentIndex ? "#F97316" :
                  answers[q.id] ? "#FDBA74" : "var(--color-border)"}`,
                color: i === currentIndex ? "white" :
                  answers[q.id] ? "#F97316" : "var(--color-text-muted)",
              }}
            >
              {i + 1}
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}
