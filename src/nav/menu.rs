use crate::prelude::*;

#[component]
pub(crate) fn Menu() -> Element {
    let mut state = use_context::<State>();
    let page = state.page.read();
    rsx! {
        div { class: "fullscreen",
            div { class: "buttons",
                if *page != Navigation::Import {
                    button { class: "button is-primary",
                        onclick: move |_| state.page.set(Navigation::Import),
                        span {
                            class: "icon",
                            i {
                                class: "fa-solid fa-gear"
                            }
                        }
                    }
                }
                if *page != Navigation::Table {
                    button { class: "button is-primary",
                        onclick: move |_| state.page.set(Navigation::Table),
                        span {
                            class: "icon",
                            i {
                                class: "fa-solid fa-table"
                            }
                        }
                    }
                }
                if *page != Navigation::Chart {
                    button { class: "button is-primary",
                        onclick: move |_| state.page.set(Navigation::Chart),
                        span {
                            class: "icon",
                            i {
                                class: "fa-solid fa-chart-line"
                            }
                        }
                    }
                }
                button { class: "button is-primary is-large",
                    onclick: move |_| state.page.set(Navigation::Import),
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
