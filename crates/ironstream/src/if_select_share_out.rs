// FILE: if_select_share_out.rs
// occt: IFSelect_ShareOut

/// Describes how to share/distribute data to output files.
/// Composed of a list of Dispatches, each with an ID.
#[derive(Clone, Debug)]
pub struct IFSelectShareOut {
    dispatches: Vec<(usize, String)>, // (id, dispatch_name)
}

impl IFSelectShareOut {
    /// Creates an empty ShareOut
    pub fn new() -> Self {
        Self {
            dispatches: Vec::new(),
        }
    }

    /// Clears all dispatches and information
    pub fn clear(&mut self, only_dispatches: bool) {
        self.dispatches.clear();
        // TODO: If not only_dispatches, also clear modifiers and other state
    }

    /// Adds a dispatch with an ID
    pub fn add_dispatch(&mut self, id: usize, name: String) {
        self.dispatches.push((id, name));
    }

    /// Returns the count of dispatches
    pub fn nb_dispatches(&self) -> usize {
        self.dispatches.len()
    }

    /// Returns a dispatch by index (1-indexed)
    pub fn dispatch(&self, num: usize) -> Option<(usize, &str)> {
        if num >= 1 && num <= self.dispatches.len() {
            let (id, name) = &self.dispatches[num - 1];
            Some((*id, name.as_str()))
        } else {
            None
        }
    }
}

impl Default for IFSelectShareOut {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let share_out = IFSelectShareOut::new();
        assert_eq!(share_out.nb_dispatches(), 0);
    }

    #[test]
    fn test_add_dispatch() {
        let mut share_out = IFSelectShareOut::new();
        share_out.add_dispatch(1, "dispatch1".to_string());
        assert_eq!(share_out.nb_dispatches(), 1);
        assert_eq!(share_out.dispatch(1), Some((1, "dispatch1")));
    }

    #[test]
    fn test_multiple_dispatches() {
        let mut share_out = IFSelectShareOut::new();
        share_out.add_dispatch(1, "disp1".to_string());
        share_out.add_dispatch(2, "disp2".to_string());
        share_out.add_dispatch(3, "disp3".to_string());

        assert_eq!(share_out.nb_dispatches(), 3);
        assert_eq!(share_out.dispatch(1), Some((1, "disp1")));
        assert_eq!(share_out.dispatch(2), Some((2, "disp2")));
        assert_eq!(share_out.dispatch(3), Some((3, "disp3")));
        assert_eq!(share_out.dispatch(4), None);
    }

    #[test]
    fn test_clear() {
        let mut share_out = IFSelectShareOut::new();
        share_out.add_dispatch(1, "disp1".to_string());
        assert_eq!(share_out.nb_dispatches(), 1);
        share_out.clear(true);
        assert_eq!(share_out.nb_dispatches(), 0);
    }

    #[test]
    fn test_default() {
        let share_out = IFSelectShareOut::default();
        assert_eq!(share_out.nb_dispatches(), 0);
    }
}
