use crate::prelude::*;

#[component]
pub(crate) fn Settings() -> Element {
    let mut state: NavigationState = use_context();
    rsx! {
        section { class: "section is-medium",
            aside { class: "menu",
                p { class: "menu-label", "Personal" }
                ul { class: "menu-list",
                    li {
                        a {
                            onclick: move |_| state.set(Navigation::Goals),
                            span { class: "icon has-text-grey-dark",
                                i { class: "fa-solid fa-ruler-vertical" }
                            }
                            span { "Set Height, BMI and Goals" }
                        }
                    }
                }
                p { class: "menu-label", "Data" }
                ul { class: "menu-list",
                    li {
                        a {
                            onclick: move |_| state.set(Navigation::Import),
                            span { class: "icon has-text-grey-dark",
                                i { class: "fa-solid fa-upload" }
                            }
                            span { "Import Data from File" }
                        }
                    }
                    li {
                        a {
                            onclick: move |_| state.set(Navigation::Export),
                            span { class: "icon has-text-grey-dark",
                                i { class: "fa-solid fa-download" }
                            }
                            span { "Export Data to File" }
                        }
                    }
                }
            }
        }
    }
}
