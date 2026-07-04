// FILE: iges_draw_tool_views_visible.rs
// occt: IGESDraw_ToolViewsVisible

/// Tool to work on ViewsVisible
pub struct IgesDrawToolViewsVisible;

impl IgesDrawToolViewsVisible {
    pub fn new() -> Self {
        IgesDrawToolViewsVisible
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
        let tool = IgesDrawToolViewsVisible::new();
        assert_eq!(std::mem::size_of_val(&tool), 0);
    }
}
