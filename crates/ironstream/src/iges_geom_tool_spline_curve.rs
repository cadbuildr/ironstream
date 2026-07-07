// FILE: iges_geom_tool_spline_curve.rs
// occt: IGESGeom_ToolSplineCurve

pub struct ToolSplineCurve;

impl ToolSplineCurve {
    pub fn new() -> Self {
        ToolSplineCurve
    }
}

impl Default for ToolSplineCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ToolSplineCurve::new();
    }
}
