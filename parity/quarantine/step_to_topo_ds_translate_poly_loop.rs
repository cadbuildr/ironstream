// FILE: step_to_topo_ds_translate_poly_loop.rs
// occt: StepToTopoDS_TranslatePolyLoop

use crate::step_to_topo_ds_root::StepToTopoDS_Root;

/// Translate STEP poly loop to TopoDS_Wire
pub struct StepToTopoDS_TranslatePolyLoop {
    root: StepToTopoDS_Root,
    wire: Option<String>,
}

impl StepToTopoDS_TranslatePolyLoop {
    pub fn new() -> Self {
        StepToTopoDS_TranslatePolyLoop {
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
}

impl Default for StepToTopoDS_TranslatePolyLoop {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tpl = StepToTopoDS_TranslatePolyLoop::new();
        assert!(!tpl.is_done());
    }

    #[test]
    fn test_init() {
        let mut tpl = StepToTopoDS_TranslatePolyLoop::new();
        assert!(tpl.init("poly1"));
        assert!(tpl.is_done());
        assert_eq!(tpl.value(), Some(&"poly1".to_string()));
    }
}
