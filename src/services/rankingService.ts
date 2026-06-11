import { invoke } from "@tauri-apps/api/core";

export interface CategoryScore {
  score: number;
  max_score: number;
  percentage: number;
  label: string;
  icon: string;
}

export interface TagScore {
  tag: string;
  page_count: number;
  avg_mastery: number;
}

export interface StatsSummary {
  streak_day: number;
  completed_tasks: number;
  total_tasks: number;
  total_reviews: number;
  avg_quality: number;
  total_minutes: number;
}

export interface KnowledgeRanking {
  total_score: number;
  level: string;
  level_color: string;
  breadth: CategoryScore;
  depth: CategoryScore;
  mastery: CategoryScore;
  discovery: CategoryScore;
  connections: CategoryScore;
  tag_distribution: TagScore[];
}

export interface LearningRanking {
  total_score: number;
  level: string;
  level_color: string;
  consistency: CategoryScore;
  completion: CategoryScore;
  quality: CategoryScore;
  dedication: CategoryScore;
  stats_summary: StatsSummary;
}

export async function getKnowledgeRanking(): Promise<KnowledgeRanking> {
  return invoke("get_knowledge_ranking");
}

export async function getLearningRanking(): Promise<LearningRanking> {
  return invoke("get_learning_ranking");
}
