// FILE: b_rep_curve_on2_surfaces.rs
// occt: BRep_CurveOn2Surfaces

/// Represents a curve defined on two surfaces.
pub struct BRepCurveOn2Surfaces {
    surface1_id: u32,
    surface2_id: u32,
    curve2d_1: Vec<f64>,
    curve2d_2: Vec<f64>,
    curve_3d: Vec<f64>,
}

impl BRepCurveOn2Surfaces {
    /// Create a new curve on 2 surfaces.
    pub fn new(
        surface1: u32,
        surface2: u32,
        curve_2d_1: Vec<f64>,
        curve_2d_2: Vec<f64>,
        curve_3d: Vec<f64>,
    ) -> Self {
        Self {
            surface1_id: surface1,
            surface2_id: surface2,
            curve2d_1: curve_2d_1,
            curve2d_2: curve_2d_2,
            curve_3d: curve_3d,
        }
    }

    /// Get the first surface id.
    pub fn surface1(&self) -> u32 {
        self.surface1_id
    }

    /// Get the second surface id.
    pub fn surface2(&self) -> u32 {
        self.surface2_id
    }

    /// Get the 2D curve on surface 1.
    pub fn curve_2d_on_surface1(&self) -> &[f64] {
        &self.curve2d_1
    }

    /// Get the 2D curve on surface 2.
    pub fn curve_2d_on_surface2(&self) -> &[f64] {
        &self.curve2d_2
    }

    /// Get the 3D curve.
    pub fn curve_3d(&self) -> &[f64] {
        &self.curve_3d
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let curve = BRepCurveOn2Surfaces::new(
            1,
            2,
            vec![0.0, 1.0],
            vec![1.0, 2.0],
            vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        );

        assert_eq!(curve.surface1(), 1);
        assert_eq!(curve.surface2(), 2);
        assert_eq!(curve.curve_2d_on_surface1(), &[0.0, 1.0]);
        assert_eq!(curve.curve_2d_on_surface2(), &[1.0, 2.0]);
    }
}
