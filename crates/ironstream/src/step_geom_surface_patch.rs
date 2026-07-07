// FILE: step_geom_surface_patch.rs
// occt: StepGeom_SurfacePatch

/// Represents a patch (section) of a surface
pub struct StepGeomSurfacePatch {
    /// Surface ID for this patch
    surface_id: i32,
    /// U direction transition code
    u_transition: i32,
    /// V direction transition code
    v_transition: i32,
}

impl StepGeomSurfacePatch {
    pub fn new(surface_id: i32, u_transition: i32, v_transition: i32) -> Self {
        StepGeomSurfacePatch {
            surface_id,
            u_transition,
            v_transition,
        }
    }

    pub fn surface_id(&self) -> i32 {
        self.surface_id
    }

    pub fn u_transition(&self) -> i32 {
        self.u_transition
    }

    pub fn v_transition(&self) -> i32 {
        self.v_transition
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_patch() {
        let patch = StepGeomSurfacePatch::new(1, 0, 0);
        assert_eq!(patch.surface_id(), 1);
        assert_eq!(patch.u_transition(), 0);
        assert_eq!(patch.v_transition(), 0);
    }
}
