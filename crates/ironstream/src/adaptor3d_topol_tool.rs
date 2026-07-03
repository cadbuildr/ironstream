// FILE: adaptor3d_topol_tool.rs
// occt: Adaptor3d_TopolTool

pub struct Adaptor3dTopolTool {
    vertex_count: usize,
    edge_count: usize,
}

impl Adaptor3dTopolTool {
    pub fn new() -> Self {
        Adaptor3dTopolTool {
            vertex_count: 0,
            edge_count: 0,
        }
    }

    pub fn get_vertex_count(&self) -> usize {
        self.vertex_count
    }

    pub fn get_edge_count(&self) -> usize {
        self.edge_count
    }

    pub fn set_vertex_count(&mut self, count: usize) {
        self.vertex_count = count;
    }

    pub fn set_edge_count(&mut self, count: usize) {
        self.edge_count = count;
    }
}

impl Default for Adaptor3dTopolTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topol_tool_new() {
        let tool = Adaptor3dTopolTool::new();
        assert_eq!(tool.get_vertex_count(), 0);
        assert_eq!(tool.get_edge_count(), 0);
    }

    #[test]
    fn test_topol_tool_set_counts() {
        let mut tool = Adaptor3dTopolTool::new();
        tool.set_vertex_count(4);
        tool.set_edge_count(6);
        assert_eq!(tool.get_vertex_count(), 4);
        assert_eq!(tool.get_edge_count(), 6);
    }
}
