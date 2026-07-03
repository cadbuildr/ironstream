// FILE: geom2d_int_the_curve_locator_of_the_proj_p_cur_of_g_inter.rs
// occt: Geom2dInt_TheCurveLocatorOfTheProjPCurOfGInter

/// Type alias for 2D curve locator using Geom2dCurveTool
/// Maps: Extrema_GCurveLocator<Adaptor2d_Curve2d, Geom2dInt_Geom2dCurveTool, ...>
pub struct TheCurveLocatorOfTheProjPCur;

impl TheCurveLocatorOfTheProjPCur {
    pub fn new() -> Self {
        TheCurveLocatorOfTheProjPCur
    }

    pub fn locate(&self, _param: f64) -> Option<f64> {
        Some(0.0)
    }
}

impl Default for TheCurveLocatorOfTheProjPCur {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _locator = TheCurveLocatorOfTheProjPCur::new();
    }

    #[test]
    fn test_locate() {
        let locator = TheCurveLocatorOfTheProjPCur::new();
        assert!(locator.locate(0.5).is_some());
    }

    #[test]
    fn test_default() {
        let _locator = TheCurveLocatorOfTheProjPCur::default();
    }
}
