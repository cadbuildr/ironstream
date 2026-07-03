// FILE: geom_merge.rs
//! Geometry merging utilities, mirroring OpenCascade's
//! `GeomConvert_CompCurveToBSplineCurve` and `BRepAlgo_Merge` packages.
//!
//! Provides:
//! - [`CompCurveToBSpline`]  — assembles a sequence of curves into a single
//!   composite B-spline representation
//! - [`MergeAlgo`]           — merges a collection of topological shapes into a
//!   unified result shape
//! - [`merge_curves`]        — convenience function wrapping [`CompCurveToBSpline`]

// ---------------------------------------------------------------------------
// CompCurveToBSpline
// ---------------------------------------------------------------------------

/// Assembles a chain of curves into a single composite B-spline curve.
///
/// Mirrors OpenCascade's `GeomConvert_CompCurveToBSplineCurve`.
// occt: GeomConvert_CompCurveToBSplineCurve
#[derive(Debug, Clone)]
pub struct CompCurveToBSpline {
    /// Input curve identifiers (names / tags) in assembly order.
    pub curves: Vec<String>,
    /// Control poles of the assembled B-spline in 3-D homogeneous space.
    pub result_poles: Vec<[f64; 3]>,
    /// Knot vector of the assembled B-spline (flat / clamped form).
    pub result_knots: Vec<f64>,
    /// Polynomial degree shared by all spans of the assembled curve.
    pub degree: u32,
    /// Set to `true` once at least one segment has been successfully merged.
    done: bool,
}

impl CompCurveToBSpline {
    /// Creates an empty assembler for curves of the given polynomial `degree`.
    ///
    /// Corresponds to `GeomConvert_CompCurveToBSplineCurve::GeomConvert_CompCurveToBSplineCurve`.
    pub fn new(degree: u32) -> Self {
        Self {
            curves: Vec::new(),
            result_poles: Vec::new(),
            result_knots: Vec::new(),
            degree,
            done: false,
        }
    }

    /// Appends `curve` to the assembly.
    ///
    /// When `after` is `true` the segment is appended at the tail of the
    /// current chain; when `false` it is prepended at the head.
    ///
    /// Corresponds to `GeomConvert_CompCurveToBSplineCurve::Add`.
    pub fn add(&mut self, curve: &str, after: bool) {
        if after {
            self.curves.push(curve.to_owned());
        } else {
            self.curves.insert(0, curve.to_owned());
        }

        // Rebuild a trivial stub knot vector and pole set so that the public
        // fields remain consistent with the logical state.  A production
        // implementation would convert each curve to B-spline form and merge
        // the knot vectors at the C0/G1 junction.
        let n = self.curves.len();
        let d = self.degree as usize;

        // Number of poles: (degree + 1) control points per segment, shared
        // interior knots reduce the total by one per junction.
        let nb_poles = d * n + 1;
        self.result_poles = (0..nb_poles)
            .map(|i| {
                let t = if nb_poles > 1 {
                    i as f64 / (nb_poles - 1) as f64
                } else {
                    0.0
                };
                [t, 0.0, 0.0]
            })
            .collect();

        // Clamped uniform knot vector: (degree+1) repeated endpoints,
        // (nb_poles - degree - 1) interior knots.
        let nb_knots = nb_poles + d + 1;
        self.result_knots = (0..nb_knots)
            .map(|i| {
                if i <= d {
                    0.0
                } else if i >= nb_knots - d - 1 {
                    1.0
                } else {
                    (i - d) as f64 / (nb_poles - d) as f64
                }
            })
            .collect();

        self.done = true;
    }

    /// Returns `true` when the assembly contains at least one valid segment.
    ///
    /// Corresponds to `GeomConvert_CompCurveToBSplineCurve::IsDone`.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns the number of control poles in the assembled B-spline.
    pub fn nb_poles(&self) -> usize {
        self.result_poles.len()
    }

    /// Evaluates the assembled B-spline at parameter `t` ∈ [0, 1] using
    /// de Boor's algorithm.
    ///
    /// Returns the origin when the assembler holds no poles.
    pub fn evaluate(&self, t: f64) -> [f64; 3] {
        let poles = &self.result_poles;
        let knots = &self.result_knots;
        let n = poles.len();
        if n == 0 {
            return [0.0, 0.0, 0.0];
        }

        let d = self.degree as usize;
        let m = knots.len();

        // Clamp parameter to valid domain.
        let u = t.clamp(knots[d], knots[m - d - 1]);

        // Find the active knot span index k such that knots[k] <= u < knots[k+1].
        let mut span = d;
        for i in d..(m - d - 1) {
            if u < knots[i + 1] || i == m - d - 2 {
                span = i;
                break;
            }
        }

        // de Boor's algorithm (non-rational, degree d).
        let mut pts: Vec<[f64; 3]> = (0..=d)
            .map(|j| {
                let idx = (span + j).saturating_sub(d);
                if idx < n {
                    poles[idx]
                } else {
                    [0.0, 0.0, 0.0]
                }
            })
            .collect();

        for r in 1..=d {
            for j in (r..=d).rev() {
                let i = span + j - d;
                let denom = knots[i + d - r + 1] - knots[i];
                let alpha = if denom.abs() < f64::EPSILON {
                    0.0
                } else {
                    (u - knots[i]) / denom
                };
                let lo = pts[j - 1];
                let hi = pts[j];
                pts[j] = [
                    (1.0 - alpha) * lo[0] + alpha * hi[0],
                    (1.0 - alpha) * lo[1] + alpha * hi[1],
                    (1.0 - alpha) * lo[2] + alpha * hi[2],
                ];
            }
        }

        pts[d]
    }
}

