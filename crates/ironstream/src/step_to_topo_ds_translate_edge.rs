// FILE: step_to_topo_ds_translate_edge.rs
// occt: StepToTopoDS_TranslateEdge

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

/// Translate STEP edge to TopoDS_Edge
pub struct StepToTopoDS_TranslateEdge {
    root: StepToTopoDS_Root,
    edge: Option<String>,
}

impl StepToTopoDS_TranslateEdge {
    pub fn new() -> Self {
        StepToTopoDS_TranslateEdge {
            root: StepToTopoDS_Root::new(),
            edge: None,
        }
    }

    pub fn init(&mut self, edge_key: &str) -> bool {
        self.edge = Some(edge_key.to_string());
        self.root.set_done(true);
        true
    }

    pub fn value(&self) -> Option<&String> {
        self.edge.as_ref()
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

impl Default for StepToTopoDS_TranslateEdge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let te = StepToTopoDS_TranslateEdge::new();
        assert!(!te.is_done());
    }

    #[test]
    fn test_init() {
        let mut te = StepToTopoDS_TranslateEdge::new();
        assert!(te.init("edge1"));
        assert!(te.is_done());
        assert_eq!(te.value(), Some(&"edge1".to_string()));
    }
}
