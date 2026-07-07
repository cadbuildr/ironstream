// FILE: xs_control_select_for_transfer.rs
// occt: XSControl_SelectForTransfer

/// Selector for identifying entities to transfer in the control framework.
/// Filters and selects entities based on transfer criteria.
#[derive(Clone, Debug)]
pub struct XSControlSelectForTransfer {
    /// Selection criteria flags
    criteria: u32,
    /// Number of selected entities
    nb_selected: u32,
}

impl XSControlSelectForTransfer {
    /// Creates a new transfer selector.
    pub fn new() -> Self {
        Self {
            criteria: 0,
            nb_selected: 0,
        }
    }

    /// Returns the selection criteria.
    pub fn criteria(&self) -> u32 {
        self.criteria
    }

    /// Sets the selection criteria.
    pub fn set_criteria(&mut self, criteria: u32) {
        self.criteria = criteria;
    }

    /// Returns the number of selected entities.
    pub fn nb_selected(&self) -> u32 {
        self.nb_selected
    }

    /// Sets the number of selected entities.
    pub fn set_nb_selected(&mut self, count: u32) {
        self.nb_selected = count;
    }
}

impl Default for XSControlSelectForTransfer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let selector = XSControlSelectForTransfer::new();
        assert_eq!(selector.criteria(), 0);
        assert_eq!(selector.nb_selected(), 0);
    }

    #[test]
    fn test_set_criteria() {
        let mut selector = XSControlSelectForTransfer::new();
        selector.set_criteria(0xFF);
        assert_eq!(selector.criteria(), 0xFF);
    }

    #[test]
    fn test_set_nb_selected() {
        let mut selector = XSControlSelectForTransfer::new();
        selector.set_nb_selected(5);
        assert_eq!(selector.nb_selected(), 5);
    }
}
