// FILE: step_selections_select_derived.rs
// occt: STEPSelections_SelectDerived

/// Selector for derived entities in STEP
pub struct STEPSelections_SelectDerived;

impl STEPSelections_SelectDerived {
    pub fn new() -> Self {
        STEPSelections_SelectDerived
    }
}

impl Default for STEPSelections_SelectDerived {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _selector = STEPSelections_SelectDerived::new();
    }
}
