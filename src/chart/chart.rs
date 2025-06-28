use crate::prelude::*;
use std::error::Error;
use std::fmt::{Display, Formatter};
use ChartError::*;

#[allow(clippy::absolute_paths)]
#[component]
pub fn Chart() -> Element {
    let state = use_context::<State>();
    let entries = state.entries.read().clone();
    let data = Data::new(entries).ok_or(InsufficientData)?;
    rsx! {
        svg {
            view_box: "-0.1 -0.1 1.2 1.2",
            preserve_aspect_ratio: "xMidYMid slice",
            role: "img",
            for entry in data.entries.clone() {
                if let Some(weight) = entry.weight {
                    circle {
                        cx: data.get_x(entry.date),
                        cy: data.get_y(weight),
                        r: 0.0025,
                        fill: "#1f2937"
                    }
                }
                if let Some(weight) = data.get_7_day_average(entry.date) {
                    circle {
                        cx: data.get_x(entry.date),
                        cy: data.get_y(weight),
                        r: if entry.shot.is_some() { 0.0075 } else { 0.005 },
                        fill: get_color(entry.shot)
                    }
                }
            }
        }
    }
}

struct Data {
    entries: Vec<Entry>,
    min_date: NaiveDate,
    max_date: NaiveDate,
    min_weight: f32,
    max_weight: f32,
    x_scale: f32,
    y_scale: f32,
}

impl Data {
    fn new(entries: Vec<Entry>) -> Option<Self> {
        let (min_date, max_date) = get_date_range(&entries)?;
        let total_days = (max_date - min_date).num_days();
        let (min_weight, max_weight) = get_weight_range(&entries)?;
        let weight_span = max_weight - min_weight;
        let x_scale = 1.0 / total_days as f32;
        let y_scale = 1.0 / weight_span;
        Some(Self {
            entries,
            min_date,
            max_date,
            min_weight,
            max_weight,
            x_scale,
            y_scale,
        })
    }

    fn get_day(&self, date: NaiveDate) -> usize {
        (date - self.min_date).num_days() as usize
    }

    fn get_x(&self, date: NaiveDate) -> f32 {
        self.get_day(date) as f32 * self.x_scale
    }

    fn get_y(&self, weight: f32) -> f32 {
        1.0 - (weight - self.min_weight) * self.y_scale
    }

    fn get_7_day_average(&self, date: NaiveDate) -> Option<f32> {
        let weights: Vec<_> = self
            .entries
            .iter()
            .filter(|x| {
                let days = (x.date - date).num_days();
                days >= -3 && days <= 3
            })
            .filter_map(|x| x.weight)
            .collect();
        if weights.is_empty() {
            warn!("no weights found for date {}", date);
            return None;
        }
        trace!("found {} weights for date {}", weights.len(), date);
        let sum: f32 = weights.iter().sum();
        Some(sum / weights.len() as f32)
    }
}

fn get_date_range(entries: &[Entry]) -> Option<(NaiveDate, NaiveDate)> {
    let mut dates: Vec<_> = entries.iter().map(|x| x.date).collect();
    if dates.len() < 2 {
        return None;
    }
    dates.sort();
    let min = *dates.first().expect("should be at least 2 entries");
    let max = *dates.last().expect("should be at least 2 entries");
    Some((min, max))
}

fn get_weight_range(entries: &[Entry]) -> Option<(f32, f32)> {
    let mut weights: Vec<_> = entries.iter().filter_map(|x| x.weight).collect();
    if weights.len() < 2 {
        return None;
    }
    weights.sort_by(|a, b| a.partial_cmp(b).expect("weights should be comparable"));
    let min = *weights.first().expect("should be at least 2 weights");
    let max = *weights.last().expect("should be at least 2 weights");
    Some((min, max))
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
