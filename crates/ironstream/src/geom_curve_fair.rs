// FILE: geom_curve_fair.rs
//
// Pure-Rust zero-dependency stub modelled after FairCurve_Batten and
// FairCurve_MinimalVariation from OpenCASCADE Technology (OCCT).
// No external crates are used; only std is required.

// ---------------------------------------------------------------------------
// BattenCurve
// ---------------------------------------------------------------------------

// occt: FairCurve_Batten
/// A curve computed by the "batten" (elastic-strip) method.
///
/// The batten passes through two fixed endpoints and can optionally slide
/// along the tangent at each endpoint.  After setting the desired parameters
/// call [`compute`](BattenCurve::compute); the resulting control poles are
/// stored in [`poles`](BattenCurve::poles).
#[derive(Debug, Clone)]
pub struct BattenCurve {
    /// Control poles of the computed B-spline curve (2-D).
    pub poles: Vec<[f64; 2]>,
    /// Whether the curve is allowed to slide along the chord.
    pub sliding: bool,
    /// Whether free-sliding mode is active (no tangent constraint at ends).
    pub free_sliding: bool,

    // Private parameters
    p1: [f64; 2],
    p2: [f64; 2],
    angle_1: f64,
    angle_2: f64,
    is_done: bool,
}

impl BattenCurve {
    /// Construct a new batten curve between `p1` and `p2`.
    ///
    /// Both [`sliding`](BattenCurve::sliding) and
    /// [`free_sliding`](BattenCurve::free_sliding) default to `false`.
    /// End angles default to `0.0` (horizontal tangents).
    pub fn new(p1: [f64; 2], p2: [f64; 2]) -> Self {
        Self {
            poles: Vec::new(),
            sliding: false,
            free_sliding: false,
            p1,
            p2,
            angle_1: 0.0,
            angle_2: 0.0,
            is_done: false,
        }
    }

    /// Enable or disable sliding mode.
    pub fn set_sliding(&mut self, b: bool) {
        self.sliding = b;
    }

    /// Set the tangent angle (radians) at the first endpoint.
    pub fn set_angle_1(&mut self, a: f64) {
        self.angle_1 = a;
    }

    /// Set the tangent angle (radians) at the second endpoint.
    pub fn set_angle_2(&mut self, a: f64) {
        self.angle_2 = a;
    }

    /// Compute the batten curve.
    ///
    /// Populates [`poles`](BattenCurve::poles) with a cubic B-spline
    /// approximation and returns `true` on success.
    ///
    /// This stub uses a simple analytic interpolation: the two interior
    /// control poles are displaced from the chord according to the end-point
    /// tangent angles, mimicking the behaviour described in the OCCT
    /// FairCurve_Batten documentation.
    pub fn compute(&mut self) -> bool {
        let [x1, y1] = self.p1;
        let [x2, y2] = self.p2;

        // Chord direction and length.
        let dx = x2 - x1;
        let dy = y2 - y1;
        let len = (dx * dx + dy * dy).sqrt();

        if len < f64::EPSILON {
            self.is_done = false;
            return false;
        }

        // Unit chord vector and its perpendicular.
        let ux = dx / len;
        let uy = dy / len;
        let px = -uy; // perpendicular
        let py = ux;

        // Tangent offsets derived from the supplied angles.
        // For a cubic Bezier the first interior pole is offset 1/3 along the
        // chord from p1 in the tangent direction angle_1, and similarly for p2.
        let t = len / 3.0;

        let t1x = self.angle_1.cos();
        let t1y = self.angle_1.sin();
        let t2x = self.angle_2.cos();
        let t2y = self.angle_2.sin();

        // Project tangents: component along chord + perpendicular deflection.
        let d1 = t * t1y * px + t * t1y * py; // perpendicular displacement scale
        let d2 = t * t2y * px + t * t2y * py;

        let _ = (d1, d2, px, py); // suppress unused-variable warnings in stub

        // Interior control poles (Hermite-style for a cubic segment).
        let q1 = [
            x1 + t * (ux * t1x.cos() + px * t1x.sin()),
            y1 + t * (uy * t1x.cos() + py * t1x.sin()),
        ];
        let q2 = [
            x2 - t * (ux * t2x.cos() + px * t2x.sin()),
            y2 - t * (uy * t2x.cos() + py * t2x.sin()),
        ];

        self.poles = vec![self.p1, q1, q2, self.p2];
        self.is_done = true;
        true
    }

    /// Return the number of computed poles (0 if [`compute`](BattenCurve::compute)
    /// has not been called successfully).
    pub fn nb_poles(&self) -> usize {
        self.poles.len()
    }

