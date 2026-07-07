// FILE: if_select_select_range.rs
// occt: IFSelect_SelectRange

/// Keeps or rejects a sub-set of input based on rank within iteration range.
/// For example, keeps entities ranked 2nd through 6th.
#[derive(Clone, Debug)]
pub struct IFSelectSelectRange {
    lower: Option<i32>,
    upper: Option<i32>,
}

impl IFSelectSelectRange {
    /// Creates a SelectRange (default: keep all input)
    pub fn new() -> Self {
        Self {
            lower: None,
            upper: None,
        }
    }

    /// Sets a range with lower and upper limits
    pub fn set_range(&mut self, rankfrom: i32, rankto: i32) {
        assert!(rankto >= rankfrom, "rankto must be >= rankfrom");
        self.lower = Some(rankfrom);
        self.upper = Some(rankto);
    }

    /// Sets a unique rank (only one entity)
    pub fn set_one(&mut self, rank: i32) {
        self.lower = Some(rank);
        self.upper = Some(rank);
    }

    /// Sets a lower limit with no upper limit
    pub fn set_from(&mut self, rankfrom: i32) {
        self.lower = Some(rankfrom);
        self.upper = None;
    }

    /// Sets an upper limit with no lower limit (equivalent to lower 1)
    pub fn set_until(&mut self, rankto: i32) {
        self.lower = None;
        self.upper = Some(rankto);
    }

    /// Returns true if a lower limit is defined
    pub fn has_lower(&self) -> bool {
        self.lower.is_some()
    }

    /// Returns lower limit value, or 0 if none is defined
    pub fn lower_value(&self) -> i32 {
        self.lower.unwrap_or(0)
    }

    /// Returns true if an upper limit is defined
    pub fn has_upper(&self) -> bool {
        self.upper.is_some()
    }

    /// Returns upper limit value, or 0 if none is defined
    pub fn upper_value(&self) -> i32 {
        self.upper.unwrap_or(0)
    }

    /// Checks if rank is within the selected range
    pub fn is_in_range(&self, rank: i32) -> bool {
        let lower_ok = self.lower.map_or(true, |l| rank >= l);
        let upper_ok = self.upper.map_or(true, |u| rank <= u);
        lower_ok && upper_ok
    }

    /// Returns a text defining the criterium
    pub fn extract_label(&self) -> String {
        match (self.lower, self.upper) {
            (Some(l), Some(u)) if l == u => format!("Rank no {}", l),
            (Some(l), Some(u)) => format!("From {} Until {}", l, u),
            (Some(l), None) => format!("From {}", l),
            (None, Some(u)) => format!("Until {}", u),
            (None, None) => "All".to_string(),
        }
    }
}

impl Default for IFSelectSelectRange {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sel = IFSelectSelectRange::new();
        assert!(!sel.has_lower());
        assert!(!sel.has_upper());
        assert!(sel.is_in_range(1));
        assert!(sel.is_in_range(999));
    }

    #[test]
    fn test_set_range() {
        let mut sel = IFSelectSelectRange::new();
        sel.set_range(2, 6);
        assert!(sel.has_lower());
        assert!(sel.has_upper());
        assert_eq!(sel.lower_value(), 2);
        assert_eq!(sel.upper_value(), 6);
        assert!(!sel.is_in_range(1));
        assert!(sel.is_in_range(2));
        assert!(sel.is_in_range(4));
        assert!(sel.is_in_range(6));
        assert!(!sel.is_in_range(7));
    }

    #[test]
    fn test_set_one() {
        let mut sel = IFSelectSelectRange::new();
        sel.set_one(5);
        assert!(sel.is_in_range(5));
        assert!(!sel.is_in_range(4));
        assert!(!sel.is_in_range(6));
    }

    #[test]
    fn test_set_from() {
        let mut sel = IFSelectSelectRange::new();
        sel.set_from(3);
        assert!(!sel.is_in_range(2));
        assert!(sel.is_in_range(3));
        assert!(sel.is_in_range(1000));
    }

    #[test]
    fn test_set_until() {
        let mut sel = IFSelectSelectRange::new();
        sel.set_until(7);
        assert!(sel.is_in_range(1));
        assert!(sel.is_in_range(7));
        assert!(!sel.is_in_range(8));
    }

    #[test]
    fn test_extract_label() {
        let mut sel = IFSelectSelectRange::new();
        assert_eq!(sel.extract_label(), "All");

        sel.set_one(5);
        assert_eq!(sel.extract_label(), "Rank no 5");

        sel.set_range(2, 6);
        assert_eq!(sel.extract_label(), "From 2 Until 6");

        sel.set_from(3);
        assert_eq!(sel.extract_label(), "From 3");

        let mut sel2 = IFSelectSelectRange::new();
        sel2.set_until(10);
        assert_eq!(sel2.extract_label(), "Until 10");
    }
}
