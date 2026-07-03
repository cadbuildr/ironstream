// FILE: top_ope_b_rep_tool_face.rs
// occt: TopOpeBRepTool_face

#[allow(dead_code)]
/// face stub implementation.
pub struct face;

impl face {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        face
    }
}

impl Default for face {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = face::new();
    }

    #[test]
    fn test_default() {
        let _ = face::default();
    }
}
