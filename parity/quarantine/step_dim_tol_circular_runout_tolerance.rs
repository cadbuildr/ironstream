// FILE: step_dim_tol_circular_runout_tolerance.rs
// occt: StepDimTol_CircularRunoutTolerance

//! Represents a circular runout tolerance in GD&T
pub struct StepDimTolCircularRunoutTolerance {
    value: f64,
}

impl StepDimTolCircularRunoutTolerance {
    //! Creates a CircularRunoutTolerance
    pub fn new(value: f64) -> Self {
        StepDimTolCircularRunoutTolerance { value }
    }

    //! Returns the value
    pub fn value(&self) -> f64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_runout_tolerance_new() {
        let tol = StepDimTolCircularRunoutTolerance::new(0.1);
        assert!((tol.value() - 0.1).abs() < 1e-10);
    }
}
