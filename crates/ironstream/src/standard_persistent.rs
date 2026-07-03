// FILE: standard_persistent.rs
// occt: Standard_Persistent

//! Root of "persistent" classes, a legacy support of object oriented databases.
//! Now outdated but preserved for compatibility.

/// Legacy persistent object class.
/// Contains type and reference numbers for database storage.
#[derive(Debug, Clone)]
pub struct StandardPersistent {
    type_num: i32,
    ref_num: i32,
}

impl StandardPersistent {
    /// Create a new persistent object.
    pub fn new() -> Self {
        StandardPersistent {
            type_num: 0,
            ref_num: 0,
        }
    }

    /// Get the type number.
    pub fn type_num(&self) -> i32 {
        self.type_num
    }

    /// Set the type number.
    pub fn set_type_num(&mut self, num: i32) {
        self.type_num = num;
    }

    /// Get the reference number.
    pub fn ref_num(&self) -> i32 {
        self.ref_num
    }

    /// Set the reference number.
    pub fn set_ref_num(&mut self, num: i32) {
        self.ref_num = num;
    }
}

impl Default for StandardPersistent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_persistent_creation() {
        let obj = StandardPersistent::new();
        assert_eq!(obj.type_num(), 0);
        assert_eq!(obj.ref_num(), 0);
    }

    #[test]
    fn test_persistent_type_num() {
        let mut obj = StandardPersistent::new();
        obj.set_type_num(42);
        assert_eq!(obj.type_num(), 42);
    }

    #[test]
    fn test_persistent_ref_num() {
        let mut obj = StandardPersistent::new();
        obj.set_ref_num(100);
        assert_eq!(obj.ref_num(), 100);
    }

    #[test]
    fn test_persistent_default() {
        let obj = StandardPersistent::default();
        assert_eq!(obj.type_num(), 0);
    }

    #[test]
    fn test_persistent_clone() {
        let mut obj1 = StandardPersistent::new();
        obj1.set_type_num(55);
        let obj2 = obj1.clone();
        assert_eq!(obj2.type_num(), 55);
    }
}
