// FILE: iges_draw_tool_circ_array_subfigure.rs
// occt: IGESDraw_ToolCircArraySubfigure

/// Tool to work on a CircArraySubfigure
pub struct IgesDrawToolCircArraySubfigure;

impl IgesDrawToolCircArraySubfigure {
    pub fn new() -> Self {
        IgesDrawToolCircArraySubfigure
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
        let tool = IgesDrawToolCircArraySubfigure::new();
        assert_eq!(std::mem::size_of_val(&tool), 0);
    }
}
