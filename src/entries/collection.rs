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
        entry.day = range.get_day(entry.date);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() -> Result<(), EntryRangeError> {
        // Arrange
        let json = include_str!("../../samples/entries.json");
        let entries = Entry::from_json(json).expect("Entries sample should be valid");
        // Act
        let collection = EntryCollection::new(entries)?;
        // Assert
        let stats: Vec<_> = collection
            .entries
            .into_iter()
            .map(|entry| entry.statistics)
            .collect();
        let verified = Verify::new()
            .multiple(&stats)
            .expect("Verify should not fail");
        assert!(verified);
        Ok(())
    }
}
