// FILE: iges_solid_tool_edge_list.rs
// occt: IGESSolid_ToolEdgeList

pub struct IGESSolidToolEdgeList;

impl IGESSolidToolEdgeList {
    pub fn new() -> Self {
        IGESSolidToolEdgeList
    }

    pub fn label(&self) -> &str {
        "EdgeList"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _t = IGESSolidToolEdgeList::new();
    }

    #[test]
    fn test_label() {
        assert_eq!(IGESSolidToolEdgeList::new().label(), "EdgeList");
    }
}
