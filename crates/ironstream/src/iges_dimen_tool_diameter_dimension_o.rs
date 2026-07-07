// FILE: iges_dimen_tool_diameter_dimension_o.rs
// occt: IGESDimen_ToolDiameterDimension

//! Tool to work on a DiameterDimension. Called by various Modules
//! (ReadWriteModule, GeneralModule, SpecificModule)

/// Tool for working with IGES DiameterDimension entities
pub struct IGESDimenToolDiameterDimension;

impl IGESDimenToolDiameterDimension {
    /// Returns a ToolDiameterDimension, ready to work
    pub fn new() -> Self {
        IGESDimenToolDiameterDimension
    }
}

impl Default for IGESDimenToolDiameterDimension {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let tool = IGESDimenToolDiameterDimension::new();
        // Tool is successfully constructed and ready to work
    }

    #[test]
    fn test_default() {
        let tool = IGESDimenToolDiameterDimension::default();
        // Tool can be created via default trait
    }
}
