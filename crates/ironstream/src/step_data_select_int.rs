// FILE: step_data_select_int.rs
// occt: StepData_SelectInt

// A SelectInt is a SelectMember specialised for a basic integer type
pub struct StepDataSelectInt {
    kind: i32,
    value: i32,
}

impl StepDataSelectInt {
    // Creates a SelectInt
    pub fn new() -> Self {
        StepDataSelectInt { kind: 0, value: 0 }
    }

    // Returns the kind
    pub fn kind(&self) -> i32 {
        self.kind
    }

    // Sets the kind
    pub fn set_kind(&mut self, kind: i32) {
        self.kind = kind;
    }

    // Returns the integer value
    pub fn int(&self) -> i32 {
        self.value
    }

    // Sets the integer value
    pub fn set_int(&mut self, val: i32) {
        self.value = val;
    }
}

impl Default for StepDataSelectInt {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_int_new() {
        let sel = StepDataSelectInt::new();
        assert_eq!(sel.kind(), 0);
        assert_eq!(sel.int(), 0);
    }

    #[test]
    fn test_set_int() {
        let mut sel = StepDataSelectInt::new();
        sel.set_int(42);
        assert_eq!(sel.int(), 42);
    }

    #[test]
    fn test_set_kind() {
        let mut sel = StepDataSelectInt::new();
        sel.set_kind(5);
        assert_eq!(sel.kind(), 5);
    }
}
