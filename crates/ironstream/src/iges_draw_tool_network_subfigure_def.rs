// FILE: iges_draw_tool_network_subfigure_def.rs
// occt: IGESDraw_ToolNetworkSubfigureDef

/// Tool to work on a NetworkSubfigureDef
pub struct IgesDrawToolNetworkSubfigureDef;

impl IgesDrawToolNetworkSubfigureDef {
    pub fn new() -> Self {
        IgesDrawToolNetworkSubfigureDef
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
        let tool = IgesDrawToolNetworkSubfigureDef::new();
        assert_eq!(std::mem::size_of_val(&tool), 0);
    }
}
