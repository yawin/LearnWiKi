import { create } from "zustand";
import type { ClozeQuestion, DueReviewItem, ErrorHuntQuestion, ExplainFeedback, ExplainQuestion, OrderingSteps, QuizQuestion, ReviewHealthItem, ReviewHealthStats, ReviewStats, VariantQuestion } from "../types/learning";
import {
  getDueReviews,
  submitReviewFeedback,
  getReviewStats,
  getHealthSchedules,
  getReviewHealthStats,
  generateQuizQuestions,
  generateOrderingSteps,
  generateErrorHunt,
  generateCloze,
  generateExplainReview,
  submitExplainAnswer,
  generateVariantQuestion,
} from "../services/learningService";

interface LearningState {
  // Review state
  dueReviews: DueReviewItem[];
  reviewStats: ReviewStats | null;
  currentReviewIndex: number;
  reviewLoading: boolean;
  reviewError: string | null;

  // Quiz
  rapidFireQuestions: QuizQuestion[];
  rapidFireLoading: boolean;
  rapidFireError: string | null;

  // Ordering
  orderingSteps: OrderingSteps | null;
  orderingLoading: boolean;
  orderingError: string | null;

  // Error Hunt
  errorHuntQuestion: ErrorHuntQuestion | null;
  errorHuntLoading: boolean;
  errorHuntError: string | null;

  // Cloze
  clozeQuestion: ClozeQuestion | null;
  clozeLoading: boolean;
  clozeError: string | null;

  // Explain
  explainQuestion: ExplainQuestion | null;
  explainFeedback: ExplainFeedback | null;
  explainLoading: boolean;
  explainFeedbackLoading: boolean;
  explainError: string | null;

  // Variant
  variantQuestion: VariantQuestion | null;
  variantLoading: boolean;
  variantError: string | null;

  // Health
  healthItems: ReviewHealthItem[];
  healthStats: ReviewHealthStats | null;
  healthLoading: boolean;

  isSinglePageReview: boolean;
  reviewSessionId: string | null;

  // Review actions
  fetchDueReviews: () => Promise<void>;
  fetchReviewStats: () => Promise<void>;
  startReview: () => void;
  startSinglePageReview: (wikiPageId: string) => Promise<void>;
  submitReview: (quality: number, reviewFormat?: string, responseTimeSeconds?: number) => Promise<void>;
  nextReview: () => void;

  // Health actions
  fetchHealthSchedules: () => Promise<void>;
  fetchHealthStats: () => Promise<void>;

  // Quiz actions
  fetchQuizQuestions: (wikiPageId: string, count?: number) => Promise<QuizQuestion[]>;
  clearRapidFire: () => void;

  // Ordering actions
  fetchOrderingSteps: (wikiPageId: string) => Promise<OrderingSteps>;
  clearOrdering: () => void;

  // Error Hunt actions
  fetchErrorHuntQuestion: (wikiPageId: string) => Promise<ErrorHuntQuestion>;
  clearErrorHunt: () => void;

  // Cloze actions
  fetchClozeQuestion: (wikiPageId: string) => Promise<ClozeQuestion>;
  clearCloze: () => void;

  // Explain actions
  fetchExplainQuestion: (wikiPageId: string) => Promise<ExplainQuestion>;
  fetchExplainFeedback: (wikiPageId: string, userExplanation: string) => Promise<ExplainFeedback>;
  clearExplain: () => void;

  // Variant actions
  fetchVariantQuestion: (wikiPageId: string, currentFormat: string, variantGeneration: number) => Promise<VariantQuestion>;
  clearVariant: () => void;
}

