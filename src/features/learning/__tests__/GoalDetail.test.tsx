import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { GoalDetail } from "../GoalDetail";

const { state } = vi.hoisted(() => ({
  state: {
    goalResult: {
      id: "g1",
      title: "掌握 Rust 所有权",
      description: "",
      keywords: "[]",
      status: "active",
      progress: 45,
      created_at: "",
      updated_at: "",
    },
    linksResult: [
      {
        id: "l1",
        goal_id: "g1",
        wiki_page_id: "uuid-aaa",
        wiki_title: "所有权入门",
        relevance_score: 0.85,
        source: "auto",
        is_new: true,
        created_at: "",
        review_count: 0,
        next_review_at: null,
        last_reviewed_at: null,
      },
      {
        id: "l2",
        goal_id: "g1",
        wiki_page_id: "uuid-bbb",
        wiki_title: "Borrow Checker",
        relevance_score: 0.72,
        source: "auto",
        is_new: false,
        created_at: "",
        review_count: 0,
        next_review_at: null,
        last_reviewed_at: null,
      },
    ],
    readStatuses: {} as Record<string, boolean>,
    reviewSessionsResult: null as any,
  },
}));

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn((cmd: string, args?: any) => {
    switch (cmd) {
      case "get_goal":
        return Promise.resolve(state.goalResult);
      case "get_goal_wiki_pages":
        return Promise.resolve(state.linksResult);
      case "get_wiki_read_status":
        return Promise.resolve(
          state.readStatuses[args?.wikiPageId as string] ?? false
        );
      case "mark_goal_links_seen":
        return Promise.resolve();
      case "get_due_reviews":
        return Promise.resolve([{ schedule: { id: "s1", wiki_page_id: "uuid-aaa" }, wiki_title: "t", wiki_summary: null, wiki_tags: null, next_format: "quiz" }]);
      case "get_goal_recommendations":
        return Promise.resolve([]);
      case "get_goal_review_sessions":
        return Promise.resolve(state.reviewSessionsResult ?? []);
      case "get_goal_exams":
        return Promise.resolve([]);
      default:
        return Promise.resolve(null);
    }
  }),
}));

