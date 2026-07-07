// FILE: step_ap214_applied_date_assignment.rs
// occt: StepAP214_AppliedDateAssignment

/// Representation of STEP AP214 AppliedDateAssignment entity.
/// Extends DateAssignment with a list of items to which the date assignment applies.
#[derive(Clone, Debug)]
pub struct AppliedDateAssignment {
    items: Vec<DateItem>,
}

#[derive(Clone, Debug)]
pub struct DateItem {
    // Placeholder for DateItem structure
}

impl AppliedDateAssignment {
    /// Creates a new AppliedDateAssignment.
    pub fn new() -> Self {
        AppliedDateAssignment {
            items: Vec::new(),
        }
    }

    /// Initializes with items.
    pub fn init(&mut self, items: Vec<DateItem>) {
        self.items = items;
    }

    /// Sets the items.
    pub fn set_items(&mut self, items: Vec<DateItem>) {
        self.items = items;
    }

    /// Returns the items collection.
    pub fn items(&self) -> &[DateItem] {
        &self.items
    }

    /// Returns the item at the given index (1-based, as in OCCT).
    pub fn items_value(&self, num: usize) -> Option<&DateItem> {
        if num > 0 && num <= self.items.len() {
            Some(&self.items[num - 1])
        } else {
            None
        }
    }

    /// Returns the number of items.
    pub fn nb_items(&self) -> usize {
        self.items.len()
    }
}

impl Default for AppliedDateAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_applied_date_assignment() {
        let assignment = AppliedDateAssignment::new();
        assert_eq!(assignment.nb_items(), 0);
    }

    #[test]
    fn test_set_items() {
        let mut assignment = AppliedDateAssignment::new();
        let items = vec![DateItem {}];
        assignment.set_items(items);
        assert_eq!(assignment.nb_items(), 1);
    }

    #[test]
    fn test_items_value() {
        let mut assignment = AppliedDateAssignment::new();
        let items = vec![DateItem {}, DateItem {}];
        assignment.set_items(items);
        assert!(assignment.items_value(1).is_some());
        assert!(assignment.items_value(2).is_some());
        assert!(assignment.items_value(3).is_none());
    }
}
