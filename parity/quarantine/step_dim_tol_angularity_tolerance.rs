// FILE: step_dim_tol_angularity_tolerance.rs
// occt: StepDimTol_AngularityTolerance

//! Represents an angularity tolerance in GD&T
pub struct StepDimTolAngularityTolerance {
    value: f64,
}

impl StepDimTolAngularityTolerance {
    //! Creates an AngularityTolerance
    pub fn new(value: f64) -> Self {
        StepDimTolAngularityTolerance { value }
    }

    //! Returns the value
    pub fn value(&self) -> f64 {
        self.value
    }

    //! Sets the value
    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angularity_tolerance_new() {
        let tol = StepDimTolAngularityTolerance::new(0.5);
        assert!((tol.value() - 0.5).abs() < 1e-10);
    }
}
