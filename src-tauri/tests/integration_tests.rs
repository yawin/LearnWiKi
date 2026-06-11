use app_lib::storage::database::Database;
use app_lib::storage::models::{Goal, GoalWikiLink, GoalWikiLinkWithTitle, WikiPage};
use app_lib::storage::repository::Repository;
use std::sync::Arc;

fn setup_db() -> Arc<Database> {
    Arc::new(Database::new_in_memory().expect("Failed to create test DB"))
}

fn make_page(id: &str, title: &str, tags: &str, body: &str) -> WikiPage {
    WikiPage {
        id: id.to_string(),
        title: title.to_string(),
        slug: id.to_string(),
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

fn make_goal(id: &str, title: &str) -> Goal {
    Goal {
        id: id.to_string(),
        title: title.to_string(),
        description: String::new(),
        keywords: "[]".to_string(),
        status: "active".to_string(),
        progress: 0.0,
        created_at: String::new(),
        updated_at: String::new(),
    }
}

#[test]
fn test_wiki_read_status_upsert_and_read() {
    let db = setup_db();
    let repo = Repository::new(db);

    // Initially false
    assert!(!repo.get_wiki_read_status("w1").unwrap());

    // Set to true
    repo.set_wiki_read_status("w1", true).unwrap();
    assert!(repo.get_wiki_read_status("w1").unwrap());

    // Set back to false
    repo.set_wiki_read_status("w1", false).unwrap();
    assert!(!repo.get_wiki_read_status("w1").unwrap());
}

#[test]
fn test_goal_wiki_links_enrichment() {
    let db = setup_db();
    let repo = Repository::new(db);

    // Create a wiki page and a goal, then link them
    let page = make_page("wp1", "Rust 所有权", "rust", "测试内容");
    repo.save_wiki_page(&page).unwrap();

    let goal = make_goal("g1", "掌握 Rust");
    repo.save_goal(&goal).unwrap();

    let link = GoalWikiLink {
        id: "l1".to_string(),
        goal_id: "g1".to_string(),
        wiki_page_id: "wp1".to_string(),
        relevance_score: 0.8,
        source: "auto".to_string(),
        is_new: true,
        created_at: String::new(),
    };
    repo.save_goal_wiki_link(&link).unwrap();

    let enriched: Vec<GoalWikiLinkWithTitle> =
        repo.get_goal_wiki_links_with_titles("g1").unwrap();
    assert_eq!(enriched.len(), 1);
    assert_eq!(enriched[0].wiki_title, "Rust 所有权");
    assert_eq!(enriched[0].goal_id, "g1");
    assert_eq!(enriched[0].wiki_page_id, "wp1");
}

#[test]
fn test_goal_wiki_link_mark_seen_and_lookup() {
    let db = setup_db();
    let repo = Repository::new(db);

    // Create wiki page and goal
    let page = make_page("wp2", "Rust 所有权", "rust", "Rust 所有权详细内容");
    repo.save_wiki_page(&page).unwrap();

    let goal = make_goal("g2", "掌握 Rust");
    repo.save_goal(&goal).unwrap();

    // Link goal to wiki page
    let link = GoalWikiLink {
        id: "l2".to_string(),
        goal_id: "g2".to_string(),
        wiki_page_id: "wp2".to_string(),
        relevance_score: 0.9,
        source: "auto".to_string(),
        is_new: true,
        created_at: String::new(),
    };
    repo.save_goal_wiki_link(&link).unwrap();

    // Verify link is visible from page-side lookup
    let page_links = repo.get_goal_wiki_links_for_page("wp2").unwrap();
    assert_eq!(page_links.len(), 1);
    assert_eq!(page_links[0].goal_id, "g2");
    assert!(page_links[0].is_new);

    // Mark links seen for this goal
    repo.mark_goal_wiki_links_seen("g2").unwrap();

    // After mark, is_new should be false
    let page_links2 = repo.get_goal_wiki_links_for_page("wp2").unwrap();
    assert_eq!(page_links2.len(), 1);
    assert!(!page_links2[0].is_new);
}
