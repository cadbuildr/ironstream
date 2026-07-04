// FILE: moni_tool_int_val.rs
// occt: MoniTool_IntVal

/// An integer through a handle (i.e. managed as TShared)
pub struct MoniToolIntVal {
    theval: i32,
}

impl MoniToolIntVal {
    pub fn new(val: i32) -> Self {
        MoniToolIntVal { theval: val }
    }

    pub fn value(&self) -> i32 {
        self.theval
    }

    pub fn cvalue(&mut self) -> &mut i32 {
        &mut self.theval
    }

    pub fn set_value(&mut self, val: i32) {
        self.theval = val;
    }
}

impl Default for MoniToolIntVal {
    fn default() -> Self {
        MoniToolIntVal { theval: 0 }
    }
}

impl Clone for MoniToolIntVal {
    fn clone(&self) -> Self {
        MoniToolIntVal {
            theval: self.theval,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let val = MoniToolIntVal::new(42);
        assert_eq!(val.value(), 42);
    }

    #[test]
    fn test_set_value() {
        let mut val = MoniToolIntVal::new(10);
        val.set_value(20);
        assert_eq!(val.value(), 20);
    }

    #[test]
    fn test_cvalue() {
        let mut val = MoniToolIntVal::new(5);
        *val.cvalue() = 15;
        assert_eq!(val.value(), 15);
    }
}
