// FILE: step_to_topo_ds_translate_vertex_loop.rs
// occt: StepToTopoDS_TranslateVertexLoop

use crate::step_to_topo_ds_root::StepToTopoDS_Root;

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
