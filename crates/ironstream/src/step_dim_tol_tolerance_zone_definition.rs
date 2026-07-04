// FILE: step_dim_tol_tolerance_zone_definition.rs
// occt: StepDimTol_ToleranceZoneDefinition

pub struct ToleranceZoneDefinition {
    pub name: Option<String>,
    pub zone_depth: Option<f64>,
}

impl ToleranceZoneDefinition {
    pub fn new() -> Self {
        ToleranceZoneDefinition {
            name: None,
            zone_depth: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_zone_depth(&mut self, depth: f64) {
        self.zone_depth = Some(depth);
    }

    pub fn get_zone_depth(&self) -> Option<f64> {
        self.zone_depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let def = ToleranceZoneDefinition::new();
        assert!(def.name.is_none());
        assert!(def.zone_depth.is_none());
    }

    #[test]
    fn test_set_zone_depth() {
        let mut def = ToleranceZoneDefinition::new();
        def.set_zone_depth(2.5);
        assert_eq!(def.get_zone_depth(), Some(2.5));
    }

    #[test]
    fn test_set_and_get_name() {
        let mut def = ToleranceZoneDefinition::new();
        def.set_name("zone_def".to_string());
        assert_eq!(def.get_name(), Some("zone_def"));
    }
}
