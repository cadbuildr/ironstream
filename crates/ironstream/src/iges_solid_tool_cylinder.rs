// FILE: iges_solid_tool_cylinder.rs
// occt: IGESSolid_ToolCylinder

pub struct IGESSolidToolCylinder;

impl IGESSolidToolCylinder {
    pub fn new() -> Self {
        IGESSolidToolCylinder
    }

    pub fn label(&self) -> &str {
        "Cylinder"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _t = IGESSolidToolCylinder::new();
    }

    #[test]
    fn test_label() {
        assert_eq!(IGESSolidToolCylinder::new().label(), "Cylinder");
    }
}
