// FILE: iges_dimen_tool_new_dimensioned_geometry.rs
// occt-ref: IGESDimen_dimentoolnewdimensionedgeometry

pub struct IGESDimen_dimentoolnewdimensionedgeometry;

impl IGESDimen_dimentoolnewdimensionedgeometry {
    pub fn new() -> Self {
        IGESDimen_dimentoolnewdimensionedgeometry
    }
}

impl Default for IGESDimen_dimentoolnewdimensionedgeometry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let _tool = IGESDimen_dimentoolnewdimensionedgeometry::new();
    }
}
