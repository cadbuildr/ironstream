// FILE: std_prs_hlr_tool_shape.rs
// occt: StdPrs_HLRToolShape

/// Tool for HLR shape operations.
#[derive(Clone, Debug)]
pub struct HlrToolShape {
    pub tool_id: u32,
    pub shape_count: usize,
}

impl HlrToolShape {
    pub fn new(tool_id: u32) -> Self {
        Self {
            tool_id,
            shape_count: 0,
        }
    }

    pub fn add_shape(&mut self) {
        self.shape_count += 1;
    }

    pub fn nb_shapes(&self) -> usize {
        self.shape_count
    }

    pub fn clear(&mut self) {
        self.shape_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_tool() {
        let tool = HlrToolShape::new(1);
        assert_eq!(tool.tool_id, 1);
        assert_eq!(tool.nb_shapes(), 0);
    }

    #[test]
    fn add_shapes() {
        let mut tool = HlrToolShape::new(1);
        tool.add_shape();
        tool.add_shape();
        tool.add_shape();
        assert_eq!(tool.nb_shapes(), 3);
    }

    #[test]
    fn clear() {
        let mut tool = HlrToolShape::new(1);
        tool.add_shape();
        tool.clear();
        assert_eq!(tool.nb_shapes(), 0);
    }
}
