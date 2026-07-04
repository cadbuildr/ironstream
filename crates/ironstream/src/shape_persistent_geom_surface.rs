// FILE: shape_persistent_geom_surface.rs
// occt: ShapePersistent_Geom_Surface

/// Surface geometry persistence
pub struct ShapePersistentGeomSurface;

impl ShapePersistentGeomSurface {
    /// Create surface persistence manager
    pub fn new() -> Self {
        ShapePersistentGeomSurface
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = ShapePersistentGeomSurface::new();
    }
}
