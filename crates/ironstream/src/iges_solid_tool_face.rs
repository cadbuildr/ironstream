// FILE: iges_solid_tool_face.rs
// occt: IGESSolid_ToolFace

pub struct IGESSolidToolFace;

impl IGESSolidToolFace {
    pub fn new() -> Self {
        IGESSolidToolFace
    }

    pub fn label(&self) -> &str {
        "Face"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _t = IGESSolidToolFace::new();
    }

    #[test]
    fn test_label() {
        assert_eq!(IGESSolidToolFace::new().label(), "Face");
    }
}
