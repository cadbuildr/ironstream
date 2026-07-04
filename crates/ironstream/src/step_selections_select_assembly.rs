// FILE: step_selections_select_assembly.rs
// occt: STEPSelections_SelectAssembly

/// Selector for assemblies in STEP
pub struct STEPSelections_SelectAssembly;

impl STEPSelections_SelectAssembly {
    pub fn new() -> Self {
        STEPSelections_SelectAssembly
    }
}

impl Default for STEPSelections_SelectAssembly {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _selector = STEPSelections_SelectAssembly::new();
    }
}
