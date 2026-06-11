use crate::commands::capture::AppState;
use chrono::Local;
use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;
use tauri::State;

fn get_db_path() -> Result<PathBuf, String> {
    dirs::data_dir()
        .ok_or_else(|| "Cannot determine data directory".to_string())
        .map(|d| d.join("com.learnwiki.app").join("learnwiki.db"))
}

fn get_backup_dir() -> Result<PathBuf, String> {
    dirs::home_dir()
        .ok_or_else(|| "Cannot determine home directory".to_string())
        .map(|d| d.join(".learnwiki").join("backups"))
}

#[tauri::command]
pub fn export_backup(state: State<'_, AppState>, path: String) -> Result<String, String> {
    let dest = PathBuf::from(&path);
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create destination directory: {}", e))?;
    }

    let conn = state
        .db
        .conn
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;

    // Vacuum into a single-file copy — handles WAL mode cleanly
    conn.execute_batch(&format!(
        "VACUUM INTO '{}'",
        path.replace('\'', "''")
    ))
    .map_err(|e| format!("Export failed: {}", e))?;

    Ok(path)
}

#[tauri::command]
pub fn import_backup(
    state: State<'_, AppState>,
    path: String,
    mode: String,
) -> Result<String, String> {
    let src = PathBuf::from(&path);
    if !src.exists() {
        return Err(format!("Backup file not found: {}", path));
    }

    let conn = state
        .db
        .conn
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;

    let attach_sql = format!("ATTACH DATABASE '{}' AS backup", path.replace('\'', "''"));

    if mode == "replace" {
        // Gather all user table names before dropping anything
        let mut tables: Vec<String> = Vec::new();
        {
            let mut stmt = conn
                .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'")
                .map_err(|e| format!("Failed to list tables: {}", e))?;
            let rows = stmt
                .query_map([], |row| row.get(0))
                .map_err(|e| format!("Failed to query tables: {}", e))?;
            for row in rows {
                tables.push(row.map_err(|e| format!("Row error: {}", e))?);
            }
        }

        // Wrap entire replace operation in a transaction for atomicity
        conn.execute_batch("BEGIN IMMEDIATE")
            .map_err(|e| format!("Failed to start transaction: {}", e))?;

        conn.execute_batch("PRAGMA foreign_keys=OFF")
            .map_err(|e| format!("Failed to disable foreign keys: {}", e))?;

        // Drop all user tables
        for table in &tables {
            conn.execute_batch(&format!("DROP TABLE IF EXISTS \"{}\"", table))
                .map_err(|e| format!("Failed to drop table {}: {}", table, e))?;
        }

        // Attach backup and recreate tables + data
        conn.execute_batch(&attach_sql)
            .map_err(|e| format!("Failed to attach backup: {}", e))?;

        // Copy tables back from backup
        for table in &tables {
            let create_sql = format!(
                "CREATE TABLE \"{}\" AS SELECT * FROM backup.\"{}\"",
                table, table
            );
            conn.execute_batch(&create_sql).or_else(|_| {
                // Table may not exist in backup — skip
                Ok::<_, rusqlite::Error>(())
            }).map_err(|e| format!("Failed to restore table {}: {}", table, e))?;
        }

        // Restore indexes and triggers from backup
        let mut schema_items: Vec<(String, String)> = Vec::new();
        {
            let mut stmt = conn
                .prepare("SELECT type, sql FROM backup.sqlite_master WHERE sql IS NOT NULL AND type IN ('index', 'trigger')")
                .map_err(|e| format!("Failed to list backup schema: {}", e))?;
            let rows = stmt
                .query_map([], |row| {
                    Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
                })
                .map_err(|e| format!("Failed to query backup schema: {}", e))?;
            for row in rows {
                schema_items.push(row.map_err(|e| format!("Row error: {}", e))?);
            }
        }
        for (_, sql) in &schema_items {
            conn.execute_batch(sql)
                .map_err(|e| format!("Failed to restore schema: {}", e))?;
        }

        conn.execute_batch("DETACH DATABASE backup")
            .map_err(|e| format!("Failed to detach backup: {}", e))?;
        conn.execute_batch("PRAGMA foreign_keys=ON")
            .map_err(|e| format!("Failed to re-enable foreign keys: {}", e))?;
        // Commit the transaction — all or nothing
        conn.execute_batch("COMMIT")
            .map_err(|e| format!("Failed to commit transaction: {}", e))?;
    } else {
        // Merge mode: insert rows from backup that don't conflict
        conn.execute_batch("BEGIN IMMEDIATE")
            .map_err(|e| format!("Failed to start transaction: {}", e))?;
        conn.execute_batch(&attach_sql)
            .map_err(|e| format!("Failed to attach backup: {}", e))?;

        let mut tables: Vec<String> = Vec::new();
        {
            let mut stmt = conn
                .prepare("SELECT name FROM backup.sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'")
                .map_err(|e| format!("Failed to list backup tables: {}", e))?;
            let rows = stmt
                .query_map([], |row| row.get(0))
                .map_err(|e| format!("Failed to query backup tables: {}", e))?;
            for row in rows {
                tables.push(row.map_err(|e| format!("Row error: {}", e))?);
            }
        }

        conn.execute_batch("PRAGMA foreign_keys=OFF")
            .map_err(|e| format!("Failed to disable foreign keys: {}", e))?;

        for table in &tables {
            conn.execute_batch(&format!(
                "INSERT OR IGNORE INTO \"{}\" SELECT * FROM backup.\"{}\"",
                table, table
            ))
            .map_err(|e| format!("Failed to merge table {}: {}", table, e))?;
        }

        conn.execute_batch("DETACH DATABASE backup")
            .map_err(|e| format!("Failed to detach backup: {}", e))?;
        conn.execute_batch("PRAGMA foreign_keys=ON")
            .map_err(|e| format!("Failed to re-enable foreign keys: {}", e))?;
        conn.execute_batch("COMMIT")
            .map_err(|e| format!("Failed to commit transaction: {}", e))?;
    }

    Ok(path)
}

#[tauri::command]
pub fn auto_backup(state: State<'_, AppState>) -> Result<Option<String>, String> {
    let backup_dir = get_backup_dir()?;
    fs::create_dir_all(&backup_dir)
        .map_err(|e| format!("Failed to create backup directory: {}", e))?;

    let today = Local::now().format("%Y-%m-%d").to_string();
    let backup_path = backup_dir.join(format!("learnwiki-backup-{}.db", today));

    if backup_path.exists() {
        return Ok(None); // Already backed up today
    }

    let conn = state
        .db
        .conn
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;

    let path_str = backup_path.to_string_lossy().replace('\'', "''");
    conn.execute_batch(&format!("VACUUM INTO '{}'", path_str))
        .map_err(|e| format!("Auto backup failed: {}", e))?;

    Ok(Some(backup_path.to_string_lossy().to_string()))
}
