// FILE: step_ap214_auto_design_organization_assignment.rs
// occt: StepAP214_AutoDesignOrganizationAssignment

#[derive(Clone, Debug)]
pub struct AutoDesignGeneralOrgItem {}

#[derive(Clone, Debug)]
pub struct AutoDesignOrganizationAssignment {
    items: Vec<AutoDesignGeneralOrgItem>,
}

impl AutoDesignOrganizationAssignment {
    pub fn new() -> Self {
        AutoDesignOrganizationAssignment {
            items: Vec::new(),
        }
    }

    pub fn init(&mut self, items: Vec<AutoDesignGeneralOrgItem>) {
        self.items = items;
    }

    pub fn set_items(&mut self, items: Vec<AutoDesignGeneralOrgItem>) {
        self.items = items;
    }

    pub fn items(&self) -> &[AutoDesignGeneralOrgItem] {
        &self.items
    }

    pub fn items_value(&self, num: usize) -> Option<&AutoDesignGeneralOrgItem> {
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

impl Default for AutoDesignOrganizationAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let assignment = AutoDesignOrganizationAssignment::new();
        assert_eq!(assignment.nb_items(), 0);
    }

    #[test]
    fn test_set_items() {
        let mut assignment = AutoDesignOrganizationAssignment::new();
        let items = vec![AutoDesignGeneralOrgItem {}];
        assignment.set_items(items);
        assert_eq!(assignment.nb_items(), 1);
    }
}
