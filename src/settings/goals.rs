use crate::prelude::*;

#[component]
pub(crate) fn Goals() -> Element {
    let state = use_context::<State>();

    // Calculate BMI values
    let mut recommended = None;
    let mut healthy = None;
    let mut overwight = None;
    if let Some(height) = *state.height.read() {
        recommended = Some(BodyMassIndex::weight_from_height_bmi(
            height,
            BodyMassIndex::CENTER,
        ));
        healthy = Some(BodyMassIndex::weight_from_height_bmi(
            height,
            BodyMassIndex::HEALTHY,
        ));
        overwight = Some(BodyMassIndex::weight_from_height_bmi(
            height,
            BodyMassIndex::OVERWEIGHT,
        ));
    }

    rsx! {
        section { class: "section is-medium",
            section { class: "section",
                h1 { class: "subtitle", "Set" }
                Height {}
            }
            section { class: "section",
                h1 { class: "subtitle", "Body Mass Index" }
                div { class: "columns",
                    div { class: "column",
                        div { class: "field",
                            label { class: "label", "Recommended" }
                            div { class: "field has-addons",
                                p { class: "control",
                                    input {
                                        class: "input",
                                        disabled: true,
                                        r#type: "text",
                                        value: display_weight(recommended)
                                    }
                                }
                                p { class: "control",
                                    a { class: "button is-static", "kg" }
                                }
                            }
                        }

                    }
                    div { class: "column",
                        div { class: "field",
                            label { class: "label", "Healthy" }
                            div { class: "field has-addons",
                                p { class: "control",
                                    input {
                                        class: "input",
                                        disabled: true,
                                        r#type: "text",
                                        value: display_weight(healthy)
                                    }
                                }
                                p { class: "control",
                                    a { class: "button is-static", "kg" }
                                }
                            }
                        }
                    }
                    div { class: "column",
                        div { class: "field",
                            label { class: "label", "Overweight" }
                            div { class: "field has-addons",
                                p { class: "control",
                                    input {
                                        class: "input",
                                        disabled: true,
                                        r#type: "text",
                                        value: display_weight(overwight)
                                    }
                                }
                                p { class: "control",
                                    a { class: "button is-static", "kg" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn display_weight(weight: Option<f32>) -> String {
    if let Some(weight) = weight {
        format!("{weight:.1}")
    } else {
        String::new()
    }
}
