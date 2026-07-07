// FILE: adv_app2_var_strip.rs
// occt: AdvApp2Var_Strip

//! Deprecated NCollection type alias.
//! Represents a strip in approximation algorithms.

/// Strip structure for approximation.
#[derive(Clone, Debug)]
pub struct AdvApp2VarStrip {
    pub id: u32,
    pub value: f64,
}

impl AdvApp2VarStrip {
    /// Create a new strip.
    pub fn new(id: u32, value: f64) -> Self {
        Self { id, value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_creation() {
        let strip = AdvApp2VarStrip::new(1, 1.5);
        assert_eq!(strip.id, 1);
        assert_eq!(strip.value, 1.5);
    }
}
