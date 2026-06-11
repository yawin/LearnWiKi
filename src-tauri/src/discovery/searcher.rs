use serde::Deserialize;
use serde::Serialize;

/// A single search result from any source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
    pub source_name: String,
}

/// Raw SerpAPI response shape
#[derive(Debug, Deserialize)]
struct SerpApiResponse {
    #[serde(default)]
    organic_results: Vec<SerpApiOrganic>,
    #[serde(default)]
    error: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SerpApiOrganic {
    title: String,
    link: String,
    snippet: String,
    source: Option<String>,
}

/// Search the web using SerpAPI (Google engine).
///
/// Calls `https://serpapi.com/search` with the given query and API key,
/// returns the top N organic results.
pub async fn search_web(
    query: &str,
    api_key: &str,
    num_results: Option<u32>,
) -> Result<Vec<SearchResult>, String> {
    let num = num_results.unwrap_or(5).min(10);

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    let response = client
        .get("https://serpapi.com/search")
        .query(&[
            ("q", query),
            ("api_key", api_key),
            ("engine", "google"),
            ("num", &num.to_string()),
        ])
        .send()
        .await
        .map_err(|e| format!("SerpAPI request failed: {}", e))?;

    let status = response.status();
    let body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read SerpAPI response: {}", e))?;

    if !status.is_success() {
        return Err(format!("SerpAPI returned error {}: {}", status, body));
    }

    let parsed: SerpApiResponse = serde_json::from_str(&body)
        .map_err(|e| format!("Failed to parse SerpAPI response: {} body: {}", e, body))?;

    if let Some(err) = parsed.error {
        return Err(format!("SerpAPI error: {}", err));
    }

    let results = parsed
        .organic_results
        .into_iter()
        .map(|r| SearchResult {
            title: r.title,
            url: r.link,
            snippet: r.snippet,
            source_name: r.source.unwrap_or_else(|| "web".to_string()),
        })
        .collect();

    Ok(results)
}

// ==================== ArXiv Search ====================

/// Search ArXiv via the public API.
///
/// Calls `http://export.arxiv.org/api/query?search_query=all:{query}&start=0&max_results=5&sortBy=submittedDate&sortOrder=descending`
/// and parses the Atom XML response.
pub async fn search_arxiv(query: &str) -> Result<Vec<SearchResult>, String> {
    let url = format!(
        "http://export.arxiv.org/api/query?search_query=all:{}&start=0&max_results=5&sortBy=submittedDate&sortOrder=descending",
        urlencoding(query)
    );

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    let response = client
        .get(&url)
        .header("User-Agent", "LearnWiki/1.0")
        .send()
        .await
        .map_err(|e| format!("ArXiv API request failed: {}", e))?;

    let status = response.status();
    let body = response.text().await.map_err(|e| format!("Failed to read ArXiv response: {}", e))?;

    if !status.is_success() {
        return Err(format!("ArXiv API returned error {}: {}", status, body));
    }

    // Parse the Atom XML using simple string extraction
    // ArXiv returns <entry> elements with <title>, <id>, <summary>, <published>
    let mut results = Vec::new();

    // Split on <entry> boundaries
    let entries: Vec<&str> = body.split("<entry>").collect();
    // Skip the first split (everything before the first <entry>)
    for entry_str in entries.iter().skip(1) {
        let entry = match entry_str.find("</entry>") {
            Some(end) => &entry_str[..end],
            None => continue,
        };

        let title = extract_xml_tag(entry, "title").unwrap_or_default();
        let arxiv_id = extract_xml_tag(entry, "id").unwrap_or_default();
        let summary = extract_xml_tag(entry, "summary").unwrap_or_default();

        // The id field looks like "http://arxiv.org/abs/XXXX.XXXXXvN"
        let abs_url = arxiv_id.trim().replace("http://", "https://");

        // Truncate summary to ~200 chars
        let snippet = summary.chars().take(200).collect::<String>().replace('\n', " ");

        if !title.is_empty() && !abs_url.is_empty() {
            results.push(SearchResult {
                title: title.trim().to_string(),
                url: abs_url,
                snippet: snippet.trim().to_string(),
                source_name: "arxiv".to_string(),
            });
        }
    }

    // Limit to top 5
    results.truncate(5);
    Ok(results)
}

// ==================== RSS Feed Search ====================

/// Fetch and parse an RSS/Atom feed.
///
/// Supports both RSS 2.0 (item → title/link/description) and
/// Atom (entry → title/link[@rel=\"alternate\"]/summary).
/// Returns up to 10 most recent entries.
pub async fn search_rss(feed_url: &str, feed_name: &str) -> Result<Vec<SearchResult>, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    let response = client
        .get(feed_url)
        .header("User-Agent", "LearnWiki/1.0")
        .send()
        .await
        .map_err(|e| format!("RSS feed request failed: {}", e))?;

    let status = response.status();
    let body = response.text().await.map_err(|e| format!("Failed to read RSS response: {}", e))?;

    if !status.is_success() {
        return Err(format!("RSS feed returned error {}: {}", status, body));
    }

    let mut results = Vec::new();
    let source_name = format!("rss:{}", feed_name);

