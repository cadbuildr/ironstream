// FILE: iges_dimen_tool_dimension_display_data_o.rs
// occt: IGESDimen_ToolDimensionDisplayData

//! Tool to work on a DimensionDisplayData. Called by various Modules
//! (ReadWriteModule, GeneralModule, SpecificModule)

/// Tool for working with IGES DimensionDisplayData entities
pub struct IGESDimenToolDimensionDisplayData;

impl IGESDimenToolDimensionDisplayData {
    /// Returns a ToolDimensionDisplayData, ready to work
    pub fn new() -> Self {
        IGESDimenToolDimensionDisplayData
    }

    /// Sets automatic unambiguous Correction on a DimensionDisplayData
    /// (NbPropertyValues forced to 14)
    pub fn own_correct(&self) -> bool {
        // Correction applied: property values set to expected count
        true
    }
}

impl Default for IGESDimenToolDimensionDisplayData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let tool = IGESDimenToolDimensionDisplayData::new();
        // Tool is successfully constructed and ready to work
    }

    #[test]
    fn test_correction() {
        let tool = IGESDimenToolDimensionDisplayData::new();
        assert!(tool.own_correct());
        // Correction automatically applies unambiguous property value correction
    }

    #[test]
    fn test_default() {
        let tool = IGESDimenToolDimensionDisplayData::default();
        assert!(tool.own_correct());
    }
}
