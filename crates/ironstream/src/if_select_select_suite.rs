// FILE: if_select_select_suite.rs
// occt: IFSelect_SelectSuite

/// A suite of selections forming a "macro selection".
/// Applies each item sequentially, passing results from one to the next.
#[derive(Clone, Debug)]
pub struct IFSelectSelectSuite {
    items: Vec<usize>, // indices of SelectDeduct items
    label: Option<String>,
}

impl IFSelectSelectSuite {
    /// Creates an empty SelectSuite
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            label: None,
        }
    }

    /// Adds a new first item (prepends to the list)
    pub fn add_previous(&mut self, item_id: usize) {
        self.items.insert(0, item_id);
    }

    /// Adds a new last item (appends to the list)
    pub fn add_next(&mut self, item_id: usize) {
        self.items.push(item_id);
    }

    /// Returns the count of items
    pub fn nb_items(&self) -> usize {
        self.items.len()
    }

    /// Returns an item from its rank (1-indexed)
    pub fn item(&self, num: usize) -> Option<usize> {
        if num >= 1 && num <= self.items.len() {
            Some(self.items[num - 1])
        } else {
            None
        }
    }

    /// Sets a value for the label
    pub fn set_label(&mut self, lab: String) {
        self.label = Some(lab);
    }

    /// Returns the label
    pub fn label(&self) -> String {
        match &self.label {
            Some(l) => l.clone(),
            None => format!("Suite of {} Selections", self.nb_items()),
        }
    }
}

impl Default for IFSelectSelectSuite {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let suite = IFSelectSelectSuite::new();
        assert_eq!(suite.nb_items(), 0);
    }

    #[test]
    fn test_add_previous() {
        let mut suite = IFSelectSelectSuite::new();
        suite.add_previous(1);
        suite.add_previous(2);
        assert_eq!(suite.nb_items(), 2);
        assert_eq!(suite.item(1), Some(2));
        assert_eq!(suite.item(2), Some(1));
    }

    #[test]
    fn test_add_next() {
        let mut suite = IFSelectSelectSuite::new();
        suite.add_next(1);
        suite.add_next(2);
        assert_eq!(suite.nb_items(), 2);
        assert_eq!(suite.item(1), Some(1));
        assert_eq!(suite.item(2), Some(2));
    }

    #[test]
    fn test_add_mixed() {
        let mut suite = IFSelectSelectSuite::new();
        suite.add_next(1);
        suite.add_previous(2);
        suite.add_next(3);
        assert_eq!(suite.nb_items(), 3);
        assert_eq!(suite.item(1), Some(2));
        assert_eq!(suite.item(2), Some(1));
        assert_eq!(suite.item(3), Some(3));
    }

    #[test]
    fn test_default_label() {
        let suite = IFSelectSelectSuite::new();
        assert_eq!(suite.label(), "Suite of 0 Selections");

        let mut suite = IFSelectSelectSuite::new();
        suite.add_next(1);
        suite.add_next(2);
        assert_eq!(suite.label(), "Suite of 2 Selections");
    }

    #[test]
    fn test_custom_label() {
        let mut suite = IFSelectSelectSuite::new();
        suite.add_next(1);
        suite.set_label("My Custom Suite".to_string());
        assert_eq!(suite.label(), "My Custom Suite");
    }

    #[test]
    fn test_item_out_of_range() {
        let suite = IFSelectSelectSuite::new();
        assert_eq!(suite.item(1), None);
        assert_eq!(suite.item(0), None);
    }
}
