use dioxus::prelude::*;

#[component]
pub(crate) fn Coming() -> Element {
    rsx! {
        super::Header{}

        div {
            class: "relative rounded-3xl bg-white shadow-xl ring-1 ring-gray-900/5",
            style: "margin: 2%; padding: 5%;",
            h1 { "Something wonderful is coming" }
        }

        super::Footer{}
    }
}
