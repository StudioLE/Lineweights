use crate::prelude::*;

#[component]
pub(crate) fn FloatingActions() -> Element {
    let mut state: NavigationState = use_context();
    rsx! {
        div { class: "fullscreen",
            div { class: "buttons",
                button { class: "button is-primary",
                    disabled: state.is_active(Navigation::Settings),
                    onclick: move |_| state.set(Navigation::Settings),
                    span {
                        class: "icon",
                        i {
                            class: "fa-solid fa-gear"
                        }
                    }
                }
                button { class: "button is-primary",
                    disabled: state.is_active(Navigation::Chart),
                    onclick: move |_| state.set(Navigation::Chart),
                    span {
                        class: "icon",
                        i {
                            class: "fa-solid fa-chart-line"
                        }
                    }
                }
                button { class: "button is-primary",
                    disabled: state.is_active(Navigation::Table),
                    onclick: move |_| state.set(Navigation::Table),
                    span {
                        class: "icon",
                        i {
                            class: "fa-solid fa-table"
                        }
                    }
                }
                button { class: "button is-primary is-large",
                    disabled: state.is_active(Navigation::Add),
                    onclick: move |_| state.set(Navigation::Add),
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
