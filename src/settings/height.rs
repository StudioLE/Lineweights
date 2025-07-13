use crate::prelude::*;

#[component]
pub(crate) fn Height() -> Element {
    let state: HeightState = use_context();
    let is_height_valid = use_signal(|| true);
    let height_value = use_signal(|| state.get_cm_string());
    rsx! {
        div { class: "field",
            label { class: "label", "Height" }
            div { class: "field has-addons",
                p { class: "control",
                    input {
                        oninput: move |e| {
                            oninput(e, height_value, state.clone(), is_height_valid);
                        },
                        class: if !*is_height_valid.read() { "input is-danger" } else { "input"},
                        r#type: "text",
                        placeholder: "Example: 162",
                        value: height_value
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
}

fn oninput(
    event: Event<FormData>,
    mut input: Signal<String>,
    state: HeightState,
    mut is_height_valid: Signal<bool>,
) {
    event.prevent_default();
    let input_value = event.value();
    input.set(input_value.clone());
    match parse_input(input_value) {
        Ok(height) => {
            state.set(Some(height));
            is_height_valid.set(true);
        }
        Err(_) => {
            state.set(None);
            is_height_valid.set(false);
        }
    }
}

pub fn parse_input(input: String) -> Result<f32, String> {
    let Ok(cm) = input.parse::<f32>() else {
        return Err("Height must be a number".to_owned());
    };
    if !(50.0..=300.0).contains(&cm) {
        return Err("Height must be between 50 and 300 cm".to_owned());
    }
    Ok(cm / 100.0)
}
