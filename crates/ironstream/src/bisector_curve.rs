// FILE: bisector_curve.rs
// occt: Bisector_Curve

/// Abstract base class for bisector curves.
#[derive(Debug, Clone)]
pub struct BisectorCurve;

impl BisectorCurve {
    pub fn new() -> Self {
        BisectorCurve
    }
}

impl Default for BisectorCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bisector_curve() {
        let _bc = BisectorCurve::new();
    }
}
