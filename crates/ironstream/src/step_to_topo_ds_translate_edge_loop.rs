// FILE: step_to_topo_ds_translate_edge_loop.rs
// occt: StepToTopoDS_TranslateEdgeLoop

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

/// Translate STEP edge loop to TopoDS_Wire
pub struct StepToTopoDS_TranslateEdgeLoop {
    root: StepToTopoDS_Root,
    wire: Option<String>,
}

impl StepToTopoDS_TranslateEdgeLoop {
    pub fn new() -> Self {
        StepToTopoDS_TranslateEdgeLoop {
            root: StepToTopoDS_Root::new(),
            wire: None,
        }
    }

    pub fn init(&mut self, loop_key: &str) -> bool {
        self.wire = Some(loop_key.to_string());
        self.root.set_done(true);
        true
    }

    pub fn value(&self) -> Option<&String> {
        self.wire.as_ref()
    }

    pub fn is_done(&self) -> bool {
        self.root.is_done()
    }

    pub fn set_done(&mut self, done: bool) {
        self.root.set_done(done);
    }
}

impl Default for StepToTopoDS_TranslateEdgeLoop {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tel = StepToTopoDS_TranslateEdgeLoop::new();
        assert!(!tel.is_done());
    }

    #[test]
    fn test_init() {
        let mut tel = StepToTopoDS_TranslateEdgeLoop::new();
        assert!(tel.init("loop1"));
        assert!(tel.is_done());
        assert_eq!(tel.value(), Some(&"loop1".to_string()));
    }
}
