// FILE: b_rep_top_adaptor_map_of_shape_tool.rs
// occt: BRepTopAdaptor_MapOfShapeTool

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ShapeTool {
    shape_id: usize,
    tool_type: String,
}

impl ShapeTool {
    pub fn new(shape_id: usize, tool_type: String) -> Self {
        ShapeTool { shape_id, tool_type }
    }

    pub fn shape_id(&self) -> usize {
        self.shape_id
    }

    pub fn tool_type(&self) -> &str {
        &self.tool_type
    }
}

pub struct BrepTopAdaptorMapOfShapeTool {
    data: HashMap<usize, ShapeTool>,
}

impl BrepTopAdaptorMapOfShapeTool {
    pub fn new() -> Self {
        BrepTopAdaptorMapOfShapeTool {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: usize, tool: ShapeTool) {
        self.data.insert(key, tool);
    }

    pub fn get(&self, key: usize) -> Option<&ShapeTool> {
        self.data.get(&key)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for BrepTopAdaptorMapOfShapeTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let tool = ShapeTool::new(1, "edge".to_string());
        assert_eq!(tool.shape_id(), 1);
        assert_eq!(tool.tool_type(), "edge");
    }

    #[test]
    fn test_map_add_get() {
        let mut map = BrepTopAdaptorMapOfShapeTool::new();
        let tool = ShapeTool::new(5, "face".to_string());
        map.add(1, tool);
        assert_eq!(map.get(1).unwrap().shape_id(), 5);
    }
}
