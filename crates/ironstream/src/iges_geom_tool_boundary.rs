// FILE: iges_geom_tool_boundary.rs
// occt: IGESGeom_ToolBoundary

pub struct ToolBoundary;

impl ToolBoundary {
    pub fn new() -> Self {
        ToolBoundary
    }
}

impl Default for ToolBoundary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ToolBoundary::new();
    }
}
