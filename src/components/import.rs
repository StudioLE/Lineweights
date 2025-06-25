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
    let mut file_content: Signal<Data> = use_signal(Data::new);
    let on_file_changed = move |event| async move {
        match read_file(event).await {
            Ok(data) => {
                info!(
                    "Loaded {} shots and {} weights",
                    data.shots.len(),
                    data.weights.len()
                );
                file_content.set(data);
            }
            Err(error) => {
                warn!("Failed to read file: {error:?}");
            }
        }
    };
    rsx! {
        input {
            r#type: "file",
            accept: ".csv",
            multiple: false,
            onchange: on_file_changed
        }
    }
}

async fn read_file(evt: Event<FormData>) -> Result<Data, ImportError> {
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
    let mut data = Data::new();
    for (i, result) in reader.deserialize::<ShotsyData>().enumerate() {
        match result {
            Ok(shotsy) => {
                if !&data.add(&shotsy) {
                    warn!("Failed to add shotsy");
                }
            }
            Err(error) => {
                warn!("Failed to read line {}: {error}", i + 1);
            }
        }
    }
    Ok(data)
}
