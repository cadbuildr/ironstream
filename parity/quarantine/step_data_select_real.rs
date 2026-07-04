// FILE: step_data_select_real.rs
// occt: StepData_SelectReal

const KIND_REAL: i32 = 5;

//! A SelectReal is a SelectMember specialised for a basic real type
pub struct StepDataSelectReal {
    value: f64,
}

impl StepDataSelectReal {
    //! Creates a SelectReal
    pub fn new() -> Self {
        StepDataSelectReal { value: 0.0 }
    }

    //! Returns the kind
    pub fn kind(&self) -> i32 {
        KIND_REAL
    }

    //! Returns the real value
    pub fn real(&self) -> f64 {
        self.value
    }

    //! Sets the real value
    pub fn set_real(&mut self, val: f64) {
        self.value = val;
    }
}

impl Default for StepDataSelectReal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_real_new() {
        let sel = StepDataSelectReal::new();
        assert_eq!(sel.kind(), KIND_REAL);
        assert_eq!(sel.real(), 0.0);
    }

    #[test]
    fn test_set_real() {
        let mut sel = StepDataSelectReal::new();
        sel.set_real(3.14);
        assert!((sel.real() - 3.14).abs() < 1e-10);
    }
}
