use dioxus::prelude::*;

#[component]
pub fn Cv() -> Element {
    rsx! {
        div { class: "min-h-screen bg-surface-base text-ink-primary font-mono",
            section { class: "py-12 max-w-[960px] mx-auto px-6",
                div { class: "flex items-baseline justify-between gap-3 mb-12",
                    h2 { class: "font-mono text-[11px] font-medium text-ink-disabled uppercase tracking-wider", "Curriculum Vitae" }
                    a {
                        href: "#",
                        class: "inline-flex items-center gap-2 px-4 py-1.5 font-mono text-xs font-medium rounded-md bg-transparent border border-border text-ink-primary hover:border-accent hover:text-accent hover:bg-accent/10 transition-all duration-150 no-underline",
                        "Download PDF ↓"
                    }
                }
                div { class: "bg-surface-raised border border-border rounded-md p-6 space-y-8",
                    div {
                        h1 { class: "text-xl font-bold text-ink-primary mb-1", "Jefriansyah Marpaung" }
                        p { class: "text-xs text-accent mb-2", "Full-Stack AI Developer" }
                        p { class: "text-xs text-ink-secondary leading-relaxed",
                            "Full-stack engineer specializing in React, Go, Laravel, C# WPF, and AI integrations."
                        }
                    }
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
                }
            }
        }
    }
}
