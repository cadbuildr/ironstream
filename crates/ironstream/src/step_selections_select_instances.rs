// FILE: step_selections_select_instances.rs
// occt: STEPSelections_SelectInstances

/// Selector for instances in STEP
pub struct STEPSelections_SelectInstances;

impl STEPSelections_SelectInstances {
    pub fn new() -> Self {
        STEPSelections_SelectInstances
    }
}

impl Default for STEPSelections_SelectInstances {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _selector = STEPSelections_SelectInstances::new();
    }
}
