use crate::prelude::*;

/// App is the main component of our app. Components are the building blocks of dioxus apps.
/// Each component is a function that takes some props and returns an Element. In this case,
/// App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages,
/// and autocomplete
#[component]
pub(super) fn App() -> Element {
    use_context_provider(State::from_local_storage);
    let state = use_context::<State>();
    let page = state.page.read();
    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        document::Link { rel: "stylesheet", href: asset!("/node_modules/bulma/css/bulma.css") }
        document::Link { rel: "stylesheet", href: asset!("/node_modules/@fortawesome/fontawesome-free/css/fontawesome.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/fonts.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/app.css") }
        FloatingActions {}
        div { class: "container is-max-tablet",
            if *page == Navigation::Settings {
                Settings {}
            }
            else if *page == Navigation::Import {
                Import {}
            }
            else if *page == Navigation::Chart {
                Chart {}
            }
            else if *page == Navigation::Table {
                Table {}
            }
            else {
                "Not Implemented"
            }
        }
    }
}
