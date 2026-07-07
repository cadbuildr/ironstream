// FILE: step_dim_tol_geometric_tolerance_relationship.rs
// occt: StepDimTol_GeometricToleranceRelationship

pub struct GeometricToleranceRelationship {
    pub name: Option<String>,
    pub description: Option<String>,
    pub relating_geometric_tolerance: Option<String>,
    pub related_geometric_tolerance: Option<String>,
}

impl GeometricToleranceRelationship {
    pub fn new() -> Self {
        GeometricToleranceRelationship {
            name: None,
            description: None,
            relating_geometric_tolerance: None,
            related_geometric_tolerance: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn set_relating_geometric_tolerance(&mut self, tol: String) {
        self.relating_geometric_tolerance = Some(tol);
    }

    pub fn get_relating_geometric_tolerance(&self) -> Option<&str> {
        self.relating_geometric_tolerance.as_deref()
    }

    pub fn set_related_geometric_tolerance(&mut self, tol: String) {
        self.related_geometric_tolerance = Some(tol);
    }

    pub fn get_related_geometric_tolerance(&self) -> Option<&str> {
        self.related_geometric_tolerance.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let rel = GeometricToleranceRelationship::new();
        assert!(rel.name.is_none());
        assert!(rel.relating_geometric_tolerance.is_none());
    }

    #[test]
    fn test_set_and_get_name() {
        let mut rel = GeometricToleranceRelationship::new();
        rel.set_name("rel1".to_string());
        assert_eq!(rel.get_name(), Some("rel1"));
    }

    #[test]
    fn test_set_tolerances() {
        let mut rel = GeometricToleranceRelationship::new();
        rel.set_relating_geometric_tolerance("tol1".to_string());
        rel.set_related_geometric_tolerance("tol2".to_string());
        assert_eq!(rel.get_relating_geometric_tolerance(), Some("tol1"));
        assert_eq!(rel.get_related_geometric_tolerance(), Some("tol2"));
    }
}
