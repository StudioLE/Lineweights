use dioxus::prelude::*;
use super::*;

#[derive(Clone, Debug)]
pub struct State {
    pub weights: Signal<Vec<WeightData>>,
    pub shots: Signal<Vec<ShotData>>,
    pub page: Signal<Page>
}

impl State {
    pub fn new() -> Self {
        Self {
            weights: Signal::new(Vec::new()),
            shots: Signal::new(Vec::new()),
            page: Signal::new(Page::Import),
        }
    }
}