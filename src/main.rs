use std::time::Duration;

mod components;
mod icons;
mod models;
mod storage;

use dioxus::prelude::*;

use components::{AddForm, Sidebar, TodoList};
use icons::{CircleIcon, FlameIcon, MinusIcon, SearchIcon};
use models::{count_by_status, get_categories, Priority, Tab, Todo};
use storage::{load_todos, save_todos};

fn persist_snapshot(todos: &Signal<Vec<Todo>>) {
    let snapshot = todos.read().clone();
    save_todos(&snapshot);
}

#[allow(non_snake_case, clippy::too_many_lines)]
fn App() -> Element {
    let mut todos = use_signal(load_todos);
    let mut next_id = use_signal(|| {
        let ts = todos.read();
        ts.iter().map(|t| t.id).max().unwrap_or(0) + 1
    });

    let mut selected_tab = use_signal(|| Tab::Today);
    let mut selected_category = use_signal(String::new);
    let mut selected_priority = use_signal(|| Option::<Priority>::None);
    let mut edit_id = use_signal(|| Option::<u64>::None);
    let search_query = use_signal(String::new);
    let mut search_input = use_signal(String::new);

    // Build view data — single chain filter
    let todos_snapshot = todos.read();
    let tab_filtered = selected_tab.read().filter_todos(&todos_snapshot);
    let category_filter = selected_category.read().clone();
    let selected_priority_value = *selected_priority.read();
    let mut view_todos: Vec<Todo> = tab_filtered
        .into_iter()
        .filter(|t| category_filter.is_empty() || t.category == category_filter)
        .filter(|t| selected_priority_value.is_none_or(|p| t.priority == p))
        .cloned()
        .collect();
    view_todos.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    let counts = count_by_status(&todos_snapshot);
    let categories = get_categories(&todos_snapshot);
    drop(todos_snapshot);

    rsx! {
        style { {include_str!("../assets/app.css")} }

        div { class: "app-root",
            Sidebar {
                counts,
                selected_tab: *selected_tab.read(),
                selected_category: selected_category.read().clone(),
                categories: categories.clone(),
                on_tab: move |tab| selected_tab.set(tab),
                on_category: move |cat| selected_category.set(cat),
                on_clear_completed: move |()| {
                    todos.write().retain(|t| !t.completed);
                    persist_snapshot(&todos);
                },
            }

            div { class: "main-content",
                div { class: "header-outer",
                    div { class: "centered-wrap",
                        div { class: "header-card",
                            div { class: "header-row",
                                div { class: "header-labels",
                                    p { class: "label-sm", "Workspace" }
                                    h2 { class: "title-md",
                                        "{selected_tab.read().title()}"
                                    }
                                    p { class: "subtitle-sm",
                                        if view_todos.is_empty() { "当前分组暂无任务" } else { "当前分组共 {view_todos.len()} 项" }
                                    }
                                }

                                div { class: "header-actions",
                                    div { class: "search-wrap",
                                        span { class: "search-icon", SearchIcon { size: 13 } }
                                        input {
                                            class: "search-input input-focus search-field",
                                            r#type: "text",
                                            placeholder: "搜索任务...",
                                            value: "{search_input}",
                                            oninput: move |e: FormEvent| {
                                                let next = e.value();
                                                search_input.set(next.clone());
                                                spawn({
                                                    let current_input = search_input;
                                                    let mut query = search_query;
                                                    async move {
                                                        tokio::time::sleep(Duration::from_millis(220)).await;
                                                        if *current_input.read() == next {
                                                            query.set(next);
                                                        }
                                                    }
                                                });
                                            },
                                        }
                                    }

                                    div { class: "filter-row",
                                        button {
                                            class: if selected_priority.read().is_none() { "filter-btn active" } else { "filter-btn" },
                                            onclick: move |_| selected_priority.set(None),
                                            "全部"
                                        }
                                        button {
                                            class: if *selected_priority.read() == Some(Priority::Low) { "filter-btn active-low" } else { "filter-btn" },
                                            onclick: move |_| selected_priority.set(Some(Priority::Low)),
                                            CircleIcon { size: 9 }
                                            "低"
                                        }
                                        button {
                                            class: if *selected_priority.read() == Some(Priority::Medium) { "filter-btn active-med" } else { "filter-btn" },
                                            onclick: move |_| selected_priority.set(Some(Priority::Medium)),
                                            MinusIcon { size: 9 }
                                            "中"
                                        }
                                        button {
                                            class: if *selected_priority.read() == Some(Priority::High) { "filter-btn active-high" } else { "filter-btn" },
                                            onclick: move |_| selected_priority.set(Some(Priority::High)),
                                            FlameIcon { size: 9 }
                                            "高"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "content-scroll",
                    div { class: "content-inner",
                        AddForm {
                            categories,
                            active_tab: *selected_tab.read(),
                            on_add: move |(title, category, priority, date): (String, String, Priority, String)| {
                                let id = *next_id.read();
                                next_id.set(id + 1);
                                let new_todo = Todo {
                                    id,
                                    title,
                                    completed: false,
                                    priority,
                                    category,
                                    date,
                                    created_at: std::time::SystemTime::now()
                                        .duration_since(std::time::UNIX_EPOCH)
                                        .unwrap_or_default()
                                        .as_secs(),
                                };
                                todos.write().insert(0, new_todo);
                                persist_snapshot(&todos);
                            },
                        }

                        TodoList {
                            todos: view_todos,
                            edit_id: *edit_id.read(),
                            search_query: search_query.read().clone(),
                            on_toggle: move |id| {
                                for t in todos.write().iter_mut() {
                                    if t.id == id { t.completed = !t.completed; break; }
                                }
                                persist_snapshot(&todos);
                            },
                            on_delete: move |id| {
                                todos.write().retain(|t| t.id != id);
                                persist_snapshot(&todos);
                            },
                            on_edit_start: move |id| {
                                edit_id.set(Some(id));
                            },
                            on_edit_save: move |(id, new_title): (u64, String)| {
                                for t in todos.write().iter_mut() {
                                    if t.id == id {
                                        t.title = new_title.trim().to_string();
                                        break;
                                    }
                                }
                                edit_id.set(None);
                                persist_snapshot(&todos);
                            },
                            on_edit_cancel: move |()| {
                                edit_id.set(None);
                            },
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    dioxus::LaunchBuilder::new().launch(App);
}
