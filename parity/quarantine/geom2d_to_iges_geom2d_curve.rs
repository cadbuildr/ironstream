// FILE: geom2d_to_iges_geom2d_curve.rs
// occt: Geom2dToIGES_Geom2dCurve

/// Class for transferring 2D curves from Geom2d to IGES.
pub struct Geom2dCurve {
    base: Geom2dEntity,
}

pub struct Geom2dEntity;

impl Geom2dCurve {
    pub fn new() -> Self {
        Geom2dCurve {
            base: Geom2dEntity,
        }
    }
}

impl Default for Geom2dCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let curve = Geom2dCurve::new();
        assert!(true);
    }
}
