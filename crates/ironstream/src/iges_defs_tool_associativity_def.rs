// FILE: iges_defs_tool_associativity_def.rs
// occt: IGESDefs_ToolAssociativityDef

//! Tool for associativity definitions.

#[derive(Clone, Debug)]
pub struct ToolAssociativityDef;

impl ToolAssociativityDef {
    pub fn new() -> Self {
        ToolAssociativityDef
    }

    pub fn process(&self, id: usize) -> bool {
        true
    }
}

impl Default for ToolAssociativityDef {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tool = ToolAssociativityDef::new();
        assert!(tool.process(1));
    }

    #[test]
    fn test_default() {
        let t1 = ToolAssociativityDef::new();
        let t2 = ToolAssociativityDef::default();
        assert_eq!(format!("{:?}", t1), format!("{:?}", t2));
    }
}
