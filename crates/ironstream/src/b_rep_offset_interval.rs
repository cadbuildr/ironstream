// FILE: b_rep_offset_interval.rs
// occt: BRepOffset_Interval

use crate::ch_fi_ds_type_of_concavity::ChFiDsTypeOfConcavity;

/// Represents an interval on an edge with concavity type.
#[derive(Debug, Clone)]
pub struct BRepOffsetInterval {
    /// First parameter of interval
    first: f64,
    /// Last parameter of interval
    last: f64,
    /// Type of concavity in this interval
    concavity_type: ChFiDsTypeOfConcavity,
}

impl BRepOffsetInterval {
    /// Create a new interval
    pub fn new() -> Self {
        Self {
            first: 0.0,
            last: 0.0,
            concavity_type: ChFiDsTypeOfConcavity::Other,
        }
    }

    /// Create an interval with parameters and type
    pub fn with_params(first: f64, last: f64, concavity_type: ChFiDsTypeOfConcavity) -> Self {
        Self {
            first,
            last,
            concavity_type,
        }
    }

    /// Get first parameter
    pub fn first(&self) -> f64 {
        self.first
    }

    /// Set first parameter
    pub fn set_first(&mut self, u: f64) {
        self.first = u;
    }

    /// Get last parameter
    pub fn last(&self) -> f64 {
        self.last
    }

    /// Set last parameter
    pub fn set_last(&mut self, u: f64) {
        self.last = u;
    }

    /// Get concavity type
    pub fn concavity_type(&self) -> ChFiDsTypeOfConcavity {
        self.concavity_type
    }

    /// Set concavity type
    pub fn set_concavity_type(&mut self, ct: ChFiDsTypeOfConcavity) {
        self.concavity_type = ct;
    }
}

impl Default for BRepOffsetInterval {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let interval = BRepOffsetInterval::new();
        assert_eq!(interval.first(), 0.0);
        assert_eq!(interval.last(), 0.0);
    }

    #[test]
    fn test_with_params() {
        let interval = BRepOffsetInterval::with_params(1.0, 5.0, ChFiDsTypeOfConcavity::Concave);
        assert_eq!(interval.first(), 1.0);
        assert_eq!(interval.last(), 5.0);
        assert_eq!(interval.concavity_type(), ChFiDsTypeOfConcavity::Concave);
    }

    #[test]
    fn test_set_parameters() {
        let mut interval = BRepOffsetInterval::new();
        interval.set_first(2.0);
        interval.set_last(8.0);
        assert_eq!(interval.first(), 2.0);
        assert_eq!(interval.last(), 8.0);
    }

    #[test]
    fn test_set_concavity_type() {
        let mut interval = BRepOffsetInterval::new();
        interval.set_concavity_type(ChFiDsTypeOfConcavity::Convex);
        assert_eq!(interval.concavity_type(), ChFiDsTypeOfConcavity::Convex);
    }

    #[test]
    fn test_default() {
        let interval = BRepOffsetInterval::default();
        assert_eq!(interval.first(), 0.0);
    }
}
