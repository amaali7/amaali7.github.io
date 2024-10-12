#![allow(non_snake_case)]

pub mod components;
pub mod routes;
pub mod utils;

use crate::routes::Route;
use dioxus::prelude::*;

pub static BASE_URL: &str = "https://raw.githubusercontent.com/amaali7/markdown_files/refs/heads/main/";
pub static BASE_LIST: &str = "markdown_list.json";

pub fn App() -> Element {
    rsx! {
        div { class: "bg-nord7", Router::<Route> {} }
    }
}
