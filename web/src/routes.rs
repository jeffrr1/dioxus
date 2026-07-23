use dioxus::prelude::*;

use crate::components::NavBar;
use crate::views::{Contact, Cv, Experience, Home, NotFound, ProjectDetail, Projects};

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[route("/projects")]
        Projects {},
        #[route("/projects/:slug")]
        ProjectDetail { slug: String },
        #[route("/experience")]
        Experience {},
        #[route("/cv")]
        Cv {},
        #[route("/contact")]
        Contact {},
    // Catch-all route deferred: #[route("/:..seg")] with Vec<String>
    // causes EmptyBuilder error in Dioxus 0.7.9. Static /404 used instead.
    #[route("/404")]
    NotFound {},
}
