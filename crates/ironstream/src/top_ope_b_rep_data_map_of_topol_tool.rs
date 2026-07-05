// FILE: top_ope_b_rep_data_map_of_topol_tool.rs
// occt: TopOpeBRep_DataMapOfTopolTool

use std::collections::HashMap;

/// ShapeKey: Shape identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShapeKey {
    id: usize,
}

impl ShapeKey {
    pub fn new(id: usize) -> Self {
        ShapeKey { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// TopolTool: Topological tool for shape processing.
#[derive(Clone, Debug)]
pub struct TopolTool {
    tool_id: usize,
}

impl TopolTool {
    pub fn new(tool_id: usize) -> Self {
        TopolTool { tool_id }
    }

    pub fn tool_id(&self) -> usize {
        self.tool_id
    }
}

/// DataMapOfTopolTool: Maps Shape to TopolTool.
#[derive(Clone, Debug)]
pub struct DataMapOfTopolTool {
    data: HashMap<ShapeKey, TopolTool>,
}

impl DataMapOfTopolTool {
    pub fn new() -> Self {
        DataMapOfTopolTool {
            data: HashMap::new(),
        }
    }

    pub fn bind(&mut self, shape: ShapeKey, tool: TopolTool) -> bool {
        self.data.insert(shape, tool).is_none()
    }

    pub fn contains(&self, shape: &ShapeKey) -> bool {
        self.data.contains_key(shape)
    }

    pub fn find(&self, shape: &ShapeKey) -> Option<&TopolTool> {
        self.data.get(shape)
    }

    pub fn remove(&mut self, shape: &ShapeKey) -> bool {
        self.data.remove(shape).is_some()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ShapeKey, &TopolTool)> {
        self.data.iter()
    }
}

impl Default for DataMapOfTopolTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topol_tool() {
        let tool = TopolTool::new(99);
        assert_eq!(tool.tool_id(), 99);
    }

    #[test]
    fn test_data_map_bind() {
        let mut map = DataMapOfTopolTool::new();
        let shape = ShapeKey::new(5);
        assert!(map.bind(shape.clone(), TopolTool::new(50)));
        assert!(!map.bind(shape, TopolTool::new(51)));
    }

    #[test]
    fn test_data_map_find() {
        let mut map = DataMapOfTopolTool::new();
        let shape = ShapeKey::new(3);
        map.bind(shape.clone(), TopolTool::new(30));
        let found = map.find(&shape).unwrap();
        assert_eq!(found.tool_id(), 30);
    }
}
