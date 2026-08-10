use crate::Route;
use crate::components::pieces::Piece;
use crate::database::entity::prelude::*;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;

#[component]
pub fn Markdown(md: String) -> Element {
    let parser = pulldown_cmark::Parser::new(&*md);

    // Write to a new String buffer.
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    rsx! {
        div {
            dangerous_inner_html: html_output
        }
    }
}
