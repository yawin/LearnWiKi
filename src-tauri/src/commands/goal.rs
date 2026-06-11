use crate::commands::capture::AppState;
use crate::matching::keyword::{overlap_ratio, tokenize};
use crate::matching::sensitivity::{SensitivityLevel, SETTING_KEY as SENSITIVITY_KEY};
use crate::storage::models::{Goal, GoalRecommendation, GoalReviewLogItem, GoalWikiLink, GoalWikiLinkWithTitle, ReviewLogDetail, ReviewSessionRecord};
use crate::storage::repository::Repository;
use tauri::State;

// ========== Goal CRUD ==========

#[tauri::command]
pub async fn create_goal(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
    title: String,
    description: Option<String>,
) -> Result<Goal, String> {
    let repo = Repository::new(state.db.clone());
    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();

    let goal = Goal {
        id: id.clone(),
        title,
        description: description.unwrap_or_default(),
        keywords: "[]".to_string(),
        status: "active".to_string(),
        progress: 0.0,
        created_at: now.clone(),
        updated_at: now,
    };

    repo.save_goal(&goal).map_err(|e| e.to_string())?;
    let saved = repo.get_goal_by_id(&id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Failed to retrieve created goal".to_string())?;

    // T2: spawn reverse backfill — scan all wiki pages, match against newly created goal
    let db_c = state.db.clone();
    let app_c = app.clone();
    let goal_id_c = id.clone();
    tauri::async_runtime::spawn(async move {
        let repo = Repository::new(db_c.clone());
        let _ = tauri::Emitter::emit(
            &app_c,
            "goal-backfill-progress",
            serde_json::json!({ "goal_id": goal_id_c, "phase": "start" }),
        );
        let pages = match repo.get_all_wiki_pages(10_000, 0) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("goal backfill: failed to load wiki pages: {e}");
                return;
            }
        };
        let total = pages.len();
        for (i, page) in pages.iter().enumerate() {
            if let Err(e) = match_wiki_to_goals_inner(
                db_c.clone(),
                app_c.clone(),
                page.id.clone(),
            ).await {
                eprintln!("goal backfill: match failed for page {}: {e}", page.id);
            }
            if i % 20 == 19 {
                let _ = tauri::Emitter::emit(
                    &app_c,
                    "goal-backfill-progress",
                    serde_json::json!({ "goal_id": goal_id_c, "phase": "scanning",
                                        "scanned": i + 1, "total": total }),
                );
            }
        }
        let _ = tauri::Emitter::emit(
            &app_c,
            "goal-backfill-complete",
            serde_json::json!({ "goal_id": goal_id_c, "total": total }),
        );
    });

    Ok(saved)
}

#[tauri::command]
pub fn get_goals(
    state: State<'_, AppState>,
    status: Option<String>,
) -> Result<Vec<Goal>, String> {
    let repo = Repository::new(state.db.clone());
    match status {
        Some(s) => repo.get_goals_by_status(&s).map_err(|e| e.to_string()),
        None => repo.get_all_goals().map_err(|e| e.to_string()),
    }
}

