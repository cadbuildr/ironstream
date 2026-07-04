// FILE: step_repr_shape_aspect_transition.rs
// occt: StepRepr_ShapeAspectTransition

/// Placeholder for ShapeAspect
#[derive(Clone, Debug, PartialEq)]
pub struct ShapeAspect {
    id: String,
}

/// Represents a transition relationship between shape aspects.
/// This is a specialized form of ShapeAspectRelationship.
pub struct ShapeAspectTransition {
    name: Option<String>,
    description: Option<String>,
    relating_shape_aspect: Option<ShapeAspect>,
    related_shape_aspect: Option<ShapeAspect>,
}

impl ShapeAspectTransition {
    /// Create a new ShapeAspectTransition
    pub fn new() -> Self {
        ShapeAspectTransition {
            name: None,
            description: None,
            relating_shape_aspect: None,
            related_shape_aspect: None,
        }
    }

    /// Initialize transition with name, optional description, and shape aspects
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

impl Default for ShapeAspectTransition {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let trans = ShapeAspectTransition::new();
        assert_eq!(trans.name(), None);
        assert!(!trans.has_description());
        assert_eq!(trans.relating_shape_aspect(), None);
        assert_eq!(trans.related_shape_aspect(), None);
    }

    #[test]
    fn test_init_with_description() {
        let mut trans = ShapeAspectTransition::new();
        let aspect1 = ShapeAspect {
            id: "aspect1".to_string(),
        };
        let aspect2 = ShapeAspect {
            id: "aspect2".to_string(),
        };
        trans.init(
            "transition_name".to_string(),
            true,
            Some("transition_description".to_string()),
            aspect1.clone(),
            aspect2.clone(),
        );
        assert_eq!(trans.name(), Some("transition_name"));
        assert!(trans.has_description());
        assert_eq!(trans.description(), Some("transition_description"));
        assert_eq!(trans.relating_shape_aspect(), Some(&aspect1));
        assert_eq!(trans.related_shape_aspect(), Some(&aspect2));
    }

    #[test]
    fn test_set_aspects() {
        let mut trans = ShapeAspectTransition::new();
        let aspect1 = ShapeAspect {
            id: "a1".to_string(),
        };
        let aspect2 = ShapeAspect {
            id: "a2".to_string(),
        };
        trans.set_relating_shape_aspect(aspect1.clone());
        trans.set_related_shape_aspect(aspect2.clone());
        assert_eq!(trans.relating_shape_aspect(), Some(&aspect1));
        assert_eq!(trans.related_shape_aspect(), Some(&aspect2));
    }
}
