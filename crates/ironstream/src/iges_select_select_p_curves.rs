// FILE: iges_select_select_p_curves.rs
// occt: IGESSelect_SelectPCurves

/// Selects PCurves (parametric curves) which lie on a face.
/// Operates in two modes: global (CompositeCurves not explored) or basic (all components listed).
pub struct IgesSelectSelectPCurves {
    basic: bool,
}

impl IgesSelectSelectPCurves {
    /// Creates a SelectPCurves selector.
    ///
    /// # Arguments
    /// - `basic`: true = lists all components of PCurves
    ///           false = lists the uppermost level definitions (stops at CompositeCurve)
    pub fn new(basic: bool) -> Self {
        IgesSelectSelectPCurves { basic }
    }

    /// Returns whether basic mode is enabled.
    pub fn is_basic(&self) -> bool {
        self.basic
    }

    /// Explores an entity to extract its contained PCurves.
    /// Only faces are explored; independent curves are ignored.
    ///
    /// # Arguments
    /// - `_level`: The exploration level
    /// - `_entity`: The entity to explore
    /// - `_is_face`: Whether the entity is a face
    ///
    /// Returns true if exploration yielded PCurves
    pub fn explore(&self, _level: i32, _is_face: bool) -> bool {
        // Real implementation would iterate through entity's PCurves
        _is_face
    }

    /// Returns the exploration criterium description.
    pub fn explore_label(&self) -> String {
        if self.basic {
            "Basic PCurves".to_string()
        } else {
            "Global PCurves".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_p_curves_basic_true() {
        let spc = IgesSelectSelectPCurves::new(true);
        assert!(spc.is_basic());
        assert_eq!(spc.explore_label(), "Basic PCurves".to_string());
    }

    #[test]
    fn test_select_p_curves_basic_false() {
        let spc = IgesSelectSelectPCurves::new(false);
        assert!(!spc.is_basic());
        assert_eq!(spc.explore_label(), "Global PCurves".to_string());
    }

    #[test]
    fn test_select_p_curves_explore_face() {
        let spc = IgesSelectSelectPCurves::new(true);
        assert!(spc.explore(0, true));
    }

    #[test]
    fn test_select_p_curves_explore_non_face() {
        let spc = IgesSelectSelectPCurves::new(true);
        assert!(!spc.explore(0, false));
    }
}