    // Try Atom format first (<entry> elements)
    if body.contains("<entry") {
        let entries: Vec<&str> = body.split("<entry").collect();
        for entry_str in entries.iter().skip(1) {
            let entry = match entry_str.find("</entry>") {
                Some(end) => &entry_str[..end],
                None => continue,
            };

            let title = extract_xml_tag(entry, "title").unwrap_or_default();
            let summary = extract_xml_tag(entry, "summary").unwrap_or_default();

            // Atom links look like: <link href="..." /> or <link rel="alternate" href="..." />
            let link = extract_atom_link(entry);

            if !title.is_empty() {
                let snippet = summary.chars().take(200).collect::<String>().replace('\n', " ");
                results.push(SearchResult {
                    title: title.trim().to_string(),
                    url: link,
                    snippet: snippet.trim().to_string(),
                    source_name: source_name.clone(),
                });
            }
        }
    }

    // Try RSS 2.0 format (<item> elements)
    if results.is_empty() && body.contains("<item") {
        let items: Vec<&str> = body.split("<item").collect();
        for item_str in items.iter().skip(1) {
            let item = match item_str.find("</item>") {
                Some(end) => &item_str[..end],
                None => continue,
            };

            let title = extract_xml_tag(item, "title").unwrap_or_default();
            let link = extract_cdata_or_content(item, "link");
            let description = extract_xml_tag(item, "description").unwrap_or_default();
            // Also try content:encoded
            let content = if description.is_empty() {
                extract_xml_tag(item, "content:encoded").unwrap_or_default()
            } else {
                description
            };

            if !title.is_empty() {
                let snippet = content.chars().take(200).collect::<String>().replace('\n', " ");
                results.push(SearchResult {
                    title: title.trim().to_string(),
                    url: link,
                    snippet: snippet.trim().to_string(),
                    source_name: source_name.clone(),
                });
            }
        }
    }

    // Limit to top 10
    results.truncate(10);
    Ok(results)
}

// ==================== XML Helper ====================

/// Extract the text content of an XML tag (handles CDATA).
/// Returns the text between `<tag>` and `</tag>`.
fn extract_xml_tag(xml: &str, tag: &str) -> Option<String> {
    let open_tag = format!("<{}>", tag);
    let close_tag = format!("</{}>", tag);

    if let Some(start) = xml.find(&open_tag) {
        let content_start = start + open_tag.len();
        if let Some(end) = xml[content_start..].find(&close_tag) {
            let raw = &xml[content_start..content_start + end];
            // Handle CDATA
            if raw.starts_with("<![CDATA[") && raw.ends_with("]]>") {
                let inner = &raw[9..raw.len() - 3];
                return Some(inner.to_string());
            }
            // Strip any HTML entities
            let decoded = raw
                .replace("&amp;", "&")
                .replace("&lt;", "<")
                .replace("&gt;", ">")
                .replace("&quot;", "\"")
                .replace("&#39;", "'")
                .replace("&apos;", "'");
            return Some(decoded);
        }
    }
    None
}

/// Extract a link from an Atom entry (looks for `<link href="..." />` or `<link rel="alternate" href="..." />`).
fn extract_atom_link(entry: &str) -> String {
    // Try to find <link> tags
    let mut start = 0;
    while let Some(link_start) = entry[start..].find("<link") {
        let section_start = start + link_start;
        if let Some(close) = entry[section_start..].find('>') {
            let link_tag = &entry[section_start..section_start + close + 1];

            // Prefer rel="alternate"
            let is_alternate = link_tag.contains("rel=\"alternate\"") || link_tag.contains("rel='alternate'");
            if let Some(href_start) = link_tag.find("href=\"") {
                let value_start = href_start + 6;
                if let Some(href_end) = link_tag[value_start..].find('"') {
                    let url = &link_tag[value_start..value_start + href_end];
                    if is_alternate || !url.contains("pdf") {
                        return url.to_string();
                    }
                }
            }
            start = section_start + close;
        } else {
            break;
        }
    }
    String::new()
}

/// Extract content from an RSS tag that might have CDATA wrapping.
fn extract_cdata_or_content(xml: &str, tag: &str) -> String {
    let open_tag = format!("<{}", tag);
    let close_tag = format!("</{}>", tag);

    if let Some(start) = xml.find(&open_tag) {
        // Find the closing >
        let after_open = &xml[start..];
        let tag_close = after_open.find('>').unwrap_or(0);
        let content_start = start + tag_close + 1;
        if let Some(end) = xml[content_start..].find(&close_tag) {
            let raw = &xml[content_start..content_start + end];
            if raw.starts_with("<![CDATA[") && raw.ends_with("]]>") {
                return raw[9..raw.len() - 3].to_string();
            }
            return raw.to_string();
        }
    }
    String::new()
}

/// URL-encode a query string for ArXiv API.
fn urlencoding(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '_' | '-' | '.' => result.push(c),
            ' ' => result.push_str("%20"),
            other => {
                for b in other.to_string().as_bytes() {
                    result.push_str(&format!("%{:02X}", b));
                }
            }
        }
    }
    result
}
