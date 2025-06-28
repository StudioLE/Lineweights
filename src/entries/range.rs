use crate::prelude::*;
use std::error::Error;
use std::fmt::{Display, Formatter};
use EntryRangeError::*;

#[derive(Debug)]
pub struct EntryRange {
    min_date: NaiveDate,
    max_date: NaiveDate,
    min_weight: f32,
    max_weight: f32,
    x_scale: f32,
    y_scale: f32,
}

impl EntryRange {
    fn default() -> Self {
        Self {
            min_date: NaiveDate::default(),
            max_date: NaiveDate::default(),
            min_weight: f32::default(),
            max_weight: f32::default(),
            x_scale: f32::default(),
            y_scale: f32::default(),
        }
    }

    pub fn new(entries: &Vec<Entry>) -> Result<EntryRange, EntryRangeError> {
        let mut range = Self::default();
        range.set_date_range(&entries)?;
        range.set_weight_range(&entries)?;
        range.set_scales();
        Ok(range)
    }

    fn set_date_range(&mut self, entries: &[Entry]) -> Result<(), EntryRangeError> {
        let mut dates: Vec<_> = entries.iter().map(|x| x.date).collect();
        if dates.len() < 2 {
            return Err(NeedTwoDates);
        }
        dates.sort();
        self.min_date = *dates.first().expect("should be at least 2 entries");
        self.max_date = *dates.last().expect("should be at least 2 entries");
        Ok(())
    }

    fn set_weight_range(&mut self, entries: &[Entry]) -> Result<(), EntryRangeError> {
        let mut weights: Vec<_> = entries.iter().filter_map(|x| x.weight).collect();
        if weights.len() < 2 {
            return Err(NeedTwoWeights);
        }
        weights.sort_by(|a, b| a.partial_cmp(b).expect("weights should be comparable"));
        self.min_weight = *weights.first().expect("should be at least 2 weights");
        self.max_weight = *weights.last().expect("should be at least 2 weights");
        Ok(())
    }

    fn set_scales(&mut self) {
        let total_days = (self.max_date - self.min_date).num_days();
        let weight_span = self.max_weight - self.min_weight;
        self.x_scale = 1.0 / total_days as f32;
        self.y_scale = 1.0 / weight_span;
    }

    pub fn get_day(&self, date: NaiveDate) -> usize {
        (date - self.min_date).num_days() as usize
    }

    pub fn get_x(&self, date: NaiveDate) -> f32 {
        self.get_day(date) as f32 * self.x_scale
    }

    pub fn get_y(&self, weight: f32) -> f32 {
        1.0 - (weight - self.min_weight) * self.y_scale
    }
}

#[derive(Debug)]
pub enum EntryRangeError {
    NeedTwoDates,
    NeedTwoWeights,
}

impl Display for EntryRangeError {
    #[allow(clippy::absolute_paths)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for EntryRangeError {}
