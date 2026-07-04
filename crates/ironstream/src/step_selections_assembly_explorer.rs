// FILE: step_selections_assembly_explorer.rs
// occt: STEPSelections_AssemblyExplorer

/// Explorer for STEP assembly structures
pub struct STEPSelections_AssemblyExplorer;

impl STEPSelections_AssemblyExplorer {
    pub fn new() -> Self {
        STEPSelections_AssemblyExplorer
    }
}

impl Default for STEPSelections_AssemblyExplorer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _explorer = STEPSelections_AssemblyExplorer::new();
    }
}
