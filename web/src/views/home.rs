use dioxus::prelude::*;
use crate::routes::Route;
use crate::components::{ProjectCard, ProjectGrid};
use crate::data::load_projects;

#[component]
pub fn Home() -> Element {
    let projects = load_projects();

    rsx! {
        div { class: "min-h-screen bg-surface-base text-ink-primary font-mono",
            // Hero section
            section { class: "py-24 max-w-[960px] mx-auto px-6",
                div { class: "pt-0 md:pt-6",
                    p { class: "text-ink-secondary text-xs mb-2",
                        "Full-Stack AI Developer"
                    }
                    h1 { class: "font-mono text-3xl md:text-4xl font-semibold leading-tight mb-4 text-ink-primary",
                        "Jefriansyah Marpaung"
                    }
                    p { class: "text-ink-secondary text-xs leading-relaxed mb-8",
                        "I build across the stack — React frontends, Go APIs, Laravel platforms, and C# WPF desktop applications — with a growing focus on integrating AI capabilities into real-world products. Driven by curiosity: when something needs to be built, I dive in until it's done right."
                    }
                    div { class: "flex gap-3",
                        Link {
                            to: Route::Projects {},
                            class: "inline-flex items-center gap-2 px-5 py-3.5 font-mono text-xs font-medium rounded-md border border-accent bg-transparent text-accent hover:bg-accent/10 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent transition-all duration-150 no-underline",
                            "View Projects "
                            span { "aria-hidden": "true", "\u{2192}" }
                        }
                        Link {
                            to: Route::Cv {},
                            class: "inline-flex items-center gap-2 px-5 py-3.5 font-mono text-xs font-medium rounded-md border border-accent bg-transparent text-accent hover:bg-accent/10 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent transition-all duration-150 no-underline",
                            "Download CV"
                        }
                    }
                }
            }

            // Featured Projects section
            section { class: "py-12 max-w-[960px] mx-auto px-6",
                div { class: "flex items-baseline gap-3 mb-8",
                    h2 { class: "font-mono text-[11px] font-medium text-ink-disabled uppercase tracking-wider", "Featured Projects" }
                    hr { class: "flex-1 border-none h-[1px] bg-border" }
                }
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3",
                    {projects.iter().filter(|p| p.featured).take(3).map(|p| rsx! {
                        ProjectCard {
                            title: p.title.clone(),
                            description: p.description.clone(),
                            slug: p.slug.clone(),
                            tech_stack: p.tech_stack.clone(),
                            thumbnail: p.thumbnail.clone(),
                        }
                    })}
                }
            }

            // All Projects preview section — non-featured projects from real data
            section { class: "py-12 max-w-[960px] mx-auto px-6",
                div { class: "flex items-baseline gap-3 mb-8",
                    h2 { class: "font-mono text-[11px] font-medium text-ink-disabled uppercase tracking-wider", "More Projects" }
                    hr { class: "flex-1 border-none h-[1px] bg-border" }
                }
                ProjectGrid {
                    projects: projects.iter().filter(|p| !p.featured).cloned().collect()
                }
            }

            // Experience section
            section { class: "py-12 max-w-[960px] mx-auto px-6",
                div { class: "flex items-baseline gap-3 mb-12",
                    h2 { class: "font-mono text-[11px] font-medium text-ink-disabled uppercase tracking-wider", "Experience" }
                    hr { class: "flex-1 border-none h-[1px] bg-border" }
                }
                div { class: "relative pl-6 before:absolute before:left-2 before:top-0 before:bottom-0 before:w-[1px] before:bg-border",
                    div { class: "relative pb-6 last:pb-0 before:absolute before:-left-[21px] before:top-1 before:w-[9px] before:h-[9px] before:rounded-full before:bg-accent before:border-2 before:border-surface-base",
                        div { class: "font-mono text-[10px] text-ink-disabled mb-1", "2024 — present" }
                        h3 { class: "font-mono text-xs font-semibold text-ink-primary mb-1", "Company Name" }
                        div { class: "text-xs text-ink-secondary mb-2", "Full-Stack Developer" }
                        ul { class: "pl-4 list-disc text-xs text-ink-secondary space-y-1 leading-relaxed",
                            li { "Built and maintained RESTful APIs serving 10K+ daily requests" }
                            li { "Developed internal tooling that reduced deployment time by 40%" }
                            li { "Led migration from legacy monolith to microservices architecture" }
                        }
                    }
                    div { class: "relative pb-6 last:pb-0 before:absolute before:-left-[21px] before:top-1 before:w-[9px] before:h-[9px] before:rounded-full before:bg-accent before:border-2 before:border-surface-base",
                        div { class: "font-mono text-[10px] text-ink-disabled mb-1", "2022 — 2024" }
                        h3 { class: "font-mono text-xs font-semibold text-ink-primary mb-1", "Previous Company" }
                        div { class: "text-xs text-ink-secondary mb-2", "Software Developer" }
                        ul { class: "pl-4 list-disc text-xs text-ink-secondary space-y-1 leading-relaxed",
                            li { "Designed and implemented customer-facing web portal used by 5K+ users" }
                            li { "Integrated third-party payment gateways across multiple regions" }
                            li { "Mentored 2 junior developers through onboarding and code reviews" }
                        }
                    }
                }
            }

            // Contact section
            section { class: "py-12 max-w-[960px] mx-auto px-6 mb-12",
                div { class: "flex items-baseline gap-3 mb-12",
                    h2 { class: "font-mono text-[11px] font-medium text-ink-disabled uppercase tracking-wider", "Contact" }
                    hr { class: "flex-1 border-none h-[1px] bg-border" }
                }
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                    a {
                        href: "mailto:jefri@example.com",
                        class: "bg-surface-raised border border-border rounded-md p-5 text-center transition-colors hover:border-accent block no-underline",
                        "aria-label": "Send email",
                        div { class: "font-mono text-[10px] text-ink-disabled uppercase tracking-wider mb-2", "Email" }
                        div { class: "font-mono text-xs font-medium text-ink-primary", "jefri@example.com" }
                    }
                    a {
                        href: "https://linkedin.com/in/jefri-marpaung",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "bg-surface-raised border border-border rounded-md p-5 text-center transition-colors hover:border-accent block no-underline",
                        "aria-label": "LinkedIn profile",
                        div { class: "font-mono text-[10px] text-ink-disabled uppercase tracking-wider mb-2", "LinkedIn" }
                        div { class: "font-mono text-xs font-medium text-ink-primary", "/in/jefri" }
                    }
                    a {
                        href: "https://upwork.com",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "bg-surface-raised border border-border rounded-md p-5 text-center transition-colors hover:border-accent block no-underline",
                        "aria-label": "Upwork profile",
                        div { class: "font-mono text-[10px] text-ink-disabled uppercase tracking-wider mb-2", "Upwork" }
                        div { class: "font-mono text-xs font-medium text-ink-primary", "/freelancer/jefri" }
                    }
                }
            }
        }
    }
}
