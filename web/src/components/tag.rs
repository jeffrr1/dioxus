use dioxus::prelude::*;

#[component]
pub fn Tag(label: String, accent: Option<bool>) -> Element {
    let is_accent = accent.unwrap_or(false);
    let color_class = if is_accent { "text-accent" } else { "text-ink-secondary" };

    rsx! {
        span {
            class: "bg-surface-overlay {color_class} rounded-sm text-xs font-mono px-2 py-0.5 inline-block focus-visible:ring-2 focus-visible:ring-accent focus-visible:outline-none",
            "{label}"
        }
    }
}
