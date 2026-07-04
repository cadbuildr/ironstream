// FILE: step_to_topo_ds_translate_edge.rs
// occt: StepToTopoDS_TranslateEdge

use crate::step_to_topo_ds_root::StepToTopoDS_Root;

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
