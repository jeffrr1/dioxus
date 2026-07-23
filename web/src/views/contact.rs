use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    rsx! {
        div { class: "min-h-screen bg-surface-base text-ink-primary font-mono",
            section { class: "py-12 max-w-[960px] mx-auto px-6",
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
