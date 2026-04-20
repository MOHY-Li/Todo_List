use std::fs;
use std::path::PathBuf;

use chrono::{Local, TimeZone};

use crate::models::{three_day_window, Todo};

const STORAGE_FILE: &str = "todos.json";

fn storage_path() -> PathBuf {
    let dir = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    let app_dir = dir.join("dioxus-todo");
    fs::create_dir_all(&app_dir).ok();
    app_dir.join(STORAGE_FILE)
}

/// Migrate old entries where `date` is empty: derive from `created_at`.
fn migrate_dates(todos: &mut [Todo]) {
    let fallback = Local::now().date_naive().format("%Y-%m-%d").to_string();
    for t in todos.iter_mut() {
        if t.date.is_empty() {
            t.date = Local::timestamp_opt(&Local, t.created_at.cast_signed(), 0)
                .single()
                .map_or_else(
                    || fallback.clone(),
                    |dt| dt.date_naive().format("%Y-%m-%d").to_string(),
                );
        }
    }
}

/// Remove tasks whose date falls outside the three-day window (yesterday/today/tomorrow).
fn purge_expired(todos: &mut Vec<Todo>) -> bool {
    let window = three_day_window();
    let before = todos.len();
    todos.retain(|t| window.contains(&t.date));
    todos.len() != before
}

pub fn load_todos() -> Vec<Todo> {
    let path = storage_path();
    let mut todos: Vec<Todo> = fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();

    migrate_dates(&mut todos);
    let purged = purge_expired(&mut todos);

    // Auto-save if migration or purge happened
    if purged || !todos.is_empty() {
        save_todos(&todos);
    }

    todos
}

pub fn save_todos(todos: &[Todo]) {
    let path = storage_path();
    if let Ok(json) = serde_json::to_string_pretty(todos) {
        let tmp_path = path.with_extension("json.tmp");
        if fs::write(&tmp_path, json).is_ok() {
            let _ = fs::rename(tmp_path, path);
        }
    }
}
