#![allow(non_snake_case)]

use crate::page::register::{Home, Register};
use dioxus::prelude::*;
use dioxus_router::prelude::*;
// use fermi::use_init_atom_root;

pub fn App(cx: Scope) -> Element {
    // use_init_atom_root(cx);
    // render! { Router::<Route> {} }
    render! { Register {} }
}
// pub fn Register(cx: Scope) -> Element {
//     render! {
//         form { class: "flex flex-col gap-5", prevent_default: "onsubnit", onsubmit: move |_| {},
//             button {
//                 class: "px-4 py-2 rounded font-semibold text-sm bg-slate-600 shadow-sm text-white",
//                 r#type: "submit",
//                 "Signup"
//             }
//         }
//     }
// }
// An enum of all of the possible routes in the app.
// #[derive(Routable, Clone, PartialEq)]
// enum Route {
//     // The home page is at the / route
//     #[route("/")]
//     Home {},
//     #[route("/account/register")]
//     Register {},
//     //  if the current location doesn't match any of the above routes, render the NotFound component
//     #[route("/:..segments")]
//     Err404 { segments: Vec<String> },
// }

#[allow(unused)]
#[component]
pub fn Err404(cx: Scope, segments: Vec<String>) -> Element {
    render! {
        section { div { class: "p-5 bg-slate-500", "Error404" } }
    }
}
