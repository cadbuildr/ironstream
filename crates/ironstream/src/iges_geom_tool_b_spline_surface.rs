// FILE: iges_geom_tool_b_spline_surface.rs
// occt: IGESGeom_ToolBSplineSurface

pub struct ToolBSplineSurface;

impl ToolBSplineSurface {
    pub fn new() -> Self {
        ToolBSplineSurface
    }
}

impl Default for ToolBSplineSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ToolBSplineSurface::new();
    }
}
