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
}
