// FILE: iges_dimen_tool_linear_dimension_o.rs
// occt: IGESDimen_ToolLinearDimension

//! Tool to work on a LinearDimension. Called by various Modules
//! (ReadWriteModule, GeneralModule, SpecificModule)

/// Tool for working with IGES LinearDimension entities
pub struct IGESDimenToolLinearDimension;

impl IGESDimenToolLinearDimension {
    /// Returns a ToolLinearDimension, ready to work
    pub fn new() -> Self {
        IGESDimenToolLinearDimension
    }
}

impl Default for IGESDimenToolLinearDimension {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let tool = IGESDimenToolLinearDimension::new();
        // Tool is successfully constructed and ready to work
    }

    #[test]
    fn test_default() {
        let tool = IGESDimenToolLinearDimension::default();
        // Tool can be created via default trait
    }
}
