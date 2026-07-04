// FILE: step_ap214_applied_security_classification_assignment.rs
// occt: StepAP214_AppliedSecurityClassificationAssignment

#[derive(Clone, Debug)]
pub struct SecurityClassificationItem {}

#[derive(Clone, Debug)]
pub struct AppliedSecurityClassificationAssignment {
    items: Vec<SecurityClassificationItem>,
}

impl AppliedSecurityClassificationAssignment {
    pub fn new() -> Self {
        AppliedSecurityClassificationAssignment {
            items: Vec::new(),
        }
    }

    pub fn init(&mut self, items: Vec<SecurityClassificationItem>) {
        self.items = items;
    }

    pub fn set_items(&mut self, items: Vec<SecurityClassificationItem>) {
        self.items = items;
    }

    pub fn items(&self) -> &[SecurityClassificationItem] {
        &self.items
    }

    pub fn items_value(&self, num: usize) -> Option<&SecurityClassificationItem> {
        if num > 0 && num <= self.items.len() {
            Some(&self.items[num - 1])
        } else {
            None
        }
    }

    pub fn nb_items(&self) -> usize {
        self.items.len()
    }
}

impl Default for AppliedSecurityClassificationAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let assignment = AppliedSecurityClassificationAssignment::new();
        assert_eq!(assignment.nb_items(), 0);
    }

    #[test]
    fn test_set_items() {
        let mut assignment = AppliedSecurityClassificationAssignment::new();
        let items = vec![SecurityClassificationItem {}];
        assignment.set_items(items);
        assert_eq!(assignment.nb_items(), 1);
    }
}
