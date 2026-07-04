// FILE: step_ap214_auto_design_date_and_person_assignment.rs
// occt: StepAP214_AutoDesignDateAndPersonAssignment

#[derive(Clone, Debug)]
pub struct AutoDesignDateAndPersonItem {}

#[derive(Clone, Debug)]
pub struct AutoDesignDateAndPersonAssignment {
    items: Vec<AutoDesignDateAndPersonItem>,
}

impl AutoDesignDateAndPersonAssignment {
    pub fn new() -> Self {
        AutoDesignDateAndPersonAssignment {
            items: Vec::new(),
        }
    }

    pub fn init(&mut self, items: Vec<AutoDesignDateAndPersonItem>) {
        self.items = items;
    }

    pub fn set_items(&mut self, items: Vec<AutoDesignDateAndPersonItem>) {
        self.items = items;
    }

    pub fn items(&self) -> &[AutoDesignDateAndPersonItem] {
        &self.items
    }

    pub fn items_value(&self, num: usize) -> Option<&AutoDesignDateAndPersonItem> {
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

impl Default for AutoDesignDateAndPersonAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let assignment = AutoDesignDateAndPersonAssignment::new();
        assert_eq!(assignment.nb_items(), 0);
    }

    #[test]
    fn test_set_items() {
        let mut assignment = AutoDesignDateAndPersonAssignment::new();
        let items = vec![AutoDesignDateAndPersonItem {}];
        assignment.set_items(items);
        assert_eq!(assignment.nb_items(), 1);
    }
}
