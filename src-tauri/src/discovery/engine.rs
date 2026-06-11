use crate::discovery::searcher::{search_arxiv, search_rss, search_web, SearchResult};
use crate::storage::models::{PendingContent, WikiPage};
use crate::storage::repository::Repository;
use crate::ai::client::AiClient;
use sha2::{Sha256, Digest};
use std::sync::Arc;

/// The main discovery orchestration engine.
///
/// Runs search queries for monitored wiki pages, scores results
/// via LLM, and stores them as pending content for user review.
pub struct DiscoveryEngine {
    repo: Arc<Repository>,
    ai_client: Option<AiClient>,
    serpapi_key: Option<String>,
}

impl DiscoveryEngine {
    pub fn new(
        repo: Arc<Repository>,
        ai_client: Option<AiClient>,
        serpapi_key: Option<String>,
    ) -> Self {
        DiscoveryEngine {
            repo,
            ai_client,
            serpapi_key,
        }
    }

    /// Construct an AiClient from settings stored in the DB.
    pub fn from_repo(repo: Arc<Repository>) -> Self {
        let ai_api_key = repo.get_setting("ai_api_key").ok().flatten();
        let ai_provider = repo
            .get_setting("ai_provider")
            .ok()
            .flatten()
            .unwrap_or_else(|| "openai".to_string());
        let ai_model = repo
            .get_setting("ai_model")
            .ok()
            .flatten()
            .unwrap_or_else(|| "gpt-4o-mini".to_string());
        let serpapi_key = repo.get_setting("serpapi_api_key").ok().flatten();

        let ai_client = ai_api_key.map(|key| {
            AiClient::new(key, ai_provider, ai_model)
        });

        DiscoveryEngine {
            repo,
            ai_client,
            serpapi_key,
        }
    }

