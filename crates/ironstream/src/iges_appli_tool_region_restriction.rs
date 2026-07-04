// FILE: iges_appli_tool_region_restriction.rs
// occt: IGESAppli_ToolRegionRestriction

#[derive(Clone, Debug)]
pub struct IgesAppliToolRegionRestriction;

impl IgesAppliToolRegionRestriction {
    pub fn new() -> Self {
        Self
    }
}

impl Default for IgesAppliToolRegionRestriction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IgesAppliToolRegionRestriction::new();
    }
}
