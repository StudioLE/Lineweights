use std::ops::Deref;
use super::*;
use dioxus::prelude::*;
use crate::schema::{Page, State};

/// App is the main component of our app. Components are the building blocks of dioxus apps.
/// Each component is a function that takes some props and returns an Element. In this case,
/// App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages,
/// and autocomplete
#[component]
pub(crate) fn App() -> Element {
    use_context_provider(|| State::new());
    let mut state = use_context::<State>();
    let page = state.page.read();
    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        document::Link { rel: "stylesheet", href: asset!("/node_modules/bulma/css/bulma.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/app.css") }
        div { class: "tabs is-centered",
            ul {
                li {
                    class: if *page == Page::Import { "is-active" },
                    a { onclick: move |_| state.page.set(Page::Import), "Import" }
                }
                li {
                    class: if *page == Page::Chart { "is-active" },
                    a { onclick: move |_| state.page.set(Page::Chart), "Chart" }
                }
                li {
                    class: if *page == Page::Table { "is-active" },
                    a { onclick: move |_| state.page.set(Page::Table), "Table" }
                }
            }
        }
        div { class: "container is-max-tablet",
            section { class: "section is-large",
                if *page == Page::Import {
                    Import {}
                }
                if *page == Page::Chart {
                    Table {}
                }
                if *page == Page::Table {
                    Table {}
                }
            }
        }
    }
}
