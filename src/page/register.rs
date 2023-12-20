#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Register(cx: Scope) -> Element {
    render! {
        form { class: "flex flex-col gap-5", prevent_default: "onsubnit", onsubmit: move |_| {},
            button {
                class: "px-4 py-2 rounded font-semibold text-sm bg-slate-600 shadow-sm text-yellow-400",
                r#type: "submit",
                "Signup"
            }
        }
    }
}

pub fn Home(cx: Scope) -> Element {
    render! { h1 { "Hello from homepage" } }
}
