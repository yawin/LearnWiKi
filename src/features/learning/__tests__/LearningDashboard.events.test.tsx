import { describe, it, expect, vi } from "vitest";
import { render, waitFor } from "@testing-library/react";
import { LearningDashboard } from "../LearningDashboard";

const { listeners, state } = vi.hoisted(() => {
  const listeners: Record<string, Array<() => void>> = {};
  const state = { invokeCalls: [] as string[] };
  return { listeners, state };
});

vi.mock("@tauri-apps/api/event", () => ({
  listen: (event: string, cb: () => void) => {
    listeners[event] = listeners[event] || [];
    listeners[event].push(cb);
    return Promise.resolve(() => {
      listeners[event] = listeners[event].filter((c) => c !== cb);
    });
  },
}));

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn((cmd: string) => {
    state.invokeCalls.push(cmd);
    return Promise.resolve([]);
  }),
}));

describe("LearningDashboard event listeners", () => {
  it("loads goals on mount", async () => {
    state.invokeCalls.length = 0;
    const onSelectGoal = vi.fn();
    render(<LearningDashboard onSelectGoal={onSelectGoal} />);

    await waitFor(() =>
      expect(state.invokeCalls.filter((c) => c === "get_goals").length).toBe(1)
    );
  });
});
