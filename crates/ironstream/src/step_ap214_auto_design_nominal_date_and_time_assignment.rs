// FILE: step_ap214_auto_design_nominal_date_and_time_assignment.rs
// occt: StepAP214_AutoDesignNominalDateAndTimeAssignment

#[derive(Clone, Debug)]
pub struct AutoDesignDateAndTimeItem {}

#[derive(Clone, Debug)]
pub struct AutoDesignNominalDateAndTimeAssignment {
    items: Vec<AutoDesignDateAndTimeItem>,
}

impl AutoDesignNominalDateAndTimeAssignment {
    pub fn new() -> Self {
        AutoDesignNominalDateAndTimeAssignment {
            items: Vec::new(),
        }
    }

    pub fn init(&mut self, items: Vec<AutoDesignDateAndTimeItem>) {
        self.items = items;
    }

    pub fn set_items(&mut self, items: Vec<AutoDesignDateAndTimeItem>) {
        self.items = items;
    }

    pub fn items(&self) -> &[AutoDesignDateAndTimeItem] {
        &self.items
    }

    pub fn items_value(&self, num: usize) -> Option<&AutoDesignDateAndTimeItem> {
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

impl Default for AutoDesignNominalDateAndTimeAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let assignment = AutoDesignNominalDateAndTimeAssignment::new();
        assert_eq!(assignment.nb_items(), 0);
    }

    #[test]
    fn test_set_items() {
        let mut assignment = AutoDesignNominalDateAndTimeAssignment::new();
        let items = vec![AutoDesignDateAndTimeItem {}];
        assignment.set_items(items);
        assert_eq!(assignment.nb_items(), 1);
    }
}
