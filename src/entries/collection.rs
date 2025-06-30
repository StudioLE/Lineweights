use crate::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct EntryCollection {
    pub entries: Vec<Entry>,
    pub range: EntryRange,
}

impl EntryCollection {
    pub fn new(mut entries: Vec<Entry>) -> Result<Self, EntryRangeError> {
        let range = EntryRange::new(&entries)?;
        set_days(&mut entries, &range);
        WeightStatistics::set(&mut entries, &range);
        Ok(Self { entries, range })
    }
}

fn set_days(entries: &mut [Entry], range: &EntryRange) {
    for entry in entries.iter_mut() {
        entry.day = Some(range.get_day(entry.date));
    }
}