    /// Run discovery for a single wiki page using multiple source types.
    /// Returns the number of new items discovered.
    pub async fn run_discovery_for_page(&self, page: &WikiPage) -> Result<usize, String> {
        // 1. Determine search query
        let query = self.determine_search_query(page);

        log::info!(
            "[Discovery] Running discovery for page '{}' with query: {}",
            page.title,
            query
        );

        // 2. Parse monitor sources
        let sources = self.parse_monitor_sources(page);

        if sources.is_empty() {
            log::info!("[Discovery] No monitor sources configured for page '{}'. Defaulting to web_search.", page.title);
        }

        // 3. Run searches for each configured source type
        let mut all_results: Vec<SearchResult> = Vec::new();

        for source in &sources {
            match source.as_str() {
                "web_search" => {
                    let serpapi_key = match &self.serpapi_key {
                        Some(k) => k.clone(),
                        None => {
                            log::warn!("[Discovery] SerpAPI key not configured, skipping web_search");
                            continue;
                        }
                    };

                    match search_web(&query, &serpapi_key, Some(5)).await {
                        Ok(results) => {
                            log::info!("[Discovery] Web search returned {} results", results.len());
                            all_results.extend(results);
                        }
                        Err(e) => {
                            log::warn!("[Discovery] Web search failed: {}", e);
                        }
                    }
                }
                "arxiv" => {
                    match search_arxiv(&query).await {
                        Ok(results) => {
                            log::info!("[Discovery] ArXiv search returned {} results", results.len());
                            all_results.extend(results);
                        }
                        Err(e) => {
                            log::warn!("[Discovery] ArXiv search failed: {}", e);
                        }
                    }
                }
                other if other.starts_with("rss:") => {
                    // RSS feeds are configured in knowledge_monitor_source table with source_type="rss"
                    // The source name in the array is just "rss" — the actual feeds are fetched separately
                    log::info!("[Discovery] RSS source type detected, checking for configured RSS feeds");
                }
                "rss" => {
                    log::info!("[Discovery] RSS source type detected, checking for configured RSS feeds");
                }
                _ => {
                    log::warn!("[Discovery] Unknown source type: {}", source);
                }
            }
        }

        // If no sources configured, default to web_search only (backward compatibility)
        if sources.is_empty() {
            if let Some(serpapi_key) = &self.serpapi_key {
                match search_web(&query, serpapi_key, Some(5)).await {
                    Ok(results) => {
                        all_results = results;
                    }
                    Err(e) => {
                        log::warn!("[Discovery] Web search failed: {}", e);
                    }
                }
            }
        }

        // Also fetch any configured RSS feeds
        if sources.contains(&"rss".to_string()) || sources.is_empty() {
            let rss_sources = self.repo.get_monitor_sources_for_page(&page.id)
                .map_err(|e| format!("Failed to fetch RSS sources: {}", e))?;

            for rss_source in rss_sources.iter().filter(|s| s.source_type == "rss") {
                if let Some(ref rss_url) = rss_source.rss_url {
                    let feed_name = rss_source.search_query.clone();
                    match search_rss(rss_url, &feed_name).await {
                        Ok(results) => {
                            log::info!("[Discovery] RSS feed '{}' returned {} results", feed_name, results.len());
                            all_results.extend(results);
                        }
                        Err(e) => {
                            log::warn!("[Discovery] RSS feed '{}' failed: {}", feed_name, e);
                        }
                    }
                }
            }
        }

        if all_results.is_empty() {
            log::info!("[Discovery] No results found for '{}'", page.title);
            return Ok(0);
        }

        // 4. Dedup and score each result
        let mut new_count = 0usize;
        let now = chrono::Utc::now().to_rfc3339();

        for result in &all_results {
            // Check dedup by URL
            if let Ok(true) = self.repo.pending_content_exists_by_url(&result.url) {
                log::info!("[Discovery] Skipping duplicate URL: {}", result.url);
                continue;
            }

            // Check dedup by content hash
            let content_hash = compute_content_hash(&result.url);
            if let Ok(true) = self.repo.pending_content_exists_by_url(&result.url) {
                continue;
            }

            // 5. LLM relevance scoring
            let relevance_score = self.score_relevance(&query, result).await.unwrap_or(0.5);

            // 6. Skip low relevance
            if relevance_score < 0.3 {
                log::info!(
                    "[Discovery] Skipping low-relevance result ({}): {}",
                    relevance_score,
                    result.title
                );
                continue;
            }

            // 7. Store as PendingContent
            let pending_id = uuid::Uuid::new_v4().to_string();
            let pending = PendingContent {
                id: pending_id.clone(),
                title: result.title.clone(),
                source_url: Some(result.url.clone()),
                source_name: Some(result.source_name.clone()),
                content_summary: Some(result.snippet.clone()),
                source_page_id: Some(page.id.clone()),
                source_page_title: Some(page.title.clone()),
                match_reason: Some(format!("Keyword match on '{}'", query)),
                match_keywords: Some(format!("[\\\"{}\\\"]", query)),
                relevance_score,
                full_content: None,
                content_hash: Some(content_hash),
                status: "unread".to_string(),
                read_at: None,
                imported_content_id: None,
                discovered_at: now.clone(),
                created_at: now.clone(),
            };

            match self.repo.create_pending_content(&pending) {
                Ok(_) => {
                    new_count += 1;
                    log::info!(
                        "[Discovery] Stored new pending content: {}",
                        result.title
                    );
                }
                Err(e) => {
                    log::warn!(
                        "[Discovery] Failed to store pending content '{}': {}",
                        result.title,
                        e
                    );
                }
            }
        }

        // 8. Update the wiki page's last_discovered_at and pending_count
        if new_count > 0 {
            let _ = self.repo.update_wiki_page_last_discovered(&page.id, &now);
            if let Ok(count) = self.repo.count_pending_by_status("unread") {
                let _ = self.repo.update_wiki_page_pending_count(&page.id, count);
            }
        }

        Ok(new_count)
    }

