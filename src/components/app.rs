use super::*;
use dioxus::prelude::*;
use crate::schema::State;

/// App is the main component of our app. Components are the building blocks of dioxus apps.
/// Each component is a function that takes some props and returns an Element. In this case,
/// App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages,
/// and autocomplete
#[component]
pub(crate) fn App() -> Element {
    use_context_provider(|| State::new());
    let state = use_context::<State>();
    let is_shots_empty = state.shots.read().iter().count() == 0;
    let is_weights_empty = state.weights.read().iter().count() == 0;
    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        document::Link { rel: "stylesheet", href: asset!("/node_modules/bulma/css/bulma.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/app.css") }
        div { class: "container is-max-tablet",
            section { class: "section is-large",
                if is_shots_empty && is_weights_empty {                    
                    Import {}
                } else {
                    Table {}                    
                }
            }
        }
    }
}
