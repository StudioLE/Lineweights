use crate::schema::Shot;
use chrono::NaiveDate;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Entry {
    pub date: NaiveDate,
    pub weight: Option<f32>,
    pub shot: Option<Shot>,
}
