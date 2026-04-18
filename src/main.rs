use std::time::Duration;

mod icons;
mod models;
mod storage;
mod components;

use dioxus::prelude::*;

use components::*;
use icons::{CircleIcon, FlameIcon, MinusIcon, SearchIcon};
use models::*;
use storage::*;

fn persist_snapshot(todos: &Signal<Vec<Todo>>) {
    let snapshot = todos.read().clone();
    save_todos(&snapshot);
}

#[allow(non_snake_case)]
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

    // Build view data
    let todos_snapshot = todos.read();
    let tab_filtered = selected_tab.read().filter_todos(&todos_snapshot);
    let category_filter = selected_category.read().clone();
    let selected_priority_value = *selected_priority.read();
    let view_refs: Vec<&Todo> = if category_filter.is_empty() {
        tab_filtered
    } else {
        tab_filtered
            .into_iter()
            .filter(|t| t.category == category_filter)
            .collect()
    };

    let priority_filtered_refs: Vec<&Todo> = if let Some(priority) = selected_priority_value {
        view_refs
            .into_iter()
            .filter(|t| t.priority == priority)
            .collect()
    } else {
        view_refs
    };

    let mut view_todos: Vec<Todo> = priority_filtered_refs.into_iter().cloned().collect();
    view_todos.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    let counts = count_by_status(&todos_snapshot);
    let categories = get_categories(&todos_snapshot);
    drop(todos_snapshot);

    rsx! {
        link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" }
        link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@2.2.19/dist/tailwind.min.css" }
        style { {include_str!("../assets/app.css")} }

        div { class: "flex h-screen bg-gray-100 dark:bg-gray-900 font-sans select-none",
            Sidebar {
                counts,
                selected_tab: *selected_tab.read(),
                selected_category: selected_category.read().clone(),
                categories: categories.clone(),
                on_tab: move |tab| selected_tab.set(tab),
                on_category: move |cat| selected_category.set(cat),
                on_clear_completed: move |_| {
                    todos.write().retain(|t| !t.completed);
                    persist_snapshot(&todos);
                },
            }

            div { class: "flex-1 flex flex-col min-w-0 overflow-hidden",
                div { class: "px-4 pt-4 pb-3 shrink-0",
                    div { class: "max-w-5xl mx-auto w-full",
                        div { class: "bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-2xl px-4 py-3 shadow-sm",
                            div { class: "flex items-center justify-between gap-3",
                                div { class: "min-w-0",
                                    p { class: "text-xs text-gray-400 uppercase tracking-wide", "Workspace" }
                                    h2 { class: "text-lg font-semibold text-gray-800 dark:text-gray-100 truncate mt-0.5",
                                        "{selected_tab.read().title()}"
                                    }
                                    p { class: "text-xs text-gray-400 mt-1",
                                        if view_todos.is_empty() { "当前分组暂无任务" } else { "当前分组共 {view_todos.len()} 项" }
                                    }
                                }

                                div { class: "shrink-0 flex flex-col items-end justify-center gap-1.5",
                                    div { class: "relative",
                                        span { class: "absolute left-3 inset-y-0 flex items-center text-gray-400 pointer-events-none", SearchIcon { size: 13 } }
                                        input {
                                            class: "h-9 pl-8 pr-3 text-xs bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl outline-none focus:ring-2 focus:ring-blue-400/30 focus:border-blue-400 text-gray-700 dark:text-gray-200 placeholder-gray-400 w-72 transition-all focus:w-[40rem]",
                                            r#type: "text",
                                            placeholder: "搜索任务...",
                                            value: "{search_input}",
                                            oninput: move |e: FormEvent| {
                                                let next = e.value().clone();
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

                                    div { class: "flex items-center gap-1",
                                        button {
                                            class: "h-6 px-2 rounded-lg border border-gray-200 dark:border-gray-700 text-[10px] transition-colors flex items-center gap-1",
                                            class: if selected_priority.read().is_none() { "bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-200" } else { "bg-white dark:bg-gray-800 text-gray-500 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-700" },
                                            onclick: move |_| selected_priority.set(None),
                                            "全部"
                                        }
                                        button {
                                            class: "h-6 px-2 rounded-lg border border-emerald-200 dark:border-emerald-700 text-[10px] transition-colors flex items-center gap-1",
                                            class: if *selected_priority.read() == Some(Priority::Low) { "bg-emerald-50 dark:bg-emerald-900/30 text-emerald-600 dark:text-emerald-300" } else { "bg-white dark:bg-gray-800 text-emerald-500/80 hover:bg-emerald-50/60 dark:hover:bg-emerald-900/20" },
                                            onclick: move |_| selected_priority.set(Some(Priority::Low)),
                                            CircleIcon { size: 9 }
                                            "低"
                                        }
                                        button {
                                            class: "h-6 px-2 rounded-lg border border-amber-200 dark:border-amber-700 text-[10px] transition-colors flex items-center gap-1",
                                            class: if *selected_priority.read() == Some(Priority::Medium) { "bg-amber-50 dark:bg-amber-900/30 text-amber-600 dark:text-amber-300" } else { "bg-white dark:bg-gray-800 text-amber-500/80 hover:bg-amber-50/60 dark:hover:bg-amber-900/20" },
                                            onclick: move |_| selected_priority.set(Some(Priority::Medium)),
                                            MinusIcon { size: 9 }
                                            "中"
                                        }
                                        button {
                                            class: "h-6 px-2 rounded-lg border border-red-200 dark:border-red-700 text-[10px] transition-colors flex items-center gap-1",
                                            class: if *selected_priority.read() == Some(Priority::High) { "bg-red-50 dark:bg-red-900/30 text-red-600 dark:text-red-300" } else { "bg-white dark:bg-gray-800 text-red-500/80 hover:bg-red-50/60 dark:hover:bg-red-900/20" },
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

                div { class: "flex-1 overflow-y-auto px-4 pb-4",
                    div { class: "max-w-5xl mx-auto w-full space-y-3",
                        AddForm {
                            categories: categories.clone(),
                            on_add: move |(title, category, priority): (String, String, Priority)| {
                                let id = *next_id.read();
                                next_id.set(id + 1);
                                let new_todo = Todo {
                                    id,
                                    title,
                                    completed: false,
                                    priority,
                                    category,
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
                            on_edit_cancel: move |_| {
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
