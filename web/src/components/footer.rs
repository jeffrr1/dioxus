use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "border-t border-border mt-auto bg-surface-base text-ink-primary font-mono",
            div { class: "max-w-5xl mx-auto px-4 md:px-6 py-8 md:py-9",
                div { class: "flex flex-col md:flex-row justify-between items-center gap-4",
                    span { class: "text-ink-disabled text-xs font-mono", "© 2026 Jefriansyah Marpaung" }
                    div { class: "flex items-center gap-6 md:gap-8",
                        a {
                            href: "https://github.com/jeffrr1",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "text-ink-secondary hover:text-accent transition-colors text-xs font-mono focus-visible:ring-2 focus-visible:ring-accent focus-visible:outline-none",
                            "GitHub"
                        }
                        a {
                            href: "https://linkedin.com/in/jefri-marpaung",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "text-ink-secondary hover:text-accent transition-colors text-xs font-mono focus-visible:ring-2 focus-visible:ring-accent focus-visible:outline-none",
                            "LinkedIn"
                        }
                        a {
                            href: "mailto:jefri@example.com",
                            class: "text-ink-secondary hover:text-accent transition-colors text-xs font-mono focus-visible:ring-2 focus-visible:ring-accent focus-visible:outline-none",
                            "Email"
                        }
                    }
                }
            }
        }
    }
}