describe("GoalDetail", () => {
  beforeEach(() => {
    state.readStatuses = {};
    vi.clearAllMocks();
  });

  it("shows wiki titles instead of UUIDs", async () => {
    render(
      <GoalDetail goalId="g1" onBack={vi.fn()} />
    );

    await waitFor(() => {
      expect(screen.getByText("所有权入门")).toBeInTheDocument();
      expect(screen.getByText("Borrow Checker")).toBeInTheDocument();
    });

    expect(screen.queryByText("uuid-aaa")).not.toBeInTheDocument();
    expect(screen.queryByText("uuid-bbb")).not.toBeInTheDocument();
  });

  it("renders both wiki titles without emoji", async () => {
    state.readStatuses = { "uuid-aaa": true, "uuid-bbb": false };

    render(
      <GoalDetail goalId="g1" onBack={vi.fn()} />
    );

    await waitFor(() => {
      expect(screen.getByText("所有权入门")).toBeInTheDocument();
      expect(screen.getByText("Borrow Checker")).toBeInTheDocument();
    });

    // Emoji icons replaced with Lucide components
    expect(screen.queryByText("✅")).not.toBeInTheDocument();
    expect(screen.queryByText("⬜")).not.toBeInTheDocument();
  });

  it("groups unread items in 待阅读 and read items in 复习概览", async () => {
    state.readStatuses = { "uuid-aaa": false, "uuid-bbb": true };

    render(
      <GoalDetail goalId="g1" onBack={vi.fn()} />
    );

    await waitFor(() => {
      expect(screen.getByText(/待阅读/)).toBeInTheDocument();
      expect(screen.getByText("复习概览")).toBeInTheDocument();
    });

    // unread item (uuid-aaa / 所有权入门) should be in 待阅读 section
    // read item (uuid-bbb / Borrow Checker) should be in 复习概览 section
    const unreadSection = screen.getByText(/待阅读/).closest("div")?.parentElement;
    expect(unreadSection?.textContent).toContain("所有权入门");

    const reviewCard = screen.getByText("复习概览").closest("div")?.parentElement;
    expect(reviewCard?.textContent).toContain("Borrow Checker");
  });

  it("dispatches navigate-to-wiki-page event on click", async () => {
    const listener = vi.fn();
    window.addEventListener("navigate-to-wiki-page", listener);

    render(
      <GoalDetail goalId="g1" onBack={vi.fn()} />
    );

    await waitFor(() => screen.getByText("所有权入门"));
    fireEvent.click(screen.getByText("所有权入门"));

    expect(listener).toHaveBeenCalled();
    const event = listener.mock.calls[0][0] as CustomEvent;
    expect(event.detail.pageId).toBe("uuid-aaa");

    window.removeEventListener("navigate-to-wiki-page", listener);
  });

  it("updates read status when wiki-read-status-changed event fires", async () => {
    render(<GoalDetail goalId="g1" onBack={vi.fn()} />);

    await waitFor(() => screen.getByText("所有权入门"));

    // Initially both items are unread (in 待阅读 section)
    expect(screen.getByText(/待阅读 \(2\)/)).toBeInTheDocument();
    expect(screen.queryByText("复习概览")).not.toBeInTheDocument();

    // Fire event: mark uuid-aaa as read
    window.dispatchEvent(new CustomEvent("wiki-read-status-changed", {
      detail: { wikiPageId: "uuid-aaa", isRead: true },
    }));

    // After event: uuid-aaa moves to 复习概览, uuid-bbb stays in 待阅读
    await waitFor(() => {
      expect(screen.getByText(/待阅读 \(1\)/)).toBeInTheDocument();
      expect(screen.getByText("复习概览")).toBeInTheDocument();
    });

    // 所有权入门 (now read) should be in the review card
    const reviewCard = screen.getByText("复习概览").closest("div")?.parentElement;
    expect(reviewCard?.textContent).toContain("所有权入门");

    // 待阅读 section should only contain Borrow Checker
    const unreadSection = screen.getByText(/待阅读 \(1\)/).closest("div")?.parentElement;
    expect(unreadSection?.textContent).toContain("Borrow Checker");
  });

  it("shows review overview for read links", async () => {
    state.readStatuses = { "uuid-aaa": true, "uuid-bbb": true };
    state.linksResult = [
      { ...state.linksResult[0], next_review_at: "2020-01-01T00:00:00Z", last_reviewed_at: "2020-01-01T00:00:00Z", review_count: 2 } as any,
      { ...state.linksResult[1], next_review_at: "2020-01-01T00:00:00Z", last_reviewed_at: "2020-01-01T00:00:00Z", review_count: 1 } as any,
    ];

    render(<GoalDetail goalId="g1" onBack={vi.fn()} />);

    await waitFor(() => {
      expect(screen.getByText("复习概览")).toBeInTheDocument();
    });
  });

  it("shows exam section", async () => {
    render(<GoalDetail goalId="g1" onBack={vi.fn()} />);

    await waitFor(() => {
      expect(screen.getByText("考试")).toBeInTheDocument();
    });
  });

  it("does not show emoji for read status", async () => {
    state.readStatuses = { "uuid-aaa": true, "uuid-bbb": false };

    render(<GoalDetail goalId="g1" onBack={vi.fn()} />);

    await waitFor(() => screen.getByText("所有权入门"));
    expect(screen.queryByText("✅")).not.toBeInTheDocument();
    expect(screen.queryByText("⬜")).not.toBeInTheDocument();
  });

  it("handles empty review logs gracefully", async () => {
    render(<GoalDetail goalId="g1" onBack={vi.fn()} />);

    await waitFor(() => screen.getByText("所有权入门"));
    // Should not crash
    expect(screen.getByText("所有权入门")).toBeInTheDocument();
  });

  it("groups unread links separately", async () => {
    state.readStatuses = {};

    render(<GoalDetail goalId="g1" onBack={vi.fn()} />);

    await waitFor(() => {
      expect(screen.getByText(/待阅读/)).toBeInTheDocument();
    });
  });

  it("navigates to review detail when clicking a review session item", async () => {
    const listener = vi.fn();
    window.addEventListener("navigate-to-review-log", listener);

    state.readStatuses = { "uuid-aaa": true };
    state.linksResult = [
      {
        id: "l1", goal_id: "g1", wiki_page_id: "uuid-aaa", wiki_title: "所有权入门",
        relevance_score: 0.85, source: "auto", is_new: false, created_at: "",
        review_count: 2, next_review_at: null, last_reviewed_at: null,
      },
    ];
    state.reviewSessionsResult = [
      {
        session_id: "sess-1",
        reviewed_at: "2026-06-11T06:30:00Z",
        total_count: 1,
        correct_count: 1,
        items: [
          {
            log_id: "log-1",
            wiki_page_id: "uuid-aaa",
            wiki_title: "所有权入门",
            quality: 2,
            review_format: "choice",
          },
        ],
      },
    ];

    render(<GoalDetail goalId="g1" onBack={vi.fn()} />);

    // Multiple elements show "所有权入门" — find the review session item button
    const itemTags = await screen.findAllByText("所有权入门");
    const sessionTag = itemTags.find(
      (el) => el.tagName === "BUTTON"
    );
    expect(sessionTag).toBeDefined();
    fireEvent.click(sessionTag!);

    expect(listener).toHaveBeenCalled();
    const event = listener.mock.calls[0][0] as CustomEvent;
    expect(event.detail.logId).toBe("log-1");

    window.removeEventListener("navigate-to-review-log", listener);
  });
});
