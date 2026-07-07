// FILE: step_ap214_applied_group_assignment.rs
// occt: StepAP214_AppliedGroupAssignment

#[derive(Clone, Debug)]
pub struct GroupItem {}

#[derive(Clone, Debug)]
pub struct AppliedGroupAssignment {
    items: Vec<GroupItem>,
}

impl AppliedGroupAssignment {
    pub fn new() -> Self {
        AppliedGroupAssignment {
            items: Vec::new(),
        }
    }

    pub fn init(&mut self, items: Vec<GroupItem>) {
        self.items = items;
    }

    pub fn set_items(&mut self, items: Vec<GroupItem>) {
        self.items = items;
    }

    pub fn items(&self) -> &[GroupItem] {
        &self.items
    }

    pub fn nb_items(&self) -> usize {
        self.items.len()
    }
}

impl Default for AppliedGroupAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let assignment = AppliedGroupAssignment::new();
        assert_eq!(assignment.nb_items(), 0);
    }

    #[test]
    fn test_set_items() {
        let mut assignment = AppliedGroupAssignment::new();
        let items = vec![GroupItem {}];
        assignment.set_items(items);
        assert_eq!(assignment.nb_items(), 1);
    }
}
