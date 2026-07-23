use crate::components::Timeline;
use crate::data::load_experience;
use dioxus::prelude::*;

#[component]
pub fn Experience() -> Element {
    let experiences = load_experience();

    rsx! {
        div { class: "min-h-screen bg-surface-base text-ink-primary font-mono",
            section { class: "py-12 max-w-[960px] mx-auto px-6",
                // Section header
                div { class: "flex items-baseline gap-3 mb-12",
                    h2 { class: "font-mono text-[11px] font-medium text-ink-disabled uppercase tracking-wider", "Experience" }
                    hr { class: "flex-1 border-none h-[1px] bg-border" }
                }

                // Empty state or timeline with vertical connecting line in `border` color
                if experiences.is_empty() {
                    div { class: "text-xs text-ink-secondary", "Experience data coming soon." }
                } else {
                    div { class: "relative pl-6 before:absolute before:left-2 before:top-0 before:bottom-0 before:w-[1px] before:bg-border",
                        // Render all entries in reverse-chronological order
                        // (data is already most-recent-first from experience.json)
                        for experience in experiences {
                            Timeline { experience }
                        }
                    }
                }
            }
        }
    }
}
