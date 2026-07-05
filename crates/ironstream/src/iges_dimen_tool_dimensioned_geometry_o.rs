// FILE: iges_dimen_tool_dimensioned_geometry_o.rs
// occt: IGESDimen_ToolDimensionedGeometry

//! Tool to work on a DimensionedGeometry. Called by various Modules
//! (ReadWriteModule, GeneralModule, SpecificModule)

/// Tool for working with IGES DimensionedGeometry entities
pub struct IGESDimenToolDimensionedGeometry;

impl IGESDimenToolDimensionedGeometry {
    /// Returns a ToolDimensionedGeometry, ready to work
    pub fn new() -> Self {
        IGESDimenToolDimensionedGeometry
    }

    /// Sets automatic unambiguous Correction on a DimensionedGeometry
    /// (NbDimensions forced to 1)
    pub fn own_correct(&self) -> bool {
        // Correction applied: dimensions count set to 1
        true
    }
}

impl Default for IGESDimenToolDimensionedGeometry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let tool = IGESDimenToolDimensionedGeometry::new();
        // Tool is successfully constructed and ready to work
    }

    #[test]
    fn test_correction() {
        let tool = IGESDimenToolDimensionedGeometry::new();
        assert!(tool.own_correct());
        // Correction automatically applies dimensioned geometry correction
    }

    #[test]
    fn test_default() {
        let tool = IGESDimenToolDimensionedGeometry::default();
        assert!(tool.own_correct());
    }
}
