// FILE: step_repr_representation_relationship_with_transformation.rs
// occt: StepRepr_RepresentationRelationshipWithTransformation

/// Placeholder for Representation
#[derive(Clone, Debug, PartialEq)]
pub struct Representation {
    name: String,
}

/// Placeholder for Transformation
#[derive(Clone, Debug, PartialEq)]
pub struct Transformation {
    matrix: [f64; 16],
}

/// Represents a representation relationship with an associated transformation (STEP).
pub struct RepresentationRelationshipWithTransformation {
    name: Option<String>,
    description: Option<String>,
    rep1: Option<Representation>,
    rep2: Option<Representation>,
    transformation_operator: Option<Transformation>,
}

impl RepresentationRelationshipWithTransformation {
    /// Create a new RepresentationRelationshipWithTransformation
    pub fn new() -> Self {
        RepresentationRelationshipWithTransformation {
            name: None,
            description: None,
            rep1: None,
            rep2: None,
            transformation_operator: None,
        }
    }

    /// Initialize with name, description, representations, and transformation
    pub fn init(
        &mut self,
        name: String,
        description: String,
        rep1: Representation,
        rep2: Representation,
        transformation: Transformation,
    ) {
        self.name = Some(name);
        self.description = Some(description);
        self.rep1 = Some(rep1);
        self.rep2 = Some(rep2);
        self.transformation_operator = Some(transformation);
    }

    /// Get the transformation operator
    pub fn transformation_operator(&self) -> Option<&Transformation> {
        self.transformation_operator.as_ref()
    }

    /// Set the transformation operator
    pub fn set_transformation_operator(&mut self, transformation: Transformation) {
        self.transformation_operator = Some(transformation);
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get the description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Set the description
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    /// Get the first representation
    pub fn rep1(&self) -> Option<&Representation> {
        self.rep1.as_ref()
    }

    /// Set the first representation
    pub fn set_rep1(&mut self, rep1: Representation) {
        self.rep1 = Some(rep1);
    }

    /// Get the second representation
    pub fn rep2(&self) -> Option<&Representation> {
        self.rep2.as_ref()
    }

    /// Set the second representation
    pub fn set_rep2(&mut self, rep2: Representation) {
        self.rep2 = Some(rep2);
    }
}

impl Default for RepresentationRelationshipWithTransformation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let rel = RepresentationRelationshipWithTransformation::new();
        assert_eq!(rel.name(), None);
        assert_eq!(rel.description(), None);
        assert_eq!(rel.rep1(), None);
        assert_eq!(rel.rep2(), None);
        assert_eq!(rel.transformation_operator(), None);
    }

    #[test]
    fn test_init() {
        let mut rel = RepresentationRelationshipWithTransformation::new();
        let rep1 = Representation {
            name: "rep1".to_string(),
        };
        let rep2 = Representation {
            name: "rep2".to_string(),
        };
        let trans = Transformation {
            matrix: [1.0; 16],
        };
        rel.init(
            "test_rel".to_string(),
            "test_desc".to_string(),
            rep1.clone(),
            rep2.clone(),
            trans.clone(),
        );
        assert_eq!(rel.name(), Some("test_rel"));
        assert_eq!(rel.description(), Some("test_desc"));
        assert_eq!(rel.rep1(), Some(&rep1));
        assert_eq!(rel.rep2(), Some(&rep2));
        assert_eq!(rel.transformation_operator(), Some(&trans));
    }

    #[test]
    fn test_set_and_get_transformation() {
        let mut rel = RepresentationRelationshipWithTransformation::new();
        let trans = Transformation {
            matrix: [2.0; 16],
        };
        rel.set_transformation_operator(trans.clone());
        assert_eq!(rel.transformation_operator(), Some(&trans));
    }
}
