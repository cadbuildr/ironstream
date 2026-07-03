// FILE: hatch_parameter.rs
// occt: Hatch_Parameter

/// Stores an intersection parameter on a line with start/stop flags and index.
/// Represents a point on a hatch line with parametric position and interval information.
#[derive(Clone, Copy, Debug)]
pub struct HatchParameter {
    par1: f64,
    start: bool,
    index: i32,
    par2: f64,
}

impl HatchParameter {
    /// Create with default values
    pub fn new() -> Self {
        HatchParameter {
            par1: 0.0,
            start: false,
            index: 0,
            par2: 0.0,
        }
    }

    /// Create with explicit values
    pub fn with_values(par1: f64, start: bool, index: i32, par2: f64) -> Self {
        HatchParameter {
            par1,
            start,
            index,
            par2,
        }
    }

    /// Get the first parameter value
    pub fn par1(&self) -> f64 {
        self.par1
    }

    /// Get the start flag (true if this starts an interval)
    pub fn start(&self) -> bool {
        self.start
    }

    /// Get the index value
    pub fn index(&self) -> i32 {
        self.index
    }

    /// Get the second parameter value
    pub fn par2(&self) -> f64 {
        self.par2
    }
}

impl Default for HatchParameter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_construction() {
        let param = HatchParameter::new();
        assert_eq!(param.par1(), 0.0);
        assert_eq!(param.start(), false);
        assert_eq!(param.index(), 0);
        assert_eq!(param.par2(), 0.0);
    }

    #[test]
    fn test_with_values() {
        let param = HatchParameter::with_values(0.5, true, 42, 1.5);
        assert_eq!(param.par1(), 0.5);
        assert_eq!(param.start(), true);
        assert_eq!(param.index(), 42);
        assert_eq!(param.par2(), 1.5);
    }

    #[test]
    fn test_clone() {
        let param = HatchParameter::with_values(0.3, false, 10, 0.7);
        let param2 = param;
        assert_eq!(param2.par1(), 0.3);
        assert_eq!(param2.start(), false);
        assert_eq!(param2.index(), 10);
    }

    #[test]
    fn test_default_trait() {
        let param = HatchParameter::default();
        assert_eq!(param.par1(), 0.0);
        assert_eq!(param.start(), false);
    }
}
