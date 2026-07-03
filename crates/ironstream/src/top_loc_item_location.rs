// FILE: top_loc_item_location.rs
// occt: TopLoc_ItemLocation

/// Item in a location list.
/// Stores a transformation and index information.
#[derive(Debug, Clone)]
pub struct ItemLocation {
    pub trsf_index: i32,        // Transformation index
    pub pow: i32,               // Power/exponent for repeated transformations
    pub next_index: i32,        // Index of next item in list
}

impl ItemLocation {
    /// Create new item location.
    pub fn new() -> Self {
        ItemLocation {
            trsf_index: -1,
            pow: 0,
            next_index: -1,
        }
    }

    /// Create with specific values.
    pub fn with_values(trsf_idx: i32, power: i32, next_idx: i32) -> Self {
        ItemLocation {
            trsf_index: trsf_idx,
            pow: power,
            next_index: next_idx,
        }
    }

    /// Get transformation index.
    pub fn get_trsf_index(&self) -> i32 {
        self.trsf_index
    }

    /// Set transformation index.
    pub fn set_trsf_index(&mut self, idx: i32) {
        self.trsf_index = idx;
    }

    /// Get power.
    pub fn get_pow(&self) -> i32 {
        self.pow
    }

    /// Set power.
    pub fn set_pow(&mut self, p: i32) {
        self.pow = p;
    }

    /// Get next index.
    pub fn get_next_index(&self) -> i32 {
        self.next_index
    }

    /// Set next index.
    pub fn set_next_index(&mut self, idx: i32) {
        self.next_index = idx;
    }

    /// Check if this is a terminal item.
    pub fn is_terminal(&self) -> bool {
        self.next_index == -1
    }
}

impl Default for ItemLocation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_location_new() {
        let item = ItemLocation::new();
        assert_eq!(item.get_trsf_index(), -1);
        assert_eq!(item.get_pow(), 0);
        assert!(item.is_terminal());
    }

    #[test]
    fn test_item_location_with_values() {
        let item = ItemLocation::with_values(5, 2, 10);
        assert_eq!(item.get_trsf_index(), 5);
        assert_eq!(item.get_pow(), 2);
        assert_eq!(item.get_next_index(), 10);
        assert!(!item.is_terminal());
    }

    #[test]
    fn test_item_location_setters() {
        let mut item = ItemLocation::new();
        item.set_trsf_index(3);
        item.set_pow(4);
        item.set_next_index(7);
        assert_eq!(item.get_trsf_index(), 3);
        assert_eq!(item.get_pow(), 4);
        assert_eq!(item.get_next_index(), 7);
    }
}
