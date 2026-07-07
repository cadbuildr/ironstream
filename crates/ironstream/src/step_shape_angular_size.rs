// FILE: step_shape_angular_size.rs
// occt: StepShape_AngularSize

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

/// Represents an angular size in STEP
pub struct AngularSize {
    name: Option<String>,
    applies_to: Option<ShapeAspect>,
    angle_selection: Option<AngleRelator>,
}

impl AngularSize {
    /// Create a new AngularSize
    pub fn new() -> Self {
        AngularSize {
            name: None,
            applies_to: None,
            angle_selection: None,
        }
    }

    /// Initialize with all fields
    pub fn init(
        &mut self,
        applies_to: ShapeAspect,
        name: String,
        angle_selection: AngleRelator,
    ) {
        self.applies_to = Some(applies_to);
        self.name = Some(name);
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

    /// Get the applies to
    pub fn applies_to(&self) -> Option<&ShapeAspect> {
        self.applies_to.as_ref()
    }

    /// Set the applies to
    pub fn set_applies_to(&mut self, aspect: ShapeAspect) {
        self.applies_to = Some(aspect);
    }

    /// Get the angle selection
    pub fn angle_selection(&self) -> Option<AngleRelator> {
        self.angle_selection
    }

    /// Set the angle selection
    pub fn set_angle_selection(&mut self, angle_selection: AngleRelator) {
        self.angle_selection = Some(angle_selection);
    }
}

impl Default for AngularSize {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let size = AngularSize::new();
        assert_eq!(size.name(), None);
        assert_eq!(size.angle_selection(), None);
    }

    #[test]
    fn test_init() {
        let mut size = AngularSize::new();
        let aspect = ShapeAspect { id: "asp".to_string() };
        size.init(aspect.clone(), "AngularSize1".to_string(), AngleRelator::Small);
        assert_eq!(size.name(), Some("AngularSize1"));
        assert_eq!(size.applies_to(), Some(&aspect));
        assert_eq!(size.angle_selection(), Some(AngleRelator::Small));
    }

    #[test]
    fn test_set_angle_selection() {
        let mut size = AngularSize::new();
        size.set_angle_selection(AngleRelator::Large);
        assert_eq!(size.angle_selection(), Some(AngleRelator::Large));
    }
}
