// FILE: top_ope_b_rep_tool_geometry.rs
// occt: TopOpeBRepTool_GEOMETRY

/// GEOMETRY stub implementation.
pub struct GEOMETRY;

impl GEOMETRY {
    /// Initialize or perform setup.
    pub fn new() -> Self {
        GEOMETRY
    }
}

impl Default for GEOMETRY {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = GEOMETRY::new();
    }

    #[test]
    fn test_default() {
        let _ = GEOMETRY::default();
    }
}
