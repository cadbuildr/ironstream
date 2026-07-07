// FILE: step_fea_curve3d_element_representation.rs
// occt: StepFEA_Curve3dElementRepresentation

/// Representation of STEP entity Curve3dElementRepresentation.
#[derive(Clone)]
pub struct Curve3dElementRepresentation {
    name: Option<String>,
    definition: Option<String>,
    property: Option<String>,
}

impl Curve3dElementRepresentation {
    pub fn new() -> Self {
        Self {
            name: None,
            definition: None,
            property: None,
        }
    }

    pub fn init(
        &mut self,
        name: Option<String>,
        definition: Option<String>,
        property: Option<String>,
    ) {
        self.name = name;
        self.definition = definition;
        self.property = property;
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_name(&mut self, n: Option<String>) {
        self.name = n;
    }

    pub fn definition(&self) -> Option<&str> {
        self.definition.as_deref()
    }

    pub fn set_definition(&mut self, d: Option<String>) {
        self.definition = d;
    }

    pub fn property(&self) -> Option<&str> {
        self.property.as_deref()
    }

    pub fn set_property(&mut self, p: Option<String>) {
        self.property = p;
    }
}

impl Default for Curve3dElementRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let rep = Curve3dElementRepresentation::new();
        assert!(rep.name().is_none());
    }

    #[test]
    fn test_init() {
        let mut rep = Curve3dElementRepresentation::new();
        rep.init(
            Some("Rep1".to_string()),
            Some("Definition".to_string()),
            Some("Property".to_string()),
        );

        assert_eq!(rep.name(), Some("Rep1"));
        assert_eq!(rep.definition(), Some("Definition"));
    }
}
