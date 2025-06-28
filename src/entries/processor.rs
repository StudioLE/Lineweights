use crate::prelude::*;

#[derive(Debug)]
pub struct Processor;

impl Processor {
    pub fn execute(entries: &mut Vec<Entry>, range: &EntryRange) {
        set_days(entries, &range);
        set_weight_sma(entries);
    }
}

fn set_days(entries: &mut Vec<Entry>, range: &EntryRange) {
    for entry in entries.iter_mut() {
        entry.day = Some(range.get_day(entry.date))
    }
}

fn set_weight_sma(entries: &mut Vec<Entry>) {
    let entries_clone = entries.clone();
    for entry in entries.iter_mut() {
        let day = entry.day.expect("entry should have day set");
        entry.weight_sma = Some(get_simple_moving_average(&entries_clone, day))
    }
}

fn get_simple_moving_average(entries: &[Entry], day: usize) -> f32 {
    let weights: Vec<_> = entries
        .iter()
        .filter(|x| {
            let candidate = x.day.expect("entry should have day set") as isize;
            candidate >= day as isize - 3 && candidate <= day as isize + 3
        })
        .filter_map(|x| x.weight)
        .collect();
    assert!(!weights.is_empty(), "should have at least one weight");
    trace!("found {} weights for day {}", weights.len(), day);
    let sum: f32 = weights.iter().sum();
    sum / weights.len() as f32
}