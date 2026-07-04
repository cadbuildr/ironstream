// FILE: iges_dimen_tool_angular_dimension.rs
// occt: IGESDimen_ToolAngularDimension

pub struct IgesDimen_ToolAngularDimension;

impl IgesDimen_ToolAngularDimension {
    pub fn new() -> Self {
        IgesDimen_ToolAngularDimension
    }
}

impl Default for IgesDimen_ToolAngularDimension {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_angular_dimension_creation() {
        let _tool = IgesDimen_ToolAngularDimension::new();
    }
}
