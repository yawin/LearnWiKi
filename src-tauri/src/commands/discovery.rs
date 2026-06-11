use crate::commands::capture::AppState;
use crate::discovery::engine::DiscoveryEngine;
use crate::storage::models::{
    KnowledgeMonitorSource, PendingContent,
};
use crate::storage::repository::Repository;
use std::sync::Arc;
use tauri::State;

// ========== PendingContent Commands ==========

#[tauri::command]
pub fn create_pending_content(
    state: State<'_, AppState>,
    title: String,
    source_url: Option<String>,
    source_name: Option<String>,
    content_summary: Option<String>,
    source_page_id: Option<String>,
    source_page_title: Option<String>,
    match_reason: Option<String>,
    match_keywords: Option<String>,
    relevance_score: Option<f64>,
    full_content: Option<String>,
    content_hash: Option<String>,
    discovered_at: String,
) -> Result<PendingContent, String> {
    let repo = Repository::new(state.db.clone());
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    let content = PendingContent {
        id: id.clone(),
        title,
        source_url,
        source_name,
        content_summary,
        source_page_id: source_page_id.clone(),
        source_page_title,
        match_reason,
        match_keywords,
        relevance_score: relevance_score.unwrap_or(0.5),
        full_content,
        content_hash,
        status: "unread".to_string(),
        read_at: None,
        imported_content_id: None,
        discovered_at,
        created_at: now,
    };

    repo.create_pending_content(&content)
        .map_err(|e| e.to_string())?;

    // Increment pending_count on the source wiki page if applicable
    if let Some(ref page_id) = source_page_id {
        if let Ok(count) = repo.count_pending_by_status("unread") {
            let _ = repo.update_wiki_page_pending_count(page_id, count);
        }
    }

    repo.get_pending_content(&id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Failed to retrieve created pending content".to_string())
}

#[tauri::command]
pub fn get_pending_content(
    state: State<'_, AppState>,
    id: Option<String>,
    status_filter: Option<String>,
    limit: Option<i64>,
    source_page_id: Option<String>,
) -> Result<Vec<PendingContent>, String> {
    let repo = Repository::new(state.db.clone());

    // If an explicit id is given, return just that one item
    if let Some(ref content_id) = id {
        return match repo.get_pending_content(content_id) {
            Ok(Some(item)) => Ok(vec![item]),
            Ok(None) => Err("Pending content not found".to_string()),
            Err(e) => Err(e.to_string()),
        };
    }

    // If source_page_id is given, return items for that page
    if let Some(ref page_id) = source_page_id {
        return repo
            .get_pending_content_by_page(page_id)
            .map_err(|e| e.to_string());
    }

    // Otherwise list with optional status filter
    let lim = limit.unwrap_or(50);
    repo.list_pending_content(status_filter.as_deref(), lim)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_pending_status(
    state: State<'_, AppState>,
    id: String,
    status: String,
) -> Result<(), String> {
    let repo = Repository::new(state.db.clone());
    repo.update_pending_content_status(&id, &status)
        .map_err(|e| e.to_string())
}

// ========== KnowledgeMonitorSource Commands ==========

#[tauri::command]
pub fn create_monitor_source(
    state: State<'_, AppState>,
    page_id: Option<String>,
    search_query: String,
    source_type: String,
    rss_url: Option<String>,
) -> Result<KnowledgeMonitorSource, String> {
    let repo = Repository::new(state.db.clone());
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    let source = KnowledgeMonitorSource {
        id,
        page_id,
        search_query,
        source_type,
        rss_url,
        is_active: true,
        last_checked_at: None,
        last_found_count: 0,
        created_at: now,
    };

    repo.create_monitor_source(&source)
        .map_err(|e| e.to_string())?;

    Ok(source)
}

#[tauri::command]
pub fn update_monitor_source(
    state: State<'_, AppState>,
    id: String,
    page_id: Option<String>,
    search_query: String,
    source_type: String,
    rss_url: Option<String>,
    is_active: bool,
) -> Result<(), String> {
    let repo = Repository::new(state.db.clone());

    // Fetch the existing source to preserve fields not being updated
    let existing = repo
        .get_monitor_sources_for_page(&page_id.clone().unwrap_or_default())
        .map_err(|e| e.to_string())?
        .into_iter()
        .find(|s| s.id == id)
        .ok_or_else(|| format!("Monitor source not found: {}", id))?;

    let updated = KnowledgeMonitorSource {
        id,
        page_id,
        search_query,
        source_type,
        rss_url,
        is_active,
        last_checked_at: existing.last_checked_at,
        last_found_count: existing.last_found_count,
        created_at: existing.created_at,
    };

    repo.update_monitor_source(&updated)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_monitor_sources_for_page(
    state: State<'_, AppState>,
    page_id: String,
) -> Result<Vec<KnowledgeMonitorSource>, String> {
    let repo = Repository::new(state.db.clone());
    repo.get_monitor_sources_for_page(&page_id)
        .map_err(|e| e.to_string())
}

// ========== Discovery Engine Commands (E-7-2) ==========

#[tauri::command]
pub async fn run_discovery_for_page(
    state: State<'_, AppState>,
    page_id: String,
) -> Result<String, String> {
    let repo = Arc::new(Repository::new(state.db.clone()));
    let engine = DiscoveryEngine::from_repo(repo.clone());

    let page = repo
        .get_wiki_page_by_id(&page_id)
        .map_err(|e| format!("Failed to get wiki page: {}", e))?
        .ok_or_else(|| "Wiki page not found".to_string())?;

    let new_count = engine.run_discovery_for_page(&page).await?;
    Ok(format!("Discovery complete: {} new items found", new_count))
}

#[tauri::command]
pub async fn run_discovery_all(state: State<'_, AppState>) -> Result<String, String> {
    let repo = Arc::new(Repository::new(state.db.clone()));
    let engine = DiscoveryEngine::from_repo(repo);

    let (pages_checked, total_new) = engine.run_discovery_for_all().await?;
    Ok(format!(
        "Discovery complete: checked {} pages, found {} new items",
        pages_checked, total_new
    ))
}

#[tauri::command]
pub fn set_serpapi_key(
    state: State<'_, AppState>,
    key: String,
) -> Result<(), String> {
    let repo = Repository::new(state.db.clone());
    repo.set_setting("serpapi_api_key", &key)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_discovery_status(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let repo = Repository::new(state.db.clone());

    let total_pending = repo
        .list_pending_content(None, 10000)
        .map_err(|e| e.to_string())?;

    let unread_count = total_pending.iter().filter(|i| i.status == "unread").count();
    let imported_count = total_pending.iter().filter(|i| i.status == "imported").count();
    let dismissed_count = total_pending.iter().filter(|i| i.status == "dismissed").count();

    let monitored_pages = repo
        .get_all_wiki_pages(1000, 0)
        .map_err(|e| e.to_string())?
        .into_iter()
        .filter(|p| p.monitor_enabled)
        .count();

    let has_serpapi = repo
        .get_setting("serpapi_api_key")
        .ok()
        .flatten()
        .is_some();

    let ignored_sources = repo.get_ignored_sources().map_err(|e| e.to_string())?;

    Ok(serde_json::json!({
        "total_pending": total_pending.len(),
        "unread_count": unread_count,
        "imported_count": imported_count,
        "dismissed_count": dismissed_count,
        "monitored_pages": monitored_pages,
        "has_serpapi_key": has_serpapi,
        "ignored_sources_count": ignored_sources.len(),
    }))
}

// ========== Discovery Suppression Commands (E-7-7) ==========

#[tauri::command]
pub async fn search_arxiv(
    query: String,
    limit: Option<i32>,
) -> Result<Vec<serde_json::Value>, String> {
    let results = crate::discovery::searcher::search_arxiv(&query).await?;
    let _ = limit; // ArXiv API always returns max 5
    Ok(results
        .into_iter()
        .map(|r| {
            serde_json::json!({
                "title": r.title,
                "url": r.url,
                "snippet": r.snippet,
                "source_name": r.source_name,
            })
        })
        .collect())
}

#[tauri::command]
pub async fn test_rss_feed(url: String) -> Result<Vec<serde_json::Value>, String> {
    let results = crate::discovery::searcher::search_rss(&url, "test").await?;
    Ok(results
        .into_iter()
        .map(|r| {
            serde_json::json!({
                "title": r.title,
                "url": r.url,
                "snippet": r.snippet,
                "source_name": r.source_name,
            })
        })
        .collect())
}

#[tauri::command]
pub fn record_dismissal_feedback(
    state: State<'_, AppState>,
    source_page_id: String,
) -> Result<(), String> {
    let repo = Repository::new(state.db.clone());
    repo.record_dismissal(&source_page_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_ignored_sources(
    state: State<'_, AppState>,
) -> Result<Vec<serde_json::Value>, String> {
    let repo = Repository::new(state.db.clone());
    let sources = repo.get_ignored_sources().map_err(|e| e.to_string())?;
    Ok(sources
        .into_iter()
        .map(|(id, title, count)| {
            serde_json::json!({
                "source_page_id": id,
                "page_title": title,
                "dismiss_count": count,
            })
        })
        .collect())
}

#[tauri::command]
pub fn unignore_discovery_source(
    state: State<'_, AppState>,
    source_page_id: String,
) -> Result<(), String> {
    let repo = Repository::new(state.db.clone());
    repo.unignore_source(&source_page_id)
        .map_err(|e| e.to_string())
}
