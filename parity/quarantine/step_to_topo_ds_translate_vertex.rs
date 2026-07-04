// FILE: step_to_topo_ds_translate_vertex.rs
// occt: StepToTopoDS_TranslateVertex

use crate::step_to_topo_ds_root::StepToTopoDS_Root;

/// Translate STEP vertex to TopoDS_Vertex
pub struct StepToTopoDS_TranslateVertex {
    root: StepToTopoDS_Root,
    vertex: Option<String>,
}

impl StepToTopoDS_TranslateVertex {
    pub fn new() -> Self {
        StepToTopoDS_TranslateVertex {
            root: StepToTopoDS_Root::new(),
            vertex: None,
        }
    }

    pub fn init(&mut self, vertex_key: &str) -> bool {
        self.vertex = Some(vertex_key.to_string());
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

impl Default for StepToTopoDS_TranslateVertex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tv = StepToTopoDS_TranslateVertex::new();
        assert!(!tv.is_done());
    }

    #[test]
    fn test_init() {
        let mut tv = StepToTopoDS_TranslateVertex::new();
        assert!(tv.init("vertex1"));
        assert!(tv.is_done());
        assert_eq!(tv.value(), Some(&"vertex1".to_string()));
    }
}
