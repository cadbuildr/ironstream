// FILE: iges_solid_tool_plane_surface.rs
// occt: IGESSolid_ToolPlaneSurface

pub struct IGESSolidToolPlaneSurface;

impl IGESSolidToolPlaneSurface {
    pub fn new() -> Self {
        IGESSolidToolPlaneSurface
    }

    pub fn label(&self) -> &str {
        "PlaneSurface"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _t = IGESSolidToolPlaneSurface::new();
    }

    #[test]
    fn test_label() {
        assert_eq!(IGESSolidToolPlaneSurface::new().label(), "PlaneSurface");
    }
}
