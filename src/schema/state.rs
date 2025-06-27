use dioxus::prelude::*;
use super::*;

#[derive(Clone, Debug)]
pub struct State {
    pub entries: Signal<Vec<Entry>>,
    pub page: Signal<Page>
}

impl State {
    pub fn new() -> Self {
        Self {
            entries: Signal::new(Vec::new()),
            page: Signal::new(Page::Import),
        }
    }
}