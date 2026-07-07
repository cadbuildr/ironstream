// FILE: iges_draw_tool_drawing.rs
// occt: IGESDraw_ToolDrawing

/// Tool to work on a Drawing
pub struct IgesDrawToolDrawing;

impl IgesDrawToolDrawing {
    pub fn new() -> Self {
        IgesDrawToolDrawing
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
        let tool = IgesDrawToolDrawing::new();
        assert_eq!(std::mem::size_of_val(&tool), 0);
    }
}
