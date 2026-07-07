// FILE: step_dim_tol_placed_datum_target_feature.rs
// occt: StepDimTol_PlacedDatumTargetFeature

pub struct PlacedDatumTargetFeature {
    pub name: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub target_id: Option<String>,
}

impl PlacedDatumTargetFeature {
    pub fn new() -> Self {
        PlacedDatumTargetFeature {
            name: None,
            description: None,
            location: None,
            target_id: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_location(&mut self, location: String) {
        self.location = Some(location);
    }

    pub fn get_location(&self) -> Option<&str> {
        self.location.as_deref()
    }

    pub fn set_target_id(&mut self, target_id: String) {
        self.target_id = Some(target_id);
    }

    pub fn get_target_id(&self) -> Option<&str> {
        self.target_id.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let target = PlacedDatumTargetFeature::new();
        assert!(target.name.is_none());
        assert!(target.location.is_none());
    }

    #[test]
    fn test_set_and_get_location() {
        let mut target = PlacedDatumTargetFeature::new();
        target.set_location("point1".to_string());
        assert_eq!(target.get_location(), Some("point1"));
    }

    #[test]
    fn test_set_target_id() {
        let mut target = PlacedDatumTargetFeature::new();
        target.set_target_id("target_1".to_string());
        assert_eq!(target.get_target_id(), Some("target_1"));
    }
}
