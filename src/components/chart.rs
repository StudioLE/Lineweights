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
    let shots = state.shots.read().clone();
    let weights = state.weights.read().clone();
    let (min_date, max_date) = get_date_range(&shots, &weights)?;
    let total_days = (max_date - min_date).num_days();
    let (min_weight, max_weight) = get_weight_range(&weights)?;
    let weight_span = max_weight - min_weight;
    rsx! {
        svg {
            view_box: "0 {min_weight} {total_days} {weight_span}",
            preserve_aspect_ratio: "xMidYMid slice",
            role: "img",            
            for data in weights {
                circle {
                    cx: (max_date - data.date).num_days(),
                    cy: data.weight,
                    r: 0.25,
                    style: "fill:var(--bulma-primary)"
                }                
            }
        }
    }
}

fn get_date_range(
    shots: &Vec<ShotData>,
    weights: &Vec<WeightData>,
) -> Result<(NaiveDate, NaiveDate), ChartError> {
    let mut dates: Vec<_> = shots.iter().map(|x| x.date).collect();
    dates.append(&mut weights.iter().map(|x| x.date).collect());
    if dates.len() < 2 {
        Err(InsufficientData)?;
    }
    dates.sort();
    let min = dates.first().expect("should be at least 2 dates").clone();
    let max = dates.last().expect("should be at least 2 dates").clone();
    Ok((min, max))
}

fn get_weight_range(
    weights: &Vec<WeightData>,
) -> Result<(f32, f32), ChartError> {
    let mut weights: Vec<_> = weights.iter().map(|x| x.weight).collect();
    if weights.len() < 2 {
        Err(InsufficientData)?;
    }
    weights.sort_by(|a, b| a.partial_cmp(b).expect("weights should be comparable"));
    let min = weights.first().expect("should be at least 2 weights").clone();
    let max = weights.last().expect("should be at least 2 weights").clone();
    Ok((min, max))
}

#[derive(Debug)]
pub enum ChartError {
    InsufficientData,
}

impl Display for ChartError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for ChartError {}
