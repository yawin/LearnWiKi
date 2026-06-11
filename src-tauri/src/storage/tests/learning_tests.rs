use crate::storage::database::Database;
use crate::storage::models::{
    LearningPath, Module, PracticeTask, TaskDailyLog,
};
use crate::storage::repository::Repository;
use std::sync::Arc;

fn make_db() -> Database {
    Database::new_in_memory().expect("Failed to create in-memory database")
}

fn make_repo() -> Repository {
    Repository::new(Arc::new(make_db()))
}

fn make_learning_path(id: &str) -> LearningPath {
    LearningPath {
        id: id.to_string(),
        title: format!("Learning Path {id}"),
        description: "A test learning path".to_string(),
        topic: "rust".to_string(),
        difficulty: "beginner".to_string(),
        estimated_days: 30,
        module_count: 5,
        completion_rate: 0.0,
        is_active: true,
        created_at: "2026-06-07T00:00:00Z".to_string(),
        updated_at: "2026-06-07T00:00:00Z".to_string(),
    }
}

fn make_module(id: &str, path_id: &str, sort_order: i32) -> Module {
    Module {
        id: id.to_string(),
        path_id: path_id.to_string(),
        title: format!("Module {id}"),
        sort_order,
        description: "A test module".to_string(),
        theory_markdown: "# Theory\n\nContent here".to_string(),
        reading_list_json: "[]".to_string(),
        estimated_read_minutes: 15,
        discussion_prompts: "[]".to_string(),
        community_solutions: "[]".to_string(),
        task_ids: "[]".to_string(),
        status: "available".to_string(),
        completed_at: None,
        created_at: "2026-06-07T00:00:00Z".to_string(),
        updated_at: "2026-06-07T00:00:00Z".to_string(),
    }
}

fn make_task(id: &str, module_id: &str) -> PracticeTask {
    PracticeTask {
        id: id.to_string(),
        module_id: module_id.to_string(),
        title: format!("Task {id}"),
        description: "A test practice task".to_string(),
        difficulty: "easy".to_string(),
        estimated_minutes: 10,
        prerequisites: "[]".to_string(),
        hint_content: None,
        reference_links: None,
        status: "not_started".to_string(),
        started_at: None,
        completed_at: None,
        attempt_count: 0,
        is_starred: false,
        reflection: None,
        code_snippets: None,
        screenshots_json: None,
        created_wiki_pages: "[]".to_string(),
        related_wiki_pages: "[]".to_string(),
        tags: None,
        created_at: "2026-06-07T00:00:00Z".to_string(),
        updated_at: "2026-06-07T00:00:00Z".to_string(),
    }
}

fn make_daily_log(date: &str) -> TaskDailyLog {
    TaskDailyLog {
        id: format!("log-{date}"),
        date: date.to_string(),
        total_minutes: 60,
        tasks_completed: 2,
        tasks_in_progress: 1,
        streak_day: 3,
        reflection: Some("Good progress today".to_string()),
        created_at: "2026-06-07T00:00:00Z".to_string(),
    }
}

// ========== LearningPath CRUD ==========

#[test]
fn test_learning_path_save_and_get_all() {
    let repo = make_repo();
    let lp = make_learning_path("lp1");
    repo.save_learning_path(&lp).unwrap();

    let all = repo.get_all_learning_paths().unwrap();
    assert_eq!(all.len(), 1);
    assert_eq!(all[0].id, "lp1");
    assert_eq!(all[0].title, "Learning Path lp1");
}

#[test]
fn test_learning_path_get_by_id() {
    let repo = make_repo();
    let lp = make_learning_path("lp2");
    repo.save_learning_path(&lp).unwrap();

    let found = repo.get_learning_path_by_id("lp2").unwrap().unwrap();
    assert_eq!(found.id, "lp2");
    assert_eq!(found.difficulty, "beginner");
    assert!(found.is_active);

    let not_found = repo.get_learning_path_by_id("nonexistent").unwrap();
    assert!(not_found.is_none());
}

#[test]
fn test_learning_path_update() {
    let repo = make_repo();
    let mut lp = make_learning_path("lp3");
    repo.save_learning_path(&lp).unwrap();

    lp.title = "Updated Title".to_string();
    lp.difficulty = "advanced".to_string();
    lp.completion_rate = 0.5;
    lp.is_active = false;
    repo.update_learning_path(&lp).unwrap();

    let updated = repo.get_learning_path_by_id("lp3").unwrap().unwrap();
    assert_eq!(updated.title, "Updated Title");
    assert_eq!(updated.difficulty, "advanced");
    assert!((updated.completion_rate - 0.5).abs() < 1e-6);
    assert!(!updated.is_active);
}

