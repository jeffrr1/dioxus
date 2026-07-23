use dioxus::prelude::*;

use crate::components::Footer;
use crate::routes::Route;

#[component]
pub fn NavBar() -> Element {
    let current_route = use_route::<Route>();
    let mut drawer_open = use_signal(|| false);
    let mut tracked_route = use_signal(|| current_route.clone());

    if *tracked_route.peek() != current_route {
        *drawer_open.write() = false;
        tracked_route.set(current_route.clone());
    }

    let show_drawer = *drawer_open.read();

    rsx! {
        div { class: "flex flex-col min-h-screen bg-surface-base text-ink-primary font-mono",
            nav { class: "fixed top-6 left-1/2 -translate-x-1/2 z-50 flex items-center gap-2 bg-surface-overlay border border-border rounded-full px-3 py-2 backdrop-blur-lg font-mono",
                Link {
                    to: Route::Home {},
                    class: "text-ink-primary text-xs font-medium px-3 py-1 rounded-full focus-visible:ring-2 focus-visible:ring-accent focus-visible:outline-none no-underline",
                    "~/jefri"
                }
                div { class: "hidden md:flex items-center gap-0.5",
                    {nav_link("/", "Home", &current_route)}
                    {nav_link("/projects", "Projects", &current_route)}
                    {nav_link("/experience", "Experience", &current_route)}
                    {nav_link("/cv", "CV", &current_route)}
                    {nav_link("/contact", "Contact", &current_route)}
                }
                button {
                    class: "block md:hidden p-2 focus-visible:ring-2 focus-visible:ring-accent focus-visible:outline-none",
                    onclick: move |_| {
                        let is_open = *drawer_open.read();
                        *drawer_open.write() = !is_open;
                    },
                    aria_label: "Toggle navigation menu",
                    div { class: "flex flex-col gap-1.5",
                        span { aria_hidden: "true", class: "block w-5 h-0.5 bg-ink-secondary transition-transform duration-300" }
                        span { aria_hidden: "true", class: "block w-5 h-0.5 bg-ink-secondary transition-opacity duration-300" }
                        span { aria_hidden: "true", class: "block w-5 h-0.5 bg-ink-secondary transition-transform duration-300" }
                    }
                }
            }
            div { class: "md:hidden",
                div {
                    class: "fixed inset-0 bg-black/50 z-40 transition-opacity duration-300",
                    class: if show_drawer { "opacity-100 pointer-events-auto" } else { "opacity-0 pointer-events-none" },
                    onclick: move |_| *drawer_open.write() = false,
                    onkeydown: move |e| {
                        if e.key() == Key::Escape {
                            *drawer_open.write() = false;
                        }
                    },
                    aria_label: "Close menu",
                }
                div {
                    class: "fixed top-0 right-0 h-full w-64 bg-surface-base z-50 transform transition-transform duration-300",
                    class: if show_drawer { "translate-x-0" } else { "translate-x-full" },
                    div { class: "flex flex-col gap-2 pt-16 px-6",
                        a {
                            href: "#",
                            class: "sr-only focus:not-sr-only focus:absolute focus:top-4 focus:left-4 focus:text-accent",
                            autofocus: show_drawer,
                            onkeydown: move |e| {
                                if e.key() == Key::Escape {
                                    *drawer_open.write() = false;
                                }
                            },
                            "Skip drawer"
                        }
                        {nav_link_drawer("/", "Home", &current_route, drawer_open)}
                        {nav_link_drawer("/projects", "Projects", &current_route, drawer_open)}
                        {nav_link_drawer("/experience", "Experience", &current_route, drawer_open)}
                        {nav_link_drawer("/cv", "CV", &current_route, drawer_open)}
                        {nav_link_drawer("/contact", "Contact", &current_route, drawer_open)}
                    }
                }
            }
            main { class: "flex-1 pt-24 bg-surface-base text-ink-primary font-mono",
                Outlet::<Route> {}
            }
            Footer {}
        }
    }
}

fn is_active_route(path: &str, current: &Route) -> bool {
    match (path, current) {
        ("/", Route::Home {}) => true,
        ("/projects", Route::Projects {}) => true,
        ("/projects", Route::ProjectDetail { .. }) => true,
        ("/experience", Route::Experience {}) => true,
        ("/cv", Route::Cv {}) => true,
        ("/contact", Route::Contact {}) => true,
        _ => false,
    }
}

fn nav_link(path: &str, label: &str, current: &Route) -> Element {
    let active = is_active_route(path, current);
    let class = if active {
        "text-accent bg-accent/10 pointer-events-none"
    } else {
        "text-ink-secondary hover:text-ink-primary hover:bg-surface-overlay transition-colors"
    };
    let route = match path {
        "/" => Route::Home {},
        "/projects" => Route::Projects {},
        "/experience" => Route::Experience {},
        "/cv" => Route::Cv {},
        "/contact" => Route::Contact {},
        _ => return rsx! {},
    };

    rsx! {
        Link {
            to: route,
            class: "{class} text-xs px-3 py-1.5 rounded-full focus-visible:ring-2 focus-visible:ring-accent focus-visible:outline-none",
            "{label}"
        }
    }
}

fn nav_link_drawer(path: &str, label: &str, current: &Route, mut drawer: Signal<bool>) -> Element {
    let active = is_active_route(path, current);
    let class = if active {
        "text-accent bg-accent/10 pointer-events-none block"
    } else {
        "text-ink-secondary hover:text-ink-primary hover:bg-surface-overlay transition-colors block"
    };
    let route = match path {
        "/" => Route::Home {},
        "/projects" => Route::Projects {},
        "/experience" => Route::Experience {},
        "/cv" => Route::Cv {},
        "/contact" => Route::Contact {},
        _ => return rsx! {},
    };

    rsx! {
        Link {
            to: route,
            class: "{class} rounded-full px-4 py-2 text-base focus-visible:ring-2 focus-visible:ring-accent focus-visible:outline-none",
            onclick: move |_| *drawer.write() = false,
            "{label}"
        }
    }
}
