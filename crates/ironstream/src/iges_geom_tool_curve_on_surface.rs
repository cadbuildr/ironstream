// FILE: iges_geom_tool_curve_on_surface.rs
// occt: IGESGeom_ToolCurveOnSurface

pub struct ToolCurveOnSurface;

impl ToolCurveOnSurface {
    pub fn new() -> Self {
        ToolCurveOnSurface
    }
}

impl Default for ToolCurveOnSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ToolCurveOnSurface::new();
    }
}
