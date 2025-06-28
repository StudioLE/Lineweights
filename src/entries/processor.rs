use crate::prelude::*;

const SMA: isize = 6;
const SMA_CENTERED: isize = 3;

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
    let sma = SimpleMovingAverage {
        entries: entries_clone.clone(),
        before: SMA,
        after: 0,
    };
    let smac = SimpleMovingAverage {
        entries: entries_clone.clone(),
        before: SMA_CENTERED,
        after: SMA_CENTERED,
    };
    for entry in entries.iter_mut() {
        let day = entry.day.expect("entry should have day set");
        entry.weight_sma = sma.execute(day);
        entry.weight_sma_centered = smac.execute(day);
    }
}

struct SimpleMovingAverage {
    entries: Vec<Entry>,
    before: isize,
    after: isize,
}

impl SimpleMovingAverage {
    #[allow(
        clippy::as_conversions,
        clippy::cast_possible_wrap,
        clippy::cast_precision_loss
    )]
    pub fn execute(&self, day: usize) -> Option<f32> {
        let weights = self.get_weights(day as isize);
        trace!("found {} weights for day {}", weights.len(), day);
        if weights.is_empty() {
            return None;
        }
        let sum: f32 = weights.iter().sum();
        Some(sum / weights.len() as f32)
    }

    #[allow(clippy::as_conversions, clippy::cast_possible_wrap)]
    fn get_weights(&self, day: isize) -> Vec<f32> {
        self.entries
            .iter()
            .filter(|x| {
                let candidate = x.day.expect("entry should have day set") as isize;
                candidate >= day - self.before && candidate <= day + self.after
            })
            .filter_map(|x| x.weight)
            .collect()
    }
}
