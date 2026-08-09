use std::thread::current;
use dioxus::prelude::*;

#[component]
pub fn Progress(max: Signal<i64>, current: Signal<i64>) -> Element {
    let width: Memo<f64> = use_memo(move || {
        current() as f64 * 100f64 / max() as f64
    });
    rsx! {
        div {
            class: "progress",
            role: "progressbar",
            aria_label: "Basic example",
            aria_valuenow: "{current}",
            aria_valuemin: "0",
            aria_valuemax: "{max}",
            flex_shrink: "0",
            div {
                class: "progress-bar progress-bar-striped",
                style: "width: {width}%",
                "{current}/{max}"
            }
        }
    }
}