// FILE: iges_geom_spline_surface.rs
// occt: IGESGeom_SplineSurface

/// Defines IGESSplineSurface, Type <114> Form <0,1,2> in package IGESGeom.
/// A spline surface defined by control points and degree information.
#[derive(Clone, Debug)]
pub struct SplineSurface {
    /// U direction spline degree
    u_degree: i32,
    /// V direction spline degree
    v_degree: i32,
    /// Number of control points in U direction
    num_u_points: i32,
    /// Number of control points in V direction
    num_v_points: i32,
    /// Form number
    form: i32,
    /// Entity type for IGES (always 114)
    entity_type: i32,
}

impl SplineSurface {
    pub fn new() -> Self {
        SplineSurface {
            u_degree: 0,
            v_degree: 0,
            num_u_points: 0,
            num_v_points: 0,
            form: 0,
            entity_type: 114,
        }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for SplineSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let surface = SplineSurface::new();
        assert_eq!(surface.entity_type(), 114);
    }
}
