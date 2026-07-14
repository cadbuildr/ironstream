// occt-ref: AppDef_MultiPointConstraint
// occt: AppParCurves_MultiCurve

/// Specifies the continuity order imposed at a constraint point.
///
/// Mirrors the constraint levels understood by AppDef_MultiPointConstraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintOrder {
    /// Positional continuity only (G0/C0).
    C0,
    /// Tangent continuity (G1/C1).
    C1,
    /// Curvature continuity (G2/C2).
    C2,
}

// occt-ref: AppDef_MultiPointConstraint
/// A set of 3-D points that must all be approximated at the same parameter
/// value, together with a continuity order and an optional weight.
///
/// Corresponds to `AppDef_MultiPointConstraint` from OCCT.
#[derive(Debug, Clone)]
pub struct MultiPointConstraint {
    /// The 3-D points in this constraint group.
    pub points: Vec<[f64; 3]>,
    /// Required continuity at these points.
    pub order: ConstraintOrder,
    /// Approximation weight (higher ⇒ tighter fit).
    pub weight: f64,
}

impl MultiPointConstraint {
    /// Create a new constraint from a list of 3-D points with default
    /// settings (`C0` continuity, weight `1.0`).
    pub fn new(pts: Vec<[f64; 3]>) -> Self {
        Self {
            points: pts,
            order: ConstraintOrder::C0,
            weight: 1.0,
        }
    }

    /// Return a clone of this constraint with the continuity order replaced.
    pub fn with_order(mut self, o: ConstraintOrder) -> Self {
        self.order = o;
        self
    }

    /// Return a clone of this constraint with the weight replaced.
    pub fn with_weight(mut self, w: f64) -> Self {
        self.weight = w;
        self
    }

    /// Number of points in this constraint group.
    pub fn point_count(&self) -> usize {
        self.points.len()
    }
}

// occt: AppParCurves_MultiCurve
/// A collection of polynomial Bezier curves of the same degree sharing a
/// common parameterisation.
///
/// Corresponds to `AppParCurves_MultiCurve` from OCCT.
#[derive(Debug, Clone)]
pub struct MultiCurve {
    /// Each inner `Vec<[f64;3]>` holds the control points of one curve.
    /// All curves share `degree`, so each must contain exactly `degree + 1`
    /// control points once fully constructed.
    pub curves: Vec<Vec<[f64; 3]>>,
    /// Polynomial degree of every curve in the collection.
    pub degree: u32,
}

impl MultiCurve {
    /// Create an empty `MultiCurve` with the given polynomial degree.
    pub fn new(degree: u32) -> Self {
        Self {
            curves: Vec::new(),
            degree,
        }
    }

    /// Append a curve defined by its control points.
    ///
    /// `pts` should contain exactly `degree + 1` points; this is not
    /// enforced here so that callers can populate incrementally.
    pub fn add_curve(&mut self, pts: Vec<[f64; 3]>) {
        self.curves.push(pts);
    }

    /// Evaluate curve `i` at parameter `t ∈ [0, 1]` using the de Casteljau
    /// algorithm.
    ///
    /// # Panics
    /// Panics if `i` is out of range or the curve has no control points.
    pub fn evaluate(&self, i: usize, t: f64) -> [f64; 3] {
        let ctrl = &self.curves[i];
        assert!(!ctrl.is_empty(), "curve {i} has no control points");

        // de Casteljau — O(n²) but dependency-free and numerically stable.
        let mut pts: Vec<[f64; 3]> = ctrl.clone();
        let n = pts.len();
        for r in 1..n {
            for j in 0..(n - r) {
                pts[j] = [
                    (1.0 - t) * pts[j][0] + t * pts[j + 1][0],
                    (1.0 - t) * pts[j][1] + t * pts[j + 1][1],
                    (1.0 - t) * pts[j][2] + t * pts[j + 1][2],
                ];
            }
        }
        pts[0]
    }

    /// Number of curves in this collection.
    pub fn curve_count(&self) -> usize {
        self.curves.len()
    }
}

