// FILE: shape_persistent_geom2d.rs
// occt: ShapePersistent_Geom2d

/// 2D geometric shape persistence
pub struct ShapePersistentGeom2d;

impl ShapePersistentGeom2d {
    /// Create 2D geometry persistence manager
    pub fn new() -> Self {
        ShapePersistentGeom2d
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = ShapePersistentGeom2d::new();
    }
}
