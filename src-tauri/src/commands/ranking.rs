use crate::commands::capture::AppState;
use crate::storage::models::{
    CategoryScore, KnowledgeRanking, LearningRanking, StatsSummary, TagScore,
};
use tauri::State;

fn compute_level(total: f64) -> (String, String) {
    if total >= 4500.0 {
        ("王者".into(), "#FFD700".into())
    } else if total >= 3500.0 {
        ("钻石".into(), "#4169E1".into())
    } else if total >= 2500.0 {
        ("黄金".into(), "#FF8C00".into())
    } else if total >= 1500.0 {
        ("白银".into(), "#C0C0C0".into())
    } else {
        ("青铜".into(), "#CD7F32".into())
    }
}

fn cat(score: f64, max: f64, label: &str, icon: &str) -> CategoryScore {
    let pct = if max > 0.0 { (score / max).clamp(0.0, 1.0) } else { 0.0 };
    CategoryScore {
        score,
        max_score: max,
        percentage: pct,
        label: label.to_string(),
        icon: icon.to_string(),
    }
}

#[tauri::command]
pub fn get_knowledge_ranking(state: State<AppState>) -> Result<KnowledgeRanking, String> {
    let conn = state.db.conn.lock().map_err(|e| format!("DB lock: {}", e))?;

    // -- breadth: page count --
    let page_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM wiki_pages", [], |r| r.get(0))
        .unwrap_or(0);
    let breadth_score = if page_count <= 10 {
        page_count as f64 * 50.0
    } else if page_count <= 50 {
        500.0 + (page_count as f64 - 10.0) * 12.5
    } else if page_count <= 200 {
        1000.0 + (page_count as f64 - 50.0) * 3.33
    } else {
        1500.0
    };
    let breadth_max = 1500.0;

    // -- depth: non-empty body / total --
    let non_empty: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM wiki_pages WHERE body_markdown IS NOT NULL AND body_markdown != ''",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    let depth_score = if page_count > 0 {
        (non_empty as f64 / page_count as f64) * 1000.0
    } else {
        0.0
    };
    let depth_max = 1000.0;

    // -- mastery: AVG(mastery) --
    let avg_mastery: f64 = conn
        .query_row("SELECT COALESCE(AVG(mastery), 0.0) FROM review_schedule", [], |r| {
            r.get(0)
        })
        .unwrap_or(0.0);
    let mastery_score = avg_mastery * 1000.0;
    let mastery_max = 1000.0;

    // -- discovery: imported pending_content count --
    let imported_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM pending_content WHERE status = 'imported'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    let discovery_score = if imported_count <= 5 {
        imported_count as f64 * 100.0
    } else if imported_count <= 20 {
        500.0 + (imported_count as f64 - 5.0) * 33.0
    } else {
        1000.0
    };
    let discovery_max = 1000.0;

    // -- connections: page_count * 5, max 1000 --
    let connections_raw = page_count as f64 * 5.0;
    let connections_score = connections_raw.min(1000.0);
    let connections_max = 1000.0;

    let total = breadth_score * 0.2
        + depth_score * 0.2
        + mastery_score * 0.3
        + discovery_score * 0.15
        + connections_score * 0.15;

    // -- tag distribution --
    let mut tag_distribution = Vec::new();
    {
        let mut stmt = conn
            .prepare("SELECT tags FROM wiki_pages WHERE tags IS NOT NULL AND tags != ''")
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(|e| e.to_string())?;

        // Collect all tag -> page mapping
        let mut tag_pages: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();
        let mut tag_mastery_map: std::collections::HashMap<String, Vec<f64>> =
            std::collections::HashMap::new();

        for row in rows {
            let tags_str: String = row.map_err(|e| e.to_string())?;
            // Tags are stored as JSON array or comma-separated
            let tags: Vec<String> = if tags_str.starts_with('[') {
                // JSON array
                serde_json::from_str::<Vec<String>>(&tags_str).unwrap_or_default()
            } else {
                // Comma-separated
                tags_str.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
            };

            // We need the page id for this row to look up mastery.
            // Re-fetch page ids for tag distribution later.
            for tag in tags {
                tag_pages.entry(tag.clone()).or_default().push(String::new());
            }
        }

        // Re-do with page ids for mastery lookup
        let mut stmt2 = conn
            .prepare("SELECT id, tags FROM wiki_pages WHERE tags IS NOT NULL AND tags != ''")
            .map_err(|e| e.to_string())?;
        let rows2 = stmt2
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let t: String = row.get(1)?;
                Ok((id, t))
            })
            .map_err(|e| e.to_string())?;

        tag_pages.clear();

        for row in rows2 {
            let (page_id, tags_str) = row.map_err(|e| e.to_string())?;
            let tags: Vec<String> = if tags_str.starts_with('[') {
                serde_json::from_str::<Vec<String>>(&tags_str).unwrap_or_default()
            } else {
                tags_str
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            };

            for tag in tags {
                tag_pages.entry(tag.clone()).or_default().push(page_id.clone());
            }
        }

        // For each tag, look up mastery values
        for (tag, pages) in &tag_pages {
            let page_count = pages.len() as i32;
            let mut total_mastery = 0.0_f64;
            let mut mastery_count = 0_i64;
            for pid in pages {
                let m: Option<f64> = conn
                    .query_row(
                        "SELECT mastery FROM review_schedule WHERE wiki_page_id = ?1",
                        [pid],
                        |r| r.get(0),
                    )
                    .ok();
                if let Some(val) = m {
                    total_mastery += val;
                    mastery_count += 1;
                }
            }
            let avg_m = if mastery_count > 0 {
                total_mastery / mastery_count as f64
            } else {
                0.0
            };
            tag_distribution.push(TagScore {
                tag: tag.clone(),
                page_count,
                avg_mastery: avg_m,
            });
        }
        tag_distribution.sort_by(|a, b| b.page_count.cmp(&a.page_count));
    }

    let (level, level_color) = compute_level(total);

    Ok(KnowledgeRanking {
        total_score: (total * 100.0).round() / 100.0,
        level,
        level_color,
        breadth: cat(breadth_score, breadth_max, "知识广度", "📚"),
        depth: cat(depth_score, depth_max, "知识深度", "📝"),
        mastery: cat(mastery_score, mastery_max, "知识掌握", "🎯"),
        discovery: cat(discovery_score, discovery_max, "知识发现", "🔍"),
        connections: cat(connections_score, connections_max, "图谱连接", "🔗"),
        tag_distribution,
    })
}

