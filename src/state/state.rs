use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct State {
    pub entries: Signal<EntryCollection>,
    pub height: Signal<Option<f32>>,
    pub page: Signal<Navigation>,
}

impl State {
    fn new(collection: EntryCollection, height: Option<f32>, page: Navigation) -> Self {
        Self {
            entries: use_signal(|| collection),
            height: use_signal(|| height),
            page: use_signal(|| page),
        }
    }

    pub fn init() -> Self {
        if let Some(state) = Self::from_local_storage() {
            state
        } else {
            Self::new(EntryCollection::default(), None, Navigation::Import)
        }
    }

    fn from_local_storage() -> Option<Self> {
        let entries = LocalStorage::get_entries()
            .handle_error(|e| warn!("Failed to get entries from local storage: {e:?}"))?;
        let collection = EntryCollection::new(entries)
            .handle_error(|e| warn!("Failed to determine range: {e:?}"))?;
        let height = LocalStorage::get_height()
            .handle_error(|e| warn!("Failed to get height from local storage: {e:?}"));
        Some(Self::new(collection, height, Navigation::Chart))
    }
}
