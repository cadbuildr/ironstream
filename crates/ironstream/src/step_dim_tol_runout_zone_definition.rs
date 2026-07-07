// FILE: step_dim_tol_runout_zone_definition.rs
// occt: StepDimTol_RunoutZoneDefinition

pub struct RunoutZoneDefinition {
    pub name: Option<String>,
    pub zone_radius: Option<f64>,
}

impl RunoutZoneDefinition {
    pub fn new() -> Self {
        RunoutZoneDefinition {
            name: None,
            zone_radius: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_zone_radius(&mut self, radius: f64) {
        self.zone_radius = Some(radius);
    }

    pub fn get_zone_radius(&self) -> Option<f64> {
        self.zone_radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let zone = RunoutZoneDefinition::new();
        assert!(zone.name.is_none());
        assert!(zone.zone_radius.is_none());
    }

    #[test]
    fn test_set_zone_radius() {
        let mut zone = RunoutZoneDefinition::new();
        zone.set_zone_radius(2.5);
        assert_eq!(zone.get_zone_radius(), Some(2.5));
    }

    #[test]
    fn test_set_and_get_name() {
        let mut zone = RunoutZoneDefinition::new();
        zone.set_name("runout".to_string());
        assert_eq!(zone.get_name(), Some("runout"));
    }
}