    /// Run discovery for all pages that have monitor_enabled = true.
    /// Returns (total_pages_checked, total_new_items).
    pub async fn run_discovery_for_all(&self) -> Result<(usize, usize), String> {
        let pages = match self.repo.get_all_wiki_pages(1000, 0) {
            Ok(p) => p,
            Err(e) => return Err(format!("Failed to fetch wiki pages: {}", e)),
        };

        let monitored: Vec<&WikiPage> = pages.iter().filter(|p| p.monitor_enabled).collect();
        if monitored.is_empty() {
            log::info!("[Discovery] No monitored pages found");
            return Ok((0, 0));
        }

        log::info!(
            "[Discovery] Running discovery for {} monitored pages",
            monitored.len()
        );

        let mut total_new = 0usize;

        for page in &monitored {
            match self.run_discovery_for_page(page).await {
                Ok(n) => {
                    total_new += n;
                    log::info!(
                        "[Discovery] Page '{}': {} new items found",
                        page.title,
                        n
                    );
                }
                Err(e) => {
                    log::warn!(
                        "[Discovery] Discovery failed for page '{}': {}",
                        page.title,
                        e
                    );
                    // Continue with other pages — don't crash the whole batch
                }
            }
        }

        Ok((monitored.len(), total_new))
    }

    /// Determine the search query for a page.
    /// If monitor_query is set, use it. Otherwise generate from title + tags.
    fn determine_search_query(&self, page: &WikiPage) -> String {
        if let Some(ref query) = page.monitor_query {
            if !query.is_empty() {
                return query.clone();
            }
        }

        // Generate from title + tags + "latest"
        let mut query = page.title.clone();

        if let Some(ref tags) = page.tags {
            if !tags.is_empty() {
                // Tags are stored as comma-separated or JSON array
                let cleaned = tags
                    .replace('"', "")
                    .replace('[', "")
                    .replace(']', "")
                    .replace(',', " ");
                if !cleaned.is_empty() {
                    query.push(' ');
                    query.push_str(&cleaned);
                }
            }
        }

        // Add a temporal term for freshness
        query.push_str(" latest");

        query
    }

    /// Parse the monitor_sources JSON array field from a WikiPage.
    /// Returns a list of source type strings (e.g., ["web_search", "arxiv", "rss"]).
    fn parse_monitor_sources(&self, page: &WikiPage) -> Vec<String> {
        let raw = page.monitor_sources.trim();
        if raw.is_empty() || raw == "[]" || raw == "null" {
            return Vec::new();
        }
        // Try to parse as JSON array
        if let Ok(arr) = serde_json::from_str::<Vec<String>>(raw) {
            return arr;
        }
        // Fallback: try comma-separated
        if raw.contains(',') {
            return raw
                .split(',')
                .map(|s| s.trim().trim_matches('"').trim_matches('[').trim_matches(']').to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }
        // Single value
        let cleaned = raw.trim_matches('"').trim_matches('[').trim_matches(']');
        if !cleaned.is_empty() {
            return vec![cleaned.to_string()];
        }
        Vec::new()
    }

    /// Use LLM to score relevance of a search result to the query.
    /// Returns a score from 0.0 to 1.0.
    async fn score_relevance(&self, search_query: &str, result: &SearchResult) -> Option<f64> {
        let ai_client = match &self.ai_client {
            Some(c) => c,
            None => {
                // Without AI, return a moderate default
                return Some(0.5);
            }
        };

        let system_prompt = "You are a relevance assessor. Rate how relevant this search result is to the user's topic. Rate from 0.0 (not relevant) to 1.0 (highly relevant). Return ONLY a number (e.g., 0.85).";

        let user_message = format!(
            "Topic: {search_query}\nTitle: {title}\nSnippet: {snippet}",
            search_query = search_query,
            title = result.title,
            snippet = result.snippet,
        );

        match ai_client.send_message(system_prompt, &user_message).await {
            Ok(response) => {
                let trimmed = response.text.trim();
                // Try to parse the response as a float
                // Handle cases where the model returns more than just the number
                let score: f64 = trimmed
                    .chars()
                    .filter(|c| c.is_ascii_digit() || *c == '.')
                    .collect::<String>()
                    .parse()
                    .unwrap_or(0.5);

                Some(score.clamp(0.0, 1.0))
            }
            Err(e) => {
                log::warn!("[Discovery] LLM relevance scoring failed: {}", e);
                Some(0.5) // Default on failure
            }
        }
    }
}

/// Compute a SHA-256 hash for dedup purposes.
fn compute_content_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}
