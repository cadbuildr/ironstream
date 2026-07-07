// FILE: step_ap214_applied_person_and_organization_assignment.rs
// occt: StepAP214_AppliedPersonAndOrganizationAssignment

#[derive(Clone, Debug)]
pub struct PersonAndOrganizationItem {}

#[derive(Clone, Debug)]
pub struct AppliedPersonAndOrganizationAssignment {
    items: Vec<PersonAndOrganizationItem>,
}

impl AppliedPersonAndOrganizationAssignment {
    pub fn new() -> Self {
        AppliedPersonAndOrganizationAssignment {
            items: Vec::new(),
        }
    }

    pub fn init(&mut self, items: Vec<PersonAndOrganizationItem>) {
        self.items = items;
    }

    pub fn set_items(&mut self, items: Vec<PersonAndOrganizationItem>) {
        self.items = items;
    }

    pub fn items(&self) -> &[PersonAndOrganizationItem] {
        &self.items
    }

    pub fn items_value(&self, num: usize) -> Option<&PersonAndOrganizationItem> {
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

impl Default for AppliedPersonAndOrganizationAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let assignment = AppliedPersonAndOrganizationAssignment::new();
        assert_eq!(assignment.nb_items(), 0);
    }

    #[test]
    fn test_set_items() {
        let mut assignment = AppliedPersonAndOrganizationAssignment::new();
        let items = vec![PersonAndOrganizationItem {}];
        assignment.set_items(items);
        assert_eq!(assignment.nb_items(), 1);
    }
}
