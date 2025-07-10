use crate::prelude::*;

#[component]
pub(crate) fn Goals() -> Element {
    let state = use_context::<State>();
    let is_height_valid = use_signal(|| true);
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
                div { class: "field",
                    label { class: "label", "Height" }
                    div { class: "field has-addons",
                        p { class: "control",
                            input {
                                oninput: move |e| set_height(e, state.height, is_height_valid),
                                class: if !*is_height_valid.read() { "input is-danger" } else { "input"},
                                r#type: "text",
                                placeholder: "Example: 162"
                            }
                        }
                        p { class: "control",
                            a { class: "button is-static", "cm" }
                        }
                    }
                    if !*is_height_valid.read() {
                        p { class: "help is-danger", "Height must be a number" }
                    }
                }
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

fn set_height(
    event: Event<FormData>,
    mut height: Signal<Option<f32>>,
    mut is_height_valid: Signal<bool>,
) {
    event.prevent_default();
    let Some(cm) = event.value().parse::<f32>().handle_error(|e| {
        warn!("Failed to parse height from `{}`: {e}", event.value());
        is_height_valid.set(false);
    }) else {
        return;
    };
    if !(50.0..=300.0).contains(&cm) {
        warn!("Height must be between 50 and 300 cm: {cm}");
        is_height_valid.set(false);
        return;
    }
    height.set(Some(cm / 100.0));
    is_height_valid.set(true);
}

fn display_weight(weight: Option<f32>) -> String {
    if let Some(weight) = weight {
        format!("{weight:.1}")
    } else {
        String::new()
    }
}