#[test]
fn test_learning_path_delete() {
    let repo = make_repo();
    let lp = make_learning_path("lp4");
    repo.save_learning_path(&lp).unwrap();
    assert_eq!(repo.get_all_learning_paths().unwrap().len(), 1);

    repo.delete_learning_path("lp4").unwrap();
    assert_eq!(repo.get_all_learning_paths().unwrap().len(), 0);
}

// ========== Module CRUD ==========

#[test]
fn test_module_save_by_path_and_get() {
    let repo = make_repo();
    let lp = make_learning_path("path1");
    repo.save_learning_path(&lp).unwrap();

    let modules = vec![
        make_module("m1", "path1", 1),
        make_module("m2", "path1", 2),
    ];
    repo.save_modules_by_path(&modules).unwrap();

    let fetched = repo.get_modules_by_path_id("path1").unwrap();
    assert_eq!(fetched.len(), 2);
    assert_eq!(fetched[0].id, "m1"); // Sorted by sort_order ASC
    assert_eq!(fetched[1].id, "m2");
}

#[test]
fn test_module_update_and_delete() {
    let repo = make_repo();
    let lp = make_learning_path("path2");
    repo.save_learning_path(&lp).unwrap();

    let mut m = make_module("m3", "path2", 1);
    repo.save_modules_by_path(&[m.clone()]).unwrap();

    m.title = "Updated Module".to_string();
    m.status = "completed".to_string();
    repo.update_module(&m).unwrap();

    let fetched = repo.get_modules_by_path_id("path2").unwrap();
    assert_eq!(fetched[0].title, "Updated Module");
    assert_eq!(fetched[0].status, "completed");

    repo.delete_module("m3").unwrap();
    assert_eq!(repo.get_modules_by_path_id("path2").unwrap().len(), 0);
}

// ========== PracticeTask CRUD ==========

#[test]
fn test_practice_task_save_and_get_by_module() {
    let repo = make_repo();
    let lp = make_learning_path("path3");
    repo.save_learning_path(&lp).unwrap();
    let m = make_module("mod1", "path3", 1);
    repo.save_modules_by_path(&[m]).unwrap();

    let task = make_task("t1", "mod1");
    repo.save_practice_task(&task).unwrap();

    let tasks = repo.get_tasks_by_module_id("mod1").unwrap();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].id, "t1");
    assert_eq!(tasks[0].status, "not_started");
}

#[test]
fn test_practice_task_status_transition_legal() {
    let repo = make_repo();
    let lp = make_learning_path("path4");
    repo.save_learning_path(&lp).unwrap();
    let m = make_module("mod2", "path4", 1);
    repo.save_modules_by_path(&[m]).unwrap();

    let task = make_task("t2", "mod2");
    repo.save_practice_task(&task).unwrap();

    // not_started → in_progress ✓
    repo.update_task_status("t2", "in_progress").unwrap();
    let t = &repo.get_tasks_by_module_id("mod2").unwrap()[0];
    assert_eq!(t.status, "in_progress");

    // in_progress → completed ✓
    repo.update_task_status("t2", "completed").unwrap();
    let t = &repo.get_tasks_by_module_id("mod2").unwrap()[0];
    assert_eq!(t.status, "completed");

    // completed → reviewed ✓
    repo.update_task_status("t2", "reviewed").unwrap();
    let t = &repo.get_tasks_by_module_id("mod2").unwrap()[0];
    assert_eq!(t.status, "reviewed");
}

#[test]
fn test_practice_task_status_transition_illegal() {
    let repo = make_repo();
    let lp = make_learning_path("path5");
    repo.save_learning_path(&lp).unwrap();
    let m = make_module("mod3", "path5", 1);
    repo.save_modules_by_path(&[m]).unwrap();

    let task = make_task("t3", "mod3");
    repo.save_practice_task(&task).unwrap();

    // not_started → completed ✗ (skip in_progress)
    let err = repo.update_task_status("t3", "completed").unwrap_err();
    assert!(err.to_string().contains("Invalid status transition"));

    // reviewed → anything ✗
    repo.update_task_status("t3", "in_progress").unwrap(); // not_started → in_progress
    repo.update_task_status("t3", "completed").unwrap(); // in_progress → completed
    repo.update_task_status("t3", "reviewed").unwrap(); // completed → reviewed
    let err2 = repo.update_task_status("t3", "in_progress").unwrap_err();
    assert!(err2.to_string().contains("Invalid status transition"));
}