export const useLearningStore = create<LearningState>((set, get) => ({
  // Initial state
  dueReviews: [],
  reviewStats: null,
  currentReviewIndex: 0,
  reviewLoading: true,
  reviewError: null,
  isSinglePageReview: false,
  reviewSessionId: null,
  rapidFireQuestions: [],
  rapidFireLoading: false,
  rapidFireError: null,
  orderingSteps: null,
  orderingLoading: false,
  orderingError: null,
  errorHuntQuestion: null,
  errorHuntLoading: false,
  errorHuntError: null,
  clozeQuestion: null,
  clozeLoading: false,
  clozeError: null,
  explainQuestion: null,
  explainFeedback: null,
  explainLoading: false,
  explainFeedbackLoading: false,
  explainError: null,
  variantQuestion: null,
  variantLoading: false,
  variantError: null,
  healthItems: [],
  healthStats: null,
  healthLoading: false,

  // Review actions
  fetchDueReviews: async () => {
    set({ reviewLoading: true, reviewError: null });
    try {
      const dueReviews = await getDueReviews();
      set({ dueReviews, reviewLoading: false });
    } catch (e) {
      set({ reviewError: e instanceof Error ? e.message : "获取复习列表失败", reviewLoading: false });
    }
  },

  fetchReviewStats: async () => {
    try {
      const reviewStats = await getReviewStats();
      set({ reviewStats });
    } catch (e) { console.error("Failed to load review stats:", e); }
  },

  startReview: () => {
    const sessionId = `${Date.now()}-${Math.random().toString(36).slice(2)}`;
    set({ currentReviewIndex: 0, reviewLoading: true, reviewSessionId: sessionId });
  },

  startSinglePageReview: async (wikiPageId: string) => {
    console.log("[singleReview] startSinglePageReview called with wikiPageId:", wikiPageId);
    const sessionId = `${Date.now()}-${Math.random().toString(36).slice(2)}`;
    set({ currentReviewIndex: 0, reviewLoading: true, dueReviews: [], isSinglePageReview: true, reviewError: null, reviewSessionId: sessionId });
    try {
      const { getDueReviewForPage, autoCreateReviewSchedule } = await import("../services/learningService");
      // Ensure schedule exists before fetching
      try {
        console.log("[singleReview] calling autoCreateReviewSchedule...");
        const schedule = await autoCreateReviewSchedule(wikiPageId);
        console.log("[singleReview] autoCreateReviewSchedule result:", schedule);
      } catch (e) {
        console.warn("[singleReview] autoCreateReviewSchedule failed:", e);
      }
      console.log("[singleReview] calling getDueReviewForPage...");
      const item = await getDueReviewForPage(wikiPageId);
      console.log("[singleReview] getDueReviewForPage result:", item);
      if (item) {
        console.log("[singleReview] setting dueReviews with 1 item, reviewLoading: false");
        set({ dueReviews: [item], reviewLoading: false });
      } else {
        console.warn("[singleReview] item is null, showing completion");
        set({ dueReviews: [], reviewLoading: false, reviewError: "该知识点暂无复习内容", isSinglePageReview: false });
      }
    } catch (e) {
      console.error("[singleReview] exception:", e);
      set({
        dueReviews: [],
        reviewLoading: false,
        reviewError: e instanceof Error ? e.message : "获取复习内容失败",
        isSinglePageReview: false,
      reviewSessionId: null,
      });
    }
  },

  submitReview: async (quality: number, reviewFormat?: string, responseTimeSeconds?: number) => {
    const { dueReviews, currentReviewIndex, reviewSessionId } = get();
    const current = dueReviews[currentReviewIndex];
    if (!current) return;
    try {
      const format = reviewFormat ?? current.next_format ?? "choice";
      await submitReviewFeedback(current.schedule.id, quality, format, responseTimeSeconds, reviewSessionId ?? undefined);
      if (currentReviewIndex + 1 < dueReviews.length) {
        set({ currentReviewIndex: currentReviewIndex + 1 });
      } else {
        set({ dueReviews: [], currentReviewIndex: 0, isSinglePageReview: false });
        get().fetchDueReviews();
      }
      get().fetchReviewStats();
    } catch (e) {
      set({ reviewError: e instanceof Error ? e.message : "提交反馈失败" });
    }
  },

  nextReview: () => {
    const { currentReviewIndex, dueReviews } = get();
    if (currentReviewIndex + 1 < dueReviews.length) {
      set({ currentReviewIndex: currentReviewIndex + 1 });
    }
  },

  // Health actions
  fetchHealthSchedules: async () => {
    set({ healthLoading: true });
    try {
      const items = await getHealthSchedules();
      set({ healthItems: items, healthLoading: false });
    } catch (e) { console.error(e); set({ healthLoading: false }); }
  },

  fetchHealthStats: async () => {
    try {
      const stats = await getReviewHealthStats();
      set({ healthStats: stats });
    } catch (e) { console.error(e); }
  },

  // Quiz actions
  fetchQuizQuestions: async (wikiPageId: string, count?: number) => {
    set({ rapidFireLoading: true, rapidFireError: null });
    try {
      const questions = await generateQuizQuestions(wikiPageId, count);
      set({ rapidFireQuestions: questions, rapidFireLoading: false });
      return questions;
    } catch (e) {
      const msg = e instanceof Error ? e.message : "生成题目失败";
      set({ rapidFireError: msg, rapidFireLoading: false });
      throw e;
    }
  },

  clearRapidFire: () => set({ rapidFireQuestions: [], rapidFireLoading: false, rapidFireError: null }),

  // Ordering actions
  fetchOrderingSteps: async (wikiPageId: string) => {
    set({ orderingLoading: true, orderingError: null });
    try {
      const steps = await generateOrderingSteps(wikiPageId);
      set({ orderingSteps: steps, orderingLoading: false });
      return steps;
    } catch (e) {
      set({ orderingError: e instanceof Error ? e.message : "生成排序题失败", orderingLoading: false });
      throw e;
    }
  },

  clearOrdering: () => set({ orderingSteps: null, orderingLoading: false, orderingError: null }),

  // Error Hunt actions
  fetchErrorHuntQuestion: async (wikiPageId: string) => {
    set({ errorHuntLoading: true, errorHuntError: null });
    try {
      const question = await generateErrorHunt(wikiPageId);
      set({ errorHuntQuestion: question, errorHuntLoading: false });
      return question;
    } catch (e) {
      set({ errorHuntError: e instanceof Error ? e.message : "生成找茬题失败", errorHuntLoading: false });
      throw e;
    }
  },

  clearErrorHunt: () => set({ errorHuntQuestion: null, errorHuntLoading: false, errorHuntError: null }),

  // Cloze actions
  fetchClozeQuestion: async (wikiPageId: string) => {
    set({ clozeLoading: true, clozeError: null });
    try {
      const question = await generateCloze(wikiPageId);
      set({ clozeQuestion: question, clozeLoading: false });
      return question;
    } catch (e) {
      set({ clozeError: e instanceof Error ? e.message : "生成填空题失败", clozeLoading: false });
      throw e;
    }
  },

  clearCloze: () => set({ clozeQuestion: null, clozeLoading: false, clozeError: null }),

  // Explain actions
  fetchExplainQuestion: async (wikiPageId: string) => {
    set({ explainLoading: true, explainError: null, explainQuestion: null, explainFeedback: null });
    try {
      const question = await generateExplainReview(wikiPageId);
      set({ explainQuestion: question, explainLoading: false });
      return question;
    } catch (e) {
      set({ explainError: e instanceof Error ? e.message : "生成解释题失败", explainLoading: false });
      throw e;
    }
  },

  fetchExplainFeedback: async (wikiPageId: string, userExplanation: string) => {
    set({ explainFeedbackLoading: true, explainError: null });
    try {
      const feedback = await submitExplainAnswer(wikiPageId, userExplanation);
      set({ explainFeedback: feedback, explainFeedbackLoading: false });
      return feedback;
    } catch (e) {
      set({ explainError: e instanceof Error ? e.message : "提交解释失败", explainFeedbackLoading: false });
      throw e;
    }
  },

  clearExplain: () => set({ explainQuestion: null, explainFeedback: null, explainLoading: false, explainFeedbackLoading: false, explainError: null }),

  // Variant actions
  fetchVariantQuestion: async (wikiPageId: string, currentFormat: string, variantGeneration: number) => {
    set({ variantLoading: true, variantError: null, variantQuestion: null });
    try {
      const question = await generateVariantQuestion(wikiPageId, currentFormat, variantGeneration);
      set({ variantQuestion: question, variantLoading: false });
      return question;
    } catch (e) {
      set({ variantError: e instanceof Error ? e.message : "生成变体题失败", variantLoading: false });
      throw e;
    }
  },

  clearVariant: () => set({ variantQuestion: null, variantLoading: false, variantError: null }),
}));
