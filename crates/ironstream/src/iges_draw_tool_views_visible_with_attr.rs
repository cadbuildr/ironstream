// FILE: iges_draw_tool_views_visible_with_attr.rs
// occt: IGESDraw_ToolViewsVisibleWithAttr

/// Tool to work on ViewsVisibleWithAttr
pub struct IgesDrawToolViewsVisibleWithAttr;

impl IgesDrawToolViewsVisibleWithAttr {
    pub fn new() -> Self {
        IgesDrawToolViewsVisibleWithAttr
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
        let tool = IgesDrawToolViewsVisibleWithAttr::new();
        assert_eq!(std::mem::size_of_val(&tool), 0);
    }
}
