use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct State {
    pub entries: Signal<EntryCollection>,
    pub page: Signal<Navigation>,
}

impl State {
    pub fn new() -> Self {
        Self {
            entries: Signal::new(EntryCollection::default()),
            page: Signal::new(Navigation::Import),
        }
    }

    pub fn from_local_storage() -> Self {
        let Some(entries) = LocalStorage::get_entries()
            .handle_error(|e| warn!("Failed to get state from local storage: {e:?}"))
        else {
            return Self::new();
        };
        let Some(collection) = EntryCollection::new(entries)
            .handle_error(|e| warn!("Failed to determine range: {e:?}"))
        else {
            return Self::new();
        };
        Self {
            entries: Signal::new(collection),
            page: Signal::new(Navigation::Chart),
        }
    }
}
