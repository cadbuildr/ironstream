// FILE: iges_dimen_tool_basic_dimension.rs
// occt: IGESDimen_ToolBasicDimension

pub struct IgesDimen_ToolBasicDimension;

impl IgesDimen_ToolBasicDimension {
    pub fn new() -> Self {
        IgesDimen_ToolBasicDimension
    }
}

impl Default for IgesDimen_ToolBasicDimension {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let _tool = IgesDimen_ToolBasicDimension::new();
    }
}
