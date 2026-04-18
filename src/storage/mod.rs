use std::fs;
use std::path::PathBuf;

use crate::models::Todo;

const STORAGE_FILE: &str = "todos.json";

fn storage_path() -> PathBuf {
    let dir = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    let app_dir = dir.join("dioxus-todo");
    fs::create_dir_all(&app_dir).ok();
    app_dir.join(STORAGE_FILE)
}

pub fn load_todos() -> Vec<Todo> {
    let path = storage_path();
    fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save_todos(todos: &[Todo]) {
    let path = storage_path();
    if let Ok(json) = serde_json::to_string_pretty(todos) {
        // Atomic write: write to temp first, then rename.
        // This avoids partially-written JSON when app is interrupted.
        let tmp_path = path.with_extension("json.tmp");
        if fs::write(&tmp_path, json).is_ok() {
            let _ = fs::rename(tmp_path, path);
        }
    }
}
