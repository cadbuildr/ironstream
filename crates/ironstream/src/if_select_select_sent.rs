// FILE: if_select_select_sent.rs
// occt: IFSelect_SelectSent

/// Selects entities based on their sending status to files.
/// Can select: remaining (non-sent), sent once, sent multiple times, etc.
#[derive(Clone, Debug)]
pub struct IFSelectSelectSent {
    sent_count: i32,
    at_least: bool,
}

impl IFSelectSelectSent {
    /// Creates a SelectSent.
    /// - sentcount = 0 -> remaining (non-sent) entities
    /// - sentcount = 1, atleast = true (default) -> sent (at least once)
    /// - sentcount = 2, atleast = true -> duplicated (sent at least twice)
    /// - sentcount = 1, atleast = false -> sent just once (non-duplicated)
    /// - sentcount = 2, atleast = false -> sent just twice
    pub fn new(sent_count: i32, at_least: bool) -> Self {
        Self { sent_count, at_least }
    }

    /// Returns the queried count of sending
    pub fn sent_count(&self) -> i32 {
        self.sent_count
    }

    /// Returns the at_least status.
    /// True means "at least this count", False means "exactly this count"
    pub fn at_least(&self) -> bool {
        self.at_least
    }

    /// Sort always returns false since RootResult has done the work
    pub fn sort(&self) -> bool {
        false
    }

    /// Returns a text defining the criterium
    pub fn extract_label(&self) -> String {
        match self.sent_count {
            0 => "Remaining (non-sent) entities".to_string(),
            1 if self.at_least => "Sent entities".to_string(),
            1 => "Sent once (no duplicated)".to_string(),
            2 if self.at_least => "Sent several times entities".to_string(),
            2 => "Sent twice entities".to_string(),
            count if self.at_least => format!("Sent at least {} times entities", count),
            count => format!("Sent {} times entities", count),
        }
    }
}

impl Default for IFSelectSelectSent {
    fn default() -> Self {
        Self {
            sent_count: 1,
            at_least: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remaining() {
        let sel = IFSelectSelectSent::new(0, true);
        assert_eq!(sel.sent_count(), 0);
        assert_eq!(sel.extract_label(), "Remaining (non-sent) entities");
    }

    #[test]
    fn test_sent_at_least_once() {
        let sel = IFSelectSelectSent::new(1, true);
        assert_eq!(sel.sent_count(), 1);
        assert!(sel.at_least());
        assert_eq!(sel.extract_label(), "Sent entities");
    }

    #[test]
    fn test_sent_once() {
        let sel = IFSelectSelectSent::new(1, false);
        assert!(!sel.at_least());
        assert_eq!(sel.extract_label(), "Sent once (no duplicated)");
    }

    #[test]
    fn test_sent_several_times() {
        let sel = IFSelectSelectSent::new(2, true);
        assert_eq!(sel.extract_label(), "Sent several times entities");
    }

    #[test]
    fn test_sent_exactly_twice() {
        let sel = IFSelectSelectSent::new(2, false);
        assert_eq!(sel.extract_label(), "Sent twice entities");
    }

    #[test]
    fn test_sent_at_least_n() {
        let sel = IFSelectSelectSent::new(5, true);
        assert_eq!(sel.extract_label(), "Sent at least 5 times entities");
    }

    #[test]
    fn test_sent_exactly_n() {
        let sel = IFSelectSelectSent::new(5, false);
        assert_eq!(sel.extract_label(), "Sent 5 times entities");
    }

    #[test]
    fn test_sort() {
        let sel = IFSelectSelectSent::new(1, true);
        assert!(!sel.sort());
    }

    #[test]
    fn test_default() {
        let sel = IFSelectSelectSent::default();
        assert_eq!(sel.sent_count(), 1);
        assert!(sel.at_least());
    }
}
