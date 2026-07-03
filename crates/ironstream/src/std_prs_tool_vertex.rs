// FILE: std_prs_tool_vertex.rs
// occt: StdPrs_ToolVertex

/// Tool for vertex presentation operations.
#[derive(Clone, Debug)]
pub struct ToolVertex {
    pub tool_id: u32,
    pub vertex_count: usize,
}

impl ToolVertex {
    pub fn new(tool_id: u32) -> Self {
        Self {
            tool_id,
            vertex_count: 0,
        }
    }

    pub fn add_vertex(&mut self) {
        self.vertex_count += 1;
    }

    pub fn nb_vertices(&self) -> usize {
        self.vertex_count
    }

    pub fn clear(&mut self) {
        self.vertex_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_tool() {
        let tool = ToolVertex::new(1);
        assert_eq!(tool.tool_id, 1);
        assert_eq!(tool.nb_vertices(), 0);
    }

    #[test]
    fn add_vertices() {
        let mut tool = ToolVertex::new(1);
        for _ in 0..8 {
            tool.add_vertex();
        }
        assert_eq!(tool.nb_vertices(), 8);
    }

    #[test]
    fn clear() {
        let mut tool = ToolVertex::new(1);
        tool.add_vertex();
        tool.clear();
        assert_eq!(tool.nb_vertices(), 0);
    }
}
