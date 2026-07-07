// FILE: step_shape_half_space_solid.rs
// occt: StepShape_HalfSpaceSolid

//! Representation of STEP entity HalfSpaceSolid

#[derive(Clone, Debug)]
pub struct HalfSpaceSolid {
    name: String,
    base_surface: Option<String>, // Placeholder for Surface handle
    agreement_flag: bool,
}

impl HalfSpaceSolid {
    /// Returns a HalfSpaceSolid
    pub fn new() -> Self {
        HalfSpaceSolid {
            name: String::new(),
            base_surface: None,
            agreement_flag: false,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, base_surface: Option<String>, agreement_flag: bool) {
        self.name = name;
        self.base_surface = base_surface;
        self.agreement_flag = agreement_flag;
    }

    /// Set BaseSurface
    pub fn set_base_surface(&mut self, surface: Option<String>) {
        self.base_surface = surface;
    }

    /// Returns BaseSurface
    pub fn base_surface(&self) -> &Option<String> {
        &self.base_surface
    }

    /// Set AgreementFlag
    pub fn set_agreement_flag(&mut self, flag: bool) {
        self.agreement_flag = flag;
    }

    /// Returns AgreementFlag
    pub fn agreement_flag(&self) -> bool {
        self.agreement_flag
    }

    /// Returns name field
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set name field
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for HalfSpaceSolid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let hss = HalfSpaceSolid::new();
        assert_eq!(hss.name(), "");
        assert!(!hss.agreement_flag());
        assert!(hss.base_surface().is_none());
    }

    #[test]
    fn test_init() {
        let mut hss = HalfSpaceSolid::new();
        hss.init("HalfSpace1".to_string(), Some("surf1".to_string()), true);
        assert_eq!(hss.name(), "HalfSpace1");
        assert!(hss.agreement_flag());
    }

    #[test]
    fn test_set_agreement_flag() {
        let mut hss = HalfSpaceSolid::new();
        hss.set_agreement_flag(true);
        assert!(hss.agreement_flag());
    }

    #[test]
    fn test_set_base_surface() {
        let mut hss = HalfSpaceSolid::new();
        hss.set_base_surface(Some("surface1".to_string()));
        assert_eq!(hss.base_surface(), &Some("surface1".to_string()));
    }
}
