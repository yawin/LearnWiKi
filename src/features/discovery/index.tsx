import { useState } from "react";
import DiscoveryCard from "./DiscoveryCard";
import ReadingInbox from "./ReadingInbox";

type Mode = "card" | "inbox";

/**
 * DiscoveryIndex is an internal state switcher that toggles between
 * the compact Dashboard Card view and the full Reading Inbox page.
 *
 * It is NOT a router — it uses React state for the toggle.
 * The parent (LearningDashboard) renders this component inline.
 */
export default function DiscoveryIndex() {
  const [mode, setMode] = useState<Mode>("card");

  if (mode === "inbox") {
    return <ReadingInbox onBack={() => setMode("card")} />;
  }

  return <DiscoveryCard onOpenInbox={() => setMode("inbox")} />;
}
