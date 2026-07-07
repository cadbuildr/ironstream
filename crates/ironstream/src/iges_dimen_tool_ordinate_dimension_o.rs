// FILE: iges_dimen_tool_ordinate_dimension_o.rs
// occt: IGESDimen_ToolOrdinateDimension

//! Tool to work on a OrdinateDimension. Called by various Modules
//! (ReadWriteModule, GeneralModule, SpecificModule)

/// Tool for working with IGES OrdinateDimension entities
pub struct IGESDimenToolOrdinateDimension;

impl IGESDimenToolOrdinateDimension {
    /// Returns a ToolOrdinateDimension, ready to work
    pub fn new() -> Self {
        IGESDimenToolOrdinateDimension
    }
}

impl Default for IGESDimenToolOrdinateDimension {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let tool = IGESDimenToolOrdinateDimension::new();
        // Tool is successfully constructed and ready to work
    }

    #[test]
    fn test_default() {
        let tool = IGESDimenToolOrdinateDimension::default();
        // Tool can be created via default trait
    }
}
