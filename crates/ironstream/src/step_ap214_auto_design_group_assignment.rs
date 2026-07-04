// FILE: step_ap214_auto_design_group_assignment.rs
// occt: StepAP214_AutoDesignGroupAssignment

#[derive(Clone, Debug)]
pub struct AutoDesignGroupedItem {}

#[derive(Clone, Debug)]
pub struct AutoDesignGroupAssignment {
    items: Vec<AutoDesignGroupedItem>,
}

impl AutoDesignGroupAssignment {
    pub fn new() -> Self {
        AutoDesignGroupAssignment {
            items: Vec::new(),
        }
    }

    pub fn init(&mut self, items: Vec<AutoDesignGroupedItem>) {
        self.items = items;
    }

    pub fn set_items(&mut self, items: Vec<AutoDesignGroupedItem>) {
        self.items = items;
    }

    pub fn items(&self) -> &[AutoDesignGroupedItem] {
        &self.items
    }

    pub fn items_value(&self, num: usize) -> Option<&AutoDesignGroupedItem> {
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

impl Default for AutoDesignGroupAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let assignment = AutoDesignGroupAssignment::new();
        assert_eq!(assignment.nb_items(), 0);
    }

    #[test]
    fn test_set_items() {
        let mut assignment = AutoDesignGroupAssignment::new();
        let items = vec![AutoDesignGroupedItem {}];
        assignment.set_items(items);
        assert_eq!(assignment.nb_items(), 1);
    }
}
