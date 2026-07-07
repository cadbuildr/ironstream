// FILE: iges_appli_tool_pwb_artwork_stackup.rs
// occt: IGESAppli_ToolPWBArtworkStackup

#[derive(Clone, Debug)]
pub struct IgesAppliToolPwbArtworkStackup;

impl IgesAppliToolPwbArtworkStackup {
    pub fn new() -> Self {
        Self
    }
}

impl Default for IgesAppliToolPwbArtworkStackup {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IgesAppliToolPwbArtworkStackup::new();
    }
}
