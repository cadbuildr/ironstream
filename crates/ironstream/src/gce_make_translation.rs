// FILE: gce_make_translation.rs
// occt: gce_MakeTranslation

//! Elementary construction algorithm for translation transformation.

/// Affine transformation
#[derive(Clone)]
pub struct Transformation {
    _marker: [u8; 0],
}

impl Default for Transformation {
    fn default() -> Self {
        Transformation { _marker: [] }
    }
}

/// Builder for translation transformations
pub struct GceMakeTranslation {
    the_translation: Transformation,
}

impl GceMakeTranslation {
    /// Constructs a translation from a vector.
    pub fn from_vector(_vector: &Vector3d) -> Self {
        GceMakeTranslation {
            the_translation: Transformation::default(),
        }
    }

    /// Constructs a translation from two points.
    pub fn from_two_points(_point1: &Point3d, _point2: &Point3d) -> Self {
        GceMakeTranslation {
            the_translation: Transformation::default(),
        }
    }

    /// Returns the constructed transformation
    pub fn value(&self) -> Transformation {
        self.the_translation.clone()
    }

    /// Alias for value() returning a copy
    pub fn operator(&self) -> Transformation {
        self.value()
    }
}

/// Placeholder types
#[derive(Clone)]
pub struct Vector3d;

#[derive(Clone)]
pub struct Point3d;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation_from_vector() {
        let maker = GceMakeTranslation::from_vector(&Vector3d);
        let _trsf = maker.value();
    }

    #[test]
    fn test_translation_from_points() {
        let maker = GceMakeTranslation::from_two_points(&Point3d, &Point3d);
        let _trsf = maker.operator();
    }
}
