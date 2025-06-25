use chrono::{NaiveDate, NaiveTime};

#[derive(Clone, Debug)]
pub enum Medication {
    Mounjaro,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ShotData {
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub medication: Medication,
    pub dose: f32,
    pub site: Option<String>,
    pub notes: Option<String>,
}