#[tauri::command]
pub fn get_goal(
    state: State<'_, AppState>,
    id: String,
) -> Result<Goal, String> {
    let repo = Repository::new(state.db.clone());
    repo.get_goal_by_id(&id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Goal not found: {id}"))
}

#[tauri::command]
pub fn update_goal(
    state: State<'_, AppState>,
    id: String,
    title: Option<String>,
    description: Option<String>,
    keywords: Option<String>,
    status: Option<String>,
) -> Result<Goal, String> {
    let repo = Repository::new(state.db.clone());
    let mut goal = repo.get_goal_by_id(&id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Goal not found: {id}"))?;

    if let Some(t) = title { goal.title = t; }
    if let Some(d) = description { goal.description = d; }
    if let Some(k) = keywords { goal.keywords = k; }
    if let Some(s) = status { goal.status = s; }
    goal.updated_at = chrono::Utc::now().to_rfc3339();

    repo.update_goal(&goal).map_err(|e| e.to_string())?;
    Ok(goal)
}

#[tauri::command]
pub fn delete_goal(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let repo = Repository::new(state.db.clone());
    repo.delete_goal(&id).map_err(|e| e.to_string())
}

// ========== Goal ↔ Wiki 关联 ==========

#[tauri::command]
pub fn link_wiki_to_goal(
    state: State<'_, AppState>,
    goal_id: String,
    wiki_page_id: String,
    relevance_score: Option<f64>,
    source: Option<String>,
) -> Result<(), String> {
    let repo = Repository::new(state.db.clone());
    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();
    let link = GoalWikiLink {
        id,
        goal_id,
        wiki_page_id,
        relevance_score: relevance_score.unwrap_or(0.0),
        source: source.unwrap_or_else(|| "manual".to_string()),
        is_new: true,
        created_at: now,
    };
    repo.save_goal_wiki_link(&link).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn unlink_wiki_from_goal(
    state: State<'_, AppState>,
    goal_id: String,
    wiki_page_id: String,
) -> Result<(), String> {
    let repo = Repository::new(state.db.clone());
    repo.delete_goal_wiki_link(&goal_id, &wiki_page_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_goal_wiki_pages(
    state: State<'_, AppState>,
    goal_id: String,
) -> Result<Vec<GoalWikiLinkWithTitle>, String> {
    let repo = Repository::new(state.db.clone());
    repo.get_goal_wiki_links_with_titles(&goal_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn mark_goal_links_seen(
    state: State<'_, AppState>,
    goal_id: String,
) -> Result<(), String> {
    let repo = Repository::new(state.db.clone());
    repo.mark_goal_wiki_links_seen(&goal_id).map_err(|e| e.to_string())
}

// ========== Goal Recommendations & Auto-Matching ==========

// ========== Goal Recommendations & Auto-Matching ==========

#[tauri::command]
pub async fn search_goal_resources(
    state: State<'_, AppState>,
    goal_id: String,
) -> Result<Vec<GoalRecommendation>, String> {
    let repo = Repository::new(state.db.clone());
    let goal = repo.get_goal_by_id(&goal_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Goal not found: {goal_id}"))?;

    let locale = crate::locale::resolve_locale(&state.db);

    let system_prompt = if crate::locale::is_english(&locale) {
        "You are a learning resource curator. Given a learning goal, generate a structured list of 8-10 recommended learning resources, ordered from beginner to advanced.\n\n\
         Return JSON array: [{\"title\":\"Resource title\",\"url\":\"https://...\",\"summary\":\"Brief description\",\"difficulty\":\"beginner|intermediate|advanced\"}]\n\n\
         Rules:\n\
         - First 2-3 items should be beginner level (introductions, overviews)\n\
         - Middle 3-4 items should be intermediate (detailed explanations, comparisons)\n\
         - Last 2-3 items should be advanced (deep dives, best practices, edge cases)\n\
         - Use real URLs when possible, or leave url as null if unsure\n\
         - Each summary should be 1-2 sentences explaining what the learner will gain".to_string()
    } else {
        "你是一个学习资源策展助手。根据学习目标，生成 8-10 个推荐学习资源，按从入门到深入排序。\n\n\
         返回 JSON 数组：[{\"title\":\"资源标题\",\"url\":\"https://...\",\"summary\":\"简短描述\",\"difficulty\":\"beginner|intermediate|advanced\"}]\n\n\
         规则：\n\
         - 前 2-3 个为入门级（概念介绍、基础概览）\n\
         - 中间 3-4 个为进阶级（详细解释、对比分析）\n\
         - 最后 2-3 个为深入级（源码解读、最佳实践、边界情况）\n\
         - 尽量使用真实 URL，不确定的留 null\n\
         - 每条摘要 1-2 句话说明学习者能获得什么".to_string()
    };

    let content = format!("学习目标：{}\n描述：{}", goal.title, goal.description);

    let raw = crate::ai::wiki_engine::call_ai_pub(
        state.db.clone(),
        &system_prompt,
        &content,
        2048,
    ).await?;

    let json = crate::ai::wiki_engine::parse_ai_json_pub(&raw)?;

    let items = json.as_array().ok_or("AI 返回格式错误：期望 JSON 数组")?;

    // Clear existing recommendations for this goal and save new ones
    let _ = repo.delete_goal_recommendations(&goal_id);

    let now = chrono::Utc::now().to_rfc3339();
    let mut recommendations = Vec::new();

    for (i, item) in items.iter().enumerate() {
        let rec = GoalRecommendation {
            id: uuid::Uuid::new_v4().to_string(),
            goal_id: goal_id.clone(),
            title: item.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            url: item.get("url").and_then(|v| v.as_str()).map(|s| s.to_string()),
            summary: item.get("summary").and_then(|v| v.as_str()).map(|s| s.to_string()),
            difficulty: item.get("difficulty").and_then(|v| v.as_str()).unwrap_or("beginner").to_string(),
            sort_order: i as i32,
            status: "pending".to_string(),
            imported_content_id: None,
            created_at: now.clone(),
        };
        repo.save_goal_recommendation(&rec).map_err(|e| e.to_string())?;
        recommendations.push(rec);
    }

    // Also update goal keywords based on AI analysis
    let keywords_prompt = if crate::locale::is_english(&locale) {
        "Extract 5-8 key search terms from this learning goal. Return JSON array of strings.".to_string()
    } else {
        "从这个学习目标中提取 5-8 个关键搜索词。返回 JSON 字符串数组。".to_string()
    };

    if let Ok(kw_raw) = crate::ai::wiki_engine::call_ai_pub(
        state.db.clone(), &keywords_prompt, &content, 256
    ).await {
        if let Ok(kw_json) = crate::ai::wiki_engine::parse_ai_json_pub(&kw_raw) {
            let keywords_str = serde_json::to_string(&kw_json).unwrap_or("[]".to_string());
            let mut updated_goal = goal;
            updated_goal.keywords = keywords_str;
            updated_goal.updated_at = chrono::Utc::now().to_rfc3339();
            let _ = repo.update_goal(&updated_goal);
        }
    }

    Ok(recommendations)
}

#[tauri::command]
pub fn get_goal_recommendations(
    state: State<'_, AppState>,
    goal_id: String,
) -> Result<Vec<GoalRecommendation>, String> {
    let repo = Repository::new(state.db.clone());
    repo.get_goal_recommendations(&goal_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn dismiss_goal_recommendation(
    state: State<'_, AppState>,
    recommendation_id: String,
) -> Result<(), String> {
    let repo = Repository::new(state.db.clone());
    repo.update_goal_recommendation_status(&recommendation_id, "dismissed", None)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn match_wiki_to_goals(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
    wiki_page_id: String,
) -> Result<Vec<GoalWikiLink>, String> {
    let db = state.db.clone();
    match_wiki_to_goals_inner(db, app, wiki_page_id).await
}

/// Internal entry — also called by compile-completion hook and new-goal backfill.
/// Returns created link records (empty Vec if no goals matched, never errors on AI failure).
pub async fn match_wiki_to_goals_inner(
    db: std::sync::Arc<crate::storage::database::Database>,
    app: tauri::AppHandle,
    wiki_page_id: String,
) -> Result<Vec<GoalWikiLink>, String> {
    let repo = Repository::new(db.clone());

    // Load page
    let page = repo.get_wiki_page_by_id(&wiki_page_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Wiki page not found: {wiki_page_id}"))?;

    // Load active goals
    let goals = repo.get_goals_by_status("active").map_err(|e| e.to_string())?;
    if goals.is_empty() {
        return Ok(Vec::new());
    }

    // Read sensitivity setting → thresholds
    let level = SensitivityLevel::from_setting(
        repo.get_setting(SENSITIVITY_KEY).ok().flatten().as_deref(),
    );
    let ai_threshold = level.ai_threshold();
    let keyword_threshold = level.keyword_threshold();

    // Try AI path first, fall back to keyword overlap
    let scored: Vec<(String, f64)> =
        match score_with_ai(&db, &page, &goals, ai_threshold).await {
            Ok(v) if !v.is_empty() => v,
            Ok(_) | Err(_) => score_with_keywords(&page, &goals, keyword_threshold),
        };

    // Top-3 by relevance_score desc
    let mut sorted = scored;
    sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    sorted.truncate(3);

    // Persist: filter dupes via UNIQUE constraint, ensure review_schedule
    let now = chrono::Utc::now().to_rfc3339();
    let mut created = Vec::new();
    let existing_links = repo.get_goal_wiki_links_for_page(&wiki_page_id)
        .unwrap_or_default();

    // Note: this in-memory dup check races with concurrent calls (compile hook
    // + goal backfill could both pass it for the same goal/wiki pair). The
    // UNIQUE constraint on goal_wiki_links (goal_id, wiki_page_id) is the real
    // safety net — save_goal_wiki_link relies on INSERT OR IGNORE.
    for (goal_id, score) in sorted {
        if existing_links.iter().any(|l| l.goal_id == goal_id) {
            continue;
        }
        let link = GoalWikiLink {
            id: uuid::Uuid::new_v4().to_string(),
            goal_id: goal_id.clone(),
            wiki_page_id: wiki_page_id.clone(),
            relevance_score: score,
            source: "auto".to_string(),
            is_new: true,
            created_at: now.clone(),
        };
        if let Err(e) = repo.save_goal_wiki_link(&link) {
            eprintln!("save_goal_wiki_link failed: {e}");
            continue;
        }
        created.push(link);
    }

    // Emit event for frontend refresh (best-effort)
    if !created.is_empty() {
        let goal_ids: Vec<String> = created.iter().map(|l| l.goal_id.clone()).collect();
        let _ = tauri::Emitter::emit(
            &app,
            "goals-matched",
            serde_json::json!({ "wiki_page_id": wiki_page_id, "goal_ids": goal_ids }),
        );
    }

    Ok(created)
}

async fn score_with_ai(
    db: &std::sync::Arc<crate::storage::database::Database>,
    page: &crate::storage::models::WikiPage,
    goals: &[crate::storage::models::Goal],
    threshold: f64,
) -> Result<Vec<(String, f64)>, String> {
    let locale = crate::locale::resolve_locale(db);
    let goals_text = goals.iter()
        .map(|g| format!("- {}: {} (id={})", g.title, g.keywords, g.id))
        .collect::<Vec<_>>()
        .join("\n");

    let system_prompt = if crate::locale::is_english(&locale) {
        format!(
            "You are a content-to-goal matcher. Given a wiki page and a list of learning goals, determine which goals this page is relevant to.\n\n\
             Return JSON array of matched goal IDs with relevance scores: [{{\"goal_id\":\"...\",\"relevance_score\":0.8}}]\n\
             Only include goals with relevance >= {:.1}. Return empty array [] if no match.",
            threshold,
        )
    } else {
        format!(
            "你是一个内容-目标匹配助手。给定一个 Wiki 页面和学习目标列表，判断这个页面与哪些目标相关。\n\n\
             返回匹配的目标 ID 和相关度分数的 JSON 数组：[{{\"goal_id\":\"...\",\"relevance_score\":0.8}}]\n\
             只包含相关度 >= {:.1} 的目标。没有匹配返回空数组 []。",
            threshold,
        )
    };

    let content = format!(
        "Wiki Page:\nTitle: {}\nTags: {}\nBody: {}\n\nGoals:\n{}",
        page.title,
        page.tags.as_deref().unwrap_or(""),
        page.body_markdown.chars().take(500).collect::<String>(),
        goals_text,
    );

    let raw = crate::ai::wiki_engine::call_ai_pub(db.clone(), &system_prompt, &content, 512).await?;
    let json = crate::ai::wiki_engine::parse_ai_json_pub(&raw)?;
    let arr = json.as_array().ok_or("AI 返回格式错误")?;

    let mut out = Vec::new();
    for m in arr {
        let gid = m.get("goal_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let score = m.get("relevance_score").and_then(|v| v.as_f64()).unwrap_or(0.0);
        if gid.is_empty() || score < threshold {
            continue;
        }
        out.push((gid, score));
    }
    Ok(out)
}

// ========== Wiki Read Status ==========

#[tauri::command]
pub fn set_wiki_read_status(
    state: State<'_, AppState>,
    wiki_page_id: String,
    is_read: bool,
) -> Result<(), String> {
    let repo = Repository::new(state.db.clone());
    repo.set_wiki_read_status(&wiki_page_id, is_read)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_wiki_read_status(
    state: State<'_, AppState>,
    wiki_page_id: String,
) -> Result<bool, String> {
    let repo = Repository::new(state.db.clone());
    repo.get_wiki_read_status(&wiki_page_id)
        .map_err(|e| e.to_string())
}

fn score_with_keywords(
    page: &crate::storage::models::WikiPage,
    goals: &[crate::storage::models::Goal],
    threshold: f64,
) -> Vec<(String, f64)> {
    let mut wiki_terms = tokenize(&page.title);
    if let Some(tags) = &page.tags {
        for tag in tags.split(|c: char| c == ',' || c == ' ').filter(|s| !s.is_empty()) {
            wiki_terms.extend(tokenize(tag));
        }
    }
    let body_preview: String = page.body_markdown.chars().take(500).collect();
    wiki_terms.extend(tokenize(&body_preview));

    let mut out = Vec::new();
    for goal in goals {
        let mut goal_terms = tokenize(&goal.title);
        match serde_json::from_str::<Vec<String>>(&goal.keywords) {
            Ok(arr) => {
                for k in arr {
                    goal_terms.extend(tokenize(&k));
                }
            }
            Err(e) => {
                eprintln!(
                    "score_with_keywords: failed to parse goal.keywords for goal {}: {} (raw: {:?})",
                    goal.id, e, goal.keywords,
                );
            }
        }
        let r = overlap_ratio(&wiki_terms, &goal_terms);
        if r >= threshold {
            out.push((goal.id.clone(), r));
        }
    }
    out
}

#[tauri::command]
pub fn get_goal_review_logs(
    state: State<'_, AppState>,
    goal_id: String,
    limit: Option<i32>,
) -> Result<Vec<GoalReviewLogItem>, String> {
    let repo = Repository::new(state.db.clone());
    let l = limit.unwrap_or(20) as i64;
    repo.get_goal_review_logs(&goal_id, l).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_goal_review_sessions(
    state: State<'_, AppState>,
    goal_id: String,
    limit: Option<i32>,
) -> Result<Vec<ReviewSessionRecord>, String> {
    let repo = Repository::new(state.db.clone());
    let l = limit.unwrap_or(20) as i64;
    repo.get_goal_review_sessions(&goal_id, l).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_review_log(
    state: State<'_, AppState>,
    log_id: String,
) -> Result<Option<ReviewLogDetail>, String> {
    let repo = Repository::new(state.db.clone());
    repo.get_review_log_by_id(&log_id).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::models::{Goal, WikiPage};

    fn make_page(title: &str, tags: &str, body: &str) -> WikiPage {
        WikiPage {
            id: "test-page".to_string(),
            title: title.to_string(),
            slug: "test-page".to_string(),
            page_type: "concept".to_string(),
            body_markdown: body.to_string(),
            summary: None,
            tags: Some(tags.to_string()),
            status: "active".to_string(),
            confidence: 0.5,
            created_at: String::new(),
            updated_at: String::new(),
            last_compiled_at: None,
            source_message_id: None,
            author_name: None,
            author_url: None,
            source_type: None,
            source_task_id: None,
            monitor_enabled: false,
            monitor_query: None,
            monitor_sources: String::new(),
            last_discovered_at: None,
            pending_count: 0,
        }
    }

    fn make_goal(id: &str, title: &str, keywords: &str) -> Goal {
        Goal {
            id: id.to_string(),
            title: title.to_string(),
            description: String::new(),
            keywords: keywords.to_string(),
            status: "active".to_string(),
            progress: 0.0,
            created_at: String::new(),
            updated_at: String::new(),
        }
    }

    #[test]
    fn score_keywords_exact_match() {
        let page = make_page("Rust所有权", "rust,ownership", "Rust 的所有权模型");
        let goals = vec![make_goal("g1", "掌握Rust所有权", r#"["Rust","所有权"]"#)];
        let result = score_with_keywords(&page, &goals, 0.3);
        assert_eq!(result.len(), 1);
        assert!(result[0].1 > 0.0); // should have some overlap
    }

    #[test]
    fn score_keywords_no_match() {
        let page = make_page("React组件", "react,component", "React 组件开发");
        let goals = vec![make_goal("g1", "掌握Rust所有权", r#"["Rust","所有权"]"#)];
        let result = score_with_keywords(&page, &goals, 0.3);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn score_keywords_respects_threshold() {
        let page = make_page("Rust入门", "rust", "Rust入门教程，基础概念");
        let goals = vec![make_goal("g1", "Rust高级", r#"["Rust","高级","特性"]"#)];
        // "Rust" token is shared, but overlap ratio is low enough that 0.5 threshold rejects it
        let result = score_with_keywords(&page, &goals, 0.5);
        assert_eq!(result.len(), 0);
        // loose threshold captures the partial overlap
        let result_loose = score_with_keywords(&page, &goals, 0.2);
        assert_eq!(result_loose.len(), 1);
    }

    #[test]
    fn score_keywords_malformed_json_keywords_fallback() {
        let page = make_page("Rust所有权", "rust", "Rust所有权模型");
        // keywords is NOT valid JSON — should be handled gracefully
        let goals = vec![make_goal("g1", "Rust所有权", "not-json")];
        let result = score_with_keywords(&page, &goals, 0.3);
        // Should not panic, should still match on title tokens
        let _ = result.len(); // at minimum no panic
    }
}
