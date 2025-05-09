// Minimal implementation of dioxus-free-icons that works with the existing code
#![allow(non_snake_case)]
use dioxus::prelude::*;

// Icons module that matches the dioxus-free-icons API
pub mod icons {
    pub mod go_icons {
        use dioxus::prelude::*;

        // Simple icon functions that return Element directly
        pub fn GoCheck() -> Element {
            rsx! { span { style: "color: inherit", "✓" } }
        }

        pub fn GoCopy() -> Element {
            rsx! { span { style: "color: inherit", "⎘" } }
        }

        pub fn GoPlusCircle() -> Element {
            rsx! { span { style: "color: inherit", "⊕" } }
        }

        pub fn GoSearch() -> Element {
            rsx! { span { style: "color: inherit", "🔍" } }
        }

        pub fn GoShieldCheck() -> Element {
            rsx! { span { style: "color: inherit", "🛡️✓" } }
        }

        pub fn GoShieldLock() -> Element {
            rsx! { span { style: "color: inherit", "🛡️🔒" } }
        }

        pub fn GoTrash() -> Element {
            rsx! { span { style: "color: inherit", "🗑️" } }
        }

        pub fn GoUnverified() -> Element {
            rsx! { span { style: "color: inherit", "❌" } }
        }

        pub fn GoVerified() -> Element {
            rsx! { span { style: "color: inherit", "✅" } }
        }
    }
}

// Icon component that matches the dioxus-free-icons API
#[derive(Props, Clone, PartialEq)]
pub struct Icon {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub fill: Option<String>,
    pub class: Option<String>,
    pub onclick: Option<EventHandler<MouseEvent>>,
    pub icon: Element,
}

// Function version of the Icon component
pub fn Icon(props: Icon) -> Element {
    let mut style = String::new();

    // Add color if present
    if let Some(fill) = props.fill {
        style.push_str(&format!("color: {};", fill));
    }

    // Add dimensions if present
    if let Some(width) = props.width {
        style.push_str(&format!("width: {}px;", width));
    }

    if let Some(height) = props.height {
        style.push_str(&format!("height: {}px;", height));
    }

    // Handle both cases - with and without onclick handler
    if let Some(onclick) = props.onclick {
        rsx! {
            div {
                class: props.class,
                onclick: move |e| onclick.call(e),
                style: style,
                {props.icon}
            }
        }
    } else {
        rsx! {
            div {
                class: props.class,
                style: style,
                {props.icon}
            }
        }
    }
}
