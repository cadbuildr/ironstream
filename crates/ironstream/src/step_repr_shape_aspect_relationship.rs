// FILE: step_repr_shape_aspect_relationship.rs
// occt: StepRepr_ShapeAspectRelationship

/// Placeholder for ShapeAspect
#[derive(Clone, Debug, PartialEq)]
pub struct ShapeAspect {
    id: String,
}

/// Represents a relationship between shape aspects in STEP.
pub struct ShapeAspectRelationship {
    name: Option<String>,
    description: Option<String>,
    relating_shape_aspect: Option<ShapeAspect>,
    related_shape_aspect: Option<ShapeAspect>,
}

impl ShapeAspectRelationship {
    /// Create a new ShapeAspectRelationship
    pub fn new() -> Self {
        ShapeAspectRelationship {
            name: None,
            description: None,
            relating_shape_aspect: None,
            related_shape_aspect: None,
        }
    }

    /// Initialize relationship with name, optional description, and shape aspects
    pub fn init(
        &mut self,
        name: String,
        has_description: bool,
        description: Option<String>,
        relating_shape_aspect: ShapeAspect,
        related_shape_aspect: ShapeAspect,
    ) {
        self.name = Some(name);
        if has_description {
            self.description = description;
        }
        self.relating_shape_aspect = Some(relating_shape_aspect);
        self.related_shape_aspect = Some(related_shape_aspect);
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

    /// Check if description is set
    pub fn has_description(&self) -> bool {
        self.description.is_some()
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

impl Default for ShapeAspectRelationship {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let rel = ShapeAspectRelationship::new();
        assert_eq!(rel.name(), None);
        assert!(!rel.has_description());
        assert_eq!(rel.relating_shape_aspect(), None);
        assert_eq!(rel.related_shape_aspect(), None);
    }

    #[test]
    fn test_init_with_description() {
        let mut rel = ShapeAspectRelationship::new();
        let aspect1 = ShapeAspect {
            id: "a1".to_string(),
        };
        let aspect2 = ShapeAspect {
            id: "a2".to_string(),
        };
        rel.init(
            "rel_name".to_string(),
            true,
            Some("rel_desc".to_string()),
            aspect1.clone(),
            aspect2.clone(),
        );
        assert_eq!(rel.name(), Some("rel_name"));
        assert!(rel.has_description());
        assert_eq!(rel.description(), Some("rel_desc"));
        assert_eq!(rel.relating_shape_aspect(), Some(&aspect1));
        assert_eq!(rel.related_shape_aspect(), Some(&aspect2));
    }

    #[test]
    fn test_init_without_description() {
        let mut rel = ShapeAspectRelationship::new();
        let aspect1 = ShapeAspect {
            id: "a1".to_string(),
        };
        let aspect2 = ShapeAspect {
            id: "a2".to_string(),
        };
        rel.init("name".to_string(), false, None, aspect1, aspect2);
        assert!(!rel.has_description());
    }

    #[test]
    fn test_set_aspects() {
        let mut rel = ShapeAspectRelationship::new();
        let aspect1 = ShapeAspect {
            id: "test1".to_string(),
        };
        let aspect2 = ShapeAspect {
            id: "test2".to_string(),
        };
        rel.set_relating_shape_aspect(aspect1.clone());
        rel.set_related_shape_aspect(aspect2.clone());
        assert_eq!(rel.relating_shape_aspect(), Some(&aspect1));
        assert_eq!(rel.related_shape_aspect(), Some(&aspect2));
    }
}
