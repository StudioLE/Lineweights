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
    rsx! {
        svg {
            view_box: "0 {min_weight} {total_days} {weight_span}",
            preserve_aspect_ratio: "xMidYMid slice",
            role: "img",            
            for entry in entries {
                if let Some(weight) = entry.weight {                    
                    circle {
                        cx: (max_date - entry.date).num_days(),
                        cy: weight,
                        r: 0.25,
                        fill: "var(--bulma-primary)"
                    }
                }              
            }
        }
    }
}

fn get_date_range(
    entries: &Vec<Entry>,
) -> Result<(NaiveDate, NaiveDate), ChartError> {
    let mut dates: Vec<_> = entries.iter().map(|x| x.date).collect();
    if dates.len() < 2 {
        Err(InsufficientData)?;
    }
    dates.sort();
    let min = dates.first().expect("should be at least 2 entries").clone();
    let max = dates.last().expect("should be at least 2 entries").clone();
    Ok((min, max))
}

fn get_weight_range(
    entries: &Vec<Entry>,
) -> Result<(f32, f32), ChartError> {
    let mut weights: Vec<_> = entries.iter().filter_map(|x| x.weight).collect();
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
