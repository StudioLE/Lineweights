use crate::schema::*;
use chrono::NaiveDate;
use dioxus::prelude::*;
use std::error::Error;
use std::fmt::{Display, Formatter};
use ChartError::*;

#[allow(clippy::absolute_paths)]
#[component]
pub fn Chart() -> Element {
    let state = use_context::<State>();
    let entries = state.entries.read().clone();
    let (min_date, max_date) = get_date_range(&entries)?;
    let total_days = (max_date - min_date).num_days();
    let (min_weight, max_weight) = get_weight_range(&entries)?;
    let weight_span = max_weight - min_weight;
    let view_box = format!("-1 {} {} {}", min_weight.floor() - 1.0, total_days + 2, weight_span.ceil() + 2.0);
    rsx! {
        svg {
            view_box: view_box,
            preserve_aspect_ratio: "xMidYMid slice",
            role: "img",
            for entry in entries {
                if let Some(weight) = entry.weight {
                    circle {
                        cx: (max_date - entry.date).num_days(),
                        cy: weight,
                        r: if entry.shot.is_some() { 0.4 } else { 0.3 },
                        fill: get_color(entry.shot)
                    }
                }
            }
        }
    }
}

fn get_date_range(entries: &[Entry]) -> Result<(NaiveDate, NaiveDate), ChartError> {
    let mut dates: Vec<_> = entries.iter().map(|x| x.date).collect();
    if dates.len() < 2 {
        Err(InsufficientData)?;
    }
    dates.sort();
    let min = *dates.first().expect("should be at least 2 entries");
    let max = *dates.last().expect("should be at least 2 entries");
    Ok((min, max))
}

fn get_weight_range(entries: &[Entry]) -> Result<(f32, f32), ChartError> {
    let mut weights: Vec<_> = entries.iter().filter_map(|x| x.weight).collect();
    if weights.len() < 2 {
        Err(InsufficientData)?;
    }
    weights.sort_by(|a, b| a.partial_cmp(b).expect("weights should be comparable"));
    let min = *weights.first().expect("should be at least 2 weights");
    let max = *weights.last().expect("should be at least 2 weights");
    Ok((min, max))
}

fn get_color(shot: Option<Shot>) -> String {
    let color = if let Some(shot) = shot {
        if shot.dose == 2.5 {
            "#525252"
        } else if shot.dose == 5.0 {
            "#581c87"
        } else if shot.dose == 7.5 {
            "#0f766e"
        } else if shot.dose == 10.0 {
            "#db2777"
        } else if shot.dose == 12.5 {
            "#0ea5e9"
        } else if shot.dose == 15.0 {
            "#f87171"
        } else {
            "#64748b"
        }
    } else {
        "#374151"
    };
    color.to_owned()
}

#[derive(Debug)]
pub enum ChartError {
    InsufficientData,
}

impl Display for ChartError {
    #[allow(clippy::absolute_paths)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for ChartError {}
