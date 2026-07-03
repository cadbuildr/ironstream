// FILE: math_function_sample.rs
// occt: math_FunctionSample

/// Sample/record of a function value.
#[derive(Debug, Clone)]
pub struct FunctionSample {
    pub param: f64,      // Parameter value
    pub value: f64,      // Function value
    pub derivative: f64, // Derivative value
}

impl FunctionSample {
    /// Create new sample.
    pub fn new(param: f64, value: f64, derivative: f64) -> Self {
        FunctionSample {
            param,
            value,
            derivative,
        }
    }

    /// Create sample with zero derivative.
    pub fn with_value(param: f64, value: f64) -> Self {
        FunctionSample {
            param,
            value,
            derivative: 0.0,
        }
    }

    /// Get parameter.
    pub fn get_param(&self) -> f64 {
        self.param
    }

    /// Get value.
    pub fn get_value(&self) -> f64 {
        self.value
    }

    /// Get derivative.
    pub fn get_derivative(&self) -> f64 {
        self.derivative
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_sample_new() {
        let sample = FunctionSample::new(1.0, 2.0, 3.0);
        assert_eq!(sample.get_param(), 1.0);
        assert_eq!(sample.get_value(), 2.0);
        assert_eq!(sample.get_derivative(), 3.0);
    }

    #[test]
    fn test_function_sample_with_value() {
        let sample = FunctionSample::with_value(1.5, 4.5);
        assert_eq!(sample.get_param(), 1.5);
        assert_eq!(sample.get_value(), 4.5);
        assert_eq!(sample.get_derivative(), 0.0);
    }
}
