// FILE: iges_geom_tool_composite_curve.rs
// occt: IGESGeom_ToolCompositeCurve

pub struct ToolCompositeCurve;

impl ToolCompositeCurve {
    pub fn new() -> Self {
        ToolCompositeCurve
    }
}

impl Default for ToolCompositeCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ToolCompositeCurve::new();
    }
}
