// FILE: step_ap214_applied_external_identification_assignment.rs
// occt: StepAP214_AppliedExternalIdentificationAssignment

#[derive(Clone, Debug)]
pub struct ExternalIdentificationItem {
    // Placeholder
}

/// Representation of STEP AP214 AppliedExternalIdentificationAssignment entity.
#[derive(Clone, Debug)]
pub struct AppliedExternalIdentificationAssignment {
    items: Vec<ExternalIdentificationItem>,
}

impl AppliedExternalIdentificationAssignment {
    pub fn new() -> Self {
        AppliedExternalIdentificationAssignment {
            items: Vec::new(),
        }
    }

    pub fn init(&mut self, items: Vec<ExternalIdentificationItem>) {
        self.items = items;
    }

    pub fn set_items(&mut self, items: Vec<ExternalIdentificationItem>) {
        self.items = items;
    }

    pub fn items(&self) -> &[ExternalIdentificationItem] {
        &self.items
    }

    pub fn nb_items(&self) -> usize {
        self.items.len()
    }
}

impl Default for AppliedExternalIdentificationAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let assignment = AppliedExternalIdentificationAssignment::new();
        assert_eq!(assignment.nb_items(), 0);
    }

    #[test]
    fn test_set_items() {
        let mut assignment = AppliedExternalIdentificationAssignment::new();
        let items = vec![ExternalIdentificationItem {}];
        assignment.set_items(items);
        assert_eq!(assignment.nb_items(), 1);
    }
}