#[test]
fn test_practice_task_delete() {
    let repo = make_repo();
    let lp = make_learning_path("path6");
    repo.save_learning_path(&lp).unwrap();
    let m = make_module("mod4", "path6", 1);
    repo.save_modules_by_path(&[m]).unwrap();

    let task = make_task("t4", "mod4");
    repo.save_practice_task(&task).unwrap();
    assert_eq!(repo.get_tasks_by_module_id("mod4").unwrap().len(), 1);

    repo.delete_practice_task("t4").unwrap();
    assert_eq!(repo.get_tasks_by_module_id("mod4").unwrap().len(), 0);
}

#[test]
fn test_get_all_practice_tasks() {
    let repo = make_repo();
    let lp = make_learning_path("path-all");
    repo.save_learning_path(&lp).unwrap();
    let m1 = make_module("m-all-1", "path-all", 1);
    let m2 = make_module("m-all-2", "path-all", 2);
    repo.save_modules_by_path(&[m1, m2]).unwrap();

    let t1 = make_task("t-all-1", "m-all-1");
    let t2 = make_task("t-all-2", "m-all-1");
    let t3 = make_task("t-all-3", "m-all-2");
    repo.save_practice_task(&t1).unwrap();
    repo.save_practice_task(&t2).unwrap();
    repo.save_practice_task(&t3).unwrap();

    let all = repo.get_all_practice_tasks().unwrap();
    assert_eq!(all.len(), 3);
    let ids: Vec<&str> = all.iter().map(|t| t.id.as_str()).collect();
    assert!(ids.contains(&"t-all-1"));
    assert!(ids.contains(&"t-all-2"));
    assert!(ids.contains(&"t-all-3"));
}

#[test]
fn test_get_practice_task_by_id() {
    let repo = make_repo();
    let lp = make_learning_path("path-byid");
    repo.save_learning_path(&lp).unwrap();
    let m = make_module("m-byid", "path-byid", 1);
    repo.save_modules_by_path(&[m]).unwrap();

    let task = make_task("t-byid", "m-byid");
    repo.save_practice_task(&task).unwrap();

    let found = repo.get_task_by_id("t-byid").unwrap().unwrap();
    assert_eq!(found.id, "t-byid");
    assert_eq!(found.title, "Task t-byid");
    assert_eq!(found.status, "not_started");

    let not_found = repo.get_task_by_id("nonexistent").unwrap();
    assert!(not_found.is_none());
}

// ========== TaskDailyLog CRUD ==========

#[test]
fn test_daily_log_save_or_update_today_upsert() {
    let repo = make_repo();

    let log1 = make_daily_log("2026-06-07");
    repo.save_or_update_today_log(&log1).unwrap();

    let recent = repo.get_recent_daily_logs(7).unwrap();
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].date, "2026-06-07");
    assert_eq!(recent[0].total_minutes, 60);

    // Upsert — same date, different values
    let log2 = TaskDailyLog {
        total_minutes: 90,
        tasks_completed: 3,
        ..log1
    };
    repo.save_or_update_today_log(&log2).unwrap();

    let recent = repo.get_recent_daily_logs(7).unwrap();
    assert_eq!(recent.len(), 1); // still one row
    assert_eq!(recent[0].total_minutes, 90);
    assert_eq!(recent[0].tasks_completed, 3);
}

#[test]
fn test_daily_log_get_recent() {
    let repo = make_repo();

    let d1 = make_daily_log("2026-06-05");
    let d2 = make_daily_log("2026-06-06");
    let d3 = make_daily_log("2026-06-07");
    repo.save_or_update_today_log(&d1).unwrap();
    repo.save_or_update_today_log(&d2).unwrap();
    repo.save_or_update_today_log(&d3).unwrap();

    let recent = repo.get_recent_daily_logs(2).unwrap();
    assert_eq!(recent.len(), 2); // only 2 newest
    assert_eq!(recent[0].date, "2026-06-07");
    assert_eq!(recent[1].date, "2026-06-06");

    let all = repo.get_recent_daily_logs(10).unwrap();
    assert_eq!(all.len(), 3);
}

// ========== Streak & Stats (E-3-3) ==========

