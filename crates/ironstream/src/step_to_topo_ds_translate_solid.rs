// FILE: step_to_topo_ds_translate_solid.rs
// occt: StepToTopoDS_TranslateSolid

/// Local helper mirroring StepToTopoDS_Root (external plumbing).
/// OCCT ctor: done=false, myPrecision = myMaxTol = Precision::Confusion() (1e-7).
#[allow(non_camel_case_types)]
pub struct StepToTopoDS_Root {
    done: bool,
    precision: f64,
    max_tol: f64,
}

impl StepToTopoDS_Root {
    pub fn new() -> Self {
        StepToTopoDS_Root {
            done: false,
            precision: 1.0e-7,
            max_tol: 1.0e-7,
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn set_done(&mut self, done: bool) {
        self.done = done;
    }

    pub fn precision(&self) -> f64 {
        self.precision
    }

    pub fn set_precision(&mut self, preci: f64) {
        self.precision = preci;
    }

    pub fn max_tol(&self) -> f64 {
        self.max_tol
    }

    pub fn set_max_tol(&mut self, maxtol: f64) {
        self.max_tol = maxtol;
    }
}

/// Translate STEP solid to TopoDS_Solid
pub struct StepToTopoDS_TranslateSolid {
    root: StepToTopoDS_Root,
    solid: Option<String>,
}

impl StepToTopoDS_TranslateSolid {
    pub fn new() -> Self {
        StepToTopoDS_TranslateSolid {
            root: StepToTopoDS_Root::new(),
            solid: None,
        }
    }

    pub fn init(&mut self, solid_key: &str) -> bool {
        self.solid = Some(solid_key.to_string());
        self.root.set_done(true);
        true
    }

    pub fn value(&self) -> Option<&String> {
        self.solid.as_ref()
    }

    pub fn is_done(&self) -> bool {
        self.root.is_done()
    }
}

impl Default for StepToTopoDS_TranslateSolid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ts = StepToTopoDS_TranslateSolid::new();
        assert!(!ts.is_done());
    }

    #[test]
    fn test_init() {
        let mut ts = StepToTopoDS_TranslateSolid::new();
        assert!(ts.init("solid1"));
        assert!(ts.is_done());
        assert_eq!(ts.value(), Some(&"solid1".to_string()));
    }
}
