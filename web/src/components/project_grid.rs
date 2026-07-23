// src/components/project_grid.rs
//
// ProjectGrid — renders a list of projects as a responsive grid.
// Data is loaded by the parent view via `data::load_projects()` and
// passed in as a prop, keeping this component free of any data-fetching
// logic (per AD-1 and AD-2 of the architecture spine).
use dioxus::prelude::*;
use crate::components::ProjectCard;
use crate::data::Project;

#[component]
pub fn ProjectGrid(projects: Vec<Project>) -> Element {
    rsx! {
        div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
            {projects.iter().map(|p| rsx! {
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
}