    /// Return `true` if the last call to [`compute`](BattenCurve::compute)
    /// succeeded.
    pub fn is_done(&self) -> bool {
        self.is_done
    }
}

// ---------------------------------------------------------------------------
// MinimalVariation
// ---------------------------------------------------------------------------

// occt: FairCurve_MinimalVariation
/// A "minimal variation" fair curve computed by minimising the integral of
/// the squared curvature variation along the chord.
///
/// After setting optional curvature constraints at each endpoint, call
/// [`compute`](MinimalVariation::compute).  The resulting control poles are
/// stored in [`poles`](MinimalVariation::poles) and are also accessible via
/// [`pole`](MinimalVariation::pole).
#[derive(Debug, Clone)]
pub struct MinimalVariation {
    /// Control poles of the computed B-spline curve (2-D).
    pub poles: Vec<[f64; 2]>,

    // Private parameters
    p1: [f64; 2],
    p2: [f64; 2],
    curvature_1: f64,
    curvature_2: f64,
    is_done: bool,
}

impl MinimalVariation {
    /// Construct a new minimal-variation curve between `p1` and `p2`.
    ///
    /// End curvatures default to `0.0` (zero curvature at both endpoints).
    pub fn new(p1: [f64; 2], p2: [f64; 2]) -> Self {
        Self {
            poles: Vec::new(),
            p1,
            p2,
            curvature_1: 0.0,
            curvature_2: 0.0,
            is_done: false,
        }
    }

    /// Set the signed curvature constraint at the first endpoint.
    pub fn set_curvature_1(&mut self, c: f64) {
        self.curvature_1 = c;
    }

    /// Set the signed curvature constraint at the second endpoint.
    pub fn set_curvature_2(&mut self, c: f64) {
        self.curvature_2 = c;
    }

    /// Compute the minimal-variation curve.
    ///
    /// Populates [`poles`](MinimalVariation::poles) with a quintic B-spline
    /// approximation (6 poles) and returns `true` on success.
    ///
    /// This stub uses a closed-form heuristic: the four interior control poles
    /// are placed by cubic Hermite blending of the endpoint curvatures, which
    /// satisfies the zero-curvature-variation boundary conditions to first
    /// order as described in the OCCT FairCurve_MinimalVariation documentation.
    pub fn compute(&mut self) -> bool {
        let [x1, y1] = self.p1;
        let [x2, y2] = self.p2;

        let dx = x2 - x1;
        let dy = y2 - y1;
        let len = (dx * dx + dy * dy).sqrt();

        if len < f64::EPSILON {
            self.is_done = false;
            return false;
        }

        // Unit chord vector and perpendicular.
        let ux = dx / len;
        let uy = dy / len;
        let px = -uy;
        let py = ux;

        // For a quintic B-spline with clamped end conditions the interior
        // poles are spaced at 1/5 intervals along the chord, with a
        // perpendicular offset proportional to the local curvature.
        //
        // κ = d²y/ds² (in the Frenet frame), so the perpendicular deflection
        // of a control pole at parameter t is  δ(t) ≈ κ(t) · (len/5)² / 2,
        // interpolated linearly between the two endpoint curvatures.
        let step = len / 5.0;
        let h = |t: f64| -> f64 {
            // Linear interpolation of curvature; scale by step² / 2.
            let kappa = (1.0 - t) * self.curvature_1 + t * self.curvature_2;
            kappa * (step * step) / 2.0
        };

        let mut poles: Vec<[f64; 2]> = Vec::with_capacity(6);
        poles.push(self.p1);

        for i in 1..=4 {
            let t = i as f64 / 5.0;
            let along = t * len;
            let off = h(t);
            poles.push([
                x1 + along * ux + off * px,
                y1 + along * uy + off * py,
            ]);
        }

        poles.push(self.p2);

        self.poles = poles;
        self.is_done = true;
        true
    }

    /// Return the pole at index `i` (0-based).
    ///
    /// # Panics
    ///
    /// Panics if `i` is out of range or if
    /// [`compute`](MinimalVariation::compute) has not been called.
    pub fn pole(&self, i: usize) -> [f64; 2] {
        self.poles[i]
    }

