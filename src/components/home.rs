use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx!(p {
        "This is profile home page"
    }))
}