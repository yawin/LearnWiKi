import { Award, CheckCircle2, XCircle, AlertTriangle, ChevronDown, ChevronUp } from "lucide-react";
import { useState } from "react";
import type { ExamDetail, ExamDiagnosis, ParsedQuestion } from "../../types/learning";

interface ExamReportProps {
  examDetail: ExamDetail;
  onClose: () => void;
}

export function ExamReport({ examDetail, onClose }: ExamReportProps) {
  const { exam, questions } = examDetail;
  const [expandedQ, setExpandedQ] = useState<string | null>(null);

  let diagnosis: ExamDiagnosis | null = null;
  try {
    diagnosis = exam.diagnosis_json ? JSON.parse(exam.diagnosis_json) : null;
  } catch { /* malformed JSON — keep null */ }

  const parseQuestion = (q_json: string): ParsedQuestion => {
    try {
      return JSON.parse(q_json);
    } catch {
      return { stem: "解析错误", question_type: "choice", options: [], correct_answer: "", explanation: "" };
    }
  };

  const gradeColor = (grade: string | null) => {
    switch (grade) {
      case "A": return "#16A34A";
      case "B": return "#F97316";
      case "C": return "#CA8A04";
      case "D": return "#DC2626";
      default: return "var(--color-text-muted)";
    }
  };

  return (
    <div
      className="overflow-y-auto"
      style={{ height: "calc(100vh - 44px)", color: "var(--color-text-primary)" }}
    >
      <div className="px-5 pt-5 pb-8 max-w-2xl mx-auto">
        {/* Score Header */}
        <div
          className="rounded-xl p-6 text-center mb-6"
          style={{
            backgroundColor: "var(--color-surface)",
            border: "1px solid var(--color-border)",
          }}
        >
          <Award size={32} className="mx-auto mb-2" style={{ color: gradeColor(exam.grade) }} />
          <div
            style={{
              fontSize: 48,
              fontFamily: "'Cabinet Grotesk', sans-serif",
              fontWeight: 800,
              color: gradeColor(exam.grade),
            }}
          >
            {exam.score !== null ? Math.round(exam.score) : "--"}
          </div>
          <div style={{ fontSize: 14, color: "var(--color-text-muted)", marginTop: 4 }}>
            等级：<span style={{ fontWeight: 700, color: gradeColor(exam.grade) }}>{exam.grade || "--"}</span>
          </div>

          {/* Stats */}
          <div className="flex justify-center gap-6 mt-4">
            <div className="text-center">
              <div className="flex items-center gap-1 justify-center">
                <CheckCircle2 size={14} className="text-green-500" />
                <span style={{ fontSize: 18, fontWeight: 700, color: "#16A34A" }}>
                  {diagnosis?.total_correct ?? 0}
                </span>
              </div>
              <span style={{ fontSize: 11, color: "var(--color-text-muted)" }}>正确</span>
            </div>
            <div className="text-center">
              <div className="flex items-center gap-1 justify-center">
                <XCircle size={14} className="text-red-500" />
                <span style={{ fontSize: 18, fontWeight: 700, color: "#DC2626" }}>
                  {diagnosis?.total_wrong ?? 0}
                </span>
              </div>
              <span style={{ fontSize: 11, color: "var(--color-text-muted)" }}>错误</span>
            </div>
            <div className="text-center">
              <span style={{ fontSize: 18, fontWeight: 700, color: "var(--color-text-primary)" }}>
                {questions.length}
              </span>
              <br />
              <span style={{ fontSize: 11, color: "var(--color-text-muted)" }}>总题数</span>
            </div>
          </div>
        </div>

        {/* Weak Points */}
        {diagnosis && diagnosis.weak_wiki_pages.length > 0 && (
          <div
            className="rounded-xl p-4 mb-6"
            style={{
              backgroundColor: "#FEF2F2",
              border: "1px solid #FECACA",
            }}
          >
            <div className="flex items-center gap-2 mb-2">
              <AlertTriangle size={16} className="text-red-500" />
              <span style={{ fontSize: 13, fontWeight: 600, color: "#991B1B" }}>
                薄弱知识点（{diagnosis.weak_wiki_pages.length} 个）
              </span>
            </div>
            <p style={{ fontSize: 12, color: "#7F1D1D" }}>
              这些知识点已自动加入高频复习，建议回去重新学习。
            </p>
          </div>
        )}

        {/* Question Review */}
        <div className="mb-6">
          <h3 style={{ fontSize: 14, fontWeight: 600, marginBottom: 12, color: "var(--color-text-primary)" }}>
            逐题解析
          </h3>
          <div className="space-y-2">
            {questions.map((q, i) => {
              const parsed = parseQuestion(q.question_json);
              const expanded = expandedQ === q.id;
              return (
                <div
                  key={q.id}
                  className="rounded-lg overflow-hidden"
                  style={{
                    backgroundColor: "var(--color-surface)",
                    border: "1px solid var(--color-border)",
                  }}
                >
                  <button
                    onClick={() => setExpandedQ(expanded ? null : q.id)}
                    className="w-full flex items-center gap-2 p-3 text-left"
                  >
                    <span
                      className="w-5 h-5 rounded-full flex items-center justify-center shrink-0"
                      style={{
                        backgroundColor: q.is_correct === true ? "#DCFCE7" :
                          q.is_correct === false ? "#FEE2E2" : "#F3F4F6",
                      }}
                    >
                      {q.is_correct === true ? (
                        <CheckCircle2 size={12} className="text-green-600" />
                      ) : q.is_correct === false ? (
                        <XCircle size={12} className="text-red-600" />
                      ) : (
                        <span style={{ fontSize: 10, color: "var(--color-text-muted)" }}>{i + 1}</span>
                      )}
                    </span>
                    <span
                      className="flex-1 truncate"
                      style={{ fontSize: 13, color: "var(--color-text-primary)" }}
                    >
                      {parsed.stem}
                    </span>
                    {expanded ? <ChevronUp size={14} /> : <ChevronDown size={14} />}
                  </button>

                  {expanded && (
                    <div className="px-3 pb-3 pt-0">
                      <div className="border-t pt-2" style={{ borderColor: "var(--color-border)" }}>
                        {q.user_answer && (
                          <p style={{ fontSize: 12, marginBottom: 4 }}>
                            <span style={{ color: "var(--color-text-muted)" }}>你的答案：</span>
                            <span style={{ color: q.is_correct ? "#16A34A" : "#DC2626", fontWeight: 500 }}>
                              {q.user_answer}
                            </span>
                          </p>
                        )}
                        <p style={{ fontSize: 12, marginBottom: 4 }}>
                          <span style={{ color: "var(--color-text-muted)" }}>正确答案：</span>
                          <span style={{ color: "#16A34A", fontWeight: 500 }}>
                            {q.correct_answer || parsed.correct_answer}
                          </span>
                        </p>
                        {parsed.explanation && (
                          <p style={{ fontSize: 12, color: "var(--color-text-secondary)", marginTop: 8 }}>
                            💡 {parsed.explanation}
                          </p>
                        )}
                        {q.ai_feedback && (
                          <p style={{ fontSize: 12, color: "var(--color-text-secondary)", marginTop: 4 }}>
                            🤖 {q.ai_feedback}
                          </p>
                        )}
                      </div>
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        </div>

        {/* Export Markdown */}
        <button
          onClick={async () => {
            // Generate Markdown
            const lines: string[] = [];
            lines.push(`# ${exam.title || "考试结果"}`);
            lines.push("");
            lines.push(`- 分数: ${Math.round(exam.score ?? 0)}`);
            lines.push(`- 等级: ${exam.grade || "--"}`);
            lines.push(`- 总题数: ${questions.length}`);
            if (diagnosis) {
              lines.push(`- 正确: ${diagnosis.total_correct} · 错误: ${diagnosis.total_wrong}`);
            }
            lines.push("");
            lines.push("## 逐题解析");
            questions.forEach((q, i) => {
              const parsed = parseQuestion(q.question_json);
              lines.push(`### ${i + 1}. ${parsed.stem}`);
              lines.push(`- 你的答案: ${q.user_answer || "未作答"}`);
              lines.push(`- 正确答案: ${q.correct_answer || parsed.correct_answer}`);
              lines.push(`- 结果: ${q.is_correct ? "✅ 正确" : "❌ 错误"}`);
              if (parsed.explanation) lines.push(`- 解析: ${parsed.explanation}`);
              if (q.ai_feedback) lines.push(`- AI 反馈: ${q.ai_feedback}`);
              lines.push("");
            });
            const md = lines.join("\n");

            // Copy to clipboard
            try {
              await navigator.clipboard.writeText(md);
              alert("已复制 Markdown 到剪贴板");
            } catch {
              // silently fail
            }
          }}
          className="w-full py-2 rounded-lg font-medium transition-colors mb-2 text-sm"
          style={{
            backgroundColor: "var(--color-surface)",
            border: "1px solid var(--color-border)",
            color: "var(--color-text-secondary)",
          }}
        >
          📥 导出 Markdown
        </button>

        {/* Close button */}
        <button
          onClick={onClose}
          className="w-full py-2.5 rounded-lg font-medium transition-colors"
          style={{
            fontSize: 14,
            backgroundColor: "#F97316",
            color: "white",
          }}
        >
          返回目标
        </button>
      </div>
    </div>
  );
}
