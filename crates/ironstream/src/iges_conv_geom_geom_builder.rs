// FILE: iges_conv_geom_geom_builder.rs
// occt: IGESConvGeom_GeomBuilder

/// Builder for IGES Geometry conversion.
pub struct IgesConvGeomGeomBuilder;

impl IgesConvGeomGeomBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn build_surface(&self, params: &str) -> String {
        format!("Surface({}, {})", params, "IGES")
    }

    pub fn build_curve(&self, params: &str) -> String {
        format!("Curve({}, {})", params, "IGES")
    }

    pub fn build_edge(&self, params: &str) -> String {
        format!("Edge({}, {})", params, "IGES")
    }

    pub fn validate(&self, geometry: &str) -> bool {
        !geometry.is_empty()
    }
}

impl Default for IgesConvGeomGeomBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_surface() {
        let builder = IgesConvGeomGeomBuilder::new();
        let result = builder.build_surface("test_params");
        assert!(result.contains("Surface"));
        assert!(result.contains("IGES"));
    }

    #[test]
    fn test_build_curve() {
        let builder = IgesConvGeomGeomBuilder::new();
        let result = builder.build_curve("test_params");
        assert!(result.contains("Curve"));
    }

    #[test]
    fn test_validate() {
        let builder = IgesConvGeomGeomBuilder::new();
        assert!(builder.validate("geometry_data"));
        assert!(!builder.validate(""));
    }
}
