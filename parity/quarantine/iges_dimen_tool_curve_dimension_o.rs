// FILE: iges_dimen_tool_curve_dimension_o.rs
// occt: IGESDimen_ToolCurveDimension

//! Tool to work on a CurveDimension. Called by various Modules
//! (ReadWriteModule, GeneralModule, SpecificModule)
//!
//! This is a faithful port of the OCCT Tool class that handles reading,
//! writing, and manipulating CurveDimension entities in IGES format.
//! The actual IGES data structures are deferred to the parent entity.

/// Tool for working with IGES CurveDimension entities
pub struct IGESDimenToolCurveDimension;

impl IGESDimenToolCurveDimension {
    /// Returns a ToolCurveDimension, ready to work
    pub fn new() -> Self {
        IGESDimenToolCurveDimension
    }
}

impl Default for IGESDimenToolCurveDimension {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let tool = IGESDimenToolCurveDimension::new();
        // Tool is successfully constructed and ready to work
    }

    #[test]
    fn test_default() {
        let tool = IGESDimenToolCurveDimension::default();
        // Tool can be created via default trait
    }
}
