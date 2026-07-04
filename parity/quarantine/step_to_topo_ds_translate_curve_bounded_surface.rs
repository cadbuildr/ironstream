// FILE: step_to_topo_ds_translate_curve_bounded_surface.rs
// occt: StepToTopoDS_TranslateCurveBoundedSurface

use crate::step_to_topo_ds_root::StepToTopoDS_Root;

/// Translate curve_bounded_surface into TopoDS_Face
pub struct StepToTopoDS_TranslateCurveBoundedSurface {
    root: StepToTopoDS_Root,
    face: Option<String>,
}

impl StepToTopoDS_TranslateCurveBoundedSurface {
    pub fn new() -> Self {
        StepToTopoDS_TranslateCurveBoundedSurface {
            root: StepToTopoDS_Root::new(),
            face: None,
        }
    }

    pub fn init(&mut self, cbs_key: &str) -> bool {
        // TODO: Real implementation requires StepGeom_CurveBoundedSurface handling
        self.face = Some(cbs_key.to_string());
        self.root.set_done(true);
        true
    }

    pub fn value(&self) -> Option<&String> {
        self.face.as_ref()
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

impl Default for StepToTopoDS_TranslateCurveBoundedSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tc = StepToTopoDS_TranslateCurveBoundedSurface::new();
        assert!(!tc.is_done());
    }

    #[test]
    fn test_init() {
        let mut tc = StepToTopoDS_TranslateCurveBoundedSurface::new();
        assert!(tc.init("cbs1"));
        assert!(tc.is_done());
        assert_eq!(tc.value(), Some(&"cbs1".to_string()));
    }

    #[test]
    fn test_precision() {
        let mut tc = StepToTopoDS_TranslateCurveBoundedSurface::new();
        tc.set_precision(0.01);
        assert_eq!(tc.precision(), 0.01);
    }
}
