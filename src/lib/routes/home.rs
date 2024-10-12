use dioxus::prelude::*;
use crate::{components::Markdown, utils::json::get_section_markdown};

#[component]
pub fn Home() -> Element {

    let home_section = use_resource(move || get_section_markdown("home"));

    match &*home_section.read_unchecked() {
        Some(Ok(list)) => {
            rsx! {
                div {
                    for item in list.iter() {
                        Markdown { content: item }
                    }
                }
            }
        }
        Some(Err(err)) => {
            // if there was an error, render the error
            rsx! { "An error occurred while fetching home {err}" }
        }
        None => {
            // if the future is not resolved yet, render a loading message
            rsx! { "Loading items" }
        }
    }
}
