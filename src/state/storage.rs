use crate::prelude::*;
use web_sys::wasm_bindgen::JsValue;
use web_sys::{window, Storage};
use LocalStorageError::*;

const ENTRIES_KEY: &str = "entries";

pub struct LocalStorage;

impl LocalStorage {
    #[allow(clippy::result_large_err)]
    pub fn set_entries(entries: &[Entry]) -> Result<(), LocalStorageError> {
        let serialized = serde_json::to_string(entries).map_err(Serialization)?;
        get_local_storage()?
            .set_item(ENTRIES_KEY, &serialized)
            .map_err(Js)
    }

    #[allow(clippy::result_large_err)]
    pub fn get_entries() -> Result<Vec<Entry>, LocalStorageError> {
        let serialized = get_local_storage()?
            .get_item(ENTRIES_KEY)
            .map_err(Js)?
            .ok_or(NoData)?;
        serde_json::from_str(&serialized).map_err(Deserialization)
    }
}

#[allow(clippy::result_large_err)]
fn get_local_storage() -> Result<Storage, LocalStorageError> {
    window()
        .ok_or(NoWindow)?
        .local_storage()
        .map_err(Js)?
        .ok_or(NoStorage)
}

#[allow(dead_code, clippy::absolute_paths)]
#[derive(Debug)]
pub enum LocalStorageError {
    NoWindow,
    Js(JsValue),
    NoStorage,
    Serialization(serde_json::Error),
    FlushWriter,
    ToUtf8(std::string::FromUtf8Error),
    NoData,
    Deserialization(serde_json::Error),
}
