// FILE: step_to_topo_ds_translate_edge_loop.rs
// occt: StepToTopoDS_TranslateEdgeLoop

use crate::step_to_topo_ds_root::StepToTopoDS_Root;

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
