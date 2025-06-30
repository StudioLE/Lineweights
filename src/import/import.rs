use super::*;
use crate::prelude::*;
use ImportError::*;

#[allow(clippy::absolute_paths)]
#[component]
pub fn Import() -> Element {
    let mut state = use_context::<State>();
    let on_file_changed = move |event| async move {
        let Some(entries) = read_file(event)
            .await
            .handle_error(|e| warn!("Failed to read file: {e:?}"))
        else {
            return;
        };
        let Some(collection) =
            EntryCollection::new(entries).handle_error(|e| warn!("Failed to read file: {e:?}"))
        else {
            return;
        };
        state.entries.set(collection);
        state.page.set(Navigation::Chart);
    };
    rsx! {
        h1 { class: "title", "Import data from Shotsy" }
        h2 { class: "subtitle",
            "This app is designed to visualize the data you already have stored in Shotsy."
        }
        div { class: "content",
            ol { class: "block",
                li { "Open the Shotsy app" }
                li {"Go to ", code { "Settings > Export Data > Export Data to CSV" } }
                li { "Save the ", code { "shotsy_export.csv" }, " file to your device" }
                li { "Import it with the button below" }
            }
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

async fn read_file(evt: Event<FormData>) -> Result<Vec<Entry>, ImportError> {
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
    let mut entries = Vec::new();
    for (i, result) in reader.deserialize::<ShotsyData>().enumerate() {
        match result {
            Ok(shotsy) => {
                if let Some(entry) = shotsy.to_entry() {
                    entries.push(entry);
                } else {
                    warn!("Row did not contain shot or weight data.");
                }
            }
            Err(error) => {
                warn!("Failed to read line {}: {error}", i + 1);
            }
        }
    }
    Ok(entries)
}

#[derive(Debug)]
pub enum ImportError {
    NoFileEngine,
    MultipleFiles,
    NoFiles,
    FailedToRead,
}

impl Display for ImportError {
    #[allow(clippy::absolute_paths)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for ImportError {}
