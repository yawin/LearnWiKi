import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { WikiPageCard } from "../WikiPageCard";

const mockPage = {
  id: "wp-1",
  title: "Rust 所有权入门",
  slug: "rust-ownership",
  page_type: "concept" as const,
  body_markdown: "# Rust",
  summary: "理解所有权",
  tags: '["rust"]',
  status: "active" as const,
  confidence: 1.0,
  created_at: "2026-01-01T00:00:00Z",
  updated_at: "2026-01-02T00:00:00Z",
};

describe("WikiPageCard", () => {
  it("shows 📖 when read", () => {
    render(<WikiPageCard page={mockPage} onClick={() => {}} readStatus={true} />);
    expect(screen.getByText("📖")).toBeInTheDocument();
  });

  it("shows 📄 when unread", () => {
    render(<WikiPageCard page={mockPage} onClick={() => {}} readStatus={false} />);
    expect(screen.getByText("📄")).toBeInTheDocument();
  });

  it("shows no indicator when readStatus is undefined", () => {
    render(<WikiPageCard page={mockPage} onClick={() => {}} />);
    expect(screen.queryByText("📖")).not.toBeInTheDocument();
    expect(screen.queryByText("📄")).not.toBeInTheDocument();
  });
});
