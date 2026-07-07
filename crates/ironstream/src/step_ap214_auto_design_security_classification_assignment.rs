// FILE: step_ap214_auto_design_security_classification_assignment.rs
// occt: StepAP214_AutoDesignSecurityClassificationAssignment

#[derive(Clone, Debug)]
pub struct Approval {}

#[derive(Clone, Debug)]
pub struct AutoDesignSecurityClassificationAssignment {
    items: Vec<Approval>,
}

impl AutoDesignSecurityClassificationAssignment {
    pub fn new() -> Self {
        AutoDesignSecurityClassificationAssignment {
            items: Vec::new(),
        }
    }

    pub fn init(&mut self, items: Vec<Approval>) {
        self.items = items;
    }

    pub fn set_items(&mut self, items: Vec<Approval>) {
        self.items = items;
    }

    pub fn items(&self) -> &[Approval] {
        &self.items
    }

    pub fn items_value(&self, num: usize) -> Option<&Approval> {
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

impl Default for AutoDesignSecurityClassificationAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let assignment = AutoDesignSecurityClassificationAssignment::new();
        assert_eq!(assignment.nb_items(), 0);
    }

    #[test]
    fn test_set_items() {
        let mut assignment = AutoDesignSecurityClassificationAssignment::new();
        let items = vec![Approval {}];
        assignment.set_items(items);
        assert_eq!(assignment.nb_items(), 1);
    }
}
