import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { LearningSection } from "../LearningSection";
import { useSettingsStore } from "../../../stores/settingsStore";

vi.mock("../../../stores/settingsStore");

describe("LearningSection", () => {
  const setAutoLinkSensitivity = vi.fn();

  beforeEach(() => {
    setAutoLinkSensitivity.mockReset();
    (useSettingsStore as unknown as ReturnType<typeof vi.fn>).mockImplementation(
      (selector: (s: { autoLinkSensitivity: string; setAutoLinkSensitivity: typeof setAutoLinkSensitivity; apiKey: string }) => unknown) =>
        selector({
          autoLinkSensitivity: "balanced",
          setAutoLinkSensitivity,
          apiKey: "sk-test",
        })
    );
  });

  it("renders three sensitivity options with balanced selected by default", () => {
    render(<LearningSection />);
    expect(screen.getByLabelText(/宽松/)).not.toBeChecked();
    expect(screen.getByLabelText(/平衡/)).toBeChecked();
    expect(screen.getByLabelText(/严格/)).not.toBeChecked();
  });

  it("calls setAutoLinkSensitivity when user picks 严格", () => {
    render(<LearningSection />);
    fireEvent.click(screen.getByLabelText(/严格/));
    expect(setAutoLinkSensitivity).toHaveBeenCalledWith("strict");
  });

  it("shows AI-not-configured hint when apiKey is empty", () => {
    (useSettingsStore as unknown as ReturnType<typeof vi.fn>).mockImplementation(
      (selector: (s: { autoLinkSensitivity: string; setAutoLinkSensitivity: typeof setAutoLinkSensitivity; apiKey: string }) => unknown) =>
        selector({
          autoLinkSensitivity: "balanced",
          setAutoLinkSensitivity,
          apiKey: "",
        })
    );
    render(<LearningSection />);
    expect(screen.getByText(/当前未配置 AI/)).toBeInTheDocument();
    // Should still be interactive (not disabled)
    expect(screen.getByLabelText(/严格/)).not.toBeDisabled();
  });
});
