// FILE: interface_int_val.rs
// occt: Interface_IntVal

/// An Integer through a reference (i.e. managed as Transient)
pub struct InterfaceIntVal {
    theval: i32,
}

impl InterfaceIntVal {
    /// Creates an IntVal with default value 0
    pub fn new() -> Self {
        InterfaceIntVal { theval: 0 }
    }

    /// Creates an IntVal with a given value
    pub fn with_value(val: i32) -> Self {
        InterfaceIntVal { theval: val }
    }

    /// Returns the value
    pub fn value(&self) -> i32 {
        self.theval
    }

    /// Returns a mutable reference to the value
    pub fn cvalue(&mut self) -> &mut i32 {
        &mut self.theval
    }

    /// Sets the value
    pub fn set_value(&mut self, val: i32) {
        self.theval = val;
    }
}

impl Default for InterfaceIntVal {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for InterfaceIntVal {
    fn clone(&self) -> Self {
        InterfaceIntVal {
            theval: self.theval,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let val = InterfaceIntVal::new();
        assert_eq!(val.value(), 0);
    }

    #[test]
    fn test_create_with_value() {
        let val = InterfaceIntVal::with_value(42);
        assert_eq!(val.value(), 42);
    }

    #[test]
    fn test_cvalue_mut() {
        let mut val = InterfaceIntVal::with_value(10);
        *val.cvalue() = 20;
        assert_eq!(val.value(), 20);
    }

    #[test]
    fn test_set_value() {
        let mut val = InterfaceIntVal::new();
        val.set_value(99);
        assert_eq!(val.value(), 99);
    }

    #[test]
    fn test_clone() {
        let val1 = InterfaceIntVal::with_value(42);
        let val2 = val1.clone();
        assert_eq!(val2.value(), 42);
    }
}
