use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct State {
    pub entries: Signal<Vec<Entry>>,
    pub page: Signal<Navigation>,
}

impl State {
    pub fn new() -> Self {
        Self {
            entries: Signal::new(Vec::new()),
            page: Signal::new(Navigation::Import),
        }
    }
}
