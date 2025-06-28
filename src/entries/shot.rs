use crate::prelude::*;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Shot {
    pub time: NaiveTime,
    pub medication: Medication,
    pub dose: f32,
    pub site: Option<String>,
    pub notes: Option<String>,
}
