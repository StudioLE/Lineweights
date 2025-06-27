use crate::schema::*;
use dioxus::prelude::*;

#[allow(clippy::absolute_paths)]
#[component]
pub fn Table() -> Element {
    let state = use_context::<State>();
    rsx! {
        div { class: "columns",
            div { class: "column",
                h3 { class: "title", "Shots" }
                table { class: "table is-fullwidth has-text-right",
                    thead {
                        tr {
                            th { "Date" }
                            th { "Time" }
                            th { "Dose" }
                        }
                    }
                    tbody {
                        for shot in state.shots.read().iter() {
                            tr {
                                td { "{shot.date}" }
                                td { "{shot.time}" }
                                td { "{shot.dose:.2}" }
                            }
                        }
                    }
                }
            }
            div { class: "column",
                h3 { class: "title", "Weights" }
                table { class: "table is-fullwidth has-text-right",
                    thead {
                        tr {
                            th { "Date" }
                            th { "Weight" }
                        }
                    }
                    tbody {
                        for weight in state.weights.read().iter() {
                            tr {
                                td { "{weight.date}" }
                                td { "{weight.weight:.2}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
