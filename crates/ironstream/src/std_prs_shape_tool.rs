// FILE: std_prs_shape_tool.rs
// occt: StdPrs_ShapeTool

/// Tool for shape presentation operations.
#[derive(Clone, Debug)]
pub struct ShapeTool {
    pub tool_id: u32,
    pub edge_count: usize,
    pub face_count: usize,
}

impl ShapeTool {
    pub fn new(tool_id: u32) -> Self {
        Self {
            tool_id,
            edge_count: 0,
            face_count: 0,
        }
    }

    pub fn add_edge(&mut self) {
        self.edge_count += 1;
    }

    pub fn add_face(&mut self) {
        self.face_count += 1;
    }

    pub fn nb_edges(&self) -> usize {
        self.edge_count
    }

    pub fn nb_faces(&self) -> usize {
        self.face_count
    }

    pub fn total_elements(&self) -> usize {
        self.edge_count + self.face_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_tool() {
        let tool = ShapeTool::new(1);
        assert_eq!(tool.tool_id, 1);
        assert_eq!(tool.total_elements(), 0);
    }

    #[test]
    fn add_elements() {
        let mut tool = ShapeTool::new(1);
        tool.add_edge();
        tool.add_edge();
        tool.add_face();
        assert_eq!(tool.nb_edges(), 2);
        assert_eq!(tool.nb_faces(), 1);
        assert_eq!(tool.total_elements(), 3);
    }
}
