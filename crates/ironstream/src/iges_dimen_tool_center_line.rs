// FILE: iges_dimen_tool_center_line.rs
// occt: IGESDimen_ToolCenterLine

pub struct IgesDimen_ToolCenterLine;

impl IgesDimen_ToolCenterLine {
    pub fn new() -> Self {
        IgesDimen_ToolCenterLine
    }
}

impl Default for IgesDimen_ToolCenterLine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let _tool = IgesDimen_ToolCenterLine::new();
    }
}
