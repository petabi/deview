use dioxus::prelude::*;

pub(crate) fn Footer() -> Element {
    rsx! {
        footer { class: "bg-black",
            div { class: "container mx-auto py-4",
                p { class: "text-gray-400 text-sm text-center", "© 2024 Petabi, Inc." }
            }
        }
    }
}