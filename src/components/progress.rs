use dioxus::prelude::*;

#[component]
pub fn Progress(max: Signal<i64>, current: Signal<i64>) -> Element {
    let width: Memo<f64> = use_memo(move || current() as f64 * 100f64 / max() as f64);
    rsx! {
        p {
            "{current}/{max}"
        }
        div {
            class: "progress",
            role: "progressbar",
            aria_label: "Basic example",
            aria_valuenow: "{current}",
            aria_valuemin: "0",
            aria_valuemax: "{max}",
            flex_shrink: "0",
            height: "1.5rem",
            div {
                class: "progress-bar progress-bar-striped progress-bar-animated",
                width: "{width}%",
                style: "--bs-progress-bar-transition: none;",
                "{width:.1}%"
            }
        }
    }
}
