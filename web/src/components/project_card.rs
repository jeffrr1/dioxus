use dioxus::prelude::*;
use crate::routes::Route;
use crate::components::Tag;

#[component]
pub fn ProjectCard(
    title: String,
    description: String,
    slug: String,
    tech_stack: Vec<String>,
    thumbnail: String,
) -> Element {
    let mut loaded = use_signal(|| false);

    rsx! {
        Link {
            to: Route::ProjectDetail { slug: slug.clone() },
            // `group` enables hover-driven child reveals via Tailwind group-hover utilities
            class: "group block bg-surface-raised border border-border rounded-md p-4 transition-all duration-150 hover:border-accent hover:shadow-[0_0_24px_rgba(0,212,255,0.12)] focus-visible:ring-2 focus-visible:ring-accent focus-visible:outline-none no-underline cursor-pointer",

            // Thumbnail area — always rendered; image only shown after load
            div { class: "relative w-full aspect-video bg-surface-overlay rounded-sm mb-3 overflow-hidden",

                // Actual image — hidden until onload fires (patch: single merged class attr)
                if !thumbnail.is_empty() {
                    img {
                        src: "{thumbnail}",
                        alt: "{title} screenshot",
                        loading: "lazy",
                        width: "480",
                        height: "270",
                        // Single class expression avoids the dual-class override bug
                        class: if *loaded.read() { "block w-full h-full object-cover" } else { "hidden w-full h-full object-cover" },
                        onload: move |_| *loaded.write() = true,
                    }
                }

                // Fallback placeholder — shown when thumbnail is empty OR image hasn't loaded yet
                if thumbnail.is_empty() || !*loaded.read() {
                    div { class: "absolute inset-0 flex items-center justify-center text-ink-disabled text-[10px] font-mono select-none",
                        "No preview"
                    }
                }

                // Hover overlay — "View Details" button revealed on parent hover (AC-3)
                div { class: "absolute inset-0 flex items-end justify-center pb-3 opacity-0 group-hover:opacity-100 transition-opacity duration-150 pointer-events-none",
                    span { class: "px-3 py-1.5 bg-accent text-surface-base font-mono text-[10px] font-semibold rounded-sm",
                        "View Details \u{2192}"
                    }
                }
            }

            h3 { class: "font-mono text-xs font-semibold mb-2 text-ink-primary",
                "{title}"
            }
            p { class: "text-xs text-ink-secondary leading-relaxed mb-3",
                "{description}"
            }
            div { class: "flex flex-wrap gap-1",
                {tech_stack.iter().map(|tech| rsx! {
                    Tag { label: tech.clone() }
                })}
            }
        }
    }
}
