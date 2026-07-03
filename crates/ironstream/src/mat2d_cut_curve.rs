// FILE: mat2d_cut_curve.rs
// occt: MAT2d_CutCurve

/// Cuts a curve at extremas of curvature and inflections.
#[derive(Debug, Clone)]
pub struct Mat2dCutCurve {
    nb_curves: i32,
    unmodified: bool,
}

impl Mat2dCutCurve {
    pub fn new() -> Self {
        Mat2dCutCurve {
            nb_curves: 0,
            unmodified: true,
        }
    }

    pub fn unmodified(&self) -> bool {
        self.unmodified
    }

    pub fn nb_curves(&self) -> i32 {
        self.nb_curves
    }
}

impl Default for Mat2dCutCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cut_curve() {
        let cc = Mat2dCutCurve::new();
        assert!(cc.unmodified());
    }
}
