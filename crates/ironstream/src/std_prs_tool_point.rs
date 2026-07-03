// FILE: std_prs_tool_point.rs
// occt: StdPrs_ToolPoint

/// Tool for point presentation operations.
#[derive(Clone, Debug)]
pub struct ToolPoint {
    pub tool_id: u32,
    pub point_count: usize,
}

impl ToolPoint {
    pub fn new(tool_id: u32) -> Self {
        Self {
            tool_id,
            point_count: 0,
        }
    }

    pub fn add_point(&mut self) {
        self.point_count += 1;
    }

    pub fn nb_points(&self) -> usize {
        self.point_count
    }

    pub fn clear(&mut self) {
        self.point_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_tool() {
        let tool = ToolPoint::new(1);
        assert_eq!(tool.tool_id, 1);
        assert_eq!(tool.nb_points(), 0);
    }

    #[test]
    fn add_points() {
        let mut tool = ToolPoint::new(1);
        for _ in 0..10 {
            tool.add_point();
        }
        assert_eq!(tool.nb_points(), 10);
    }

    #[test]
    fn clear() {
        let mut tool = ToolPoint::new(1);
        tool.add_point();
        tool.clear();
        assert_eq!(tool.nb_points(), 0);
    }
}
