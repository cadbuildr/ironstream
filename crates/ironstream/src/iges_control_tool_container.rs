// FILE: iges_control_tool_container.rs
// occt: IGESControl_ToolContainer

/// Container for IGES tools.
pub struct IgesControlToolContainer;

impl IgesControlToolContainer {
    pub fn new() -> Self {
        Self
    }

    pub fn get_read_tool(&self) -> String {
        "ReadTool".to_string()
    }

    pub fn get_write_tool(&self) -> String {
        "WriteTool".to_string()
    }

    pub fn get_transfer_tool(&self) -> String {
        "TransferTool".to_string()
    }
}

impl Default for IgesControlToolContainer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_read_tool() {
        let container = IgesControlToolContainer::new();
        assert_eq!(container.get_read_tool(), "ReadTool");
    }

    #[test]
    fn test_get_write_tool() {
        let container = IgesControlToolContainer::new();
        assert_eq!(container.get_write_tool(), "WriteTool");
    }

    #[test]
    fn test_get_transfer_tool() {
        let container = IgesControlToolContainer::new();
        assert_eq!(container.get_transfer_tool(), "TransferTool");
    }
}
