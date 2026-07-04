// FILE: shape_persistent_geom_curve.rs
// occt: ShapePersistent_Geom_Curve

/// 3D curve geometry persistence
pub struct ShapePersistentGeomCurve;

impl ShapePersistentGeomCurve {
    /// Create 3D curve persistence manager
    pub fn new() -> Self {
        ShapePersistentGeomCurve
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = ShapePersistentGeomCurve::new();
    }
}
