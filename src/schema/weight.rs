use chrono::NaiveDate;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct WeightData {
    pub date: NaiveDate,
    pub weight: f32,
}
