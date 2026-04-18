use dioxus::prelude::*;

use crate::components::cat_button::CatButton;
use crate::icons::*;
use crate::models::Tab;

#[derive(Props, Clone, PartialEq)]
pub struct SidebarProps {
    counts: (usize, usize),
    selected_tab: Tab,
    selected_category: String,
    categories: Vec<String>,
    on_tab: EventHandler<Tab>,
    on_category: EventHandler<String>,
    on_clear_completed: EventHandler<()>,
}

#[allow(non_snake_case)]
pub fn Sidebar(props: SidebarProps) -> Element {
    let progress = if props.counts.0 + props.counts.1 > 0 {
        (props.counts.1 as f64 / (props.counts.0 + props.counts.1) as f64 * 100.0) as u32
    } else {
        0
    };
    let cats = props.categories.clone();

    rsx! {
        div { class: "w-64 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 flex flex-col shrink-0",
            div { class: "px-4 py-4 border-b border-gray-100 dark:border-gray-700",
                div { class: "flex items-center gap-2.5",
                    div { class: "w-8 h-8 rounded-xl bg-gradient-to-br from-blue-500 to-indigo-600 flex items-center justify-center text-white shadow-sm",
                        LayoutListIcon { size: 16 }
                    }
                    div {
                        p { class: "text-sm font-semibold text-gray-800 dark:text-gray-100", "待办清单" }
                        p { class: "text-[11px] text-gray-400", "三日规划" }
                    }
                }
            }

            nav { class: "flex-1 px-3 py-3 flex flex-col gap-1 min-h-0",
                button {
                    class: "w-full flex items-center gap-2.5 px-3 py-2 rounded-xl text-[14px] leading-5 transition-colors text-left",
                    class: if props.selected_tab == Tab::Yesterday { "bg-blue-50 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400 font-medium" } else { "text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700" },
                    onclick: move |_| props.on_tab.call(Tab::Yesterday),
                    CalendarIcon { size: 16 }
                    "昨天"
                }

                button {
                    class: "w-full flex items-center gap-2.5 px-3 py-2 rounded-xl text-[14px] leading-5 transition-colors text-left",
                    class: if props.selected_tab == Tab::Today { "bg-blue-50 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400 font-medium" } else { "text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700" },
                    onclick: move |_| props.on_tab.call(Tab::Today),
                    CalendarIcon { size: 16 }
                    "今天"
                }

                button {
                    class: "w-full flex items-center gap-2.5 px-3 py-2 rounded-xl text-[14px] leading-5 transition-colors text-left",
                    class: if props.selected_tab == Tab::Tomorrow { "bg-blue-50 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400 font-medium" } else { "text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700" },
                    onclick: move |_| props.on_tab.call(Tab::Tomorrow),
                    CalendarIcon { size: 16 }
                    "明天"
                }

                if !props.categories.is_empty() {
                    div { class: "mt-3 pt-3 border-t border-gray-100 dark:border-gray-700 min-h-0 flex flex-col",
                        div { class: "px-2.5 mb-1.5 flex items-center justify-between",
                            span { class: "text-[9px] font-semibold text-gray-400 uppercase tracking-widest", "分类" }
                            span { class: "text-[9px] text-gray-400", "{props.categories.len()}" }
                        }

                        button {
                            class: "w-full flex items-center gap-2.5 px-3 py-1.5 rounded-xl text-[10px] leading-4 transition-colors text-left",
                            class: if props.selected_category.is_empty() { "bg-blue-50 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400" } else { "text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700" },
                            onclick: move |_| props.on_category.call(String::new()),
                            TagIcon { size: 14 }
                            "全部"
                        }

                        div { class: "mt-1 space-y-0.5 overflow-y-auto pr-1",
                            for cat in cats.iter() {
                                CatButton {
                                    key: "{cat}",
                                    cat: cat.clone(),
                                    selected: props.selected_category == *cat,
                                    on_click: move |c: String| props.on_category.call(c),
                                }
                            }
                        }
                    }
                }
            }

            div { class: "px-4 py-3 border-t border-gray-100 dark:border-gray-700",
                div { class: "grid grid-cols-2 gap-2 mb-2",
                    div { class: "rounded-xl border border-gray-200 dark:border-gray-700 px-2.5 py-2 text-center",
                        p { class: "text-[10px] text-gray-400", "待办" }
                        p { class: "text-sm font-semibold text-gray-700 dark:text-gray-200 mt-0.5", "{props.counts.0}" }
                    }
                    div { class: "rounded-xl border border-gray-200 dark:border-gray-700 px-2.5 py-2 text-center",
                        p { class: "text-[10px] text-gray-400", "完成" }
                        p { class: "text-sm font-semibold text-emerald-600 dark:text-emerald-400 mt-0.5", "{props.counts.1}" }
                    }
                }

                div { class: "flex items-center justify-between text-xs text-gray-400",
                    span { "完成进度" }
                    span { class: "font-medium text-emerald-500", "{progress}%" }
                }
                div { class: "w-full h-1.5 bg-gray-100 dark:bg-gray-700 rounded-full mt-2 overflow-hidden",
                    div {
                        class: "h-full bg-gradient-to-r from-emerald-400 to-emerald-500 rounded-full transition-all duration-500",
                        style: "width: {progress}%",
                    }
                }
                if props.counts.1 > 0 {
                    button {
                        class: "w-full mt-2.5 text-xs text-gray-400 hover:text-red-500 transition-colors text-left",
                        onclick: move |_| props.on_clear_completed.call(()),
                        "清除 {props.counts.1} 项已完成"
                    }
                }
            }
        }
    }
}
