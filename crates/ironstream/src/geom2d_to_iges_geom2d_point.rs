// FILE: geom2d_to_iges_geom2d_point.rs
// occt: Geom2dToIGES_Geom2dPoint

/// Class for transferring 2D points from Geom2d to IGES.
pub struct Geom2dPoint {
    base: Geom2dEntity,
}

pub struct Geom2dEntity;

impl Geom2dPoint {
    pub fn new() -> Self {
        Geom2dPoint {
            base: Geom2dEntity,
        }
    }
}

impl Default for Geom2dPoint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let point = Geom2dPoint::new();
        assert!(true);
    }
}
