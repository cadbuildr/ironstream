// FILE: top_ope_b_rep_tool_tol.rs
// occt: TopOpeBRepTool_tol

#[allow(dead_code)]
/// tol stub implementation.
pub struct tol;

impl tol {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        tol
    }
}

impl Default for tol {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = tol::new();
    }

    #[test]
    fn test_default() {
        let _ = tol::default();
    }
}
