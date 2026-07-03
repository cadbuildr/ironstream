// FILE: b_rep_top_adaptor_tool.rs
// occt: BRepTopAdaptor_Tool

/// Tool for adapting BRep topological elements to surface intersection algorithms.
#[derive(Debug, Clone)]
pub struct BRepTopAdaptorTool {
    myloaded: bool,
}

impl BRepTopAdaptorTool {
    pub fn new() -> Self {
        BRepTopAdaptorTool { myloaded: false }
    }

    pub fn is_loaded(&self) -> bool {
        self.myloaded
    }
}

impl Default for BRepTopAdaptorTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool() {
        let tool = BRepTopAdaptorTool::new();
        assert!(!tool.is_loaded());
    }
}
