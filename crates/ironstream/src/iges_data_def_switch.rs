// FILE: iges_data_def_switch.rs
// occt: IGESData_DefSwitch

//! Description of a directory component which can be either undefined (Void),
//! defined as a Reference to an entity, or as a Rank (integer value addressing a builtin table).
//! The entity reference is not included here, only reference status is kept.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DefType {
    /// Item is undefined
    Void,
    /// Item is defined as an immediate positive integer value
    Value,
    /// Item is defined as a reference to an entity
    Reference,
    /// Item could not be determined
    Any,
    /// Item is an integer but its value is incorrect
    ErrorVal,
    /// Item is an entity but not of the required type
    ErrorRef,
}

/// DefSwitch represents a component that can be Void, Reference, or Rank.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DefSwitch {
    value: i32,
}

impl DefSwitch {
    /// Creates a DefSwitch as Void
    pub fn new() -> Self {
        DefSwitch { value: 0 }
    }

    /// Sets DefSwitch to "Void" status (in file: Integer = 0)
    pub fn set_void(&mut self) {
        self.value = 0;
    }

    /// Sets DefSwitch to "Reference" Status (in file: Integer < 0)
    pub fn set_reference(&mut self) {
        self.value = -1;
    }

    /// Sets DefSwitch to "Rank" with a Value (in file: Integer > 0)
    pub fn set_rank(&mut self, val: i32) {
        if val > 0 {
            self.value = val;
        }
    }

    /// Returns DefType status (Void, Reference, Rank)
    pub fn def_type(&self) -> DefType {
        if self.value == 0 {
            DefType::Void
        } else if self.value < 0 {
            DefType::Reference
        } else {
            DefType::Value
        }
    }

    /// Returns Value as Integer (meaningful for a Rank)
    pub fn value(&self) -> i32 {
        self.value
    }
}

impl Default for DefSwitch {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_is_void() {
        let ds = DefSwitch::new();
        assert_eq!(ds.def_type(), DefType::Void);
        assert_eq!(ds.value(), 0);
    }

    #[test]
    fn test_set_void() {
        let mut ds = DefSwitch::new();
        ds.set_rank(5);
        assert_eq!(ds.value(), 5);
        ds.set_void();
        assert_eq!(ds.def_type(), DefType::Void);
        assert_eq!(ds.value(), 0);
    }

    #[test]
    fn test_set_reference() {
        let mut ds = DefSwitch::new();
        ds.set_reference();
        assert_eq!(ds.def_type(), DefType::Reference);
        assert!(ds.value() < 0);
    }

    #[test]
    fn test_set_rank() {
        let mut ds = DefSwitch::new();
        ds.set_rank(42);
        assert_eq!(ds.def_type(), DefType::Value);
        assert_eq!(ds.value(), 42);
    }

    #[test]
    fn test_set_rank_invalid() {
        let mut ds = DefSwitch::new();
        ds.set_rank(0);
        // Should not set for non-positive values
        assert_eq!(ds.value(), 0);
        ds.set_rank(-5);
        assert_eq!(ds.value(), 0);
    }

    #[test]
    fn test_def_type_transitions() {
        let mut ds = DefSwitch::new();

        // Start as Void
        assert_eq!(ds.def_type(), DefType::Void);

        // Change to Reference
        ds.set_reference();
        assert_eq!(ds.def_type(), DefType::Reference);

        // Change to Rank
        ds.set_rank(100);
        assert_eq!(ds.def_type(), DefType::Value);

        // Back to Void
        ds.set_void();
        assert_eq!(ds.def_type(), DefType::Void);
    }

    #[test]
    fn test_clone() {
        let mut ds1 = DefSwitch::new();
        ds1.set_rank(77);
        let ds2 = ds1;
        assert_eq!(ds1.value(), ds2.value());
        assert_eq!(ds1.def_type(), ds2.def_type());
    }

    #[test]
    fn test_default() {
        let ds = DefSwitch::default();
        assert_eq!(ds.def_type(), DefType::Void);
    }
}
