use dioxus::prelude::*;

use crate::components::cat_button::CatButton;
use crate::icons::{CalendarIcon, LayoutListIcon, TagIcon};
use crate::models::{today_display, Tab};

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

#[allow(non_snake_case, clippy::needless_pass_by_value, clippy::too_many_lines)]
pub fn Sidebar(props: SidebarProps) -> Element {
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation
    )]
    let progress = if props.counts.0 + props.counts.1 > 0 {
        (props.counts.1 as f64 / (props.counts.0 + props.counts.1) as f64 * 100.0) as u32
    } else {
        0
    };
    let cats = props.categories.clone();

    rsx! {
        div { class: "sidebar",
            // Header
            div { class: "sb-header",
                div { class: "sb-header-inner",
                    div { class: "gradient-icon sb-icon",
                        LayoutListIcon { size: 16 }
                    }
                    div {
                        p { class: "sb-title", "待办清单" }
                        p { class: "sb-subtitle", "{today_display()}" }
                    }
                }
            }

            // Navigation
            nav { class: "sb-nav",
                div { class: "sb-section-head",
                    span { class: "sb-label", "日期" }
                }

                button {
                    class: if props.selected_tab == Tab::Yesterday { "sb-tab active sb-tab-date" } else { "sb-tab sb-tab-date" },
                    onclick: move |_| props.on_tab.call(Tab::Yesterday),
                    CalendarIcon { size: 14 }
                    "{Tab::Yesterday.label()}"
                }

                button {
                    class: if props.selected_tab == Tab::Today { "sb-tab active sb-tab-date" } else { "sb-tab sb-tab-date" },
                    onclick: move |_| props.on_tab.call(Tab::Today),
                    CalendarIcon { size: 14 }
                    "{Tab::Today.label()}"
                }

                button {
                    class: if props.selected_tab == Tab::Tomorrow { "sb-tab active sb-tab-date" } else { "sb-tab sb-tab-date" },
                    onclick: move |_| props.on_tab.call(Tab::Tomorrow),
                    CalendarIcon { size: 14 }
                    "{Tab::Tomorrow.label()}"
                }

                // Categories section
                if !props.categories.is_empty() {
                    div { class: "sb-section",
                        div { class: "sb-section-head",
                            span { class: "sb-label", "分类" }
                            span { class: "sb-count", "{props.categories.len()}" }
                        }

                        button {
                            class: if props.selected_category.is_empty() { "sb-tab active sb-tab-cat" } else { "sb-tab sb-tab-cat" },
                            onclick: move |_| props.on_category.call(String::new()),
                            TagIcon { size: 11 }
                            "全部"
                        }

                        div { class: "sb-cat-list",
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

            // Stats footer
            div { class: "sb-footer",
                div { class: "sb-stats",
                    div { class: "sb-stat-card",
                        p { class: "sb-stat-label", "待办" }
                        p { class: "sb-stat-val", "{props.counts.0}" }
                    }
                    div { class: "sb-stat-card",
                        p { class: "sb-stat-label", "完成" }
                        p { class: "sb-stat-val done", "{props.counts.1}" }
                    }
                }

                div { class: "sb-progress-row",
                    span { "完成进度" }
                    span { class: "sb-progress-pct", "{progress}%" }
                }
                div { class: "sb-progress-bar",
                    div {
                        class: "gradient-progress sb-progress-fill",
                        style: "width:{progress}%",
                    }
                }
                if props.counts.1 > 0 {
                    button {
                        class: "sb-clear",
                        onclick: move |_| props.on_clear_completed.call(()),
                        "清除 {props.counts.1} 项已完成"
                    }
                }
            }
        }
    }
}
