// FILE: geom_convert_surf_to_ana_surf.rs
// occt: GeomConvert_SurfToAnaSurf

/// Converts a surface to an analytical surface (plane, sphere, cone, cylinder, etc.).
pub struct GeomConvertSurfToAnaSurf {
    gap: f64,
    conv_type: i32,
}

impl GeomConvertSurfToAnaSurf {
    /// Create a new surface converter.
    pub fn new() -> Self {
        GeomConvertSurfToAnaSurf {
            gap: 0.0,
            conv_type: 2, // MinGap
        }
    }

    /// Returns the gap between original and converted surface.
    pub fn gap(&self) -> f64 {
        self.gap
    }

    /// Sets the gap value.
    pub fn set_gap(&mut self, g: f64) {
        self.gap = g;
    }

    /// Returns the conversion type.
    pub fn get_conv_type(&self) -> i32 {
        self.conv_type
    }

    /// Sets the conversion type.
    pub fn set_conv_type(&mut self, ct: i32) {
        self.conv_type = ct;
    }
}

impl Default for GeomConvertSurfToAnaSurf {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_converter() {
        let conv = GeomConvertSurfToAnaSurf::new();
        assert_eq!(conv.gap(), 0.0);
        assert_eq!(conv.get_conv_type(), 2);
    }

    #[test]
    fn test_gap_getter_setter() {
        let mut conv = GeomConvertSurfToAnaSurf::new();
        conv.set_gap(0.1);
        assert!((conv.gap() - 0.1).abs() < 1e-10);
    }

    #[test]
    fn test_conv_type_getter_setter() {
        let mut conv = GeomConvertSurfToAnaSurf::new();
        conv.set_conv_type(0);
        assert_eq!(conv.get_conv_type(), 0);
    }

    #[test]
    fn test_default() {
        let conv = GeomConvertSurfToAnaSurf::default();
        assert_eq!(conv.gap(), 0.0);
    }
}
