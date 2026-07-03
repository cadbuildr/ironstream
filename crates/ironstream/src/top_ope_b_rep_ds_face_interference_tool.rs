// FILE: top_ope_b_rep_ds_face_interference_tool.rs
// occt: TopOpeBRepDS_FaceInterferenceTool

/// Tool for face interferences
#[derive(Debug, Clone)]
pub struct FaceInterferenceTool {
    /// DS reference
    ds_id: Option<usize>,
}

impl FaceInterferenceTool {
    /// Create new FaceInterferenceTool
    pub fn new() -> Self {
        FaceInterferenceTool { ds_id: None }
    }

    /// Create with data structure
    pub fn with_ds(ds_id: usize) -> Self {
        FaceInterferenceTool {
            ds_id: Some(ds_id),
        }
    }

    /// Get DS reference
    pub fn hds(&self) -> Option<usize> {
        self.ds_id
    }
}

impl Default for FaceInterferenceTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_interference_tool_new() {
        let fit = FaceInterferenceTool::new();
        assert!(fit.hds().is_none());
    }

    #[test]
    fn test_face_interference_tool_with_ds() {
        let fit = FaceInterferenceTool::with_ds(42);
        assert_eq!(fit.hds(), Some(42));
    }
}
