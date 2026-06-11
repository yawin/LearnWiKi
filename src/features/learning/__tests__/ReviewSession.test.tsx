import { describe, it, expect, vi } from "vitest";
import { render, waitFor } from "@testing-library/react";

const { fetchDueReviewsMock } = vi.hoisted(() => ({
  fetchDueReviewsMock: vi.fn(() => Promise.resolve()),
}));

// StandardReview uses useTranslation("learning"), so mock react-i18next
vi.mock("react-i18next", () => ({
  useTranslation: () => ({ t: (k: string) => k }),
}));

vi.mock("../../../stores/learningStore", () => {
  const storeState = {
    dueReviews: [
      {
        schedule: {
          id: "s1",
          wiki_page_id: "w1",
          ease_factor: 2.5,
          interval_days: 0,
          next_review_at: "",
          review_count: 0,
          last_reviewed_at: null,
          mastery: 0,
          is_archived: false,
          variant_streak: 0,
          variant_mode: 0,
          created_at: "",
          updated_at: "",
        },
        wiki_title: "Test",
        wiki_summary: null,
        wiki_tags: null,
        next_format: "quiz",
      },
    ],
    currentReviewIndex: 0,
    reviewLoading: false,
    reviewError: null,
    fetchDueReviews: fetchDueReviewsMock,
    submitReview: vi.fn(() => Promise.resolve()),
    fetchClozeQuestion: vi.fn(),
    clozeQuestion: null,
    clozeLoading: false,
    clearCloze: vi.fn(),
    explainLoading: false,
    clearExplain: vi.fn(),
  };

  return {
    useLearningStore: Object.assign(
      (selector?: any) =>
        selector ? selector(storeState) : storeState,
      { getState: () => storeState }
    ),
  };
});

import ReviewSession from "../ReviewSession";

describe("ReviewSession", () => {
  it("calls fetchDueReviews on mount", async () => {
    render(<ReviewSession onClose={vi.fn()} />);

    await waitFor(() => {
      expect(fetchDueReviewsMock).toHaveBeenCalled();
    });
  });
});
