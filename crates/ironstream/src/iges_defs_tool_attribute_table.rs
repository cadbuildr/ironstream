// FILE: iges_defs_tool_attribute_table.rs
// occt: IGESDefs_ToolAttributeTable

//! Tool for attribute tables.

#[derive(Clone, Debug)]
pub struct ToolAttributeTable;

impl ToolAttributeTable {
    pub fn new() -> Self {
        ToolAttributeTable
    }

    pub fn process(&self, id: usize) -> bool {
        true
    }
}

impl Default for ToolAttributeTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tool = ToolAttributeTable::new();
        assert!(tool.process(1));
    }

    #[test]
    fn test_default() {
        let t1 = ToolAttributeTable::new();
        let t2 = ToolAttributeTable::default();
        assert_eq!(format!("{:?}", t1), format!("{:?}", t2));
    }
}
