// FILE: step_repr_shape_aspect_deriving_relationship.rs
// occt: StepRepr_ShapeAspectDerivingRelationship

/// Placeholder for ShapeAspect
#[derive(Clone, Debug, PartialEq)]
pub struct ShapeAspect {
    id: String,
}

/// Represents a deriving relationship between shape aspects for dimensional tolerances.
pub struct ShapeAspectDerivingRelationship {
    name: Option<String>,
    relating_shape_aspect: Option<ShapeAspect>,
    related_shape_aspect: Option<ShapeAspect>,
}

impl ShapeAspectDerivingRelationship {
    /// Create a new ShapeAspectDerivingRelationship
    pub fn new() -> Self {
        ShapeAspectDerivingRelationship {
            name: None,
            relating_shape_aspect: None,
            related_shape_aspect: None,
        }
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get the relating shape aspect
    pub fn relating_shape_aspect(&self) -> Option<&ShapeAspect> {
        self.relating_shape_aspect.as_ref()
    }

    /// Set the relating shape aspect
    pub fn set_relating_shape_aspect(&mut self, aspect: ShapeAspect) {
        self.relating_shape_aspect = Some(aspect);
    }

    /// Get the related shape aspect
    pub fn related_shape_aspect(&self) -> Option<&ShapeAspect> {
        self.related_shape_aspect.as_ref()
    }

    /// Set the related shape aspect
    pub fn set_related_shape_aspect(&mut self, aspect: ShapeAspect) {
        self.related_shape_aspect = Some(aspect);
    }
}

impl Default for ShapeAspectDerivingRelationship {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let rel = ShapeAspectDerivingRelationship::new();
        assert_eq!(rel.name(), None);
        assert_eq!(rel.relating_shape_aspect(), None);
        assert_eq!(rel.related_shape_aspect(), None);
    }

    #[test]
    fn test_set_and_get_name() {
        let mut rel = ShapeAspectDerivingRelationship::new();
        rel.set_name("test_rel".to_string());
        assert_eq!(rel.name(), Some("test_rel"));
    }

    #[test]
    fn test_set_aspects() {
        let mut rel = ShapeAspectDerivingRelationship::new();
        let aspect1 = ShapeAspect {
            id: "aspect1".to_string(),
        };
        let aspect2 = ShapeAspect {
            id: "aspect2".to_string(),
        };
        rel.set_relating_shape_aspect(aspect1.clone());
        rel.set_related_shape_aspect(aspect2.clone());
        assert_eq!(rel.relating_shape_aspect(), Some(&aspect1));
        assert_eq!(rel.related_shape_aspect(), Some(&aspect2));
    }
}
