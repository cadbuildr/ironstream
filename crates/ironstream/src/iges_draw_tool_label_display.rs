// FILE: iges_draw_tool_label_display.rs
// occt: IGESDraw_ToolLabelDisplay

/// Tool to work on a LabelDisplay
pub struct IgesDrawToolLabelDisplay;

impl IgesDrawToolLabelDisplay {
    pub fn new() -> Self {
        IgesDrawToolLabelDisplay
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
        let tool = IgesDrawToolLabelDisplay::new();
        assert_eq!(std::mem::size_of_val(&tool), 0);
    }
}
