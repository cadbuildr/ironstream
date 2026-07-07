// FILE: iges_draw_tool_planar.rs
// occt: IGESDraw_ToolPlanar

/// Tool to work on a Planar
pub struct IgesDrawToolPlanar;

impl IgesDrawToolPlanar {
    pub fn new() -> Self {
        IgesDrawToolPlanar
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
        let tool = IgesDrawToolPlanar::new();
        assert_eq!(std::mem::size_of_val(&tool), 0);
    }
}