// ---------------------------------------------------------------------------
// MergeAlgo
// ---------------------------------------------------------------------------

/// Merges a set of topological shapes into a single unified shape.
///
/// Mirrors OpenCascade's `BRepAlgo_Merge`.
// occt: BRepAlgo_Merge
#[derive(Debug, Clone)]
pub struct MergeAlgo {
    /// Input shape identifiers (names / tags) to be merged.
    pub shapes: Vec<String>,
    /// Cached result shape identifier produced by [`MergeAlgo::perform`].
    result: Option<String>,
}

impl MergeAlgo {
    /// Creates a new, empty merge algorithm.
    ///
    /// Corresponds to `BRepAlgo_Merge::BRepAlgo_Merge`.
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            result: None,
        }
    }

    /// Registers `s` as an input shape.
    ///
    /// Corresponds to `BRepAlgo_Merge::Add`.
    pub fn add_shape(&mut self, s: &str) {
        self.shapes.push(s.to_owned());
        // Invalidate any cached result when the input set changes.
        self.result = None;
    }

    /// Executes the merge and returns the resulting shape identifier.
    ///
    /// A production implementation would walk the face/edge adjacency graph,
    /// remove internal seam edges shared between co-planar faces, and rebuild
    /// the compound.  This stub produces a deterministic label derived from
    /// the input identifiers.
    ///
    /// Corresponds to `BRepAlgo_Merge::Perform`.
    pub fn perform(&mut self) -> String {
        let merged = if self.shapes.is_empty() {
            String::from("MergedShape[]")
        } else {
            format!("MergedShape[{}]", self.shapes.join("+"))
        };
        self.result = Some(merged.clone());
        merged
    }

    /// Returns `true` when [`MergeAlgo::perform`] has produced a valid result.
    ///
    /// Corresponds to `BRepAlgo_Merge::IsDone`.
    pub fn is_done(&self) -> bool {
        self.result.is_some()
    }
}

impl Default for MergeAlgo {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Convenience function
// ---------------------------------------------------------------------------

/// Convenience wrapper: construct a [`CompCurveToBSpline`] from a slice of
/// curve identifiers using the default cubic degree (3), appending each curve
/// at the tail of the chain.
///
/// ```
/// let curves = vec!["arc1".to_owned(), "line2".to_owned()];
/// let bspline = ironstream::geom_merge::merge_curves(&curves);
/// assert!(bspline.is_done());
/// assert_eq!(bspline.nb_poles(), 7); // degree 3 * 2 segments + 1
/// ```
pub fn merge_curves(curves: &[String]) -> CompCurveToBSpline {
    let mut comp = CompCurveToBSpline::new(3);
    for c in curves {
        comp.add(c, true);
    }
    comp
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comp_curve_empty() {
        let c = CompCurveToBSpline::new(3);
        assert!(!c.is_done());
        assert_eq!(c.nb_poles(), 0);
        assert_eq!(c.evaluate(0.5), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn comp_curve_add_after() {
        let mut c = CompCurveToBSpline::new(3);
        c.add("seg_a", true);
        c.add("seg_b", true);
        assert!(c.is_done());
        assert_eq!(c.curves, vec!["seg_a", "seg_b"]);
        // degree 3, 2 segments → 3*2+1 = 7 poles
        assert_eq!(c.nb_poles(), 7);
    }

    #[test]
    fn comp_curve_add_before() {
        let mut c = CompCurveToBSpline::new(2);
        c.add("seg_a", true);
        c.add("seg_b", false);
        assert_eq!(c.curves, vec!["seg_b", "seg_a"]);
    }

    #[test]
    fn comp_curve_evaluate_endpoints() {
        let mut c = CompCurveToBSpline::new(3);
        c.add("seg_a", true);
        let p0 = c.evaluate(0.0);
        let p1 = c.evaluate(1.0);
        // Start pole at t=0 should map to [0,0,0], end pole to [1,0,0].
        assert!((p0[0] - 0.0).abs() < 1e-9, "start x={}", p0[0]);
        assert!((p1[0] - 1.0).abs() < 1e-9, "end x={}", p1[0]);
    }

    #[test]
    fn merge_algo_empty() {
        let mut m = MergeAlgo::new();
        assert!(!m.is_done());
        let r = m.perform();
        assert_eq!(r, "MergedShape[]");
        assert!(m.is_done());
    }

    #[test]
    fn merge_algo_shapes() {
        let mut m = MergeAlgo::new();
        m.add_shape("face1");
        m.add_shape("face2");
        assert!(!m.is_done());
        let r = m.perform();
        assert_eq!(r, "MergedShape[face1+face2]");
        assert!(m.is_done());
    }

    #[test]
    fn merge_algo_add_resets_done() {
        let mut m = MergeAlgo::new();
        m.add_shape("face1");
        m.perform();
        assert!(m.is_done());
        m.add_shape("face2");
        assert!(!m.is_done(), "adding a shape should invalidate result");
    }

    #[test]
    fn merge_curves_fn() {
        let curves: Vec<String> = ["arc1", "line2", "arc3"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let b = merge_curves(&curves);
        assert!(b.is_done());
        // degree 3, 3 segments → 3*3+1 = 10 poles
        assert_eq!(b.nb_poles(), 10);
        assert_eq!(b.curves.len(), 3);
    }
}
