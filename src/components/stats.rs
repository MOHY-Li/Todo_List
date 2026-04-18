use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct StatsProps {
    total: usize,
    active: usize,
    completed: usize,
}

#[allow(non_snake_case)]
pub fn StatsOverview(props: StatsProps) -> Element {
    let progress = if props.total > 0 {
        ((props.completed as f64 / props.total as f64) * 100.0) as u32
    } else {
        0
    };

    rsx! {
        div { class: "bg-white dark:bg-gray-800 rounded-2xl border border-gray-200 dark:border-gray-700 shadow-sm p-3",
            div { class: "flex items-center justify-between",
                h3 { class: "text-xs font-semibold text-gray-500 uppercase tracking-wide", "概览" }
                span { class: "text-[11px] text-emerald-500 font-medium", "完成率 {progress}%" }
            }

            div { class: "grid grid-cols-3 gap-2 mt-2.5",
                div { class: "rounded-xl border border-gray-200 dark:border-gray-700 px-2.5 py-2",
                    p { class: "text-[10px] text-gray-400", "总任务" }
                    p { class: "text-base font-semibold text-gray-800 dark:text-gray-100 mt-0.5", "{props.total}" }
                }
                div { class: "rounded-xl border border-amber-200/70 dark:border-amber-700/40 bg-amber-50/60 dark:bg-amber-900/15 px-2.5 py-2",
                    p { class: "text-[10px] text-amber-500", "待办" }
                    p { class: "text-base font-semibold text-amber-600 dark:text-amber-400 mt-0.5", "{props.active}" }
                }
                div { class: "rounded-xl border border-emerald-200/70 dark:border-emerald-700/40 bg-emerald-50/60 dark:bg-emerald-900/15 px-2.5 py-2",
                    p { class: "text-[10px] text-emerald-500", "完成" }
                    p { class: "text-base font-semibold text-emerald-600 dark:text-emerald-400 mt-0.5", "{props.completed}" }
                }
            }
        }
    }
}
