use dioxus::prelude::*;
use crate::routes::Route;
use crate::components::Tag;
use crate::data::load_projects;

/// Project Detail page — renders at /projects/:slug
/// - Looks up slug from compile-time loaded projects.json (AD-2)
/// - Redirects to /404 when slug not found
/// - Single-column layout, max-width 960px (UX-DR9)
#[component]
pub fn ProjectDetail(slug: String) -> Element {
    // Memoize dataset loading to avoid JSON parsing on every render
    let projects = use_hook(load_projects);
    let project = projects.iter().find(|p| p.slug == slug);

    match project {
        None => {
            // Redirect to 404 for unknown slugs using replace to prevent back-button loop (AD-3)
            let nav = navigator();
            use_effect(move || {
                nav.replace(Route::NotFound {});
            });
            // Render nothing while the effect fires
            rsx! { div {} }
        }

        Some(p) => {
            let format_url = |url: &str| -> String {
                if url.starts_with("http://") || url.starts_with("https://") {
                    url.to_string()
                } else {
                    format!("https://{}", url)
                }
            };

            rsx! {
                div { class: "min-h-screen bg-surface-base text-ink-primary font-mono",

                    section { class: "py-12 max-w-[960px] mx-auto px-6",

                        // ← Projects back link (inline-flex, accessible 44px touch target)
                        Link {
                            to: Route::Projects {},
                            class: "inline-flex items-center gap-1 text-accent text-xs font-mono hover:text-accent-hover transition-colors duration-150 mb-8 py-2.5 px-1 min-h-[44px] focus-visible:ring-2 focus-visible:ring-accent focus-visible:outline-none rounded-sm",
                            "\u{2190} Projects"
                        }

                        // Project title — single h1 per page (SEO + a11y)
                        h1 { class: "font-mono text-2xl font-semibold text-ink-primary mb-2",
                            "{p.title}"
                        }

                        // Role — rendered only when non-empty
                        if !p.role.is_empty() {
                            p { class: "text-xs text-ink-secondary font-mono mb-4",
                                "{p.role}"
                            }
                        }

                        // Tech stack tags — key specified for RSX iterator
                        if !p.tech_stack.is_empty() {
                            div { class: "flex flex-wrap gap-1 mb-8",
                                {p.tech_stack.iter().map(|tech| rsx! {
                                    Tag { key: "{tech}", label: tech.clone() }
                                })}
                            }
                        }

                        // Screenshots — rendered only when non-empty, filtered for valid src
                        if !p.screenshots.is_empty() {
                            div { class: "flex flex-col gap-3 mb-8",
                                {p.screenshots.iter().filter(|src| !src.is_empty()).map(|src| rsx! {
                                    div { key: "{src}", class: "bg-surface-raised rounded-md overflow-hidden",
                                        img {
                                            src: "{src}",
                                            alt: "{p.title} screenshot",
                                            loading: "lazy",
                                            width: "960",
                                            height: "540",
                                            class: "w-full object-cover",
                                        }
                                    }
                                })}
                            }
                        }

                        // Long description — rendered only when non-empty
                        if !p.long_description.is_empty() {
                            p { class: "text-xs text-ink-secondary leading-relaxed mb-8",
                                "{p.long_description}"
                            }
                        }

                        // External links — wrapper rendered only when at least one link exists (NFR-5)
                        if !p.live_demo.is_empty() || !p.source_code.is_empty() {
                            div { class: "flex flex-wrap gap-4",

                                if !p.live_demo.is_empty() {
                                    a {
                                        href: "{format_url(&p.live_demo)}",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        class: "text-accent text-xs font-mono hover:text-accent-hover transition-colors duration-150 focus-visible:ring-2 focus-visible:ring-accent focus-visible:outline-none rounded-sm",
                                        "Live Demo \u{2192}"
                                    }
                                }

                                if !p.source_code.is_empty() {
                                    a {
                                        href: "{format_url(&p.source_code)}",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        class: "text-accent text-xs font-mono hover:text-accent-hover transition-colors duration-150 focus-visible:ring-2 focus-visible:ring-accent focus-visible:outline-none rounded-sm",
                                        "Source Code \u{2192}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
