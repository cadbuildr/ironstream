// FILE: iges_conv_geom.rs
// occt: IGESConvGeom

/// IGES Geometry Conversion module.
pub struct IgesConvGeom;

impl IgesConvGeom {
    pub fn new() -> Self {
        Self
    }

    pub fn version() -> &'static str {
        "1.0"
    }

    pub fn module_name() -> &'static str {
        "IGESConvGeom"
    }

    pub fn has_surface_support() -> bool {
        true
    }

    pub fn has_curve_support() -> bool {
        true
    }
}

impl Default for IgesConvGeom {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(IgesConvGeom::version(), "1.0");
    }

    #[test]
    fn test_module_name() {
        assert_eq!(IgesConvGeom::module_name(), "IGESConvGeom");
    }

    #[test]
    fn test_surface_support() {
        assert!(IgesConvGeom::has_surface_support());
    }

    #[test]
    fn test_curve_support() {
        assert!(IgesConvGeom::has_curve_support());
    }
}
