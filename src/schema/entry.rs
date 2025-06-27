use chrono::NaiveDate;
use crate::schema::Shot;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Entry {
    pub date: NaiveDate,
    pub weight: Option<f32>,
    pub shot: Option<Shot>
}
