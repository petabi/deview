use dioxus::prelude::*;

#[component]
pub(crate) fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        super::Header {}

        div {
            class: "relative rounded-3xl bg-white shadow-xl ring-1 ring-gray-900/5",
            style: "margin: 2%; padding: 5%;",
            h1 { "Page not found" }
            p { "We are terribly sorry, but the page you requested doesn't exist." }
            pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
        }

        super::Footer {}
    }
}
