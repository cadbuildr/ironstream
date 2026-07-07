// FILE: step_ap214_rep_item_group.rs
// occt: StepAP214_RepItemGroup

/// Representation of STEP AP214 RepItemGroup entity.
#[derive(Clone, Debug)]
pub struct RepItemGroup {
    // Placeholder
}

impl RepItemGroup {
    pub fn new() -> Self {
        RepItemGroup {}
    }
}

impl Default for RepItemGroup {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _group = RepItemGroup::new();
    }
}
