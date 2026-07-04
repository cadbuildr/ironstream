// FILE: step_ap214_auto_design_actual_date_assignment.rs
// occt: StepAP214_AutoDesignActualDateAssignment

#[derive(Clone, Debug)]
pub struct AutoDesignDatedItem {}

#[derive(Clone, Debug)]
pub struct AutoDesignActualDateAssignment {
    items: Vec<AutoDesignDatedItem>,
}

impl AutoDesignActualDateAssignment {
    pub fn new() -> Self {
        AutoDesignActualDateAssignment {
            items: Vec::new(),
        }
    }

    pub fn init(&mut self, items: Vec<AutoDesignDatedItem>) {
        self.items = items;
    }

    pub fn set_items(&mut self, items: Vec<AutoDesignDatedItem>) {
        self.items = items;
    }

    pub fn items(&self) -> &[AutoDesignDatedItem] {
        &self.items
    }

    pub fn items_value(&self, num: usize) -> Option<&AutoDesignDatedItem> {
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

impl Default for AutoDesignActualDateAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let assignment = AutoDesignActualDateAssignment::new();
        assert_eq!(assignment.nb_items(), 0);
    }

    #[test]
    fn test_set_items() {
        let mut assignment = AutoDesignActualDateAssignment::new();
        let items = vec![AutoDesignDatedItem {}];
        assignment.set_items(items);
        assert_eq!(assignment.nb_items(), 1);
    }
}
