// FILE: std_select_b_rep_selection_tool.rs
// occt: StdSelect_BRepSelectionTool

//! Tool for selection operations on BRep shapes.

#[derive(Clone, Debug)]
pub struct BRepSelectionTool;

impl BRepSelectionTool {
    pub fn new() -> Self {
        BRepSelectionTool
    }

    pub fn version() -> &'static str {
        "1.0"
    }

    pub fn description() -> &'static str {
        "BRep shape selection tool"
    }

    pub fn is_valid_edge_for_selection(edge_id: u32) -> bool {
        edge_id > 0
    }

    pub fn is_valid_face_for_selection(face_id: u32) -> bool {
        face_id > 0
    }

    pub fn max_selection_size() -> usize {
        10000
    }
}

impl Default for BRepSelectionTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brep_selection_tool_creation() {
        let tool = BRepSelectionTool::new();
        assert_eq!(BRepSelectionTool::version(), "1.0");
    }

    #[test]
    fn brep_selection_tool_validation() {
        assert!(BRepSelectionTool::is_valid_edge_for_selection(1));
        assert!(!BRepSelectionTool::is_valid_edge_for_selection(0));
        assert!(BRepSelectionTool::is_valid_face_for_selection(1));
    }

    #[test]
    fn brep_selection_tool_limits() {
        assert!(BRepSelectionTool::max_selection_size() > 0);
    }
}
