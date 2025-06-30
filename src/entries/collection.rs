use crate::prelude::*;

const SMA: isize = 6;
const SMA_CENTERED: isize = 3;

#[derive(Clone, Debug, Default)]
pub struct EntryCollection {
    pub entries: Vec<Entry>,
    pub range: EntryRange,
}

impl EntryCollection {
    pub fn new(mut entries: Vec<Entry>) -> Result<Self, EntryRangeError> {
        let range = EntryRange::new(&entries)?;
        set_days(&mut entries, &range);
        set_weight_sma(&mut entries, &range);
        Ok(Self { entries, range })
    }
}

fn set_days(entries: &mut [Entry], range: &EntryRange) {
    for entry in entries.iter_mut() {
        entry.day = Some(range.get_day(entry.date));
    }
}

fn set_weight_sma(entries: &mut [Entry], range: &EntryRange) {
    let entries_clone = entries.to_vec();
    let sma = SimpleMovingAverage {
        entries: entries_clone.clone(),
        range: range.clone(),
        before: SMA,
        after: 0,
    };
    let smac = SimpleMovingAverage {
        entries: entries_clone.clone(),
        range: range.clone(),
        before: SMA_CENTERED,
        after: SMA_CENTERED,
    };
    for entry in entries.iter_mut() {
        let day = entry.day.expect("entry should have day set");
        entry.weight_sma = sma.execute(day);
        entry.weight_sma_centered = smac.execute(day);
    }
}
