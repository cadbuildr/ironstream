// FILE: step_selections_assembly_link.rs
// occt: STEPSelections_AssemblyLink

/// Link in assembly structure for STEP selections
pub struct STEPSelections_AssemblyLink;

impl STEPSelections_AssemblyLink {
    pub fn new() -> Self {
        STEPSelections_AssemblyLink
    }
}

impl Default for STEPSelections_AssemblyLink {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _link = STEPSelections_AssemblyLink::new();
    }
}
