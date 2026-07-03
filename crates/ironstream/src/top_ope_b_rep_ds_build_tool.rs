// FILE: top_ope_b_rep_ds_build_tool.rs
// occt: TopOpeBRepDS_BuildTool

/// Tool for building data structures
#[derive(Debug, Clone)]
pub struct BuildTool {
    /// Tool configuration ID
    config_id: usize,
}

impl BuildTool {
    /// Create new BuildTool
    pub fn new(config_id: usize) -> Self {
        BuildTool { config_id }
    }

    /// Get configuration ID
    pub fn config_id(&self) -> usize {
        self.config_id
    }

    /// Set configuration ID
    pub fn set_config_id(&mut self, id: usize) {
        self.config_id = id;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_tool_new() {
        let tool = BuildTool::new(42);
        assert_eq!(tool.config_id(), 42);
    }

    #[test]
    fn test_build_tool_set_config() {
        let mut tool = BuildTool::new(1);
        tool.set_config_id(99);
        assert_eq!(tool.config_id(), 99);
    }
}
