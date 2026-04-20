use dioxus::prelude::*;

use crate::icons::{CircleIcon, FlameIcon, MinusIcon, PlusIcon, TagIcon};
use crate::models::{Priority, Tab};

#[derive(Props, Clone, PartialEq)]
pub struct AddFormProps {
    categories: Vec<String>,
    active_tab: Tab,
    on_add: EventHandler<(String, String, Priority, String)>,
}

#[allow(non_snake_case, clippy::needless_pass_by_value, clippy::too_many_lines)]
pub fn AddForm(props: AddFormProps) -> Element {
    let mut new_title = use_signal(String::new);
    let mut new_category = use_signal(String::new);
    let mut new_priority = use_signal(|| Priority::Medium);
    let mut expanded = use_signal(|| false);

    rsx! {
        div { class: "add-form-card",
            if !*expanded.read() {
                button {
                    class: "add-form-trigger transition-colors",
                    onclick: move |_| expanded.set(true),
                    PlusIcon { size: 18 }
                    span { class: "add-form-trigger-text", "添加新任务..." }
                }
            } else {
                div { class: "add-form-expanded",
                    input {
                        class: "add-form-input input-focus transition-all",
                        r#type: "text",
                        placeholder: "任务标题",
                        value: "{new_title}",
                        oninput: move |e: FormEvent| new_title.set(e.value()),
                        autofocus: true,
                    }
                    div { class: "add-form-row",
                        div { class: "add-form-input-wrap",
                            input {
                                class: "add-form-input-sm input-focus transition-all",
                                r#type: "text",
                                placeholder: "分类标签（可选）",
                                value: "{new_category}",
                                oninput: move |e: FormEvent| new_category.set(e.value()),
                            }
                        }
                        // Priority toggle buttons
                        div { class: "prio-toggle",
                            button {
                                class: if *new_priority.read() == Priority::Low { "prio-btn sel-low" } else { "prio-btn" },
                                onclick: move |_| new_priority.set(Priority::Low),
                                CircleIcon { size: 10 }
                                "低"
                            }
                            button {
                                class: if *new_priority.read() == Priority::Medium { "prio-btn sel-med" } else { "prio-btn" },
                                onclick: move |_| new_priority.set(Priority::Medium),
                                MinusIcon { size: 10 }
                                "中"
                            }
                            button {
                                class: if *new_priority.read() == Priority::High { "prio-btn sel-high" } else { "prio-btn" },
                                onclick: move |_| new_priority.set(Priority::High),
                                FlameIcon { size: 10 }
                                "高"
                            }
                        }
                    }

                    if !props.categories.is_empty() {
                        div { class: "add-form-tags",
                            span { class: "add-form-tags-label", "常用标签" }
                            for cat in props.categories.iter() {
                                button {
                                    key: "{cat}",
                                    class: "sb-tab sb-tab-cat",
                                    onclick: {
                                        let cat_value = cat.clone();
                                        move |_| new_category.set(cat_value.clone())
                                    },
                                    TagIcon { size: 11 }
                                    "{cat}"
                                }
                            }
                        }
                    }
                    div { class: "add-form-actions",
                        button {
                            class: "btn-muted add-form-btn-cancel",
                            onclick: move |_| {
                                expanded.set(false);
                                new_title.set(String::new());
                                new_category.set(String::new());
                                new_priority.set(Priority::Medium);
                            },
                            "取消"
                        }
                        button {
                            class: "btn-accent add-form-btn-submit",
                            onclick: move |_| {
                                let title = new_title.read().trim().to_string();
                                if !title.is_empty() {
                                    let category = new_category.read().trim().to_string();
                                    let date = props.active_tab.target_date_str();
                                    props.on_add.call((title, category, *new_priority.read(), date));
                                    new_title.set(String::new());
                                    new_category.set(String::new());
                                    new_priority.set(Priority::Medium);
                                    expanded.set(false);
                                }
                            },
                            PlusIcon { size: 14 }
                            "添加任务"
                        }
                    }
                }
            }
        }
    }
}
