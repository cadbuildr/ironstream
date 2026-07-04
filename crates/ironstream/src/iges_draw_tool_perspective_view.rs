// FILE: iges_draw_tool_perspective_view.rs
// occt: IGESDraw_ToolPerspectiveView

/// Tool to work on a PerspectiveView
pub struct IgesDrawToolPerspectiveView;

impl IgesDrawToolPerspectiveView {
    pub fn new() -> Self {
        IgesDrawToolPerspectiveView
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
        let tool = IgesDrawToolPerspectiveView::new();
        assert_eq!(std::mem::size_of_val(&tool), 0);
    }
}
