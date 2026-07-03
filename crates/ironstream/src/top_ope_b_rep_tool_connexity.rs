// FILE: top_ope_b_rep_tool_connexity.rs
// occt: TopOpeBRepTool_connexity

#[allow(dead_code)]
/// connexity stub implementation.
pub struct connexity;

impl connexity {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        connexity
    }
}

impl Default for connexity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = connexity::new();
    }

    #[test]
    fn test_default() {
        let _ = connexity::default();
    }
}
