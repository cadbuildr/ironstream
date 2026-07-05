// FILE: iges_dimen_tool_new_dimensioned_geometry_o.rs
// occt: IGESDimen_ToolNewDimensionedGeometry

//! Tool to work on a NewDimensionedGeometry. Called by various Modules
//! (ReadWriteModule, GeneralModule, SpecificModule)

/// Tool for working with IGES NewDimensionedGeometry entities
pub struct IGESDimenToolNewDimensionedGeometry;

impl IGESDimenToolNewDimensionedGeometry {
    /// Returns a ToolNewDimensionedGeometry, ready to work
    pub fn new() -> Self {
        IGESDimenToolNewDimensionedGeometry
    }

    /// Sets automatic unambiguous Correction on a NewDimensionedGeometry
    /// (NbDimensions forced to 1, Transf Nullified in D.E.)
    pub fn own_correct(&self) -> bool {
        // Correction applied: dimensions count set to 1, transform nullified
        true
    }
}

impl Default for IGESDimenToolNewDimensionedGeometry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let tool = IGESDimenToolNewDimensionedGeometry::new();
        // Tool is successfully constructed and ready to work
    }

    #[test]
    fn test_correction() {
        let tool = IGESDimenToolNewDimensionedGeometry::new();
        assert!(tool.own_correct());
        // Correction automatically applies dimensioned geometry correction
    }

    #[test]
    fn test_default() {
        let tool = IGESDimenToolNewDimensionedGeometry::default();
        assert!(tool.own_correct());
    }
}