#[test]
fn test_calculate_streak_with_consecutive_days() {
    let repo = make_repo();

    // Insert 3 consecutive daily logs with tasks_completed > 0
    let d1 = TaskDailyLog {
        id: "log-2026-06-05".to_string(),
        date: "2026-06-05".to_string(),
        total_minutes: 30,
        tasks_completed: 2,
        tasks_in_progress: 1,
        streak_day: 0,
        reflection: None,
        created_at: "2026-06-05T23:00:00Z".to_string(),
    };
    let d2 = TaskDailyLog {
        id: "log-2026-06-06".to_string(),
        date: "2026-06-06".to_string(),
        total_minutes: 45,
        tasks_completed: 1,
        tasks_in_progress: 0,
        streak_day: 0,
        reflection: None,
        created_at: "2026-06-06T23:00:00Z".to_string(),
    };
    let d3 = TaskDailyLog {
        id: "log-2026-06-07".to_string(),
        date: "2026-06-07".to_string(),
        total_minutes: 60,
        tasks_completed: 3,
        tasks_in_progress: 0,
        streak_day: 0,
        reflection: None,
        created_at: "2026-06-07T23:00:00Z".to_string(),
    };

    repo.save_or_update_today_log(&d1).unwrap();
    repo.save_or_update_today_log(&d2).unwrap();
    repo.save_or_update_today_log(&d3).unwrap();

    // With 2026-06-07 as "today", streak should be 3 (all consecutive with tasks_completed > 0)
    // These are the only logs, ordered DESC, so we start from the first (index 0 = 2026-06-07)
    let logs = repo.get_recent_daily_logs(10).unwrap();
    assert_eq!(logs.len(), 3);
    assert_eq!(logs[0].date, "2026-06-07");
    assert!(logs[0].tasks_completed > 0);
}

#[test]
fn test_count_tasks_by_status_counts_correctly() {
    let repo = make_repo();
    let lp = make_learning_path("stats-path");
    repo.save_learning_path(&lp).unwrap();
    let m = make_module("stats-mod", "stats-path", 1);
    repo.save_modules_by_path(&[m]).unwrap();

    let mut t1 = make_task("stats-t1", "stats-mod");
    t1.status = "completed".to_string();
    repo.save_practice_task(&t1).unwrap();

    let mut t2 = make_task("stats-t2", "stats-mod");
    t2.status = "in_progress".to_string();
    repo.save_practice_task(&t2).unwrap();

    let mut t3 = make_task("stats-t3", "stats-mod");
    t3.status = "not_started".to_string();
    repo.save_practice_task(&t3).unwrap();

    let (total, completed, in_progress) = repo.count_tasks_by_status().unwrap();
    assert_eq!(total, 3);
    assert_eq!(completed, 1);
    assert_eq!(in_progress, 1);
}

#[test]
fn test_get_total_study_minutes_sums_correctly() {
    let repo = make_repo();

    let d1 = TaskDailyLog {
        id: "log-2026-06-05".to_string(),
        date: "2026-06-05".to_string(),
        total_minutes: 30,
        tasks_completed: 1,
        tasks_in_progress: 0,
        streak_day: 0,
        reflection: None,
        created_at: "2026-06-05T23:00:00Z".to_string(),
    };
    let d2 = TaskDailyLog {
        id: "log-2026-06-06".to_string(),
        date: "2026-06-06".to_string(),
        total_minutes: 45,
        tasks_completed: 2,
        tasks_in_progress: 0,
        streak_day: 0,
        reflection: None,
        created_at: "2026-06-06T23:00:00Z".to_string(),
    };

    repo.save_or_update_today_log(&d1).unwrap();
    repo.save_or_update_today_log(&d2).unwrap();

    let total = repo.get_total_study_minutes().unwrap();
    assert_eq!(total, 75);
}

// ========== Cascade Delete ==========

#[test]
fn test_cascade_delete_learning_path_removes_modules_and_tasks() {
    let repo = make_repo();
    let lp = make_learning_path("cascade1");
    repo.save_learning_path(&lp).unwrap();

    let m = make_module("cmod1", "cascade1", 1);
    repo.save_modules_by_path(&[m]).unwrap();

    let task = make_task("ct1", "cmod1");
    repo.save_practice_task(&task).unwrap();

    assert_eq!(repo.get_modules_by_path_id("cascade1").unwrap().len(), 1);
    assert_eq!(repo.get_tasks_by_module_id("cmod1").unwrap().len(), 1);

    // Delete the learning path → cascade should remove modules and tasks
    repo.delete_learning_path("cascade1").unwrap();

    assert_eq!(repo.get_modules_by_path_id("cascade1").unwrap().len(), 0);
    assert_eq!(repo.get_tasks_by_module_id("cmod1").unwrap().len(), 0);
}
