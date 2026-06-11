import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from 'react-i18next';

interface Question {
  id: number;
  stem: string;
  question_type: 'choice' | 'true_false' | 'short_answer';
  options: string[];
  correct_index: number;
  explanation: string;
  source_page_title: string;
}

interface QuizData {
  id: string;
  title: string;
  questions: Question[];
  source_page_ids: string[];
  created_at: string;
}

interface QuizAnswer {
  question_id: number;
  selected_index: number;
  short_answer_text: string | null;
}

interface QuestionResult {
  question_id: number;
  stem: string;
  correct: boolean;
  correct_index: number;
  selected_index: number;
  explanation: string;
  source_page_id: string | null;
  source_page_title: string | null;
  short_answer_text: string | null;
}

interface QuizResult {
  total: number;
  correct: number;
  score_percent: number;
  answers: QuestionResult[];
  weak_pages: { page_id: string; page_title: string; wrong_count: number; total_related: number }[];
  review_suggestions: string[];
}

type Stage = 'setup' | 'quiz' | 'result';

export function ComprehensiveQuiz({ wikiPageIds, learningPathId, onClose }: {
  wikiPageIds?: string[];
  learningPathId?: string;
  onClose?: () => void;
}) {
  const { t } = useTranslation('learning');
  const [stage, setStage] = useState<Stage>('setup');
  const [quiz, setQuiz] = useState<QuizData | null>(null);
  const [answers, setAnswers] = useState<Map<number, number>>(new Map());
  const [shortAnswers, setShortAnswers] = useState<Map<number, string>>(new Map());
  const [currentIndex, setCurrentIndex] = useState(0);
  const [result, setResult] = useState<QuizResult | null>(null);
  const [loading, setLoading] = useState(false);
  const [questionCount, setQuestionCount] = useState(10);
  const [error, setError] = useState('');

  const generateQuiz = async () => {
    setLoading(true);
    setError('');
    try {
      const data = await invoke<QuizData>('comprehensive_generate_quiz', {
        wikiPageIds: wikiPageIds || [],
        learningPathId: learningPathId || null,
        count: questionCount,
      });
      setQuiz(data);
      setAnswers(new Map());
      setShortAnswers(new Map());
      setCurrentIndex(0);
      setStage('quiz');
    } catch (e) {
      setError(t('comprehensiveQuiz.generateFailed', { error: `${e}` }));
    } finally {
      setLoading(false);
    }
  };

  const selectAnswer = (questionId: number, optionIndex: number) => {
    setAnswers(prev => new Map(prev).set(questionId, optionIndex));
  };

  const setShortAnswer = (questionId: number, text: string) => {
    setShortAnswers(prev => new Map(prev).set(questionId, text));
  };

  const submitQuiz = async () => {
    if (!quiz) return;
    setLoading(true);
    try {
      const answerList: QuizAnswer[] = quiz.questions.map(q => ({
        question_id: q.id,
        selected_index: q.question_type === 'short_answer' ? -1 : (answers.get(q.id) ?? -1),
        short_answer_text: q.question_type === 'short_answer' ? (shortAnswers.get(q.id) || '') : null,
      }));
      const data = await invoke<QuizResult>('comprehensive_submit_quiz', {
        quiz,
        answers: answerList,
      });
      setResult(data);
      setStage('result');
    } catch (e) {
      setError(t('comprehensiveQuiz.submitFailed', { error: `${e}` }));
    } finally {
      setLoading(false);
    }
  };

  if (stage === 'setup') {
    return (
      <div className="p-6 max-w-2xl mx-auto">
        <h2 className="text-2xl font-bold mb-4">{t('comprehensiveQuiz.title')}</h2>
        <p className="text-gray-500 mb-6">{t('comprehensiveQuiz.description')}</p>

        <div className="mb-4">
          <label className="block text-sm font-medium mb-1">{t('comprehensiveQuiz.questionCount')}</label>
          <input
            type="range"
            min={5}
            max={20}
            value={questionCount}
            onChange={e => setQuestionCount(Number(e.target.value))}
            className="w-full"
          />
          <span className="text-sm text-gray-400">{questionCount} {t('comprehensiveQuiz.questionsUnit')}</span>
        </div>

        <div className="bg-blue-50 p-4 rounded-lg mb-6">
          <h3 className="font-medium mb-2">{t('comprehensiveQuiz.instructionsTitle')}</h3>
          <ul className="text-sm text-gray-600 space-y-1">
            <li>• {t('comprehensiveQuiz.instructionMixed')}</li>
            <li>• {t('comprehensiveQuiz.instructionLevels')}</li>
            <li>• {t('comprehensiveQuiz.instructionReview')}</li>
          </ul>
        </div>

        {error && <p className="text-red-500 mb-4">{error}</p>}

        <button
          onClick={generateQuiz}
          disabled={loading}
          className="bg-blue-600 text-white px-6 py-2 rounded-lg hover:bg-blue-700 disabled:opacity-50"
        >
          {loading ? t('comprehensiveQuiz.generating') : t('comprehensiveQuiz.startQuiz')}
        </button>
      </div>
    );
  }

  if (stage === 'quiz' && quiz) {
    const q = quiz.questions[currentIndex];
    const isLast = currentIndex === quiz.questions.length - 1;
    const isShortAnswer = q.question_type === 'short_answer';
    const answered = isShortAnswer
      ? (shortAnswers.get(q.id)?.trim() || '').length > 0
      : answers.has(q.id);

    return (
      <div className="p-6 max-w-2xl mx-auto">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-xl font-bold">{quiz.title}</h2>
          <span className="text-sm text-gray-400">
            {currentIndex + 1} / {quiz.questions.length}
          </span>
        </div>

        <div className="w-full bg-gray-200 rounded-full h-2 mb-6">
          <div
            className="bg-blue-600 h-2 rounded-full transition-all"
            style={{ width: `${((currentIndex + 1) / quiz.questions.length) * 100}%` }}
          />
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <div className="mb-2">
            <span className="text-xs px-2 py-1 bg-gray-100 rounded">
              {q.question_type === 'choice' ? t('comprehensiveQuiz.choice') :
               q.question_type === 'true_false' ? t('comprehensiveQuiz.trueFalse') : t('comprehensiveQuiz.shortAnswer')}
            </span>
          </div>
          <p className="text-lg font-medium mb-4">{q.stem}</p>

          {isShortAnswer ? (
            <div>
              <textarea
                className="w-full border rounded-lg p-3 min-h-[120px]"
                placeholder={t('comprehensiveQuiz.shortAnswerPlaceholder')}
                value={shortAnswers.get(q.id) || ''}
                onChange={e => setShortAnswer(q.id, e.target.value)}
              />
              {/* L19: Note explaining short-answer questions are not auto-graded */}
              <p className="text-xs text-amber-600 mt-1">
                {t('comprehensiveQuiz.shortAnswerNote')}
              </p>
            </div>
          ) : (
            <div className="space-y-2">
              {q.options.map((opt, i) => (
                <button
                  key={i}
                  onClick={() => selectAnswer(q.id, i)}
                  className={`w-full text-left p-3 rounded-lg border transition-colors ${
                    answers.get(q.id) === i
                      ? 'border-blue-500 bg-blue-50'
                      : 'border-gray-200 hover:border-gray-300'
                  }`}
                >
                  <span className="font-medium mr-2">
                    {q.question_type === 'true_false' ? '' : String.fromCharCode(65 + i)}.
                  </span>
                  {opt}
                </button>
              ))}
            </div>
          )}
        </div>

        <div className="flex justify-between mt-6">
          <button
            onClick={() => setCurrentIndex(Math.max(0, currentIndex - 1))}
            disabled={currentIndex === 0}
            className="px-4 py-2 border rounded-lg disabled:opacity-30"
          >
            {t('comprehensiveQuiz.previous')}
          </button>

          {isLast ? (
            <button
              onClick={submitQuiz}
              disabled={loading || !answered}
              className="bg-green-600 text-white px-6 py-2 rounded-lg hover:bg-green-700 disabled:opacity-50"
            >
              {loading ? t('comprehensiveQuiz.submitting') : t('comprehensiveQuiz.submit')}
            </button>
          ) : (
            <button
              onClick={() => setCurrentIndex(currentIndex + 1)}
              className="bg-blue-600 text-white px-4 py-2 rounded-lg"
            >
              {t('comprehensiveQuiz.next')}
            </button>
          )}
        </div>
      </div>
    );
  }

  if (stage === 'result' && result) {
    return (
      <div className="p-6 max-w-2xl mx-auto">
        <h2 className="text-2xl font-bold mb-4">{t('comprehensiveQuiz.resultTitle')}</h2>

        <div className="text-center mb-6">
          <div className={`text-5xl font-bold mb-2 ${
            result.score_percent >= 80 ? 'text-green-600' :
            result.score_percent >= 60 ? 'text-yellow-600' : 'text-red-600'
          }`}>
            {Math.round(result.score_percent)}%
          </div>
          <p className="text-gray-500">
            {t('comprehensiveQuiz.correctCount', { correct: result.correct, total: result.total })}
          </p>
        </div>

        <div className="space-y-4 mb-6">
          {result.answers.map((a, i) => (
            <div key={i} className={`p-4 rounded-lg border ${
              a.correct ? 'border-green-200 bg-green-50' : 'border-red-200 bg-red-50'
            }`}>
              <div className="flex items-start gap-2">
                <span className={`mt-0.5 ${a.correct ? 'text-green-600' : 'text-red-600'}`}>
                  {a.correct ? '✓' : '✗'}
                </span>
                <div>
                  <p className="font-medium">{a.stem}</p>
                  {a.short_answer_text != null ? (
                    <p className="text-sm text-gray-500 mt-1">
                      {t('comprehensiveQuiz.yourAnswer', { answer: a.short_answer_text || t('comprehensiveQuiz.noAnswer') })}
                    </p>
                  ) : !a.correct && (
                    <p className="text-sm text-gray-500 mt-1">
                      {t('comprehensiveQuiz.yourChoice', { letter: a.selected_index >= 0 ? String.fromCharCode(65 + a.selected_index) : '-' })}
                    </p>
                  )}
                  <p className="text-sm text-gray-500 mt-1">{a.explanation}</p>
                  {a.source_page_title && (
                    <p className="text-xs text-gray-400 mt-1">{t('comprehensiveQuiz.source', { title: a.source_page_title })}</p>
                  )}
                </div>
              </div>
            </div>
          ))}
        </div>

        {result.weak_pages.length > 0 && (
          <div className="bg-orange-50 p-4 rounded-lg mb-6">
            <h3 className="font-medium mb-2">{t('comprehensiveQuiz.weakPointsTitle')}</h3>
            <ul className="text-sm text-gray-600 space-y-1">
              {result.weak_pages.map((w, i) => (
                <li key={i}>• {w.page_title}{t('comprehensiveQuiz.wrongCount', { count: w.wrong_count })}</li>
              ))}
            </ul>
          </div>
        )}

        {result.review_suggestions.length > 0 && (
          <div className="bg-yellow-50 p-4 rounded-lg mb-6">
            <h3 className="font-medium mb-2">{t('comprehensiveQuiz.reviewSuggestions')}</h3>
            <ul className="text-sm text-gray-600 space-y-1">
              {result.review_suggestions.map((s, i) => (
                <li key={i}>• {s}</li>
              ))}
            </ul>
          </div>
        )}

        <button
          onClick={onClose}
          className="bg-gray-600 text-white px-6 py-2 rounded-lg hover:bg-gray-700"
        >
          {t('comprehensiveQuiz.close')}
        </button>
      </div>
    );
  }

  return null;
}
