// FILE: iges_geom_tool_b_spline_curve.rs
// occt: IGESGeom_ToolBSplineCurve

pub struct ToolBSplineCurve;

impl ToolBSplineCurve {
    pub fn new() -> Self {
        ToolBSplineCurve
    }
}

impl Default for ToolBSplineCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ToolBSplineCurve::new();
    }
}
