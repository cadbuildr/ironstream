// FILE: step_selections_assembly_component.rs
// occt: STEPSelections_AssemblyComponent

/// Assembly component representation for STEP selections
pub struct STEPSelections_AssemblyComponent {
    id: i32,
}

impl STEPSelections_AssemblyComponent {
    pub fn new() -> Self {
        STEPSelections_AssemblyComponent { id: 0 }
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }
}

impl Default for STEPSelections_AssemblyComponent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let comp = STEPSelections_AssemblyComponent::new();
        assert_eq!(comp.get_id(), 0);
    }

    #[test]
    fn test_set_id() {
        let mut comp = STEPSelections_AssemblyComponent::new();
        comp.set_id(123);
        assert_eq!(comp.get_id(), 123);
    }
}
