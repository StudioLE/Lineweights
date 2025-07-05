use crate::prelude::*;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Entry {
    pub date: NaiveDate,
    pub weight: Option<f32>,
    pub shot: Option<Shot>,
}

impl Entry {
    pub(crate) fn from_json(json: &str) -> serde_json::Result<Vec<Entry>> {
        serde_json::from_str(json)
    }
}

pub(crate) trait EntryExtensions {
    fn to_json(self) -> serde_json::Result<String>;
    #[allow(dead_code)]
    fn to_json_pretty(self) -> serde_json::Result<String>;
}

impl EntryExtensions for &[Entry] {
    fn to_json(self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
    fn to_json_pretty(self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_json() -> Result<(), serde_json::Error> {
        // Arrange
        let csv = include_str!("../../samples/shotsy.csv");
        let data = ShotsyData::from_csv(csv);
        let entries = ShotsyData::to_entries(data);
        let verified = include_str!("../../samples/entries.json");
        // Act
        let json = entries.to_json_pretty()?;
        // Assert
        assert_eq!(json, verified);
        Ok(())
    }

    #[test]
    fn from_json() -> Result<(), serde_json::Error> {
        // Arrange
        let json = include_str!("../../samples/entries.json");
        // Act
        let entries = Entry::from_json(json)?;
        // Assert
        assert_eq!(entries.len(), 144);
        Ok(())
    }
}
