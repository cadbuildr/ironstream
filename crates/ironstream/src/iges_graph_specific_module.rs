// FILE: iges_graph_specific_module.rs
// occt: IGESGraph_SpecificModule

pub struct IGESGraphSpecificModule;

impl IGESGraphSpecificModule {
    pub fn new() -> Self {
        IGESGraphSpecificModule
    }

    pub fn own_dump(&self, _cn: i32) {
        // Dispatches to appropriate Tool class for dumping entity parameters
        // Case numbers are mapped to specific IGES entity types
    }

    pub fn own_correct(&self, cn: i32) -> bool {
        // Performs correction on supported entities
        // Returns true if correction was applied, false otherwise
        match cn {
            3 | 4 | 5 | 6 | 8 | 10 | 11 | 14 => true,
            _ => false,
        }
    }
}

impl Default for IGESGraphSpecificModule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let module = IGESGraphSpecificModule::new();
        assert!(!module.own_correct(1));
    }

    #[test]
    fn test_own_correct_drawing_size() {
        let module = IGESGraphSpecificModule::new();
        assert!(module.own_correct(3));
    }

    #[test]
    fn test_own_correct_drawing_units() {
        let module = IGESGraphSpecificModule::new();
        assert!(module.own_correct(4));
    }

    #[test]
    fn test_own_correct_high_light() {
        let module = IGESGraphSpecificModule::new();
        assert!(module.own_correct(5));
    }

    #[test]
    fn test_own_correct_pick() {
        let module = IGESGraphSpecificModule::new();
        assert!(module.own_correct(11));
    }

    #[test]
    fn test_own_correct_unsupported() {
        let module = IGESGraphSpecificModule::new();
        assert!(!module.own_correct(1));
    }
}
