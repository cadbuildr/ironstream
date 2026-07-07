// FILE: iges_appli_tool_pwb_drilled_hole.rs
// occt: IGESAppli_ToolPWBDrilledHole

#[derive(Clone, Debug)]
pub struct IgesAppliToolPwbDrilledHole;

impl IgesAppliToolPwbDrilledHole {
    pub fn new() -> Self {
        Self
    }
}

impl Default for IgesAppliToolPwbDrilledHole {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IgesAppliToolPwbDrilledHole::new();
    }
}
