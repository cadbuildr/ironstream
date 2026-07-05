// FILE: iges_solid_tool_conical_surface.rs
// occt: IGESSolid_ToolConicalSurface

//! Tool class for IGESSolid_ConicalSurface.

pub struct IGESSolidToolConicalSurface;

impl IGESSolidToolConicalSurface {
    pub fn new() -> Self {
        IGESSolidToolConicalSurface
    }

    pub fn label(&self) -> &str {
        "ConicalSurface"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _t = IGESSolidToolConicalSurface::new();
    }

    #[test]
    fn test_label() {
        let t = IGESSolidToolConicalSurface::new();
        assert_eq!(t.label(), "ConicalSurface");
    }
}
