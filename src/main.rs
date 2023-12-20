#![allow(non_snake_case)]

pub mod app;
pub mod page;

fn main() {
    dioxus_web::launch(app::App);
}
