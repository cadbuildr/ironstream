// FILE: iges_appli_tool_drilled_hole.rs
// occt: IGESAppli_ToolDrilledHole

/// Tool for reading/writing DrilledHole entities.
#[derive(Clone, Debug)]
pub struct IgesAppliToolDrilledHole;

impl IgesAppliToolDrilledHole {
    pub fn new() -> Self {
        Self
    }

    pub fn read_own_params(&self, nb_val: i32) -> (f64, f64, i32, i32, i32) {
        (0.0, 0.0, 0, 0, 0)
    }

    pub fn write_own_params(&self) {}

    pub fn own_shared(&self) {}

    pub fn own_correct(&self) -> bool {
        true
    }

    pub fn own_check(&self) {}

    pub fn own_copy(&self) {}

    pub fn own_dump(&self) {}
}

impl Default for IgesAppliToolDrilledHole {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IgesAppliToolDrilledHole::new();
    }

    #[test]
    fn test_own_correct() {
        let tool = IgesAppliToolDrilledHole::new();
        assert!(tool.own_correct());
    }
}
