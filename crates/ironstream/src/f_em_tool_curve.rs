// FILE: f_em_tool_curve.rs
// occt: FEmTool_Curve

/// Curve representation in FEmTool.
pub struct FemToolCurve {
    degree: usize,
}

impl FemToolCurve {
    pub fn new(degree: usize) -> Self {
        FemToolCurve { degree }
    }

    pub fn degree(&self) -> usize {
        self.degree
    }

    pub fn evaluate(&self, u: f64) -> f64 {
        u.powi(self.degree as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let curve = FemToolCurve::new(3);
        assert_eq!(curve.degree(), 3);
    }

    #[test]
    fn test_evaluate() {
        let curve = FemToolCurve::new(2);
        let v = curve.evaluate(2.0);
        assert_eq!(v, 4.0);
    }
}
