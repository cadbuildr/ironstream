// FILE: top_ope_b_rep_tool_project.rs
// occt: TopOpeBRepTool_PROJECT

/// PROJECT stub implementation.
pub struct PROJECT;

impl PROJECT {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        PROJECT
    }
}

impl Default for PROJECT {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = PROJECT::new();
    }

    #[test]
    fn test_default() {
        let _ = PROJECT::default();
    }
}
