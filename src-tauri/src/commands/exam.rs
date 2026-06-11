use crate::commands::capture::AppState;
use crate::storage::models::{Exam, ExamDetail, ExamSummary, ExamQuestion};
use crate::storage::repository::Repository;
use tauri::State;

// ========== Exam ==========
// Phase 2 implementation: exam creation, question generation,
// grading, and report generation commands will be added here.

/// Get all exams for a goal.
#[tauri::command]
pub fn get_exams(
    state: State<'_, AppState>,
    goal_id: String,
) -> Result<Vec<Exam>, String> {
    let repo = Repository::new(state.db.clone());
    repo.get_exams_by_goal(&goal_id).map_err(|e| e.to_string())
}

/// Get a single exam with questions.
#[tauri::command]
pub fn get_exam(
    state: State<'_, AppState>,
    id: String,
) -> Result<ExamDetail, String> {
    let repo = Repository::new(state.db.clone());
    repo.get_exam_detail(&id).map_err(|e| e.to_string())
}
