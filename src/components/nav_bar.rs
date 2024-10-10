#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::Route;

#[component]
pub(crate) fn NavBar() -> Element {
    rsx! {
        nav { style: "flex-grow: 1; border-right: 0.5mm solid rgba(0, 0, 0, 0.5);",
            ul {
                li { style: "font: small-caps bold 24px sans-serif;",
                    Link { to: Route::Home {}, "Deview" }
                }
            }
        }
    }
}
