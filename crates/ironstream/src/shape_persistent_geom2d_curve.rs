// FILE: shape_persistent_geom2d_curve.rs
// occt: ShapePersistent_Geom2d_Curve

/// 2D curve geometry persistence
pub struct ShapePersistentGeom2dCurve;

impl ShapePersistentGeom2dCurve {
    /// Create 2D curve persistence manager
    pub fn new() -> Self {
        ShapePersistentGeom2dCurve
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = ShapePersistentGeom2dCurve::new();
    }
}
