// FILE: step_repr_representation_or_representation_reference.rs
// occt: StepRepr_RepresentationOrRepresentationReference

/// Placeholder for Representation
#[derive(Clone, Debug, PartialEq)]
pub struct Representation {
    name: String,
}

/// Placeholder for RepresentationReference
#[derive(Clone, Debug, PartialEq)]
pub struct RepresentationReference {
    id: String,
}

/// SELECT type that can contain either a Representation or a RepresentationReference
#[derive(Clone, Debug, PartialEq)]
pub enum RepresentationOrRepresentationReference {
    Representation(Representation),
    RepresentationReference(RepresentationReference),
}

impl RepresentationOrRepresentationReference {
    /// Create a new selector
    pub fn new() -> Self {
        RepresentationOrRepresentationReference::Representation(Representation {
            name: String::new(),
        })
    }

    /// Get the case number (1 for Representation, 2 for RepresentationReference)
    pub fn case_num(&self) -> i32 {
        match self {
            RepresentationOrRepresentationReference::Representation(_) => 1,
            RepresentationOrRepresentationReference::RepresentationReference(_) => 2,
        }
    }

    /// Get as Representation if applicable
    pub fn representation(&self) -> Option<&Representation> {
        match self {
            RepresentationOrRepresentationReference::Representation(r) => Some(r),
            _ => None,
        }
    }

    /// Get as RepresentationReference if applicable
    pub fn representation_reference(&self) -> Option<&RepresentationReference> {
        match self {
            RepresentationOrRepresentationReference::RepresentationReference(r) => Some(r),
            _ => None,
        }
    }
}

impl Default for RepresentationOrRepresentationReference {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_representation() {
        let rep = Representation {
            name: "test_rep".to_string(),
        };
        let sel = RepresentationOrRepresentationReference::Representation(rep.clone());
        assert_eq!(sel.case_num(), 1);
        assert_eq!(sel.representation(), Some(&rep));
        assert_eq!(sel.representation_reference(), None);
    }

    #[test]
    fn test_create_representation_reference() {
        let ref_rep = RepresentationReference {
            id: "ref_001".to_string(),
        };
        let sel = RepresentationOrRepresentationReference::RepresentationReference(ref_rep.clone());
        assert_eq!(sel.case_num(), 2);
        assert_eq!(sel.representation_reference(), Some(&ref_rep));
        assert_eq!(sel.representation(), None);
    }

    #[test]
    fn test_default_is_representation() {
        let sel = RepresentationOrRepresentationReference::default();
        assert_eq!(sel.case_num(), 1);
        assert!(sel.representation().is_some());
    }
}
