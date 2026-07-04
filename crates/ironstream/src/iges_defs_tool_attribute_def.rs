// FILE: iges_defs_tool_attribute_def.rs
// occt: IGESDefs_ToolAttributeDef

//! Tool for attribute definitions.

#[derive(Clone, Debug)]
pub struct ToolAttributeDef;

impl ToolAttributeDef {
    pub fn new() -> Self {
        ToolAttributeDef
    }

    pub fn process(&self, id: usize) -> bool {
        true
    }
}

impl Default for ToolAttributeDef {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tool = ToolAttributeDef::new();
        assert!(tool.process(1));
    }

    #[test]
    fn test_default() {
        let t1 = ToolAttributeDef::new();
        let t2 = ToolAttributeDef::default();
        assert_eq!(format!("{:?}", t1), format!("{:?}", t2));
    }
}