#[tauri::command]
pub fn get_learning_ranking(state: State<AppState>) -> Result<LearningRanking, String> {
    let conn = state.db.conn.lock().map_err(|e| format!("DB lock: {}", e))?;

    // -- consistency: MAX(streak_day) * 100, max 1000 --
    let max_streak: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(streak_day), 0) FROM daily_review_summary",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    let consistency_score = (max_streak as f64 * 100.0).min(1000.0);
    let consistency_max = 1000.0;

    // -- completion: completed/total * 1000 --
    let total_tasks: i64 = conn
        .query_row("SELECT COUNT(*) FROM practice_tasks", [], |r| r.get(0))
        .unwrap_or(0);
    let completed_tasks: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM practice_tasks WHERE status IN ('completed', 'reviewed')",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    let completion_score = if total_tasks > 0 {
        (completed_tasks as f64 / total_tasks as f64) * 1000.0
    } else {
        0.0
    };
    let completion_max = 1000.0;

    // -- quality: correct_reviews / total_reviews * 1000 --
    // quality=2 is correct (highest), quality=0/1 are incorrect
    let total_reviews: i64 = conn
        .query_row("SELECT COUNT(*) FROM review_logs", [], |r| r.get(0))
        .unwrap_or(0);
    let correct_reviews: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM review_logs WHERE quality = 2",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    let quality_score = if total_reviews > 0 {
        (correct_reviews as f64 / total_reviews as f64) * 1000.0
    } else {
        0.0
    };
    let quality_max = 1000.0;

    // -- dedication: SUM(total_minutes) / 60 * 50, max 1000 (20hrs) --
    let total_minutes: i64 = conn
        .query_row(
            "SELECT COALESCE(SUM(total_minutes), 0) FROM task_daily_logs",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    let dedication_score = (total_minutes as f64 / 60.0 * 50.0).min(1000.0);
    let dedication_max = 1000.0;

    let total = consistency_score * 0.25
        + completion_score * 0.3
        + quality_score * 0.3
        + dedication_score * 0.15;

    let (level, level_color) = compute_level(total);

    // Stats summary
    let avg_q = if total_reviews > 0 {
        (correct_reviews as f64 / total_reviews as f64 * 100.0 * 100.0).round() / 100.0
    } else {
        0.0
    };

    let stats = StatsSummary {
        streak_day: max_streak,
        completed_tasks: completed_tasks as i32,
        total_tasks: total_tasks as i32,
        total_reviews: total_reviews as i32,
        avg_quality: avg_q,
        total_minutes: total_minutes as i32,
    };

    Ok(LearningRanking {
        total_score: (total * 100.0).round() / 100.0,
        level,
        level_color,
        consistency: cat(consistency_score, consistency_max, "学习连续性", "🔥"),
        completion: cat(completion_score, completion_max, "任务完成", "✅"),
        quality: cat(quality_score, quality_max, "复习质量", "📊"),
        dedication: cat(dedication_score, dedication_max, "学习投入", "⏱"),
        stats_summary: stats,
    })
}
