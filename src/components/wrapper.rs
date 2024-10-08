use dioxus::prelude::*;

#[component]
pub(crate) fn Wrapper() -> Element {
    rsx! {
        super::Header {}
        Outlet::<crate::Route> {}
        super::Footer {}
    }
}