/// Build a `MultiCurve` from a slice of `MultiPointConstraint` values.
///
/// This is a stub implementation that distributes the constraint points
/// uniformly as Bezier control points with degree `degree`, mirroring the
/// intent of `AppParCurves_MultiCurve`-based approximation in OCCT.
///
/// The construction proceeds as follows:
/// 1. Collect every point from every constraint into a single ordered list.
/// 2. If the list has fewer points than `degree + 1`, pad with repetitions of
///    the last point so the curve is always well-formed.
/// 3. Sub-sample / thin the point list to exactly `degree + 1` control points
///    by uniform striding, weighted by each constraint's `weight` field.
///
/// A real implementation would run a least-squares approximation; this stub
/// provides the correct type contract and a geometrically reasonable result
/// for downstream testing.
pub fn build_multi_curve(constraints: &[MultiPointConstraint], degree: u32) -> MultiCurve {
    let mut mc = MultiCurve::new(degree);

    if constraints.is_empty() {
        return mc;
    }

    // Flatten all constraint points into a single list, respecting weight by
    // repeating points (clamped to a reasonable multiplier to avoid explosion).
    let mut flat: Vec<[f64; 3]> = Vec::new();
    for c in constraints {
        let reps = (c.weight.ceil() as usize).max(1).min(8);
        for &pt in &c.points {
            for _ in 0..reps {
                flat.push(pt);
            }
        }
    }

    if flat.is_empty() {
        return mc;
    }

    let n_ctrl = (degree as usize) + 1;

    // Ensure we always have at least n_ctrl source points by padding.
    while flat.len() < n_ctrl {
        let last = *flat.last().unwrap();
        flat.push(last);
    }

    // Sub-sample to n_ctrl control points by uniform striding.
    let src_len = flat.len();
    let ctrl: Vec<[f64; 3]> = (0..n_ctrl)
        .map(|k| {
            // Map k uniformly into [0, src_len - 1].
            let idx = if n_ctrl == 1 {
                0
            } else {
                (k * (src_len - 1)) / (n_ctrl - 1)
            };
            flat[idx]
        })
        .collect();

    mc.add_curve(ctrl);
    mc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constraint_builder_chain() {
        let c = MultiPointConstraint::new(vec![[0.0, 0.0, 0.0], [1.0, 1.0, 1.0]])
            .with_order(ConstraintOrder::C1)
            .with_weight(2.0);
        assert_eq!(c.point_count(), 2);
        assert_eq!(c.order, ConstraintOrder::C1);
        assert!((c.weight - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn multi_curve_evaluate_endpoints() {
        let mut mc = MultiCurve::new(1);
        mc.add_curve(vec![[0.0, 0.0, 0.0], [1.0, 2.0, 3.0]]);
        let p0 = mc.evaluate(0, 0.0);
        let p1 = mc.evaluate(0, 1.0);
        assert!((p0[0] - 0.0).abs() < 1e-12);
        assert!((p1[0] - 1.0).abs() < 1e-12);
        assert!((p1[1] - 2.0).abs() < 1e-12);
        assert!((p1[2] - 3.0).abs() < 1e-12);
    }

    #[test]
    fn multi_curve_midpoint_linear() {
        let mut mc = MultiCurve::new(1);
        mc.add_curve(vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]);
        let mid = mc.evaluate(0, 0.5);
        assert!((mid[0] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn build_multi_curve_basic() {
        let constraints = vec![
            MultiPointConstraint::new(vec![[0.0, 0.0, 0.0]]),
            MultiPointConstraint::new(vec![[1.0, 1.0, 0.0]]),
            MultiPointConstraint::new(vec![[2.0, 0.0, 0.0]]),
        ];
        let mc = build_multi_curve(&constraints, 2);
        assert_eq!(mc.curve_count(), 1);
        assert_eq!(mc.curves[0].len(), 3); // degree + 1
    }

    #[test]
    fn build_multi_curve_empty() {
        let mc = build_multi_curve(&[], 3);
        assert_eq!(mc.curve_count(), 0);
    }

    #[test]
    fn curve_count() {
        let mut mc = MultiCurve::new(2);
        assert_eq!(mc.curve_count(), 0);
        mc.add_curve(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]]);
        assert_eq!(mc.curve_count(), 1);
    }
}
