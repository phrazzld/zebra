use dioxus::prelude::*;

pub mod icons {
    pub mod go_icons {
        use dioxus::prelude::*;

        // Define a simplified version of the icons we need
        #[component]
        pub fn GoCheck() -> Element {
            rsx! { div { "✓" } }
        }

        #[component]
        pub fn GoCopy() -> Element {
            rsx! { div { "⎘" } }
        }

        #[component]
        pub fn GoPlusCircle() -> Element {
            rsx! { div { "⊕" } }
        }

        #[component]
        pub fn GoSearch() -> Element {
            rsx! { div { "🔍" } }
        }

        #[component]
        pub fn GoShieldCheck() -> Element {
            rsx! { div { "🛡️✓" } }
        }

        #[component]
        pub fn GoShieldLock() -> Element {
            rsx! { div { "🛡️🔒" } }
        }

        #[component]
        pub fn GoTrash() -> Element {
            rsx! { div { "🗑️" } }
        }

        #[component]
        pub fn GoUnverified() -> Element {
            rsx! { div { "❌" } }
        }

        #[component]
        pub fn GoVerified() -> Element {
            rsx! { div { "✅" } }
        }
    }
}

// Simplified Icon component
#[component]
pub fn Icon(
    class: Option<String>,
    onclick: Option<EventHandler<MouseEvent>>,
    width: Option<u32>,
    height: Option<u32>,
    fill: Option<String>,
    icon: Option<Element>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: class.clone().unwrap_or_default(),
            onclick: move |e| {
                if let Some(onclick) = &onclick {
                    onclick.call(e);
                }
            },
            style: {
                let mut style = String::new();
                if let Some(fill) = &fill {
                    style.push_str(&format!("color: {};", fill));
                }
                if let Some(width) = width {
                    style.push_str(&format!("width: {}px;", width));
                }
                if let Some(height) = height {
                    style.push_str(&format!("height: {}px;", height));
                }
                style
            },
            {icon.unwrap_or(children)}
        }
    }
}
