use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq, Eq)]
pub struct IconProps {
    pub size: u32,
}

/// Macro to define Lucide-style SVG icon components.
/// Usage: `icon!(CheckIcon, r#"<path d="M20 6L9 17l-5-5" stroke-width="2.5"/>"#);`
macro_rules! icon {
    ($name:ident, $body:expr) => {
        #[allow(non_snake_case)]
        #[component]
        pub fn $name(props: IconProps) -> Element {
            rsx! {
                svg {
                    width: "{props.size}",
                    height: "{props.size}",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    dangerous_inner_html: $body,
                }
            }
        }
    };
}

icon!(CheckIcon, r#"<path d="M20 6L9 17l-5-5" stroke-width="2.5"/>"#);
icon!(EditIcon, r#"<path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>"#);
icon!(TrashIcon, r#"<polyline points="3 6 5 6 21 6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/><line x1="10" y1="11" x2="10" y2="17"/><line x1="14" y1="11" x2="14" y2="17"/>"#);
icon!(PlusIcon, r#"<line x1="12" y1="5" x2="12" y2="19" stroke-width="2.5"/><line x1="5" y1="12" x2="19" y2="12" stroke-width="2.5"/>"#);
icon!(CircleIcon, r#"<circle cx="12" cy="12" r="8"/>"#);
icon!(MinusIcon, r#"<line x1="5" y1="12" x2="19" y2="12"/>"#);
icon!(TagIcon, r#"<path d="M20.59 13.41l-7.17 7.17a2 2 0 0 1-2.83 0L2 12V2h10l8.59 8.59a2 2 0 0 1 0 2.82z"/><line x1="7" y1="7" x2="7.01" y2="7"/>"#);
icon!(InboxIcon, r#"<polyline points="22 12 16 12 14 15 10 15 8 12 2 12"/><path d="M5.45 5.11L2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"/>"#);
icon!(CalendarIcon, r#"<rect x="3" y="4" width="18" height="18" rx="2" ry="2"/><line x1="16" y1="2" x2="16" y2="6"/><line x1="8" y1="2" x2="8" y2="6"/><line x1="3" y1="10" x2="21" y2="10"/>"#);
icon!(FlameIcon, r#"<path d="M8.5 14.5A2.5 2.5 0 0 0 11 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 1 1-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 0 0 2.5 2.5z"/>"#);
icon!(SearchIcon, r#"<circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>"#);
icon!(LayoutListIcon, r#"<rect x="3" y="3" width="7" height="7" rx="1"/><rect x="3" y="14" width="7" height="7" rx="1"/><line x1="14" y1="4" x2="21" y2="4"/><line x1="14" y1="9" x2="21" y2="9"/><line x1="14" y1="15" x2="21" y2="15"/><line x1="14" y1="20" x2="21" y2="20"/>"#);
