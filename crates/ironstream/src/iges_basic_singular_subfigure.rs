// FILE: iges_basic_singular_subfigure.rs
// occt: IGESBasic_SingularSubfigure

/// 3D point representation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpXYZ {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl GpXYZ {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

/// SingularSubfigure, Type <408> Form <0>
/// Defines the occurrence of a single instance of the defined Subfigure.
pub struct IgesBasicSingularSubfigure {
    subfigure_def: String,
    translation: GpXYZ,
    scale_factor: f64,
    has_scale_factor: bool,
}

impl IgesBasicSingularSubfigure {
    /// Create a new SingularSubfigure with default values.
    pub fn new() -> Self {
        Self {
            subfigure_def: String::new(),
            translation: GpXYZ::new(0.0, 0.0, 0.0),
            scale_factor: 1.0,
            has_scale_factor: false,
        }
    }

    /// Set the fields of the class SingularSubfigure.
    /// - subfigure_def: the Subfigure Definition entity
    /// - translation: used to store the X,Y,Z coord
    /// - has_scale: Indicates the presence of scale factor
    /// - scale: Used to store the scale factor
    pub fn init(
        &mut self,
        subfigure_def: String,
        translation: GpXYZ,
        has_scale: bool,
        scale: f64,
    ) {
        self.subfigure_def = subfigure_def;
        self.translation = translation;
        self.has_scale_factor = has_scale;
        self.scale_factor = if has_scale { scale } else { 1.0 };
    }

    /// Returns the subfigure definition entity.
    pub fn subfigure(&self) -> &str {
        &self.subfigure_def
    }

    /// Returns the X, Y, Z coordinates.
    pub fn translation(&self) -> GpXYZ {
        self.translation
    }

    /// Returns the scale factor.
    /// If has_scale_factor is false, returns 1.0 (default).
    pub fn scale_factor(&self) -> f64 {
        if self.has_scale_factor {
            self.scale_factor
        } else {
            1.0
        }
    }

    /// Returns a boolean indicating whether scale factor is present or not.
    pub fn has_scale_factor(&self) -> bool {
        self.has_scale_factor
    }

    /// Returns the Translation after transformation.
    pub fn transformed_translation(&self) -> GpXYZ {
        let scale = self.scale_factor();
        GpXYZ::new(
            self.translation.x * scale,
            self.translation.y * scale,
            self.translation.z * scale,
        )
    }
}

impl Default for IgesBasicSingularSubfigure {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sf = IgesBasicSingularSubfigure::new();
        assert_eq!(sf.subfigure(), "");
        assert_eq!(sf.translation(), GpXYZ::new(0.0, 0.0, 0.0));
        assert_eq!(sf.scale_factor(), 1.0);
        assert!(!sf.has_scale_factor());
    }

    #[test]
    fn test_init_without_scale() {
        let mut sf = IgesBasicSingularSubfigure::new();
        sf.init("subfig1".to_string(), GpXYZ::new(1.0, 2.0, 3.0), false, 1.0);
        assert_eq!(sf.subfigure(), "subfig1");
        assert_eq!(sf.translation(), GpXYZ::new(1.0, 2.0, 3.0));
        assert_eq!(sf.scale_factor(), 1.0);
        assert!(!sf.has_scale_factor());
    }

    #[test]
    fn test_init_with_scale() {
        let mut sf = IgesBasicSingularSubfigure::new();
        sf.init("subfig2".to_string(), GpXYZ::new(1.0, 2.0, 3.0), true, 2.0);
        assert_eq!(sf.subfigure(), "subfig2");
        assert_eq!(sf.translation(), GpXYZ::new(1.0, 2.0, 3.0));
        assert_eq!(sf.scale_factor(), 2.0);
        assert!(sf.has_scale_factor());
    }

    #[test]
    fn test_transformed_translation_without_scale() {
        let mut sf = IgesBasicSingularSubfigure::new();
        sf.init("subfig".to_string(), GpXYZ::new(1.0, 2.0, 3.0), false, 1.0);
        let transformed = sf.transformed_translation();
        assert_eq!(transformed, GpXYZ::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_transformed_translation_with_scale() {
        let mut sf = IgesBasicSingularSubfigure::new();
        sf.init("subfig".to_string(), GpXYZ::new(1.0, 2.0, 3.0), true, 2.0);
        let transformed = sf.transformed_translation();
        assert_eq!(transformed, GpXYZ::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_default() {
        let sf = IgesBasicSingularSubfigure::default();
        assert_eq!(sf.subfigure(), "");
        assert_eq!(sf.scale_factor(), 1.0);
    }
}
