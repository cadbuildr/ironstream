// FILE: convert_parabola_to_b_spline_curve.rs
// occt: Convert_ParabolaToBSplineCurve

/// Converts a parabola into a non-rational B-spline curve.
///
/// The parabola is parametrized as:
/// P(U) = Loc + F * (U*U * Xdir + 2 * U * Ydir)
/// where Loc is the apex, Xdir is the axis direction, Ydir is the directrix direction,
/// and F is the focal length.
pub struct ConvertParabolaToBSplineCurve {
    // B-spline representation
    poles: Vec<[f64; 2]>,
    weights: Vec<f64>,
    knots: Vec<f64>,
    multiplicities: Vec<i32>,
    degree: i32,
    is_periodic: bool,
}

impl ConvertParabolaToBSplineCurve {
    /// Creates a B-spline representation of a parabola over [U1, U2].
    ///
    /// # Panics
    /// Panics if U1 == U2 (range is empty).
    pub fn new(
        parab_axis_x: f64,
        parab_axis_y: f64,
        parab_axis_dir_x: f64,
        parab_axis_dir_y: f64,
        parab_parameter: f64,
        u1: f64,
        u2: f64,
    ) -> Self {
        let epsilon = 1.0e-15;
        assert!(
            (u2 - u1).abs() >= epsilon,
            "Domain error: U1 and U2 are too close"
        );

        let uf = u1.min(u2);
        let ul = u1.max(u2);
        let p = parab_parameter;

        // Normalize axis directions
        let ax_len = (parab_axis_dir_x * parab_axis_dir_x
            + parab_axis_dir_y * parab_axis_dir_y)
            .sqrt();
        let ox_x = parab_axis_dir_x / ax_len;
        let ox_y = parab_axis_dir_y / ax_len;

        // Perpendicular direction (rotated 90 degrees)
        let oy_x = -ox_y;
        let oy_y = ox_x;

        // Determinant to determine orientation
        let s = if ox_x * oy_y - ox_y * oy_x > 0.0 { 1.0 } else { -1.0 };

        // Compute poles in the canonical reference frame
        // P1 = (UF*UF / (2*p), S*UF)
        let p1_x_canon = uf * uf / (2.0 * p);
        let p1_y_canon = s * uf;

        // P2 = (UF*UL / (2*p), S*(UF+UL)/2)
        let p2_x_canon = uf * ul / (2.0 * p);
        let p2_y_canon = s * (uf + ul) / 2.0;

        // P3 = (UL*UL / (2*p), S*UL)
        let p3_x_canon = ul * ul / (2.0 * p);
        let p3_y_canon = s * ul;

        // Transform to parabola's reference frame
        let p1_x = parab_axis_x + p1_x_canon * ox_x + p1_y_canon * oy_x;
        let p1_y = parab_axis_y + p1_x_canon * ox_y + p1_y_canon * oy_y;

        let p2_x = parab_axis_x + p2_x_canon * ox_x + p2_y_canon * oy_x;
        let p2_y = parab_axis_y + p2_x_canon * ox_y + p2_y_canon * oy_y;

        let p3_x = parab_axis_x + p3_x_canon * ox_x + p3_y_canon * oy_x;
        let p3_y = parab_axis_y + p3_x_canon * ox_y + p3_y_canon * oy_y;

        Self {
            poles: vec![[p1_x, p1_y], [p2_x, p2_y], [p3_x, p3_y]],
            weights: vec![1.0, 1.0, 1.0],
            knots: vec![uf, ul],
            multiplicities: vec![3, 3],
            degree: 2,
            is_periodic: false,
        }
    }

    pub fn degree(&self) -> i32 {
        self.degree
    }

    pub fn nb_poles(&self) -> usize {
        self.poles.len()
    }

    pub fn nb_knots(&self) -> usize {
        self.knots.len()
    }

    pub fn is_periodic(&self) -> bool {
        self.is_periodic
    }

    pub fn poles(&self) -> &[[f64; 2]] {
        &self.poles
    }

    pub fn weights(&self) -> &[f64] {
        &self.weights
    }

    pub fn knots(&self) -> &[f64] {
        &self.knots
    }

    pub fn multiplicities(&self) -> &[i32] {
        &self.multiplicities
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversion() {
        // Parabola with apex at origin, axis along x, parameter 1.0
        let conv = ConvertParabolaToBSplineCurve::new(0.0, 0.0, 1.0, 0.0, 1.0, -2.0, 2.0);

        assert!(!conv.is_periodic());
        assert_eq!(conv.degree(), 2);
        assert_eq!(conv.nb_poles(), 3);
        assert_eq!(conv.nb_knots(), 2);

        // Check weights are all 1.0 (polynomial, not rational)
        for w in conv.weights() {
            assert!((w - 1.0).abs() < 1.0e-15);
        }
    }

    #[test]
    fn test_small_range() {
        let conv = ConvertParabolaToBSplineCurve::new(0.0, 0.0, 1.0, 0.0, 0.5, -0.5, 0.5);

        assert_eq!(conv.degree(), 2);
        assert_eq!(conv.nb_poles(), 3);

        // Check knots match the domain
        assert!((conv.knots()[0] - (-0.5)).abs() < 1.0e-14);
        assert!((conv.knots()[1] - 0.5).abs() < 1.0e-14);
    }

    #[test]
    fn test_poles_structure() {
        let u1 = -1.0;
        let u2 = 1.0;
        let p = 1.0;
        let conv = ConvertParabolaToBSplineCurve::new(0.0, 0.0, 1.0, 0.0, p, u1, u2);

        let poles = conv.poles();
        assert_eq!(poles.len(), 3);

        // First and last poles should have y-coordinates ±u1 and ±u2
        // Middle pole should be at y = (u1 + u2) / 2 = 0
        assert!(poles[1][1].abs() < 1.0e-14);
    }

    #[test]
    fn test_orientation_matters() {
        // Same parabola but different orientation
        let conv1 = ConvertParabolaToBSplineCurve::new(0.0, 0.0, 1.0, 0.0, 1.0, -2.0, 2.0);
        let conv2 = ConvertParabolaToBSplineCurve::new(0.0, 0.0, -1.0, 0.0, 1.0, -2.0, 2.0);

        // Poles should be different due to orientation
        let p1 = conv1.poles();
        let p2 = conv2.poles();
        assert!((p1[0][0] - p2[0][0]).abs() > 1.0e-10);
    }

    #[test]
    #[should_panic]
    fn test_empty_domain() {
        ConvertParabolaToBSplineCurve::new(0.0, 0.0, 1.0, 0.0, 1.0, 0.5, 0.5);
    }
}
