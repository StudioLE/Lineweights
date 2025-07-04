use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entry {
    pub date: NaiveDate,
    pub day: usize,
    pub weight: Option<f32>,
    pub shot: Option<Shot>,
    pub statistics: WeightStatistics,
}
