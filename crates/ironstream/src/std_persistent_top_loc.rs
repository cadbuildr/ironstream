// FILE: std_persistent_top_loc.rs
// occt: StdPersistent_TopLoc

/// TopLoc persistence for locations
pub struct TopLoc {
    ref_num: i32,
}

impl TopLoc {
    /// Create a new location
    pub fn new() -> Self {
        TopLoc { ref_num: 0 }
    }

    /// Get reference number
    pub fn ref_num(&self) -> i32 {
        self.ref_num
    }

    /// Set reference number
    pub fn set_ref_num(&mut self, ref_num: i32) {
        self.ref_num = ref_num;
    }
}

impl Default for TopLoc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let loc = TopLoc::new();
        assert_eq!(loc.ref_num(), 0);
    }

    #[test]
    fn test_set_ref_num() {
        let mut loc = TopLoc::new();
        loc.set_ref_num(42);
        assert_eq!(loc.ref_num(), 42);
    }
}
