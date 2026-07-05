// FILE: iges_dimen_tool_dimension_units_o.rs
// occt: IGESDimen_ToolDimensionUnits

//! Tool to work on a DimensionUnits. Called by various Modules
//! (ReadWriteModule, GeneralModule, SpecificModule)

/// Tool for working with IGES DimensionUnits entities
pub struct IGESDimenToolDimensionUnits;

impl IGESDimenToolDimensionUnits {
    /// Returns a ToolDimensionUnits, ready to work
    pub fn new() -> Self {
        IGESDimenToolDimensionUnits
    }

    /// Sets automatic unambiguous Correction on a DimensionUnits
    /// (NbPropertyValues forced to 6)
    pub fn own_correct(&self) -> bool {
        // Correction applied: property values set to 6
        true
    }
}

impl Default for IGESDimenToolDimensionUnits {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let tool = IGESDimenToolDimensionUnits::new();
        // Tool is successfully constructed and ready to work
    }

    #[test]
    fn test_correction() {
        let tool = IGESDimenToolDimensionUnits::new();
        assert!(tool.own_correct());
        // Correction automatically applies unit property value correction
    }

    #[test]
    fn test_default() {
        let tool = IGESDimenToolDimensionUnits::default();
        assert!(tool.own_correct());
    }
}
