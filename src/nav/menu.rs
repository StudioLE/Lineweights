use crate::prelude::*;

#[component]
pub(crate) fn Menu() -> Element {
    use_context_provider(State::new);
    let mut state = use_context::<State>();
    let page = state.page.read();
    rsx! {
        div { class: "tabs is-centered",
            ul {
                li {
                    class: if *page == Navigation::Import { "is-active" },
                    a { onclick: move |_| state.page.set(Navigation::Import), "Import" }
                }
                li {
                    class: if *page == Navigation::Chart { "is-active" },
                    a { onclick: move |_| state.page.set(Navigation::Chart), "Chart" }
                }
                li {
                    class: if *page == Navigation::Table { "is-active" },
                    a { onclick: move |_| state.page.set(Navigation::Table), "Table" }
                }
            }
        }
    }
}
