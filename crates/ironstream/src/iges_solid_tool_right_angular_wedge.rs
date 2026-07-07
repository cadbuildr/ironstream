// FILE: iges_solid_tool_right_angular_wedge.rs
// occt: IGESSolid_ToolRightAngularWedge

pub struct IGESSolidToolRightAngularWedge;

impl IGESSolidToolRightAngularWedge {
    pub fn new() -> Self {
        IGESSolidToolRightAngularWedge
    }

    pub fn label(&self) -> &str {
        "RightAngularWedge"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _t = IGESSolidToolRightAngularWedge::new();
    }

    #[test]
    fn test_label() {
        assert_eq!(IGESSolidToolRightAngularWedge::new().label(), "RightAngularWedge");
    }
}
