use chrono::{NaiveDate, NaiveTime};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ShotsyData {
    #[serde(rename = "Date (UTC)")]
    pub date: Option<NaiveDate>,

    #[serde(rename = "Shot")]
    pub shot: Option<String>,

    /// Time of shot
    #[serde(rename = "Time (UTC)")]
    pub time: Option<NaiveTime>,

    #[serde(rename = "Site")]
    pub site: Option<String>,

    #[serde(rename = "Shot Notes")]
    pub shot_notes: Option<String>,

    /// Weight recorded in lb
    #[serde(rename = "Recorded Weight")]
    pub weight: Option<f32>,

    #[serde(rename = "Calories")]
    pub calories: Option<String>,

    #[serde(rename = "Protein")]
    pub protein: Option<String>,

    #[serde(rename = "Water")]
    pub water: Option<String>,

    #[serde(rename = "Day Notes")]
    pub day_notes: Option<String>,

    #[serde(rename = "Nausea")]
    pub nausea: Option<String>,

    #[serde(rename = "Heartburn")]
    pub heartburn: Option<String>,

    #[serde(rename = "Food Noise")]
    pub food_noise: Option<String>,

    #[serde(rename = "Suppressed Appetite")]
    pub suppressed_appetite: Option<String>,

    #[serde(rename = "Constipation")]
    pub constipation: Option<String>,

    #[serde(rename = "Diarrhea")]
    pub diarrhea: Option<String>,

    #[serde(rename = "Belching")]
    pub belching: Option<String>,

    #[serde(rename = "Injection Site Reaction")]
    pub injection_site_reaction: Option<String>,

    #[serde(rename = "Mood Swings")]
    pub mood_swings: Option<String>,

    #[serde(rename = "Indigestion")]
    pub indigestion: Option<String>,

    #[serde(rename = "Metallic Taste")]
    pub metallic_taste: Option<String>,

    #[serde(rename = "Stomach Pain")]
    pub stomach_pain: Option<String>,

    #[serde(rename = "Hair Loss")]
    pub hair_loss: Option<String>,

    #[serde(rename = "Fatigue")]
    pub fatigue: Option<String>,

    #[serde(rename = "Migraine")]
    pub migraine: Option<String>,
}
