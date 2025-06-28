use crate::prelude::*;

const MOVING_AVERAGE_RANGE: usize = 7;

#[derive(Debug)]
pub struct Processor;

impl Processor {
    pub fn execute(entries: &mut [Entry], range: &EntryRange) {
        set_days(entries, range);
        set_weight_sma(entries);
    }
}

fn set_days(entries: &mut [Entry], range: &EntryRange) {
    for entry in entries.iter_mut() {
        entry.day = Some(range.get_day(entry.date));
    }
}

fn set_weight_sma(entries: &mut [Entry]) {
    let entries_clone = entries.to_vec();
    for entry in entries.iter_mut() {
        let day = entry.day.expect("entry should have day set");
        entry.weight_sma = Some(get_simple_moving_average(
            &entries_clone,
            day,
            MOVING_AVERAGE_RANGE,
        ));
    }
}

#[allow(
    clippy::as_conversions,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
fn get_simple_moving_average(entries: &[Entry], day: usize, range: usize) -> f32 {
    let day = day as isize;
    let range = range as isize;
    let weights: Vec<_> = entries
        .iter()
        .filter(|x| {
            let candidate = x.day.expect("entry should have day set") as isize;
            candidate >= day - range && candidate <= day
        })
        .filter_map(|x| x.weight)
        .collect();
    assert!(!weights.is_empty(), "should have at least one weight");
    trace!("found {} weights for day {}", weights.len(), day);
    let sum: f32 = weights.iter().sum();
    sum / weights.len() as f32
}
