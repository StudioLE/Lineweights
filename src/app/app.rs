use crate::prelude::*;

/// App is the main component of our app. Components are the building blocks of dioxus apps.
/// Each component is a function that takes some props and returns an Element. In this case,
/// App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages,
/// and autocomplete
#[component]
pub(crate) fn App() -> Element {
    use_context_provider(State::new);
    let state = use_context::<State>();
    let page = state.page.read();
    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        document::Link { rel: "stylesheet", href: asset!("/node_modules/bulma/css/bulma.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/app.css") }
        document::Link { rel: "stylesheet", href: asset!("/node_modules/@fontsource/source-sans-pro/400.css") }
        Menu {}
        div { class: "container is-max-tablet",
            if *page == Navigation::Import {
                Import {}
            }
            if *page == Navigation::Chart {
                Chart {}
            }
            if *page == Navigation::Table {
                Table {}
            }
        }
    }
}
