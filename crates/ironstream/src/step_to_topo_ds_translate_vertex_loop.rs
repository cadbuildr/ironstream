// FILE: step_to_topo_ds_translate_vertex_loop.rs
// occt: StepToTopoDS_TranslateVertexLoop

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

/// Translate STEP vertex loop to TopoDS_Vertex
pub struct StepToTopoDS_TranslateVertexLoop {
    root: StepToTopoDS_Root,
    vertex: Option<String>,
}

impl StepToTopoDS_TranslateVertexLoop {
    pub fn new() -> Self {
        StepToTopoDS_TranslateVertexLoop {
            root: StepToTopoDS_Root::new(),
            vertex: None,
        }
    }

    pub fn init(&mut self, loop_key: &str) -> bool {
        self.vertex = Some(loop_key.to_string());
        self.root.set_done(true);
        true
    }

    pub fn value(&self) -> Option<&String> {
        self.vertex.as_ref()
    }

    pub fn is_done(&self) -> bool {
        self.root.is_done()
    }
}

impl Default for StepToTopoDS_TranslateVertexLoop {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tvl = StepToTopoDS_TranslateVertexLoop::new();
        assert!(!tvl.is_done());
    }

    #[test]
    fn test_init() {
        let mut tvl = StepToTopoDS_TranslateVertexLoop::new();
        assert!(tvl.init("loop1"));
        assert!(tvl.is_done());
        assert_eq!(tvl.value(), Some(&"loop1".to_string()));
    }
}
