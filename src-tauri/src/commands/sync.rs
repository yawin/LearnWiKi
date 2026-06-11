use crate::capture::content::compute_hash;
use crate::commands::capture::AppState;
use crate::storage::models::{CapturedContent, ContentType, SyncFolder, SyncRecord, SyncResult};
use crate::storage::repository::Repository;
use std::path::Path;
use tauri::State;

const SUPPORTED_EXTENSIONS: &[(&str, &str)] = &[
    ("md", "md"),
    ("txt", "txt"),
    ("pdf", "pdf"),
    ("docx", "docx"),
    ("epub", "epub"),
    ("png", "image"),
    ("jpg", "image"),
    ("jpeg", "image"),
    ("webp", "image"),
];

fn get_file_type(ext: &str) -> Option<&'static str> {
    SUPPORTED_EXTENSIONS
        .iter()
        .find(|(e, _)| *e == ext)
        .map(|(_, t)| *t)
}

#[tauri::command]
pub fn add_sync_folder(state: State<'_, AppState>, path: String) -> Result<SyncFolder, String> {
    if !Path::new(&path).is_dir() {
        return Err(format!("路径不存在或不是文件夹: {}", path));
    }

    let repo = Repository::new(state.db.clone());
    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();

    let folder = SyncFolder {
        id,
        path,
        enabled: true,
        last_synced_at: None,
        created_at: now,
    };

    repo.save_sync_folder(&folder).map_err(|e| e.to_string())?;
    Ok(folder)
}

