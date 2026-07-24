use crate::data::load_experience;
use dioxus::prelude::*;

#[component]
pub fn Cv() -> Element {
    let experiences = load_experience();

    rsx! {
        div { class: "min-h-screen bg-surface-base text-ink-primary font-mono",
            section { class: "py-12 max-w-[960px] mx-auto px-6",
                div { class: "flex items-baseline justify-between gap-3 mb-12",
                    h2 { class: "font-mono text-[11px] font-medium text-ink-disabled uppercase tracking-wider", "Curriculum Vitae" }
                    a {
                        href: "#",
                        class: "inline-flex items-center gap-2 px-4 py-2.5 font-mono text-xs font-medium rounded-md bg-transparent border border-accent text-ink-primary hover:border-accent hover:text-accent hover:bg-accent/10 transition-all duration-150 no-underline min-h-[44px]",
                        "Download PDF ↓"
                    }
                }
                div { class: "bg-surface-raised border border-border rounded-md p-6 space-y-8",
                    // Summary section
                    div {
                        h1 { class: "text-xl font-bold text-ink-primary mb-1", "Jefriansyah Marpaung" }
                        p { class: "text-xs text-accent mb-2", "Full-Stack AI Developer" }
                        p { class: "text-xs text-ink-secondary leading-relaxed",
                            "Full-stack engineer specializing in React, Go, Laravel, C# WPF, and AI integrations."
                        }
                    }
                    // Skills section
                    div { class: "border-t border-border pt-6",
                        h3 { class: "text-xs font-semibold text-ink-disabled uppercase tracking-wider mb-4", "Skills & Technologies" }
                        div { class: "grid grid-cols-2 md:grid-cols-4 gap-4 text-xs text-ink-secondary",
                            div {
                                span { class: "text-ink-primary font-medium block mb-1", "Frontend" }
                                "React, TypeScript, Tailwind CSS, Dioxus (Rust)"
                            }
                            div {
                                span { class: "text-ink-primary font-medium block mb-1", "Backend" }
                                "Go, Laravel (PHP), Node.js, REST APIs"
                            }
                            div {
                                span { class: "text-ink-primary font-medium block mb-1", "Desktop & DB" }
                                "C# WPF, PostgreSQL, MySQL"
                            }
                            div {
                                span { class: "text-ink-primary font-medium block mb-1", "AI & Tools" }
                                "Gemini API, Git, Docker"
                            }
                        }
                    }
                    // Experience section (data-driven from Story 3.1)
                    div { class: "border-t border-border pt-6",
                        h3 { class: "text-xs font-semibold text-ink-disabled uppercase tracking-wider mb-4", "Experience" }
                        div { class: "space-y-6",
                            for exp in experiences {
                                div {
                                    div { class: "flex justify-between items-baseline mb-1",
                                        h4 { class: "text-xs font-semibold text-ink-primary", "{exp.company}" }
                                        span { class: "text-xs text-ink-secondary", "{exp.date_range}" }
                                    }
                                    p { class: "text-xs text-ink-secondary mb-2", "{exp.role} · {exp.location}" }
                                    ul { class: "pl-4 list-disc text-xs text-ink-secondary space-y-1 leading-relaxed",
                                        {exp.achievements.iter().map(|achievement| rsx! {
                                            li { "{achievement}" }
                                        })}
                                    }
                                }
                            }
                        }
                    }
                    // Education section (hardcoded)
                    div { class: "border-t border-border pt-6",
                        h3 { class: "text-xs font-semibold text-ink-disabled uppercase tracking-wider mb-4", "Education" }
                        div { class: "space-y-6",
                            div {
                                div { class: "flex justify-between items-baseline mb-1",
                                    h4 { class: "text-xs font-semibold text-ink-primary", "B.Sc Computer Science" }
                                    span { class: "text-xs text-ink-secondary", "2018 — 2022" }
                                }
                                p { class: "text-xs text-ink-secondary", "University of Indonesia" }
                                p { class: "text-xs text-ink-secondary", "GPA: 3.7/4.0" }
                            }
                            div {
                                div { class: "flex justify-between items-baseline mb-1",
                                    h4 { class: "text-xs font-semibold text-ink-primary", "High School Diploma" }
                                    span { class: "text-xs text-ink-secondary", "2015 — 2018" }
                                }
                                p { class: "text-xs text-ink-secondary", "SMA Negeri 1 Jakarta" }
                            }
                        }
                    }
                }
            }
        }
    }
}
