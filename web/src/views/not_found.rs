use dioxus::prelude::*;

use crate::components::Footer;
use crate::routes::Route;

#[component]
pub fn NotFound() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen",
            main { class: "flex-1 bg-surface-base text-ink-primary font-mono p-6 flex flex-col items-center justify-center",
                h1 { class: "text-6xl font-semibold text-ink-primary", "404" }
                p { class: "text-ink-secondary mt-4", "Page not found." }
                Link { class: "text-accent hover:text-accent-hover mt-4 underline focus-visible:ring-2 focus-visible:ring-accent focus-visible:outline-none", to: Route::Home {}, "cd ~" }
            }
            Footer {}
        }
    }
}
