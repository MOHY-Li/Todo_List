use dioxus::prelude::*;

use crate::icons::*;
use crate::models::Priority;

#[derive(Props, Clone, PartialEq)]
pub struct AddFormProps {
    categories: Vec<String>,
    on_add: EventHandler<(String, String, Priority)>,
}

#[allow(non_snake_case)]
pub fn AddForm(props: AddFormProps) -> Element {
    let mut new_title = use_signal(|| String::new());
    let mut new_category = use_signal(|| String::new());
    let mut new_priority = use_signal(|| Priority::Medium);
    let mut expanded = use_signal(|| false);

    rsx! {
        div { class: "bg-white dark:bg-gray-800 rounded-xl p-3 border border-gray-200 dark:border-gray-700 shadow-sm",
            if !*expanded.read() {
                button {
                    class: "w-full flex items-center gap-3 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors py-1",
                    onclick: move |_| expanded.set(true),
                    PlusIcon { size: 18 }
                    span { class: "text-sm", "添加新任务..." }
                }
            } else {
                div { class: "flex flex-col gap-3",
                    input {
                        class: "w-full px-4 py-2.5 border border-gray-200 dark:border-gray-600 rounded-xl bg-gray-50 dark:bg-gray-700/50 text-gray-900 dark:text-gray-100 placeholder-gray-400 text-sm outline-none focus:border-blue-400 focus:ring-2 focus:ring-blue-400/20 transition-all",
                        r#type: "text",
                        placeholder: "任务标题",
                        value: "{new_title}",
                        oninput: move |e: FormEvent| new_title.set(e.value().clone()),
                        autofocus: true,
                    }
                    div { class: "flex gap-2 items-center",
                        div { class: "flex-1",
                            input {
                                class: "w-full px-4 py-2 border border-gray-200 dark:border-gray-600 rounded-xl bg-gray-50 dark:bg-gray-700/50 text-gray-900 dark:text-gray-100 placeholder-gray-400 text-sm outline-none focus:border-blue-400 focus:ring-2 focus:ring-blue-400/20 transition-all",
                                r#type: "text",
                                placeholder: "分类标签（可选）",
                                value: "{new_category}",
                                oninput: move |e: FormEvent| new_category.set(e.value().clone()),
                            }
                        }
                        // Priority toggle buttons
                        div { class: "flex items-center gap-1 bg-gray-100 dark:bg-gray-700 rounded-xl px-2 py-1",
                            button {
                                class: "h-7 px-2 rounded-lg flex items-center justify-center gap-1 text-xs transition-all",
                                class: if *new_priority.read() == Priority::Low { "bg-emerald-500 text-white shadow-sm" } else { "text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-600" },
                                onclick: move |_| new_priority.set(Priority::Low),
                                CircleIcon { size: 10 }
                                "低"
                            }
                            button {
                                class: "h-7 px-2 rounded-lg flex items-center justify-center gap-1 text-xs font-medium transition-all",
                                class: if *new_priority.read() == Priority::Medium { "bg-amber-400 text-black shadow-sm" } else { "text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-600" },
                                onclick: move |_| new_priority.set(Priority::Medium),
                                MinusIcon { size: 10 }
                                "中"
                            }
                            button {
                                class: "h-7 px-2 rounded-lg flex items-center justify-center gap-1 text-xs transition-all",
                                class: if *new_priority.read() == Priority::High { "bg-red-500 text-white shadow-sm" } else { "text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-600" },
                                onclick: move |_| new_priority.set(Priority::High),
                                FlameIcon { size: 10 }
                                "高"
                            }
                        }
                    }

                    if !props.categories.is_empty() {
                        div { class: "flex items-center gap-2 flex-wrap",
                            span { class: "text-[11px] text-gray-400", "常用标签" }
                            for cat in props.categories.iter() {
                                button {
                                    key: "{cat}",
                                    class: "px-2 py-1 rounded-lg text-[11px] border border-gray-200 dark:border-gray-600 bg-white dark:bg-gray-700/50 text-gray-500 dark:text-gray-300 hover:border-blue-300 hover:text-blue-600 dark:hover:text-blue-300 transition-colors",
                                    onclick: {
                                        let cat_value = cat.clone();
                                        move |_| new_category.set(cat_value.clone())
                                    },
                                    "{cat}"
                                }
                            }
                        }
                    }
                    div { class: "flex gap-2",
                        button {
                            class: "px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-500 rounded-xl hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors text-sm",
                            onclick: move |_| {
                                expanded.set(false);
                                new_title.set(String::new());
                                new_category.set(String::new());
                                new_priority.set(Priority::Medium);
                            },
                            "取消"
                        }
                        button {
                            class: "flex-1 px-4 py-2 bg-blue-500 text-white rounded-xl hover:bg-blue-600 transition-colors text-sm font-medium flex items-center justify-center gap-1.5",
                            onclick: move |_| {
                                let title = new_title.read().trim().to_string();
                                if !title.is_empty() {
                                    let category = new_category.read().trim().to_string();
                                    props.on_add.call((title, category, *new_priority.read()));
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
