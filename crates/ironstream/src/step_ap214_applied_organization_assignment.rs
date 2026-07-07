// FILE: step_ap214_applied_organization_assignment.rs
// occt: StepAP214_AppliedOrganizationAssignment

#[derive(Clone, Debug)]
pub struct OrganizationItem {}

#[derive(Clone, Debug)]
pub struct AppliedOrganizationAssignment {
    items: Vec<OrganizationItem>,
}

impl AppliedOrganizationAssignment {
    pub fn new() -> Self {
        AppliedOrganizationAssignment {
            items: Vec::new(),
        }
    }

    pub fn init(&mut self, items: Vec<OrganizationItem>) {
        self.items = items;
    }

    pub fn set_items(&mut self, items: Vec<OrganizationItem>) {
        self.items = items;
    }

    pub fn items(&self) -> &[OrganizationItem] {
        &self.items
    }

    pub fn items_value(&self, num: usize) -> Option<&OrganizationItem> {
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

impl Default for AppliedOrganizationAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let assignment = AppliedOrganizationAssignment::new();
        assert_eq!(assignment.nb_items(), 0);
    }

    #[test]
    fn test_set_items() {
        let mut assignment = AppliedOrganizationAssignment::new();
        let items = vec![OrganizationItem {}, OrganizationItem {}];
        assignment.set_items(items);
        assert_eq!(assignment.nb_items(), 2);
    }

    #[test]
    fn test_items_value() {
        let mut assignment = AppliedOrganizationAssignment::new();
        let items = vec![OrganizationItem {}];
        assignment.set_items(items);
        assert!(assignment.items_value(1).is_some());
        assert!(assignment.items_value(2).is_none());
    }
}
