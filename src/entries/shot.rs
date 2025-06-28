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

impl Shot {
    #[allow(clippy::float_cmp)]
    pub fn get_color(shot: &Option<Shot>) -> String {
        let color = if let Some(shot) = shot {
            if shot.dose == 2.5 {
                NEUTRAL_600
            } else if shot.dose == 5.0 {
                "#581c87"
            } else if shot.dose == 7.5 {
                PURPLE_900
            } else if shot.dose == 10.0 {
                PINK_600
            } else if shot.dose == 12.5 {
                SKY_500
            } else if shot.dose == 15.0 {
                RED_400
            } else {
                SLATE_500
            }
        } else {
            GRAY_700
        };
        color.to_owned()
    }

}