// FILE: b_rep_curve_on_closed_surface.rs
// occt: BRep_CurveOnClosedSurface

/// Represents a curve on a closed surface.
pub struct BRepCurveOnClosedSurface {
    surface_id: u32,
    curve_2d: Vec<f64>,
    curve_3d: Vec<f64>,
    closed: bool,
    first_param: f64,
    last_param: f64,
}

impl BRepCurveOnClosedSurface {
    /// Create a new curve on closed surface.
    pub fn new(
        surface_id: u32,
        curve_2d: Vec<f64>,
        curve_3d: Vec<f64>,
        first_param: f64,
        last_param: f64,
    ) -> Self {
        Self {
            surface_id,
            curve_2d,
            curve_3d,
            closed: true,
            first_param,
            last_param,
        }
    }

    /// Get the surface id.
    pub fn surface_id(&self) -> u32 {
        self.surface_id
    }

    /// Get the 2D curve.
    pub fn curve_2d(&self) -> &[f64] {
        &self.curve_2d
    }

    /// Get the 3D curve.
    pub fn curve_3d(&self) -> &[f64] {
        &self.curve_3d
    }

    /// Check if the surface is closed.
    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Get the first parameter.
    pub fn first_parameter(&self) -> f64 {
        self.first_param
    }

    /// Get the last parameter.
    pub fn last_parameter(&self) -> f64 {
        self.last_param
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let curve = BRepCurveOnClosedSurface::new(
            1,
            vec![0.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
            0.0,
            1.0,
        );

        assert_eq!(curve.surface_id(), 1);
        assert!(curve.is_closed());
        assert_eq!(curve.first_parameter(), 0.0);
        assert_eq!(curve.last_parameter(), 1.0);
    }
}
