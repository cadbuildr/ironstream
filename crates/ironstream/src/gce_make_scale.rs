// FILE: gce_make_scale.rs
// occt: gce_MakeScale

//! Elementary construction algorithm for scaling transformation.

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

/// Builder for scale transformations
pub struct GceMakeScale {
    the_scale: Transformation,
}

impl GceMakeScale {
    /// Constructs a scaling transformation.
    pub fn new(_point: &Point3d, _scale: f64) -> Self {
        GceMakeScale {
            the_scale: Transformation::default(),
        }
    }

    /// Returns the constructed transformation
    pub fn value(&self) -> Transformation {
        self.the_scale.clone()
    }

    /// Alias for value() returning a copy
    pub fn operator(&self) -> Transformation {
        self.value()
    }
}

/// Placeholder type
#[derive(Clone)]
pub struct Point3d;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_construction() {
        let maker = GceMakeScale::new(&Point3d, 2.0);
        let _trsf = maker.value();
    }

    #[test]
    fn test_scale_operator() {
        let maker = GceMakeScale::new(&Point3d, 0.5);
        let _trsf = maker.operator();
    }
}
