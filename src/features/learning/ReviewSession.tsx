import { useState, useCallback, useEffect, useRef } from "react";
import { Loader2 } from "lucide-react";
import { useLearningStore } from "../../stores/learningStore";
import ClozeReview from "./ClozeReview";
import ExplainReview from "./ExplainReview";
import { ReviewSummary } from "./review/ReviewSummary";
import { StandardReview } from "./review/StandardReview";

interface ReviewSessionProps {
  onClose: () => void;
}

export default function ReviewSession({ onClose }: ReviewSessionProps) {
  const {
    dueReviews,
    currentReviewIndex,
    reviewLoading,
    reviewError,
    isSinglePageReview,
    fetchDueReviews,
    submitReview,

    clozeQuestion,
    clozeLoading,
    clearCloze,
    explainLoading,
    clearExplain,
  } = useLearningStore();

  const [showFeedback, setShowFeedback] = useState(false);
  const [lastQuality, setLastQuality] = useState<number | null>(null);
  const [correctCount, setCorrectCount] = useState(0);
  const [isComplete, setIsComplete] = useState(false);
  const [elapsedSeconds, setElapsedSeconds] = useState(0);
  const [reviewStarted, setReviewStarted] = useState(false);
  const startTimeRef = useRef(Date.now());

  // Routing states
  const [showCloze, setShowCloze] = useState(false);
  const [showExplain, setShowExplain] = useState(false);

  useEffect(() => {
    const interval = setInterval(() => {
      setElapsedSeconds(Math.floor((Date.now() - startTimeRef.current) / 1000));
    }, 1000);
    return () => clearInterval(interval);
  }, []);

  useEffect(() => {
    console.log("[ReviewSession] mount/update - isSinglePageReview:", isSinglePageReview, "reviewLoading:", reviewLoading, "dueReviews.length:", dueReviews.length);
    if (!isSinglePageReview) {
      console.log("[ReviewSession] fetching due reviews (global mode)");
      fetchDueReviews();
    } else {
      console.log("[ReviewSession] skipping fetch (single-page mode)");
    }
  }, [fetchDueReviews, isSinglePageReview]);

  const current = dueReviews[currentReviewIndex];
  const total = dueReviews.length;

  // Determine format for current review
  const nextFormat = current?.next_format ?? "choice";

  // Start review with format routing
  const handleStartReview = useCallback(async () => {
    if (!current) return;

    if (nextFormat === "essay") {
      setShowExplain(true);
    } else {
      // choice, judgment — standard flow
      setReviewStarted(true);
    }
  }, [current, nextFormat]);

  // Handle cloze completion
  const handleClozeComplete = useCallback((results: { blankResults: boolean[]; allCorrect: boolean }) => {
    setShowCloze(false);
    clearCloze();
    if (results.allCorrect) setCorrectCount((c) => c + 1);

    if (currentReviewIndex + 1 < total) {
      useLearningStore.getState().nextReview();
    } else {
      setIsComplete(true);
    }
  }, [currentReviewIndex, total, clearCloze]);

  // Handle explain completion
  const handleExplainComplete = useCallback((quality: number, responseTimeSeconds: number) => {
    setShowExplain(false);
    if (quality >= 1) setCorrectCount((c) => c + 1);

    clearExplain();
    submitReview(quality, "explain", responseTimeSeconds).then(() => {
      if (currentReviewIndex + 1 >= total) {
        setIsComplete(true);
      }
    });
  }, [currentReviewIndex, total, clearExplain, submitReview]);

  // Standard quality-based feedback
  const handleQuality = useCallback(async (quality: number) => {
    setLastQuality(quality);
    setShowFeedback(true);
    if (quality >= 1) {
      setCorrectCount((c) => c + 1);
    }

    // Brief delay to show feedback, then submit
    setTimeout(async () => {
      await submitReview(quality, nextFormat);
      setShowFeedback(false);
      setLastQuality(null);
      setReviewStarted(false);

      // Check if we're done
      if (currentReviewIndex + 1 >= total) {
        setIsComplete(true);
      }
    }, 800);
  }, [submitReview, currentReviewIndex, total, nextFormat]);

  // Summary screen when all reviews are done
  if (isComplete || (!reviewLoading && total === 0 && dueReviews.length === 0)) {
    console.log("[ReviewSession] showing ReviewSummary — isComplete:", isComplete, "reviewLoading:", reviewLoading, "total:", total, "dueReviews.length:", dueReviews.length);
    return (
      <ReviewSummary
        total={total}
        correctCount={correctCount}
        elapsedSeconds={elapsedSeconds}
        onClose={onClose}
      />
    );
  }

  if (!current) {
    if (reviewLoading) {
      return (
        <div className="flex items-center justify-center" style={{ height: "calc(100vh - 44px)" }}>
          <Loader2 size={24} className="animate-spin text-orange-500" />
        </div>
      );
    }
    return null;
  }

  // Show ClozeReview when activated
  if (showCloze && clozeQuestion) {
    return (
      <ClozeReview
        question={clozeQuestion}
        wikiTitle={current.wiki_title}
        onComplete={handleClozeComplete}
        onClose={() => {
          setShowCloze(false);
          clearCloze();
        }}
      />
    );
  }

  // Show ExplainReview when activated
  if (showExplain) {
    return (
      <ExplainReview
        wikiPageId={current.schedule.wiki_page_id}
        onComplete={handleExplainComplete}
        onClose={() => {
          setShowExplain(false);
          clearExplain();
        }}
      />
    );
  }

  return (
    <StandardReview
      current={current}
      currentReviewIndex={currentReviewIndex}
      total={total}
      nextFormat={nextFormat}
      isVariantMode={false}
      variantData={null}
      showFeedback={showFeedback}
      lastQuality={lastQuality}
      reviewError={reviewError}
      reviewStarted={reviewStarted}
      onQuality={handleQuality}
      onStartReview={handleStartReview}
      onClose={onClose}
      loading={clozeLoading || explainLoading}
    />
  );
}
