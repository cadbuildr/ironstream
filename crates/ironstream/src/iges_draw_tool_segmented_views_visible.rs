// FILE: iges_draw_tool_segmented_views_visible.rs
// occt: IGESDraw_ToolSegmentedViewsVisible

/// Tool to work on a SegmentedViewsVisible
pub struct IgesDrawToolSegmentedViewsVisible;

impl IgesDrawToolSegmentedViewsVisible {
    pub fn new() -> Self {
        IgesDrawToolSegmentedViewsVisible
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
        let tool = IgesDrawToolSegmentedViewsVisible::new();
        assert_eq!(std::mem::size_of_val(&tool), 0);
    }
}
