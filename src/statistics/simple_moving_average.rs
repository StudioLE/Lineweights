use crate::entries::{Entry, EntryRange};
use crate::prelude::trace;

pub struct SimpleMovingAverage {
    pub entries: Vec<Entry>,
    pub range: EntryRange,
    pub before: isize,
    pub after: isize,
}

impl SimpleMovingAverage {
    #[allow(
        clippy::as_conversions,
        clippy::cast_possible_wrap,
        clippy::cast_precision_loss
    )]
    pub fn execute(&self, day: usize) -> Option<f32> {
        let day = day as isize;
        if day - self.before < 0 {
            return None;
        }
        if day + self.after > self.range.get_total_days() {
            return None;
        }
        let weights = self.get_weights(day);
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
