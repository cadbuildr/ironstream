// FILE: step_dim_tol_projected_zone_definition.rs
// occt: StepDimTol_ProjectedZoneDefinition

pub struct ProjectedZoneDefinition {
    pub name: Option<String>,
    pub projection_length: Option<f64>,
}

impl ProjectedZoneDefinition {
    pub fn new() -> Self {
        ProjectedZoneDefinition {
            name: None,
            projection_length: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_projection_length(&mut self, length: f64) {
        self.projection_length = Some(length);
    }

    pub fn get_projection_length(&self) -> Option<f64> {
        self.projection_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let proj = ProjectedZoneDefinition::new();
        assert!(proj.name.is_none());
        assert!(proj.projection_length.is_none());
    }

    #[test]
    fn test_set_projection_length() {
        let mut proj = ProjectedZoneDefinition::new();
        proj.set_projection_length(10.0);
        assert_eq!(proj.get_projection_length(), Some(10.0));
    }

    #[test]
    fn test_set_and_get_name() {
        let mut proj = ProjectedZoneDefinition::new();
        proj.set_name("projected".to_string());
        assert_eq!(proj.get_name(), Some("projected"));
    }
}
