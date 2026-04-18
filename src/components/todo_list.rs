use dioxus::prelude::*;

use crate::icons::*;
use crate::models::{Priority, Todo};

#[derive(Props, Clone, PartialEq)]
pub struct TodoListProps {
    todos: Vec<Todo>,
    edit_id: Option<u64>,
    search_query: String,
    on_toggle: EventHandler<u64>,
    on_delete: EventHandler<u64>,
    on_edit_start: EventHandler<u64>,
    on_edit_save: EventHandler<(u64, String)>,
    on_edit_cancel: EventHandler<()>,
}

#[allow(non_snake_case)]
pub fn TodoList(props: TodoListProps) -> Element {
    let edit_text = use_signal(String::new);

    let all_count = props.todos.len();

    let query = props.search_query.to_lowercase();
    let displayed: Vec<&Todo> = if query.is_empty() {
        props.todos.iter().collect()
    } else {
        props
            .todos
            .iter()
            .filter(|t| {
                t.title.to_lowercase().contains(&query)
                    || t.category.to_lowercase().contains(&query)
            })
            .collect()
    };

    rsx! {
        div { class: "bg-white dark:bg-gray-800 rounded-2xl border border-gray-200 dark:border-gray-700 shadow-sm overflow-hidden",
            div { class: "px-4 py-3 border-b border-gray-100 dark:border-gray-700 bg-gray-50/70 dark:bg-gray-700/20",
                div { class: "flex items-center justify-between gap-2",
                    div {
                        p { class: "text-[15px] font-semibold tracking-tight text-gray-800 dark:text-gray-100", "任务列表" }
                        p { class: "text-[12px] text-gray-500 dark:text-gray-400 mt-0.5", "当前显示 {displayed.len()} / {all_count}" }
                    }
                    if !props.search_query.is_empty() {
                        span { class: "text-[11px] px-2 py-1 rounded-lg bg-amber-50 text-amber-600 dark:bg-amber-900/20 dark:text-amber-300 border border-amber-100 dark:border-amber-800/50 max-w-xs truncate",
                            "搜索：{props.search_query}"
                        }
                    }
                }

            }

            if displayed.is_empty() {
                div { class: "flex flex-col items-center justify-center py-20 text-gray-300 dark:text-gray-600",
                    InboxIcon { size: 42 }
                    p { class: "mt-3 text-sm font-medium", "当前筛选下暂无任务" }
                    p { class: "text-xs mt-1", "可调整筛选条件，或在上方快速添加新任务" }
                }
            } else {
                div { class: "divide-y divide-gray-100 dark:divide-gray-700/50",
                    for todo in displayed.iter() {
                        TodoRow {
                            key: "{todo.id}",
                            todo: (*todo).clone(),
                            is_editing: props.edit_id == Some(todo.id),
                            edit_text: edit_text.clone(),
                            on_toggle: props.on_toggle.clone(),
                            on_delete: props.on_delete.clone(),
                            on_edit_start: props.on_edit_start.clone(),
                            on_edit_save: props.on_edit_save.clone(),
                            on_edit_cancel: props.on_edit_cancel.clone(),
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct TodoRowProps {
    todo: Todo,
    is_editing: bool,
    edit_text: Signal<String>,
    on_toggle: EventHandler<u64>,
    on_delete: EventHandler<u64>,
    on_edit_start: EventHandler<u64>,
    on_edit_save: EventHandler<(u64, String)>,
    on_edit_cancel: EventHandler<()>,
}

#[allow(non_snake_case)]
fn TodoRow(mut props: TodoRowProps) -> Element {
    let completed = props.todo.completed;

    rsx! {
        div {
            class: "group flex items-start gap-3 px-4 py-3.5 transition-colors hover:bg-gray-50 dark:hover:bg-gray-700/30",
            button {
                class: "w-5 h-5 rounded-full border-2 flex items-center justify-center shrink-0 mt-0.5 transition-all",
                class: if completed { "bg-emerald-500 border-emerald-500 text-white" } else { "border-gray-300 dark:border-gray-600 hover:border-emerald-400" },
                onclick: {
                    let id = props.todo.id;
                    move |_| props.on_toggle.call(id)
                },
                if completed { CheckIcon { size: 12 } }
            }

            div { class: "flex-1 min-w-0",
                if props.is_editing {
                    div { class: "flex gap-2",
                        input {
                            class: "flex-1 px-3 py-2 border border-blue-400 rounded-xl bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 text-sm outline-none ring-2 ring-blue-400/30",
                            r#type: "text",
                            value: "{props.edit_text}",
                            oninput: move |e: FormEvent| props.edit_text.set(e.value().clone()),
                        }
                        button {
                            class: "px-3 py-2 text-xs bg-blue-500 text-white rounded-xl hover:bg-blue-600 transition-colors font-medium",
                            onclick: {
                                let id = props.todo.id;
                                let text_signal = props.edit_text;
                                move |_| {
                                    let text = text_signal.read().trim().to_string();
                                    if !text.is_empty() {
                                        props.on_edit_save.call((id, text));
                                    }
                                }
                            },
                            "保存"
                        }
                        button {
                            class: "px-3 py-2 text-xs text-gray-400 hover:text-gray-600 rounded-xl transition-colors",
                            onclick: move |_| props.on_edit_cancel.call(()),
                            "取消"
                        }
                    }
                } else {
                    div { class: "flex items-start justify-between gap-3",
                        div { class: "min-w-0",
                            p {
                                class: if completed { "line-through text-gray-400 text-[14px] leading-5" } else { "text-gray-800 dark:text-gray-100 text-[15px] leading-5 font-medium tracking-tight" },
                                "{props.todo.title}"
                            }
                            div { class: "flex items-center gap-2 mt-1.5",
                                span {
                                    class: "text-[9px] px-1.5 py-[1px] rounded-md border flex items-center gap-1 {priority_badge_class(props.todo.priority)}",
                                    {priority_icon(props.todo.priority)}
                                    "{priority_label(props.todo.priority)}"
                                }
                                if !props.todo.category.is_empty() {
                                    span { class: "text-[9px] px-1.5 py-[1px] rounded-md bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-300 border border-gray-200 dark:border-gray-600 flex items-center gap-1",
                                        TagIcon { size: 8 }
                                        "{props.todo.category}"
                                    }
                                }
                            }
                        }

                        div { class: "flex items-center gap-1 shrink-0 opacity-0 group-hover:opacity-100 transition-opacity",
                            button {
                                class: "p-1.5 text-gray-400 hover:text-blue-500 rounded-lg transition-colors hover:bg-blue-50 dark:hover:bg-blue-900/30",
                                onclick: {
                                    let id = props.todo.id;
                                    let title = props.todo.title.clone();
                                    move |_| {
                                        props.edit_text.set(title.clone());
                                        props.on_edit_start.call(id);
                                    }
                                },
                                EditIcon { size: 13 }
                            }
                            button {
                                class: "p-1.5 text-gray-400 hover:text-red-500 rounded-lg transition-colors hover:bg-red-50 dark:hover:bg-red-900/30",
                                onclick: {
                                    let id = props.todo.id;
                                    move |_| props.on_delete.call(id)
                                },
                                TrashIcon { size: 13 }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn priority_label(priority: Priority) -> &'static str {
    match priority {
        Priority::High => "高",
        Priority::Medium => "中",
        Priority::Low => "低",
    }
}

fn priority_badge_class(priority: Priority) -> &'static str {
    match priority {
        Priority::High => "bg-red-50 text-red-600 border-red-200 dark:bg-red-900/20 dark:text-red-300 dark:border-red-800/50",
        Priority::Medium => "bg-amber-50 text-amber-600 border-amber-200 dark:bg-amber-900/20 dark:text-amber-300 dark:border-amber-800/50",
        Priority::Low => "bg-emerald-50 text-emerald-600 border-emerald-200 dark:bg-emerald-900/20 dark:text-emerald-300 dark:border-emerald-800/50",
    }
}

fn priority_icon(priority: Priority) -> Element {
    match priority {
        Priority::High => rsx! { FlameIcon { size: 8 } },
        Priority::Medium => rsx! { MinusIcon { size: 8 } },
        Priority::Low => rsx! { CircleIcon { size: 7 } },
    }
}
