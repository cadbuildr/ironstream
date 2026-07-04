// FILE: step_construct_context_tool.rs
// occt: STEPConstruct_ContextTool

/// Maintains global context tool for writing STEP constructs
pub struct STEPConstruct_ContextTool {
    level: i32,
    is_ap203: bool,
    is_ap214: bool,
    is_ap242: bool,
}

impl STEPConstruct_ContextTool {
    /// Create a new context tool
    pub fn new() -> Self {
        STEPConstruct_ContextTool {
            level: 0,
            is_ap203: false,
            is_ap214: false,
            is_ap242: false,
        }
    }

    /// Set the model
    pub fn set_model(&mut self) {
        // TODO: Initialize ApplicationProtocolDefinition by parsing model
    }

    /// Returns True if APD.schema_name is config_control_design
    pub fn is_ap203(&self) -> bool {
        self.is_ap203
    }

    /// Returns True if APD.schema_name is automotive_design
    pub fn is_ap214(&self) -> bool {
        self.is_ap214
    }

    /// Returns True if APD.schema_name is ap242_managed_model_based_3d_engineering
    pub fn is_ap242(&self) -> bool {
        self.is_ap242
    }

    /// Returns current assembly level
    pub fn level(&self) -> i32 {
        self.level
    }

    /// Increment assembly level
    pub fn next_level(&mut self) {
        self.level += 1;
    }

    /// Decrement assembly level
    pub fn prev_level(&mut self) {
        if self.level > 0 {
            self.level -= 1;
        }
    }
}

impl Default for STEPConstruct_ContextTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_context_tool() {
        let tool = STEPConstruct_ContextTool::new();
        assert_eq!(tool.level(), 0);
        assert!(!tool.is_ap203());
        assert!(!tool.is_ap214());
        assert!(!tool.is_ap242());
    }

    #[test]
    fn test_level_increment() {
        let mut tool = STEPConstruct_ContextTool::new();
        tool.next_level();
        assert_eq!(tool.level(), 1);
        tool.next_level();
        assert_eq!(tool.level(), 2);
    }

    #[test]
    fn test_level_decrement() {
        let mut tool = STEPConstruct_ContextTool::new();
        tool.next_level();
        tool.next_level();
        tool.prev_level();
        assert_eq!(tool.level(), 1);
    }

    #[test]
    fn test_level_cannot_go_negative() {
        let mut tool = STEPConstruct_ContextTool::new();
        tool.prev_level();
        assert_eq!(tool.level(), 0);
    }
}
