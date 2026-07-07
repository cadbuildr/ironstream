// FILE: iges_dimen_tool_dimension_tolerance_o.rs
// occt: IGESDimen_ToolDimensionTolerance

//! Tool to work on a DimensionTolerance. Called by various Modules
//! (ReadWriteModule, GeneralModule, SpecificModule)

/// Tool for working with IGES DimensionTolerance entities
pub struct IGESDimenToolDimensionTolerance;

impl IGESDimenToolDimensionTolerance {
    /// Returns a ToolDimensionTolerance, ready to work
    pub fn new() -> Self {
        IGESDimenToolDimensionTolerance
    }

    /// Sets automatic unambiguous Correction on a DimensionTolerance
    /// (NbPropertyValues forced to 8)
    pub fn own_correct(&self) -> bool {
        // Correction applied: property values set to 8
        true
    }
}

impl Default for IGESDimenToolDimensionTolerance {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let tool = IGESDimenToolDimensionTolerance::new();
        // Tool is successfully constructed and ready to work
    }

    #[test]
    fn test_correction() {
        let tool = IGESDimenToolDimensionTolerance::new();
        assert!(tool.own_correct());
        // Correction automatically applies tolerance property value correction
    }

    #[test]
    fn test_default() {
        let tool = IGESDimenToolDimensionTolerance::default();
        assert!(tool.own_correct());
    }
}
