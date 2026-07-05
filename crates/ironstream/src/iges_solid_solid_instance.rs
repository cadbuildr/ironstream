// FILE: iges_solid_solid_instance.rs
// occt: IGESSolid_SolidInstance

/// IGESSolid_SolidInstance represents a solid instance entity (Type 430, Form 0 or 1 for BREP).
/// This provides a mechanism for replicating a solid representation.
/// From IGES-5.3, Form may be 1 for a BREP, else 0 for Boolean Tree, Primitive, or other Solid Instance.
pub struct SolidInstance {
    /// The referenced solid entity
    entity: Option<String>,
    /// Type number (always 430)
    type_num: u32,
    /// Form number: 0 = non-BREP, 1 = BREP
    form_num: u8,
}

impl SolidInstance {
    /// Creates a new SolidInstance with default values
    pub fn new() -> Self {
        Self {
            entity: None,
            type_num: 430,
            form_num: 0,
        }
    }

    /// Initializes the SolidInstance with an entity reference
    /// Sets the type to 430 and form to 0 (non-BREP)
    pub fn init(&mut self, entity: String) {
        self.entity = Some(entity);
        self.type_num = 430;
        self.form_num = 0;
    }

    /// Returns whether this SolidInstance is for a BREP (Form number = 1)
    /// Default is false
    pub fn is_brep(&self) -> bool {
        self.form_num == 1
    }

    /// Sets or unsets the BREP status
    /// If true, sets form number to 1; otherwise sets to 0
    pub fn set_brep(&mut self, brep: bool) {
        self.form_num = if brep { 1 } else { 0 };
    }

    /// Returns the referenced solid entity
    pub fn entity(&self) -> Option<&str> {
        self.entity.as_deref()
    }

    /// Returns the type number (always 430)
    pub fn type_number(&self) -> u32 {
        self.type_num
    }

    /// Returns the form number
    pub fn form_number(&self) -> u8 {
        self.form_num
    }
}

impl Default for SolidInstance {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solid_instance_new() {
        let si = SolidInstance::new();
        assert_eq!(si.type_number(), 430);
        assert_eq!(si.form_number(), 0);
        assert!(!si.is_brep());
        assert_eq!(si.entity(), None);
    }

    #[test]
    fn test_solid_instance_init() {
        let mut si = SolidInstance::new();
        si.init("ENTITY_1".to_string());
        assert_eq!(si.entity(), Some("ENTITY_1"));
        assert_eq!(si.type_number(), 430);
        assert_eq!(si.form_number(), 0);
        assert!(!si.is_brep());
    }

    #[test]
    fn test_set_brep_true() {
        let mut si = SolidInstance::new();
        si.set_brep(true);
        assert!(si.is_brep());
        assert_eq!(si.form_number(), 1);
        assert_eq!(si.type_number(), 430);
    }

    #[test]
    fn test_set_brep_false() {
        let mut si = SolidInstance::new();
        si.set_brep(true);
        si.set_brep(false);
        assert!(!si.is_brep());
        assert_eq!(si.form_number(), 0);
    }

    #[test]
    fn test_default() {
        let si = SolidInstance::default();
        assert_eq!(si.type_number(), 430);
        assert_eq!(si.form_number(), 0);
        assert!(!si.is_brep());
    }
}
