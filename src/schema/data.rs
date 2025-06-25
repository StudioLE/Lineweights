use super::*;
use regex::Regex;

#[derive(Clone, Debug)]
pub struct Data {
    pub weights: Vec<WeightData>,
    pub shots: Vec<ShotData>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            weights: Vec::new(),
            shots: Vec::new(),
        }
    }

    pub fn add(&mut self, shotsy: &ShotsyData) -> bool {
        let shot = self.add_shot(shotsy);
        let weight = self.add_weight(shotsy);
        shot.is_some() || weight.is_some()
    }

    fn add_shot(&mut self, shotsy: &ShotsyData) -> Option<()> {
        self.shots.push(ShotData {
            date: shotsy.date?,
            time: shotsy.time?,
            medication: get_medication(shotsy.shot.clone())?,
            dose: get_dose(shotsy.shot.clone())?,
            site: shotsy.site.clone(),
            notes: shotsy.shot_notes.clone(),
        });
        Some(())
    }

    fn add_weight(&mut self, shotsy: &ShotsyData) -> Option<()> {
        self.weights.push(WeightData {
            date: shotsy.date?,
            weight: shotsy.weight?,
        });
        Some(())
    }
}

fn get_medication(input: Option<String>) -> Option<Medication> {
    input?
        .starts_with("Mounjaro")
        .then_some(Medication::Mounjaro)
}

fn get_dose(input: Option<String>) -> Option<f32> {
    let regex = Regex::new(r" (\d+\.\d+) mg$").expect("Regex should be valid");
    let input = input?;
    let value = regex.captures(&input)?.get(1)?.as_str();
    value.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn _get_dose() {
        assert_eq!(get_dose(Some("Mounjaro® 2.5 mg".to_owned())), Some(2.5));
        assert_eq!(get_dose(Some("Mounjaro® 2.9 mg".to_owned())), Some(2.9));
        assert_eq!(
            get_dose(Some("Mounjaro® 3.33333 mg".to_owned())),
            Some(3.33333)
        );
        assert_eq!(get_dose(Some("Mounjaro® 5.0 mg".to_owned())), Some(5.0));
        assert_eq!(get_dose(Some("Mounjaro® 7.5 mg".to_owned())), Some(7.5));
        assert_eq!(get_dose(Some("Mounjaro® 10.0 mg".to_owned())), Some(10.0));
        assert_eq!(get_dose(Some("Mounjaro® 12.5 mg".to_owned())), Some(12.5));
        assert_eq!(get_dose(Some("Mounjaro® 15.0 mg".to_owned())), Some(15.0));
    }
}
