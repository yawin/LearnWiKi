use std::collections::HashMap;
use std::io::{BufRead, Read, Write};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use tauri::State;

pub struct SandboxState {
    pub sessions: Mutex<HashMap<String, Child>>,
}

/// Start a persistent bash shell process.
#[tauri::command]
pub fn sandbox_start_shell(
    sandbox: State<'_, SandboxState>,
    task_id: String,
) -> Result<String, String> {
    let _ = task_id; // Store for future use (logging/tracking)
    let session_id = uuid::Uuid::new_v4().to_string();

    // Kill any stale sessions from previous runs to prevent orphaned processes
    {
        let mut sessions = sandbox
            .sessions
            .lock()
            .map_err(|e| format!("锁错误: {e}"))?;
        for (_, mut child) in sessions.drain() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }

    let child = Command::new("bash")
        .current_dir(std::env::var("LEARNWIKI_WORKSPACE").unwrap_or_else(|_| "/Users/macbook/llmwiki/learnwiki".to_string()))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("启动 shell 失败: {e}"))?;

    let mut sessions = sandbox
        .sessions
        .lock()
        .map_err(|e| format!("锁错误: {e}"))?;
    sessions.insert(session_id.clone(), child);
    Ok(session_id)
}

/// Write a command to the shell's stdin, with a security allowlist.
#[tauri::command]
pub fn sandbox_write_stdin(
    sandbox: State<'_, SandboxState>,
    session_id: String,
    input: String,
) -> Result<(), String> {
    // Security: only allow known-safe commands through the sandbox
    let trimmed = input.trim();
    let first_word = trimmed.split_whitespace().next().unwrap_or("");
    let allowed = [
        "ls", "cat", "head", "tail", "echo", "pwd", "cd", "mkdir", "touch",
        "cp", "mv", "rm", "grep", "find", "sort", "wc", "diff", "which",
        "python3", "python", "node", "npm", "cargo", "rustc", "git",
        "curl", "wget", "tar", "gzip", "gunzip", "unzip", "zip",
        "chmod", "chown", "ps", "top", "kill", "date", "whoami", "id",
        "env", "export", "tee", "jq", "sed", "awk",
    ];
    if !trimmed.is_empty() && !allowed.contains(&first_word) {
        // Allow running scripts/paths starting with ./ or /
        if !first_word.starts_with("./") && !first_word.starts_with('/') {
            return Err(format!("命令「{}」不在允许列表中", first_word));
        }
    }
    let mut sessions = sandbox
        .sessions
        .lock()
        .map_err(|e| format!("锁错误: {e}"))?;
    let child = sessions
        .get_mut(&session_id)
        .ok_or("Session not found")?;
    if let Some(stdin) = child.stdin.as_mut() {
        writeln!(stdin, "{}", input).map_err(|e| format!("写入 stdin 失败: {e}"))
    } else {
        Err("stdin 未打开".to_string())
    }
}

/// Read the shell's stdout output.
#[tauri::command]
pub fn sandbox_read_stdout(
    sandbox: State<'_, SandboxState>,
    session_id: String,
) -> Result<String, String> {
    let mut sessions = sandbox
        .sessions
        .lock()
        .map_err(|e| format!("锁错误: {e}"))?;
    let child = sessions
        .get_mut(&session_id)
        .ok_or("Session not found")?;
    if let Some(stdout) = child.stdout.as_mut() {
        // Use BufReader + read_line instead of read_to_string to avoid hanging
        // on persistent shell stdout that never closes (EOF).
        let mut reader = std::io::BufReader::new(stdout);
        let mut buf = String::new();
        reader
            .read_line(&mut buf)
            .map_err(|e| format!("读取 stdout 失败: {e}"))?;
        Ok(buf.trim_end().to_string())
    } else {
        Err("stdout 未打开".to_string())
    }
}

/// Kill a shell process and clean up.
#[tauri::command]
pub fn sandbox_kill_shell(
    sandbox: State<'_, SandboxState>,
    session_id: String,
) -> Result<(), String> {
    let mut sessions = sandbox
        .sessions
        .lock()
        .map_err(|e| format!("锁错误: {e}"))?;
    if let Some(mut child) = sessions.remove(&session_id) {
        child.kill().map_err(|e| format!("终止失败: {e}"))?;
        child.wait().ok(); // Reap zombie process
        Ok(())
    } else {
        Err("Session not found".to_string())
    }
}
