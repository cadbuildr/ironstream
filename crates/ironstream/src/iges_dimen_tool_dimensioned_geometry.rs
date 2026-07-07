// FILE: iges_dimen_tool_dimensioned_geometry.rs
// occt: IGESDimen_dimentooldimensionedgeometry

pub struct IGESDimen_dimentooldimensionedgeometry;

impl IGESDimen_dimentooldimensionedgeometry {
    pub fn new() -> Self {
        IGESDimen_dimentooldimensionedgeometry
    }
}

impl Default for IGESDimen_dimentooldimensionedgeometry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let _tool = IGESDimen_dimentooldimensionedgeometry::new();
    }
}
