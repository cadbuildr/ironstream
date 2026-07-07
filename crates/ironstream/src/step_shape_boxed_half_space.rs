// FILE: step_shape_boxed_half_space.rs
// occt: StepShape_BoxedHalfSpace

/// Placeholder for Surface
#[derive(Clone, Debug, PartialEq)]
pub struct Surface {
    id: String,
}

/// Placeholder for BoxDomain
#[derive(Clone, Debug, PartialEq)]
pub struct BoxDomain {
    id: String,
}

/// Represents a boxed half space in STEP
pub struct BoxedHalfSpace {
    name: Option<String>,
    base_surface: Option<Surface>,
    agreement_flag: bool,
    enclosure: Option<BoxDomain>,
}

impl BoxedHalfSpace {
    /// Create a new BoxedHalfSpace
    pub fn new() -> Self {
        BoxedHalfSpace {
            name: None,
            base_surface: None,
            agreement_flag: false,
            enclosure: None,
        }
    }

    /// Initialize with name, base surface, agreement flag, and enclosure
    pub fn init(
        &mut self,
        name: String,
        base_surface: Surface,
        agreement_flag: bool,
        enclosure: BoxDomain,
    ) {
        self.name = Some(name);
        self.base_surface = Some(base_surface);
        self.agreement_flag = agreement_flag;
        self.enclosure = Some(enclosure);
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get the base surface
    pub fn base_surface(&self) -> Option<&Surface> {
        self.base_surface.as_ref()
    }

    /// Get the agreement flag
    pub fn agreement_flag(&self) -> bool {
        self.agreement_flag
    }

    /// Set the enclosure
    pub fn set_enclosure(&mut self, enclosure: BoxDomain) {
        self.enclosure = Some(enclosure);
    }

    /// Get the enclosure
    pub fn enclosure(&self) -> Option<&BoxDomain> {
        self.enclosure.as_ref()
    }
}

impl Default for BoxedHalfSpace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let half_space = BoxedHalfSpace::new();
        assert_eq!(half_space.name(), None);
        assert_eq!(half_space.agreement_flag(), false);
        assert_eq!(half_space.enclosure(), None);
    }

    #[test]
    fn test_init() {
        let mut half_space = BoxedHalfSpace::new();
        let surface = Surface { id: "surf".to_string() };
        let enclosure = BoxDomain {
            id: "box".to_string(),
        };
        half_space.init(
            "BoxedHalfSpace1".to_string(),
            surface.clone(),
            true,
            enclosure.clone(),
        );
        assert_eq!(half_space.name(), Some("BoxedHalfSpace1"));
        assert_eq!(half_space.base_surface(), Some(&surface));
        assert!(half_space.agreement_flag());
        assert_eq!(half_space.enclosure(), Some(&enclosure));
    }

    #[test]
    fn test_set_enclosure() {
        let mut half_space = BoxedHalfSpace::new();
        let enclosure = BoxDomain {
            id: "test_box".to_string(),
        };
        half_space.set_enclosure(enclosure.clone());
        assert_eq!(half_space.enclosure(), Some(&enclosure));
    }
}
