// FILE: int_surf_interior_point_tool.rs
// occt: IntSurf_InteriorPointTool

/// Tool for interior point operations on surfaces
#[derive(Clone, Debug)]
pub struct InteriorPointTool;

impl InteriorPointTool {
    /// Create a new tool
    pub fn new() -> Self {
        InteriorPointTool
    }
}

impl Default for InteriorPointTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = InteriorPointTool::new();
    }
}