#[tauri::command]
pub fn remove_sync_folder(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let repo = Repository::new(state.db.clone());
    repo.delete_sync_folder(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_sync_folders(state: State<'_, AppState>) -> Result<Vec<SyncFolder>, String> {
    let repo = Repository::new(state.db.clone());
    repo.get_sync_folders().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_sync_folder(
    state: State<'_, AppState>,
    id: String,
    enabled: bool,
) -> Result<(), String> {
    let repo = Repository::new(state.db.clone());
    repo.update_sync_folder_enabled(&id, enabled)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn start_sync(
    state: State<'_, AppState>,
    folder_id: Option<String>,
) -> Result<SyncResult, String> {
    let repo = Repository::new(state.db.clone());
    let folders = repo.get_sync_folders().map_err(|e| e.to_string())?;

    let target_folders: Vec<SyncFolder> = match folder_id {
        Some(ref fid) => folders
            .into_iter()
            .filter(|f| f.id == *fid && f.enabled)
            .collect(),
        None => folders.into_iter().filter(|f| f.enabled).collect(),
    };

    if target_folders.is_empty() {
        return Ok(SyncResult {
            imported: vec![],
            updated: vec![],
            skipped: 0,
            errors: vec![],
        });
    }

    let mut result = SyncResult {
        imported: vec![],
        updated: vec![],
        skipped: 0,
        errors: vec![],
    };
    let now = chrono::Utc::now().to_rfc3339();

    for folder in &target_folders {
        let folder_path = Path::new(&folder.path);
        if !folder_path.is_dir() {
            result
                .errors
                .push(format!("文件夹不存在: {}", folder.path));
            continue;
        }

        let entries = match walk_dir_recursive(folder_path) {
            Ok(e) => e,
            Err(err) => {
                result
                    .errors
                    .push(format!("扫描失败 {}: {}", folder.path, err));
                continue;
            }
        };

        for entry_path in entries {
            let ext = entry_path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();

            let file_type = match get_file_type(&ext) {
                Some(t) => t,
                None => continue,
            };

            let file_path_str = entry_path.to_string_lossy().to_string();
            let file_name = entry_path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();

            let metadata = match std::fs::metadata(&entry_path) {
                Ok(m) => m,
                Err(e) => {
                    result
                        .errors
                        .push(format!("无法读取 {}: {}", file_name, e));
                    continue;
                }
            };

            let file_size = metadata.len() as i64;
            let mtime = metadata
                .modified()
                .map(|t| {
                    let dt: chrono::DateTime<chrono::Utc> = t.into();
                    dt.to_rfc3339()
                })
                .unwrap_or_else(|_| now.clone());

            let existing = repo
                .get_sync_record_by_path(&folder.id, &file_path_str)
                .map_err(|e| e.to_string())?;

            match existing {
                Some(ref rec) if rec.file_mtime == mtime => {
                    result.skipped += 1;
                    continue;
                }
                Some(_) => {
                    let content_id = import_file_content(
                        &repo,
                        &entry_path,
                        file_type,
                        &file_name,
                        &folder.path,
                    )?;
                    let record = SyncRecord {
                        id: uuid::Uuid::new_v4().to_string(),
                        folder_id: folder.id.clone(),
                        file_path: file_path_str,
                        file_name: file_name.clone(),
                        file_size: Some(file_size),
                        file_mtime: mtime,
                        file_type: file_type.to_string(),
                        content_id: Some(content_id),
                        status: "updated".to_string(),
                        synced_at: now.clone(),
                    };
                    repo.upsert_sync_record(&record)
                        .map_err(|e| e.to_string())?;
                    result.updated.push(file_name);
                }
                None => {
                    let content_id = import_file_content(
                        &repo,
                        &entry_path,
                        file_type,
                        &file_name,
                        &folder.path,
                    )?;
                    let record = SyncRecord {
                        id: uuid::Uuid::new_v4().to_string(),
                        folder_id: folder.id.clone(),
                        file_path: file_path_str,
                        file_name: file_name.clone(),
                        file_size: Some(file_size),
                        file_mtime: mtime,
                        file_type: file_type.to_string(),
                        content_id: Some(content_id),
                        status: "imported".to_string(),
                        synced_at: now.clone(),
                    };
                    repo.upsert_sync_record(&record)
                        .map_err(|e| e.to_string())?;
                    result.imported.push(file_name);
                }
            }
        }

        let _ = repo.update_sync_folder_last_synced(&folder.id, &now);
    }

    Ok(result)
}

fn walk_dir_recursive(dir: &Path) -> Result<Vec<std::path::PathBuf>, String> {
    let mut files = Vec::new();
    let entries = std::fs::read_dir(dir).map_err(|e| e.to_string())?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            if path
                .file_name()
                .map(|n| n.to_string_lossy().starts_with('.'))
                .unwrap_or(false)
            {
                continue;
            }
            let mut sub = walk_dir_recursive(&path)?;
            files.append(&mut sub);
        } else {
            files.push(path);
        }
    }
    Ok(files)
}

fn import_file_content(
    repo: &Repository,
    path: &Path,
    file_type: &str,
    file_name: &str,
    folder_path: &str,
) -> Result<String, String> {
    let now = chrono::Utc::now().to_rfc3339();
    let content_id = uuid::Uuid::new_v4().to_string();

    match file_type {
        "md" | "txt" => {
            let text = std::fs::read_to_string(path)
                .map_err(|e| format!("读取失败 {}: {}", file_name, e))?;
            let hash = compute_hash(text.as_bytes());
            let content = CapturedContent {
                id: content_id.clone(),
                content_type: ContentType::Text,
                raw_text: Some(text),
                image_path: None,
                thumbnail_path: None,
                source_app: format!("folder_sync:{}", folder_path),
                source_bundle_id: None,
                source_url: Some(path.to_string_lossy().to_string()),
                user_note: Some(format!("从文件夹同步: {}", file_name)),
                captured_at: now.clone(),
                content_hash: hash,
                byte_size: std::fs::metadata(path).map(|m| m.len() as i64).unwrap_or(0),
                is_deleted: false,
                created_at: now.clone(),
                updated_at: now,
                digested_at: None,
                digest_action: None,
                summary: None,
                tags: None,
                digest: None,
                wiki_compile_hash: None,
                wiki_assessed_hash: None,
                clean_content: None,
            };
            repo.save_content(&content).map_err(|e| e.to_string())?;
        }
        "pdf" | "docx" | "epub" => {
            let note = format!("从文件夹同步: {} (需要编译提取内容)", file_name);
            let hash = compute_hash(path.to_string_lossy().as_bytes());
            let content = CapturedContent {
                id: content_id.clone(),
                content_type: ContentType::Text,
                raw_text: Some(format!(
                    "[文件] {}\n路径: {}",
                    file_name,
                    path.to_string_lossy()
                )),
                image_path: None,
                thumbnail_path: None,
                source_app: format!("folder_sync:{}", folder_path),
                source_bundle_id: None,
                source_url: Some(path.to_string_lossy().to_string()),
                user_note: Some(note),
                captured_at: now.clone(),
                content_hash: hash,
                byte_size: std::fs::metadata(path).map(|m| m.len() as i64).unwrap_or(0),
                is_deleted: false,
                created_at: now.clone(),
                updated_at: now,
                digested_at: None,
                digest_action: None,
                summary: None,
                tags: None,
                digest: None,
                wiki_compile_hash: None,
                wiki_assessed_hash: None,
                clean_content: None,
            };
            repo.save_content(&content).map_err(|e| e.to_string())?;
        }
        "image" => {
            let hash = compute_hash(path.to_string_lossy().as_bytes());
            let content = CapturedContent {
                id: content_id.clone(),
                content_type: ContentType::Image,
                raw_text: None,
                image_path: Some(path.to_string_lossy().to_string()),
                thumbnail_path: None,
                source_app: format!("folder_sync:{}", folder_path),
                source_bundle_id: None,
                source_url: None,
                user_note: Some(format!("从文件夹同步: {}", file_name)),
                captured_at: now.clone(),
                content_hash: hash,
                byte_size: std::fs::metadata(path).map(|m| m.len() as i64).unwrap_or(0),
                is_deleted: false,
                created_at: now.clone(),
                updated_at: now,
                digested_at: None,
                digest_action: None,
                summary: None,
                tags: None,
                digest: None,
                wiki_compile_hash: None,
                wiki_assessed_hash: None,
                clean_content: None,
            };
            repo.save_content(&content).map_err(|e| e.to_string())?;
        }
        _ => {}
    }

    Ok(content_id)
}
