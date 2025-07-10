use crate::prelude::*;

#[component]
pub(crate) fn Goals() -> Element {
    let state = use_context::<State>();
    let is_height_valid = use_signal(|| true);
    rsx! {
        section { class: "section is-medium",
            div { class: "field",
                label { class: "label", "Height" }
                div { class: "control",
                    input {
                        oninput: move |e| set_height(e, state.height, is_height_valid),
                        class: if !*is_height_valid.read() { "input is-danger" } else { "input"},
                        r#type: "text",
                        placeholder: "Example: 162"
                    }
                }
                if !*is_height_valid.read() {
                    p { class: "help is-danger", "Height must be a number" }
                }
                p { class: "help", "Height in cm" }
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
    match event.value().parse::<f32>() {
        Ok(value) => {
            height.set(Some(value));
            is_height_valid.set(true);
        }
        Err(e) => {
            warn!("Failed to parse height from `{}`: {e}", event.value());
            is_height_valid.set(false);
        }
    }
}
