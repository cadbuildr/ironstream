// FILE: iges_draw_tool_drawing_with_rotation.rs
// occt: IGESDraw_ToolDrawingWithRotation

/// Tool to work on a DrawingWithRotation
pub struct IgesDrawToolDrawingWithRotation;

impl IgesDrawToolDrawingWithRotation {
    pub fn new() -> Self {
        IgesDrawToolDrawingWithRotation
    }

    pub fn read_own_params(&self) {}
    pub fn write_own_params(&self) {}
    pub fn own_shared(&self) {}
    pub fn dir_checker(&self) {}
    pub fn own_check(&self) {}
    pub fn own_copy(&self) {}
    pub fn own_dump(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tool = IgesDrawToolDrawingWithRotation::new();
        assert_eq!(std::mem::size_of_val(&tool), 0);
    }
}
