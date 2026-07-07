// FILE: iges_geom_spline_curve.rs
// occt: IGESGeom_SplineCurve

/// Defines IGESSplineCurve, Type <112> Form <0,1,2> in package IGESGeom.
/// A spline curve is defined by control points and degree information.
#[derive(Clone, Debug)]
pub struct SplineCurve {
    /// Spline degree
    degree: i32,
    /// Number of control points
    num_points: i32,
    /// Control point coordinates (flattened)
    control_points: Vec<f64>,
    /// Form number (0, 1, or 2)
    form: i32,
    /// Entity type for IGES (always 112)
    entity_type: i32,
}

impl SplineCurve {
    /// Creates a new SplineCurve entity.
    pub fn new() -> Self {
        SplineCurve {
            degree: 0,
            num_points: 0,
            control_points: Vec::new(),
            form: 0,
            entity_type: 112,
        }
    }

    /// Returns the spline degree.
    pub fn degree(&self) -> i32 {
        self.degree
    }

    /// Returns the number of control points.
    pub fn num_points(&self) -> i32 {
        self.num_points
    }

    /// Returns the entity type (always 112).
    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for SplineCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_spline_curve() {
        let curve = SplineCurve::new();
        assert_eq!(curve.degree(), 0);
        assert_eq!(curve.num_points(), 0);
        assert_eq!(curve.entity_type(), 112);
    }
}
