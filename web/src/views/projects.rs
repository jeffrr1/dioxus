use dioxus::prelude::*;
use crate::components::ProjectGrid;
use crate::data::load_projects;

#[component]
pub fn Projects() -> Element {
    let projects = load_projects();

    rsx! {
        div { class: "min-h-screen bg-surface-base text-ink-primary font-mono",
            section { class: "py-12 max-w-[960px] mx-auto px-6",
                div { class: "flex items-baseline gap-3 mb-12",
                    h1 { class: "font-mono text-[11px] font-medium text-ink-disabled uppercase tracking-wider", "All Projects" }
                    hr { class: "flex-1 border-none h-[1px] bg-border" }
                }
                ProjectGrid { projects }
            }
        }
    }
}
