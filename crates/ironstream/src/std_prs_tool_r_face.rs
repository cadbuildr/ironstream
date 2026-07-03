// FILE: std_prs_tool_r_face.rs
// occt: StdPrs_ToolRFace

/// Tool for restricted face operations.
#[derive(Clone, Debug)]
pub struct ToolRFace {
    pub tool_id: u32,
    pub face_count: usize,
}

impl ToolRFace {
    pub fn new(tool_id: u32) -> Self {
        Self {
            tool_id,
            face_count: 0,
        }
    }

    pub fn add_face(&mut self) {
        self.face_count += 1;
    }

    pub fn nb_faces(&self) -> usize {
        self.face_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_tool() {
        let tool = ToolRFace::new(1);
        assert_eq!(tool.tool_id, 1);
        assert_eq!(tool.nb_faces(), 0);
    }

    #[test]
    fn add_faces() {
        let mut tool = ToolRFace::new(1);
        tool.add_face();
        tool.add_face();
        assert_eq!(tool.nb_faces(), 2);
    }
}
