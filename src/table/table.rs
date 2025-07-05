use crate::prelude::*;

#[allow(clippy::absolute_paths)]
#[component]
pub fn Table() -> Element {
    let state = use_context::<State>();
    let collection = state.entries.read().to_owned();
    let entries = collection.entries.into_values();
    rsx! {
        table { class: "table is-fullwidth",
            thead {
                tr {
                    th { "Date" }
                    th { "Weight" }
                    th { "Shot" }
                }
            }
            tbody {
                for entry in entries {
                    tr {
                        td { "{entry.date}" }
                        td { class: "has-text-right",
                            if let Some(weight) = &entry.weight {
                                "{weight:.2}"
                            } else {
                                ""
                            }
                        }
                        td {
                            if let Some(shot) = &entry.shot {
                                "{shot.dose:.2} mg at {shot.time}"
                            } else {
                                ""
                            }
                        }
                    }
                }
            }
        }
    }
}
