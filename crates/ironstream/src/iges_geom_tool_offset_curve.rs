// FILE: iges_geom_tool_offset_curve.rs
// occt: IGESGeom_ToolOffsetCurve

pub struct ToolOffsetCurve;

impl ToolOffsetCurve {
    pub fn new() -> Self {
        ToolOffsetCurve
    }
}

impl Default for ToolOffsetCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ToolOffsetCurve::new();
    }
}
