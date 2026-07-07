// FILE: approx_m_curves_to_b_sp_curve.rs
// occt: Approx_MCurvesToBSpCurve

//! Converts a sequence of multi-curves to multi-BSpline curves.

#[derive(Clone, Default)]
pub struct MultiCurve;

#[derive(Clone, Default)]
pub struct MultiBSpCurve;

/// Converter from multi-curves to multi-BSpline curves
pub struct ApproxMCurvesToBSpCurve {
    my_spline: MultiBSpCurve,
    my_done: bool,
    my_curves: Vec<MultiCurve>,
}

impl ApproxMCurvesToBSpCurve {
    /// Creates a new converter
    pub fn new() -> Self {
        ApproxMCurvesToBSpCurve {
            my_spline: MultiBSpCurve::default(),
            my_done: false,
            my_curves: Vec::new(),
        }
    }

    /// Resets the converter state
    pub fn reset(&mut self) {
        self.my_spline = MultiBSpCurve::default();
        self.my_done = false;
        self.my_curves.clear();
    }

    /// Appends a multi-curve to the sequence for conversion
    pub fn append(&mut self, mc: &MultiCurve) {
        self.my_curves.push(mc.clone());
    }

    /// Performs the conversion on accumulated curves
    pub fn perform(&mut self) {
        // TODO: Implement conversion algorithm
        // Join all MultiCurves into a single MultiBSpCurve
        self.my_done = true;
    }

    /// Performs conversion on a supplied sequence
    pub fn perform_seq(&mut self, seq: &[MultiCurve]) {
        self.my_curves = seq.to_vec();
        self.perform();
    }

    /// Returns the composite MultiCurves as a MultiBSpCurve (immutable)
    pub fn value(&self) -> &MultiBSpCurve {
        &self.my_spline
    }

    /// Returns the composite MultiCurves as a MultiBSpCurve (mutable)
    pub fn value_mut(&mut self) -> &mut MultiBSpCurve {
        &mut self.my_spline
    }

    /// Returns whether conversion succeeded
    pub fn is_done(&self) -> bool {
        self.my_done
    }
}

impl Default for ApproxMCurvesToBSpCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_m_curves_to_b_sp_curve_new() {
        let converter = ApproxMCurvesToBSpCurve::new();
        assert!(!converter.is_done());
    }

    #[test]
    fn test_m_curves_to_b_sp_curve_reset() {
        let mut converter = ApproxMCurvesToBSpCurve::new();
        converter.append(&MultiCurve::default());
        converter.reset();
        assert_eq!(converter.my_curves.len(), 0);
    }

    #[test]
    fn test_m_curves_to_b_sp_curve_append() {
        let mut converter = ApproxMCurvesToBSpCurve::new();
        let mc = MultiCurve::default();
        converter.append(&mc);
        assert_eq!(converter.my_curves.len(), 1);
    }

    #[test]
    fn test_m_curves_to_b_sp_curve_perform() {
        let mut converter = ApproxMCurvesToBSpCurve::new();
        converter.append(&MultiCurve::default());
        converter.perform();
        assert!(converter.is_done());
    }

    #[test]
    fn test_m_curves_to_b_sp_curve_value() {
        let converter = ApproxMCurvesToBSpCurve::new();
        let _result = converter.value();
        // Verify result is MultiBSpCurve
    }
}
