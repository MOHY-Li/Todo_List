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
            class: if props.selected { "sb-tab active sb-tab-cat" } else { "sb-tab sb-tab-cat" },
            onclick: move |_| props.on_click.call(props.cat.clone()),
            TagIcon { size: 11 }
            span { class: "truncate", "{props.cat}" }
        }
    }
}
