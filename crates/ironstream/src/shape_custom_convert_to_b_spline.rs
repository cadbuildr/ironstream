// FILE: shape_custom_convert_to_b_spline.rs
// occt: ShapeCustom_ConvertToBSpline

/// A modification class that converts surfaces of Linear Extrusion,
/// Revolution, and Offset surfaces into BSpline surfaces.
///
/// This is a concrete modifier algorithm used with BRepTools to transform
/// certain surface types according to configuration flags.
pub struct ShapeCustomConvertToBSpline {
    extrusion_mode: bool,
    revolution_mode: bool,
    offset_mode: bool,
    plane_mode: bool,
}

impl ShapeCustomConvertToBSpline {
    /// Creates a new instance with all conversion modes disabled.
    pub fn new() -> Self {
        ShapeCustomConvertToBSpline {
            extrusion_mode: false,
            revolution_mode: false,
            offset_mode: false,
            plane_mode: false,
        }
    }

    /// Sets the conversion mode for surfaces of linear extrusion.
    pub fn set_extrusion_mode(&mut self, mode: bool) {
        self.extrusion_mode = mode;
    }

    /// Gets the conversion mode for surfaces of linear extrusion.
    pub fn extrusion_mode(&self) -> bool {
        self.extrusion_mode
    }

    /// Sets the conversion mode for surfaces of revolution.
    pub fn set_revolution_mode(&mut self, mode: bool) {
        self.revolution_mode = mode;
    }

    /// Gets the conversion mode for surfaces of revolution.
    pub fn revolution_mode(&self) -> bool {
        self.revolution_mode
    }

    /// Sets the conversion mode for offset surfaces.
    pub fn set_offset_mode(&mut self, mode: bool) {
        self.offset_mode = mode;
    }

    /// Gets the conversion mode for offset surfaces.
    pub fn offset_mode(&self) -> bool {
        self.offset_mode
    }

    /// Sets the conversion mode for plane surfaces.
    pub fn set_plane_mode(&mut self, mode: bool) {
        self.plane_mode = mode;
    }

    /// Gets the conversion mode for plane surfaces.
    pub fn plane_mode(&self) -> bool {
        self.plane_mode
    }
}

impl Default for ShapeCustomConvertToBSpline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_default_modes() {
        let converter = ShapeCustomConvertToBSpline::new();
        assert!(!converter.extrusion_mode());
        assert!(!converter.revolution_mode());
        assert!(!converter.offset_mode());
        assert!(!converter.plane_mode());
    }

    #[test]
    fn test_set_extrusion_mode() {
        let mut converter = ShapeCustomConvertToBSpline::new();
        converter.set_extrusion_mode(true);
        assert!(converter.extrusion_mode());
        converter.set_extrusion_mode(false);
        assert!(!converter.extrusion_mode());
    }

    #[test]
    fn test_set_revolution_mode() {
        let mut converter = ShapeCustomConvertToBSpline::new();
        converter.set_revolution_mode(true);
        assert!(converter.revolution_mode());
        converter.set_revolution_mode(false);
        assert!(!converter.revolution_mode());
    }

    #[test]
    fn test_set_offset_mode() {
        let mut converter = ShapeCustomConvertToBSpline::new();
        converter.set_offset_mode(true);
        assert!(converter.offset_mode());
        converter.set_offset_mode(false);
        assert!(!converter.offset_mode());
    }

    #[test]
    fn test_set_plane_mode() {
        let mut converter = ShapeCustomConvertToBSpline::new();
        converter.set_plane_mode(true);
        assert!(converter.plane_mode());
        converter.set_plane_mode(false);
        assert!(!converter.plane_mode());
    }

    #[test]
    fn test_all_modes_independent() {
        let mut converter = ShapeCustomConvertToBSpline::new();
        converter.set_extrusion_mode(true);
        converter.set_revolution_mode(true);
        converter.set_offset_mode(false);
        converter.set_plane_mode(true);

        assert!(converter.extrusion_mode());
        assert!(converter.revolution_mode());
        assert!(!converter.offset_mode());
        assert!(converter.plane_mode());
    }

    #[test]
    fn test_default_trait() {
        let converter = ShapeCustomConvertToBSpline::default();
        assert!(!converter.extrusion_mode());
        assert!(!converter.revolution_mode());
        assert!(!converter.offset_mode());
        assert!(!converter.plane_mode());
    }
}
