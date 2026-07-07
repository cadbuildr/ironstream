// FILE: step_fea_freedom_and_coefficient.rs
// occt: StepFEA_FreedomAndCoefficient

/// Representation of STEP entity FreedomAndCoefficient
#[derive(Debug, Clone)]
pub struct StepFeaFreedomAndCoefficient {
    freedom: i32,
    coefficient: Option<f64>,
}

impl StepFeaFreedomAndCoefficient {
    /// Creates a new empty FreedomAndCoefficient
    pub fn new() -> Self {
        StepFeaFreedomAndCoefficient {
            freedom: 0,
            coefficient: None,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, freedom: i32, coefficient: Option<f64>) {
        self.freedom = freedom;
        self.coefficient = coefficient;
    }

    /// Returns field Freedom
    pub fn freedom(&self) -> i32 {
        self.freedom
    }

    /// Set field Freedom
    pub fn set_freedom(&mut self, freedom: i32) {
        self.freedom = freedom;
    }

    /// Returns field A (coefficient)
    pub fn coefficient(&self) -> Option<f64> {
        self.coefficient
    }

    /// Set field A (coefficient)
    pub fn set_coefficient(&mut self, coefficient: Option<f64>) {
        self.coefficient = coefficient;
    }
}

impl Default for StepFeaFreedomAndCoefficient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freedom_and_coefficient_creation() {
        let fc = StepFeaFreedomAndCoefficient::new();
        assert_eq!(fc.freedom(), 0);
        assert_eq!(fc.coefficient(), None);
    }

    #[test]
    fn test_freedom_and_coefficient_init() {
        let mut fc = StepFeaFreedomAndCoefficient::new();
        fc.init(1, Some(1.5));

        assert_eq!(fc.freedom(), 1);
        assert_eq!(fc.coefficient(), Some(1.5));
    }

    #[test]
    fn test_freedom_and_coefficient_setters() {
        let mut fc = StepFeaFreedomAndCoefficient::new();
        fc.set_freedom(2);
        fc.set_coefficient(Some(2.5));

        assert_eq!(fc.freedom(), 2);
        assert_eq!(fc.coefficient(), Some(2.5));
    }
}
