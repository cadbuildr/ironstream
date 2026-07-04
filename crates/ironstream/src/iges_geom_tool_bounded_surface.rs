// FILE: iges_geom_tool_bounded_surface.rs
// occt: IGESGeom_ToolBoundedSurface

pub struct ToolBoundedSurface;

impl ToolBoundedSurface {
    pub fn new() -> Self {
        ToolBoundedSurface
    }
}

impl Default for ToolBoundedSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ToolBoundedSurface::new();
    }
}
