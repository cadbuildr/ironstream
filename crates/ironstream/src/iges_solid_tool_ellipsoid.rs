// FILE: iges_solid_tool_ellipsoid.rs
// occt: IGESSolid_ToolEllipsoid

pub struct IGESSolidToolEllipsoid;

impl IGESSolidToolEllipsoid {
    pub fn new() -> Self {
        IGESSolidToolEllipsoid
    }

    pub fn label(&self) -> &str {
        "Ellipsoid"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _t = IGESSolidToolEllipsoid::new();
    }

    #[test]
    fn test_label() {
        assert_eq!(IGESSolidToolEllipsoid::new().label(), "Ellipsoid");
    }
}
