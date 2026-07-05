// FILE: iges_solid_tool_cylindrical_surface.rs
// occt: IGESSolid_ToolCylindricalSurface

pub struct IGESSolidToolCylindricalSurface;

impl IGESSolidToolCylindricalSurface {
    pub fn new() -> Self {
        IGESSolidToolCylindricalSurface
    }

    pub fn label(&self) -> &str {
        "CylindricalSurface"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _t = IGESSolidToolCylindricalSurface::new();
    }

    #[test]
    fn test_label() {
        assert_eq!(IGESSolidToolCylindricalSurface::new().label(), "CylindricalSurface");
    }
}
