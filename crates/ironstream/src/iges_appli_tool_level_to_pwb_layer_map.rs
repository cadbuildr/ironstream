// FILE: iges_appli_tool_level_to_pwb_layer_map.rs
// occt: IGESAppli_ToolLevelToPWBLayerMap

#[derive(Clone, Debug)]
pub struct IgesAppliToolLevelToPwbLayerMap;

impl IgesAppliToolLevelToPwbLayerMap {
    pub fn new() -> Self {
        Self
    }
}

impl Default for IgesAppliToolLevelToPwbLayerMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IgesAppliToolLevelToPwbLayerMap::new();
    }
}
