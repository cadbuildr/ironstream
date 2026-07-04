// FILE: step_to_topo_ds_translate_composite_curve.rs
// occt: StepToTopoDS_TranslateCompositeCurve

use crate::step_to_topo_ds_root::StepToTopoDS_Root;

/// Translate STEP entity composite_curve to TopoDS_Wire
/// If surface is given, the curve is assumed to lie on that surface and in case
/// if any segment of it is a curve_on_surface, the pcurve for that segment will be taken.
pub struct StepToTopoDS_TranslateCompositeCurve {
    root: StepToTopoDS_Root,
    wire: Option<String>,
    infinite_segment: bool,
}

impl StepToTopoDS_TranslateCompositeCurve {
    pub fn new() -> Self {
        StepToTopoDS_TranslateCompositeCurve {
            root: StepToTopoDS_Root::new(),
            wire: None,
            infinite_segment: false,
        }
    }

    pub fn init(&mut self, cc_key: &str) -> bool {
        // TODO: Real implementation requires StepGeom_CompositeCurve handling
        self.wire = Some(cc_key.to_string());
        self.root.set_done(true);
        true
    }

    pub fn init_with_surface(&mut self, cc_key: &str, _surf_key: &str) -> bool {
        // TODO: Real implementation requires surface integration
        self.wire = Some(cc_key.to_string());
        self.root.set_done(true);
        true
    }

    pub fn value(&self) -> Option<&String> {
        self.wire.as_ref()
    }

    pub fn is_infinite_segment(&self) -> bool {
        self.infinite_segment
    }

    pub fn set_infinite_segment(&mut self, infinite: bool) {
        self.infinite_segment = infinite;
    }

    pub fn is_done(&self) -> bool {
        self.root.is_done()
    }

    pub fn set_done(&mut self, done: bool) {
        self.root.set_done(done);
    }

    pub fn precision(&self) -> f64 {
        self.root.precision()
    }

    pub fn set_precision(&mut self, preci: f64) {
        self.root.set_precision(preci);
    }
}

impl Default for StepToTopoDS_TranslateCompositeCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tc = StepToTopoDS_TranslateCompositeCurve::new();
        assert!(!tc.is_infinite_segment());
        assert!(!tc.is_done());
    }

    #[test]
    fn test_init() {
        let mut tc = StepToTopoDS_TranslateCompositeCurve::new();
        assert!(tc.init("cc1"));
        assert!(tc.is_done());
        assert_eq!(tc.value(), Some(&"cc1".to_string()));
    }

    #[test]
    fn test_infinite_segment() {
        let mut tc = StepToTopoDS_TranslateCompositeCurve::new();
        tc.set_infinite_segment(true);
        assert!(tc.is_infinite_segment());
    }

    #[test]
    fn test_precision() {
        let mut tc = StepToTopoDS_TranslateCompositeCurve::new();
        tc.set_precision(0.001);
        assert_eq!(tc.precision(), 0.001);
    }
}
