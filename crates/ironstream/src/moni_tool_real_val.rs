// FILE: moni_tool_real_val.rs
// occt: MoniTool_RealVal

/// A real (float) through a handle
pub struct MoniToolRealVal {
    theval: f64,
}

impl MoniToolRealVal {
    pub fn new(val: f64) -> Self {
        MoniToolRealVal { theval: val }
    }

    pub fn value(&self) -> f64 {
        self.theval
    }

    pub fn cvalue(&mut self) -> &mut f64 {
        &mut self.theval
    }

    pub fn set_value(&mut self, val: f64) {
        self.theval = val;
    }
}

impl Default for MoniToolRealVal {
    fn default() -> Self {
        MoniToolRealVal { theval: 0.0 }
    }
}

impl Clone for MoniToolRealVal {
    fn clone(&self) -> Self {
        MoniToolRealVal {
            theval: self.theval,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let val = MoniToolRealVal::new(3.14);
        assert!((val.value() - 3.14).abs() < 0.001);
    }

    #[test]
    fn test_set_value() {
        let mut val = MoniToolRealVal::new(1.0);
        val.set_value(2.71);
        assert!((val.value() - 2.71).abs() < 0.001);
    }
}