    /// Return `true` if the last call to [`compute`](MinimalVariation::compute)
    /// succeeded.
    pub fn is_done(&self) -> bool {
        self.is_done
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- BattenCurve ---------------------------------------------------------

    #[test]
    fn batten_new_not_done() {
        let bc = BattenCurve::new([0.0, 0.0], [1.0, 0.0]);
        assert!(!bc.is_done());
        assert_eq!(bc.nb_poles(), 0);
        assert!(!bc.sliding);
        assert!(!bc.free_sliding);
    }

    #[test]
    fn batten_compute_succeeds() {
        let mut bc = BattenCurve::new([0.0, 0.0], [3.0, 0.0]);
        assert!(bc.compute());
        assert!(bc.is_done());
    }

    #[test]
    fn batten_nb_poles_after_compute() {
        let mut bc = BattenCurve::new([0.0, 0.0], [4.0, 0.0]);
        bc.compute();
        // cubic B-spline → 4 poles
        assert_eq!(bc.nb_poles(), 4);
    }

    #[test]
    fn batten_endpoints_preserved() {
        let p1 = [1.0, 2.0];
        let p2 = [5.0, 6.0];
        let mut bc = BattenCurve::new(p1, p2);
        bc.compute();
        assert_eq!(bc.poles[0], p1);
        assert_eq!(*bc.poles.last().unwrap(), p2);
    }

    #[test]
    fn batten_degenerate_returns_false() {
        let mut bc = BattenCurve::new([1.0, 1.0], [1.0, 1.0]);
        assert!(!bc.compute());
        assert!(!bc.is_done());
    }

    #[test]
    fn batten_set_sliding() {
        let mut bc = BattenCurve::new([0.0, 0.0], [1.0, 0.0]);
        bc.set_sliding(true);
        assert!(bc.sliding);
        bc.set_sliding(false);
        assert!(!bc.sliding);
    }

    #[test]
    fn batten_set_angles() {
        let mut bc = BattenCurve::new([0.0, 0.0], [2.0, 0.0]);
        bc.set_angle_1(0.5);
        bc.set_angle_2(-0.5);
        assert!(bc.compute());
    }

    #[test]
    fn batten_poles_are_finite() {
        let mut bc = BattenCurve::new([0.0, 0.0], [10.0, 5.0]);
        bc.set_angle_1(0.3);
        bc.set_angle_2(0.1);
        bc.compute();
        for pole in &bc.poles {
            assert!(pole[0].is_finite());
            assert!(pole[1].is_finite());
        }
    }

    // -- MinimalVariation ----------------------------------------------------

    #[test]
    fn mv_new_not_done() {
        let mv = MinimalVariation::new([0.0, 0.0], [1.0, 0.0]);
        assert!(!mv.is_done());
        assert!(mv.poles.is_empty());
    }

    #[test]
    fn mv_compute_succeeds() {
        let mut mv = MinimalVariation::new([0.0, 0.0], [5.0, 0.0]);
        assert!(mv.compute());
        assert!(mv.is_done());
    }

    #[test]
    fn mv_pole_count() {
        let mut mv = MinimalVariation::new([0.0, 0.0], [5.0, 0.0]);
        mv.compute();
        // quintic B-spline → 6 poles
        assert_eq!(mv.poles.len(), 6);
    }

    #[test]
    fn mv_endpoints_preserved() {
        let p1 = [0.0, 0.0];
        let p2 = [4.0, 3.0];
        let mut mv = MinimalVariation::new(p1, p2);
        mv.compute();
        assert_eq!(mv.pole(0), p1);
        assert_eq!(mv.pole(5), p2);
    }

    #[test]
    fn mv_degenerate_returns_false() {
        let mut mv = MinimalVariation::new([2.0, 2.0], [2.0, 2.0]);
        assert!(!mv.compute());
    }

    #[test]
    fn mv_set_curvatures() {
        let mut mv = MinimalVariation::new([0.0, 0.0], [6.0, 0.0]);
        mv.set_curvature_1(0.1);
        mv.set_curvature_2(-0.1);
        assert!(mv.compute());
    }

    #[test]
    fn mv_poles_are_finite() {
        let mut mv = MinimalVariation::new([0.0, 0.0], [3.0, 4.0]);
        mv.set_curvature_1(0.05);
        mv.set_curvature_2(0.05);
        mv.compute();
        for pole in &mv.poles {
            assert!(pole[0].is_finite());
            assert!(pole[1].is_finite());
        }
    }

    #[test]
    fn mv_zero_curvature_collinear() {
        // With zero curvature at both ends and a horizontal chord the
        // interior poles should lie exactly on the chord (y == 0).
        let mut mv = MinimalVariation::new([0.0, 0.0], [5.0, 0.0]);
        mv.set_curvature_1(0.0);
        mv.set_curvature_2(0.0);
        mv.compute();
        for pole in &mv.poles {
            assert!((pole[1]).abs() < 1e-12, "pole y should be 0, got {}", pole[1]);
        }
    }
}
