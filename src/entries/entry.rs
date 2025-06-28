use crate::prelude::*;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Entry {
    pub date: NaiveDate,
    pub day: Option<usize>,
    pub weight: Option<f32>,
    pub weight_sma: Option<f32>,
    pub weight_sma_centered: Option<f32>,
    pub shot: Option<Shot>,
}
