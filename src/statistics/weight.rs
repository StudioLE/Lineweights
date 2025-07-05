use crate::prelude::*;

#[allow(clippy::integer_division, clippy::identity_op)]
const SMA1: isize = 1 * 7 - 1;
#[allow(clippy::integer_division, clippy::identity_op)]
const SMA1C: isize = (1 * 7) / 2;
#[allow(clippy::integer_division)]
const SMA2: isize = 2 * 7 - 1;
#[allow(clippy::integer_division)]
const SMA2C: isize = (2 * 7) / 2;
#[allow(clippy::integer_division)]
const SMA4: isize = 4 * 7 - 1;
#[allow(clippy::integer_division)]
const SMA4C: isize = (4 * 7) / 2;
#[allow(clippy::integer_division)]
const SMA8: isize = 8 * 7 - 1;
#[allow(clippy::integer_division)]
const SMA8C: isize = (8 * 7) / 2;

#[allow(dead_code)]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WeightStatistics {
    /// Simple moving average over the past week
    pub sma1: Option<f32>,
    /// Simple moving average centered to 1 week
    pub sma1c: Option<f32>,
    /// Simple moving average over the past 2 weeks
    pub sma2: Option<f32>,
    /// Simple moving average centered over 2 weeks
    pub sma2c: Option<f32>,
    /// Simple moving average over the past 4 weeks
    pub sma4: Option<f32>,
    /// Simple moving average centered over 4 weeks
    pub sma4c: Option<f32>,
    /// Simple moving average over the past 8 weeks
    pub sma8: Option<f32>,
    /// Simple moving average centered over 8 weeks
    pub sma8c: Option<f32>,
}

impl WeightStatistics {
    pub fn set(entries: &mut [Entry], range: &EntryRange) {
        let sma1 = SimpleMovingAverage::new(entries, range, SMA1);
        let sma1c = SimpleMovingAverage::new_centered(entries, range, SMA1C);
        let sma2 = SimpleMovingAverage::new(entries, range, SMA2);
        let sma2c = SimpleMovingAverage::new_centered(entries, range, SMA2C);
        let sma4 = SimpleMovingAverage::new(entries, range, SMA4);
        let sma4c = SimpleMovingAverage::new_centered(entries, range, SMA4C);
        let sma8 = SimpleMovingAverage::new(entries, range, SMA8);
        let sma8c = SimpleMovingAverage::new_centered(entries, range, SMA8C);
        for entry in entries.iter_mut() {
            entry.statistics = WeightStatistics {
                sma1: sma1.execute(entry.day),
                sma1c: sma1c.execute(entry.day),
                sma2: sma2.execute(entry.day),
                sma2c: sma2c.execute(entry.day),
                sma4: sma4.execute(entry.day),
                sma4c: sma4c.execute(entry.day),
                sma8: sma8.execute(entry.day),
                sma8c: sma8c.execute(entry.day),
            };
        }
    }
}
