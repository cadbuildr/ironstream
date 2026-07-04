// FILE: shape_persistent_geom.rs
// occt: ShapePersistent_Geom

/// Geometric shape persistence base
pub struct ShapePersistentGeom;

impl ShapePersistentGeom {
    /// Create geometry persistence manager
    pub fn new() -> Self {
        ShapePersistentGeom
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = ShapePersistentGeom::new();
    }
}
