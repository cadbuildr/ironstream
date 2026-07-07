// FILE: step_shape_angular_location.rs
// occt: StepShape_AngularLocation

/// Enumeration for angle relator types
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AngleRelator {
    Equal,
    Large,
    Small,
}

/// Placeholder for ShapeAspect
#[derive(Clone, Debug, PartialEq)]
pub struct ShapeAspect {
    id: String,
}

/// Represents an angular location in STEP
pub struct AngularLocation {
    name: Option<String>,
    description: Option<String>,
    relating_shape_aspect: Option<ShapeAspect>,
    related_shape_aspect: Option<ShapeAspect>,
    angle_selection: Option<AngleRelator>,
}

impl AngularLocation {
    /// Create a new AngularLocation
    pub fn new() -> Self {
        AngularLocation {
            name: None,
            description: None,
            relating_shape_aspect: None,
            related_shape_aspect: None,
            angle_selection: None,
        }
    }

    /// Initialize with all fields
    pub fn init(
        &mut self,
        name: String,
        has_description: bool,
        description: Option<String>,
        relating_shape_aspect: ShapeAspect,
        related_shape_aspect: ShapeAspect,
        angle_selection: AngleRelator,
    ) {
        self.name = Some(name);
        if has_description {
            self.description = description;
        }
        self.relating_shape_aspect = Some(relating_shape_aspect);
        self.related_shape_aspect = Some(related_shape_aspect);
        self.angle_selection = Some(angle_selection);
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get the angle selection
    pub fn angle_selection(&self) -> Option<AngleRelator> {
        self.angle_selection
    }

    /// Set the angle selection
    pub fn set_angle_selection(&mut self, angle_selection: AngleRelator) {
        self.angle_selection = Some(angle_selection);
    }

    /// Get relating shape aspect
    pub fn relating_shape_aspect(&self) -> Option<&ShapeAspect> {
        self.relating_shape_aspect.as_ref()
    }

    /// Get related shape aspect
    pub fn related_shape_aspect(&self) -> Option<&ShapeAspect> {
        self.related_shape_aspect.as_ref()
    }
}

impl Default for AngularLocation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let ang = AngularLocation::new();
        assert_eq!(ang.name(), None);
        assert_eq!(ang.angle_selection(), None);
    }

    #[test]
    fn test_init() {
        let mut ang = AngularLocation::new();
        let rel = ShapeAspect { id: "rel".to_string() };
        let related = ShapeAspect {
            id: "related".to_string(),
        };
        ang.init(
            "AngularLoc1".to_string(),
            true,
            Some("Description".to_string()),
            rel.clone(),
            related.clone(),
            AngleRelator::Equal,
        );
        assert_eq!(ang.name(), Some("AngularLoc1"));
        assert_eq!(ang.angle_selection(), Some(AngleRelator::Equal));
        assert_eq!(ang.relating_shape_aspect(), Some(&rel));
        assert_eq!(ang.related_shape_aspect(), Some(&related));
    }

    #[test]
    fn test_set_angle_selection() {
        let mut ang = AngularLocation::new();
        ang.set_angle_selection(AngleRelator::Large);
        assert_eq!(ang.angle_selection(), Some(AngleRelator::Large));
    }
}
