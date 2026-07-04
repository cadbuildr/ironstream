// FILE: iges_geom_tool_spline_surface.rs
// occt: IGESGeom_ToolSplineSurface

pub struct ToolSplineSurface;

impl ToolSplineSurface {
    pub fn new() -> Self {
        ToolSplineSurface
    }
}

impl Default for ToolSplineSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ToolSplineSurface::new();
    }
}
