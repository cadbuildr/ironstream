// FILE: topo_ds_to_step_tool.rs
// occt: TopoDSToStep_Tool

/// Tool class for TopoDSToStep conversion process.
pub struct Tool {
    faceted_context: bool,
    pc_curve_mode: i32,
}

impl Tool {
    pub fn new() -> Self {
        Tool {
            faceted_context: false,
            pc_curve_mode: 0,
        }
    }

    pub fn is_faceted(&self) -> bool {
        self.faceted_context
    }

    pub fn set_faceted(&mut self, faceted: bool) {
        self.faceted_context = faceted;
    }

    pub fn pc_curve_mode(&self) -> i32 {
        self.pc_curve_mode
    }

    pub fn set_pc_curve_mode(&mut self, mode: i32) {
        self.pc_curve_mode = mode;
    }
}

impl Default for Tool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tool = Tool::new();
        assert!(!tool.is_faceted());
        assert_eq!(tool.pc_curve_mode(), 0);
    }

    #[test]
    fn test_faceted() {
        let mut tool = Tool::new();
        tool.set_faceted(true);
        assert!(tool.is_faceted());
    }
}
