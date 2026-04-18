use dioxus::prelude::*;

use crate::icons::TagIcon;

#[derive(Props, Clone, PartialEq)]
pub struct CatButtonProps {
    cat: String,
    selected: bool,
    on_click: EventHandler<String>,
}

#[allow(non_snake_case)]
pub fn CatButton(props: CatButtonProps) -> Element {
    rsx! {
        button {
            class: "w-full flex items-center gap-2 px-3 py-1.5 rounded-xl text-[9px] leading-4 transition-colors text-left",
            class: if props.selected { "bg-blue-50 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400 font-medium" } else { "text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700" },
            onclick: move |_| props.on_click.call(props.cat.clone()),
            TagIcon { size: 11 }
            span { class: "truncate", "{props.cat}" }
        }
    }
}
