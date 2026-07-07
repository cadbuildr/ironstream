// FILE: iges_appli_tool_node.rs
// occt: IGESAppli_ToolNode

#[derive(Clone, Debug)]
pub struct IgesAppliToolNode;

impl IgesAppliToolNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for IgesAppliToolNode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IgesAppliToolNode::new();
    }
}
