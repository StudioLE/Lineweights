use crate::prelude::*;

const SMA: usize = 6;
const SMA_CENTERED: usize = 3;

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
            SMA,
            0
        ));
        entry.weight_sma_centered = Some(get_simple_moving_average(
            &entries_clone,
            day,
            SMA_CENTERED,
            SMA_CENTERED,
        ));
    }
}

#[allow(
    clippy::as_conversions,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
fn get_simple_moving_average(entries: &[Entry], day: usize, before: usize, after: usize) -> f32 {
    let day = day as isize;
    let before = before as isize;
    let after = after as isize;
    let weights: Vec<_> = entries
        .iter()
        .filter(|x| {
            let candidate = x.day.expect("entry should have day set") as isize;
            candidate >= day - before && candidate <= day + after
        })
        .filter_map(|x| x.weight)
        .collect();
    assert!(!weights.is_empty(), "should have at least one weight");
    trace!("found {} weights for day {}", weights.len(), day);
    let sum: f32 = weights.iter().sum();
    sum / weights.len() as f32
}
