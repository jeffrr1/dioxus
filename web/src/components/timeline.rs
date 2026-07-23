use crate::data::Experience;
use dioxus::prelude::*;

/// Timeline component renders a single experience entry as a `surface-raised` card
/// with `rounded/md` corners. Displays company name (heading weight), role title,
/// date range and location in `ink-secondary`, and 2-3 achievement bullet points.
#[component]
pub fn Timeline(experience: Experience) -> Element {
    rsx! {
        div { class: "relative pb-6 last:pb-0",
            // Dot indicator on the vertical connecting line
            div { class: "absolute -left-[21px] top-1 w-[9px] h-[9px] rounded-full bg-accent border-2 border-surface-base", }
            // Date marker — left-aligned, ink-secondary
            div { class: "font-mono text-[10px] text-ink-secondary mb-1", "{experience.date_range}" }
            // Content card — surface-raised with rounded/md
            div { class: "bg-surface-raised rounded-md p-4",
                // Company name — heading weight
                h3 { class: "font-mono text-xs font-semibold text-ink-primary mb-1", "{experience.company}" }
                // Role title, date range, and location — ink-secondary
                div { class: "text-xs text-ink-secondary mb-2", "{experience.role} · {experience.location}" }
                // Achievement bullet points
                ul { class: "pl-4 list-disc text-xs text-ink-secondary space-y-1 leading-relaxed",
                    {experience.achievements.iter().map(|achievement| rsx! {
                        li { "{achievement}" }
                    })}
                }
            }
        }
    }
}
