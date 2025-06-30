use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Entry {
    pub date: NaiveDate,
    pub day: usize,
    pub weight: Option<f32>,
    pub shot: Option<Shot>,
    pub statistics: WeightStatistics,
}
