use crate::prelude::*;

#[allow(
    clippy::as_conversions,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::comparison_chain
)]
pub fn get_trend<F: Fn(&WeightStatistics) -> Option<f32>>(
    entries: &[Entry],
    period: usize,
    predicate: F,
) -> Option<Vec<(usize, f32)>> {
    let period = period as isize;
    let mut entries: Vec<_> = entries
        .iter()
        .filter(|entry| predicate(&entry.statistics).is_some())
        .collect();
    entries.reverse();
    let mut values = Vec::new();
    let start = entries.first()?;
    values.push((
        start.day,
        predicate(&start.statistics).expect("value should be some"),
    ));
    let mut target = start.day as isize - period;
    for pair in entries.windows(2) {
        let previous = pair.first().expect("should be two values");
        let entry = pair.get(1).expect("should be two values");
        let day = entry.day as isize;
        if day == target {
            values.push((
                target as usize,
                predicate(&entry.statistics).expect("value should be some"),
            ));
            target -= period;
        } else if day < target {
            let entry_value = predicate(&entry.statistics).expect("value should be some");
            let previous_value = predicate(&previous.statistics).expect("value should be some");
            let modifier =
                (previous.day as f32 - target as f32) / (previous.day as f32 - day as f32);
            let diff = previous_value - entry_value;
            let value = previous_value - diff * modifier;
            values.push((target as usize, value));
            target -= period;
        }
    }
    Some(values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _get_trend() -> Result<(), EntryRangeError> {
        // Arrange
        let json = include_str!("../../samples/entries.json");
        let entries = Entry::from_json(json).expect("Entries sample should be valid");
        // Act
        let collection = EntryCollection::new(entries)?;
        // Assert
        let trend = get_trend(&collection.entries, 7, |x| x.sma1c).unwrap_or_default();
        let verified = Verify::new()
            .multiple(&trend)
            .expect("Verify should not fail");
        assert!(verified);
        Ok(())
    }
}
