use dioxus::prelude::*;

use crate::icons::{
    CheckIcon, CircleIcon, EditIcon, FlameIcon, InboxIcon, MinusIcon, TagIcon, TrashIcon,
};
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

#[allow(non_snake_case, clippy::needless_pass_by_value)]
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
        div { class: "tl-card",
            div { class: "tl-header",
                div { class: "tl-header-row",
                    div {
                        p { class: "tl-title", "任务列表" }
                        p { class: "tl-subtitle", "当前显示 {displayed.len()} / {all_count}" }
                    }
                    if !props.search_query.is_empty() {
                        span { class: "tl-search-badge",
                            "搜索：{props.search_query}"
                        }
                    }
                }

            }

            if displayed.is_empty() {
                div { class: "tl-empty",
                    InboxIcon { size: 42 }
                    p { class: "tl-empty-text", "当前筛选下暂无任务" }
                    p { class: "tl-empty-sub", "可调整筛选条件，或在上方快速添加新任务" }
                }
            } else {
                div {
                    for todo in displayed.iter() {
                        TodoRow {
                            key: "{todo.id}",
                            todo: (*todo).clone(),
                            is_editing: props.edit_id == Some(todo.id),
                            edit_text,
                            on_toggle: props.on_toggle,
                            on_delete: props.on_delete,
                            on_edit_start: props.on_edit_start,
                            on_edit_save: props.on_edit_save,
                            on_edit_cancel: props.on_edit_cancel,
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

#[allow(non_snake_case, clippy::needless_pass_by_value, clippy::too_many_lines)]
fn TodoRow(mut props: TodoRowProps) -> Element {
    let completed = props.todo.completed;

    rsx! {
        div {
            class: "action-parent transition-colors tl-row",
            button {
                class: if completed { "tl-check tl-check-done" } else { "tl-check tl-check-pending hover-muted" },
                onclick: {
                    let id = props.todo.id;
                    move |_| props.on_toggle.call(id)
                },
                if completed { CheckIcon { size: 12 } }
            }

            div { class: "tl-body",
                if props.is_editing {
                    div { class: "tl-edit-row",
                        input {
                            class: "input-focus tl-edit-input",
                            r#type: "text",
                            value: "{props.edit_text}",
                            oninput: move |e: FormEvent| props.edit_text.set(e.value()),
                        }
                        button {
                            class: "btn-accent tl-edit-save",
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
                            class: "btn-ghost tl-edit-cancel",
                            onclick: move |_| props.on_edit_cancel.call(()),
                            "取消"
                        }
                    }
                } else {
                    div { class: "tl-row-inner",
                        div { class: "tl-title-area",
                            p {
                                class: if completed { "tl-todo-done" } else { "tl-todo-active" },
                                "{props.todo.title}"
                            }
                            div { class: "tl-tags",
                                span {
                                    class: "tl-priority-badge {priority_badge_class(props.todo.priority)}",
                                    {priority_icon(props.todo.priority)}
                                    "{priority_label(props.todo.priority)}"
                                }
                                if !props.todo.category.is_empty() {
                                    span { class: "tl-category",
                                        TagIcon { size: 8 }
                                        "{props.todo.category}"
                                    }
                                }
                            }
                        }

                        div { class: "action-group tl-actions",
                            button {
                                class: "btn-ghost tl-action-btn",
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
                                class: "btn-ghost hover-danger tl-action-btn",
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

const fn priority_label(priority: Priority) -> &'static str {
    match priority {
        Priority::High => "高",
        Priority::Medium => "中",
        Priority::Low => "低",
    }
}

const fn priority_badge_class(priority: Priority) -> &'static str {
    match priority {
        Priority::High => "badge-high",
        Priority::Medium => "badge-medium",
        Priority::Low => "badge-low",
    }
}

fn priority_icon(priority: Priority) -> Element {
    match priority {
        Priority::High => rsx! { FlameIcon { size: 8 } },
        Priority::Medium => rsx! { MinusIcon { size: 8 } },
        Priority::Low => rsx! { CircleIcon { size: 7 } },
    }
}
