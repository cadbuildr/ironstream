// FILE: step_dim_tol_non_uniform_zone_definition.rs
// occt: StepDimTol_NonUniformZoneDefinition

pub struct NonUniformZoneDefinition {
    pub name: Option<String>,
    pub zone_depth: Option<f64>,
}

impl NonUniformZoneDefinition {
    pub fn new() -> Self {
        NonUniformZoneDefinition {
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
        let zone = NonUniformZoneDefinition::new();
        assert!(zone.name.is_none());
        assert!(zone.zone_depth.is_none());
    }

    #[test]
    fn test_set_zone_depth() {
        let mut zone = NonUniformZoneDefinition::new();
        zone.set_zone_depth(5.5);
        assert_eq!(zone.get_zone_depth(), Some(5.5));
    }

    #[test]
    fn test_set_and_get_name() {
        let mut zone = NonUniformZoneDefinition::new();
        zone.set_name("zone1".to_string());
        assert_eq!(zone.get_name(), Some("zone1"));
    }
}
