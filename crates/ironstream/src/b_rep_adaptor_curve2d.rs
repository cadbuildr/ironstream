// FILE: b_rep_adaptor_curve2d.rs
// occt: BRepAdaptor_Curve2d

/// Minimal implementation of BRepAdaptor_Curve2d
pub struct BRepAdaptorCurve2d {}

impl Default for BRepAdaptorCurve2d {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepAdaptorCurve2d::default();
    }
}
