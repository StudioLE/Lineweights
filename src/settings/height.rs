use crate::prelude::*;

#[component]
pub(crate) fn Height() -> Element {
    let state = use_context::<State>();
    let is_height_valid = use_signal(|| true);
    let height_value = use_signal(|| {
        state
            .height
            .read()
            .as_ref()
            .map(|height| (height * 100.0).to_string())
            .unwrap_or_default()
    });

    rsx! {
        div { class: "field",
            label { class: "label", "Height" }
            div { class: "field has-addons",
                p { class: "control",
                    input {
                        oninput: move |e| {
                            set_height(e, height_value, state.height, is_height_valid);
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

fn set_height(
    event: Event<FormData>,
    mut input: Signal<String>,
    mut state: Signal<Option<f32>>,
    mut is_height_valid: Signal<bool>,
) {
    event.prevent_default();
    let value = event.value();
    input.set(value.clone());
    let Some(cm) = value.parse::<f32>().handle_error(|e| {
        warn!("Failed to parse height from `{value}`: {e}");
        is_height_valid.set(false);
    }) else {
        return;
    };
    if !(50.0..=300.0).contains(&cm) {
        warn!("Height must be between 50 and 300 cm: {cm}");
        is_height_valid.set(false);
        return;
    }
    let height = cm / 100.0;
    state.set(Some(height));
    LocalStorage::set_height(height).handle_error(|e| warn!("Failed to set height: {e:?}"));
    is_height_valid.set(true);
}
