use crate::prelude::*;

#[component]
pub(crate) fn FloatingActions() -> Element {
    let mut state = use_context::<State>();
    let page = state.page.read();
    rsx! {
        div { class: "fullscreen",
            div { class: "buttons",
                button { class: "button is-primary",
                    disabled: *page == Navigation::Settings,
                    onclick: move |_| state.page.set(Navigation::Settings),
                    span {
                        class: "icon",
                        i {
                            class: "fa-solid fa-gear"
                        }
                    }
                }
                button { class: "button is-primary",
                    disabled: *page == Navigation::Chart,
                    onclick: move |_| state.page.set(Navigation::Chart),
                    span {
                        class: "icon",
                        i {
                            class: "fa-solid fa-chart-line"
                        }
                    }
                }
                button { class: "button is-primary",
                    disabled: *page == Navigation::Table,
                    onclick: move |_| state.page.set(Navigation::Table),
                    span {
                        class: "icon",
                        i {
                            class: "fa-solid fa-table"
                        }
                    }
                }
                button { class: "button is-primary is-large",
                    disabled: *page == Navigation::Add,
                    onclick: move |_| state.page.set(Navigation::Add),
                    span {
                        class: "icon",
                        i {
                            class: "fa-solid fa-plus fa-lg"
                        }
                    }
                }
            }
        }
    }
}
