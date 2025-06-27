use crate::schema::*;
use dioxus::logger::tracing::{info, warn};
use dioxus::prelude::*;
use ImportError::*;

#[derive(Debug)]
pub enum ImportError {
    NoFileEngine,
    MultipleFiles,
    NoFiles,
    FailedToRead,
}

#[allow(clippy::absolute_paths)]
#[component]
pub fn Import() -> Element {    
    let mut state = use_context::<State>();
    let on_file_changed = move |event| async move {
        match read_file(event).await {
            Ok((shots, weights)) => {
                info!(
                    "Loaded {} shots and {} weights",
                    shots.len(),
                    weights.len()
                );
                state.shots.set(shots);
                state.weights.set(weights);
            }
            Err(error) => {
                warn!("Failed to read file: {error:?}");
            }
        }
    };
    rsx! {
        section { class: "section is-large",
            h1 { class: "title", "Import data from Shotsy" }
            h2 { class: "subtitle",
                "This app is designed to visualize the data you already have stored in Shotsy."
            }
            ul { class: "block",
                li { "Open the Shotsy app" }
                li {"Go to ", code { "Settings > Export Data > Export Data to CSV" } }
                li { "Save the ", code { "shotsy_export.csv" }, " file to your device" }
                li { "Import it with the button below" }
            }
            div { class: "file",
                label { class: "file-label",
                    input {
                        class: "file-input",
                        r#type: "file",
                        accept: ".csv",
                        multiple: false,
                        onchange: on_file_changed
                    }
                    span { class: "file-cta",
                        span { class: "file-label", " Choose a file… " }
                    }
                }
            }
        }
    }
}

async fn read_file(evt: Event<FormData>) -> Result<(Vec<ShotData>, Vec<WeightData>), ImportError> {
    let Some(file_engine) = evt.files() else {
        return Err(NoFileEngine);
    };
    let files = file_engine.files();
    if files.is_empty() {
        return Err(NoFiles);
    }
    if files.len() > 1 {
        return Err(MultipleFiles);
    }
    let file = files.first().expect("should be exactly one file");
    let Some(content) = file_engine.read_file_to_string(file).await else {
        return Err(FailedToRead);
    };
    let mut reader = csv::Reader::from_reader(content.as_bytes());
    let mut shots = Vec::new();
    let mut weights = Vec::new();
    for (i, result) in reader.deserialize::<ShotsyData>().enumerate() {
        match result {
            Ok(shotsy) => {
                let shot = shotsy.clone().to_shot();
                let weight = shotsy.to_weight();
                if shot.is_none() && weight.is_none() {
                    warn!("Row did not contain shot or weight data.");
                }
                if let Some(shot) = shot {
                    shots.push(shot);
                }
                if let Some(weight) = weight {
                    weights.push(weight);                    
                }
            }
            Err(error) => {
                warn!("Failed to read line {}: {error}", i + 1);
            }
        }
    }
    Ok((shots, weights))
}
