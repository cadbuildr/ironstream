// FILE: src/geom_subdivide.rs
//
// Pure-Rust zero-dependency stubs modelled after OCCT ShapeUpgrade_SplitCurve3d
// and ShapeUpgrade_SplitSurface.  No external crates are used; only std.

// ---------------------------------------------------------------------------
// SplitCurve3d
// ---------------------------------------------------------------------------

// occt: ShapeUpgrade_SplitCurve3d
/// Stub for the OCCT class that splits a 3-D curve at prescribed parameter
/// values.  The `status` field mirrors ShapeExtend_Status: 0 = not done,
/// 1 = done (OK), 2 = done (approximated), 3 = fail.
#[derive(Debug, Clone)]
pub struct SplitCurve3d {
    /// occt-note: handle to the underlying 3-D curve (represented as its name/id)
    pub curve: String,
    /// occt-note: ordered list of parameter values at which the curve is split
    pub split_values: Vec<f64>,
    /// occt-note: status code after perform() — 0 = not done, 1 = done, 3 = fail
    pub status: u32,

    first: f64,
    last: f64,
    results: Vec<String>,
}

impl SplitCurve3d {
    /// occt: ShapeUpgrade_SplitCurve3d // ::ShapeUpgrade_SplitCurve3d
    ///
    /// Create a new splitter for the curve identified by `curve`.  The
    /// parameter range defaults to [0.0, 1.0]; call [`init`] to override.
    pub fn new(curve: &str) -> Self {
        Self {
            curve: curve.to_owned(),
            split_values: Vec::new(),
            status: 0,
            first: 0.0,
            last: 1.0,
            results: Vec::new(),
        }
    }

    /// occt: ShapeUpgrade_SplitCurve3d // ::Init
    ///
    /// (Re-)initialise the splitter for the given curve and parameter range
    /// `[first, last]`.  Any previously accumulated split values or results are
    /// discarded.
    pub fn init(&mut self, curve: &str, first: f64, last: f64) {
        self.curve = curve.to_owned();
        self.first = first;
        self.last = last;
        self.split_values.clear();
        self.results.clear();
        self.status = 0;
    }

    /// occt: ShapeUpgrade_SplitCurve3d // ::SetSplitValues
    ///
    /// Supply the full ordered list of interior split parameters.  Values
    /// outside `(first, last)` are silently dropped.  Duplicates are removed.
    pub fn set_split_values(&mut self, vals: Vec<f64>) {
        let lo = self.first;
        let hi = self.last;
        let mut filtered: Vec<f64> = vals
            .into_iter()
            .filter(|&v| v > lo && v < hi)
            .collect();
        filtered.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        filtered.dedup_by(|a, b| (*a - *b).abs() < 1.0e-12);
        self.split_values = filtered;
    }

    /// occt: ShapeUpgrade_SplitCurve3d // ::Perform
    ///
    /// Execute the split algorithm.  When `approximate` is `true` the result
    /// segments would be approximated by B-splines in a real implementation;
    /// this stub records `status = 2` in that case instead of `status = 1`.
    ///
    /// Each segment is represented as a string of the form
    /// `"<curve>[<t0>..<t1>]"`.
    pub fn perform(&mut self, approximate: bool) {
        if self.curve.is_empty() {
            self.status = 3; // fail
            self.results.clear();
            return;
        }

        // Build break-point list: first + interior splits + last
        let mut breaks: Vec<f64> = Vec::with_capacity(self.split_values.len() + 2);
        breaks.push(self.first);
        breaks.extend_from_slice(&self.split_values);
        breaks.push(self.last);

        self.results.clear();
        for w in breaks.windows(2) {
            self.results.push(format!("{}[{}..{}]", self.curve, w[0], w[1]));
        }

        self.status = if approximate { 2 } else { 1 };
    }

    /// occt: ShapeUpgrade_SplitCurve3d // ::NbResults
    ///
    /// Return the number of curve segments produced by the last [`perform`]
    /// call.  Returns 0 if [`perform`] has not yet been called or failed.
    pub fn nb_results(&self) -> usize {
        self.results.len()
    }

    /// occt: ShapeUpgrade_SplitCurve3d // ::Status
    ///
    /// Return the status set by the last [`perform`] call:
    /// * `0` — not yet executed
    /// * `1` — done (exact)
    /// * `2` — done (approximate)
    /// * `3` — failed
    pub fn status(&self) -> u32 {
        self.status
    }
}

// ---------------------------------------------------------------------------
// SplitSurface
// ---------------------------------------------------------------------------

// occt: ShapeUpgrade_SplitSurface
/// Stub for the OCCT class that splits a surface along iso-lines in the U
/// and/or V parametric directions at prescribed parameter values.
#[derive(Debug, Clone)]
pub struct SplitSurface {
    /// occt-note: handle to the underlying surface (represented as its name/id)
    pub surface: String,
    /// occt-note: ordered list of U parameter values at which the surface is split
    pub u_splits: Vec<f64>,
    /// occt-note: ordered list of V parameter values at which the surface is split
    pub v_splits: Vec<f64>,

    results: Vec<String>,
    status: u32,
}

impl SplitSurface {
    /// occt: ShapeUpgrade_SplitSurface // ::ShapeUpgrade_SplitSurface
    ///
    /// Create a new splitter for the surface identified by `surface`.
    pub fn new(surface: &str) -> Self {
        Self {
            surface: surface.to_owned(),
            u_splits: Vec::new(),
            v_splits: Vec::new(),
            results: Vec::new(),
            status: 0,
        }
    }

    /// occt: ShapeUpgrade_SplitSurface // ::SetUSplitValues
    ///
    /// Supply the full ordered list of interior U split parameters.  Duplicates
    /// are removed.
    pub fn set_u_splits(&mut self, vals: Vec<f64>) {
        let mut v = vals;
        v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        v.dedup_by(|a, b| (*a - *b).abs() < 1.0e-12);
        self.u_splits = v;
    }

    /// occt: ShapeUpgrade_SplitSurface // ::SetVSplitValues
    ///
    /// Supply the full ordered list of interior V split parameters.  Duplicates
    /// are removed.
    pub fn set_v_splits(&mut self, vals: Vec<f64>) {
        let mut v = vals;
        v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        v.dedup_by(|a, b| (*a - *b).abs() < 1.0e-12);
        self.v_splits = v;
    }

    /// occt: ShapeUpgrade_SplitSurface // ::Perform
    ///
    /// Execute the split algorithm.  Each rectangular patch defined by
    /// consecutive U and V break-points is recorded as a result string of the
    /// form `"<surface>[u:<u0>..<u1>,v:<v0>..<v1>]"`.
    ///
    /// When `approximate` is `true` the patch boundaries would be approximated
    /// by B-spline surfaces in a real implementation; this stub records the
    /// flag for future use but otherwise behaves identically.
    pub fn perform(&mut self, _approximate: bool) {
        if self.surface.is_empty() {
            self.status = 3; // fail
            self.results.clear();
            return;
        }

        // If no explicit splits were provided, the whole surface is one patch.
        // We use sentinel values so that windows(2) always yields at least one pair.
        let u_breaks: Vec<f64> = if self.u_splits.is_empty() {
            vec![0.0, 1.0]
        } else {
            // Wrap with domain defaults of 0.0 / 1.0 as outer boundaries.
            let mut b = Vec::with_capacity(self.u_splits.len() + 2);
            b.push(0.0_f64.min(*self.u_splits.first().unwrap() - 1.0e-9));
            b.extend_from_slice(&self.u_splits);
            b.push(1.0_f64.max(*self.u_splits.last().unwrap() + 1.0e-9));
            b
        };

        let v_breaks: Vec<f64> = if self.v_splits.is_empty() {
            vec![0.0, 1.0]
        } else {
            let mut b = Vec::with_capacity(self.v_splits.len() + 2);
            b.push(0.0_f64.min(*self.v_splits.first().unwrap() - 1.0e-9));
            b.extend_from_slice(&self.v_splits);
            b.push(1.0_f64.max(*self.v_splits.last().unwrap() + 1.0e-9));
            b
        };

        self.results.clear();
        for uw in u_breaks.windows(2) {
            for vw in v_breaks.windows(2) {
                self.results.push(format!(
                    "{}[u:{}..{},v:{}..{}]",
                    self.surface, uw[0], uw[1], vw[0], vw[1]
                ));
            }
        }

        self.status = 1; // done
    }

    /// occt: ShapeUpgrade_SplitSurface // ::NbResults
    ///
    /// Return the number of surface patches produced by the last [`perform`]
    /// call.  Returns 0 if [`perform`] has not yet been called or failed.
    pub fn nb_results(&self) -> usize {
        self.results.len()
    }
}

// ---------------------------------------------------------------------------
// Free-standing helpers
// ---------------------------------------------------------------------------

/// Split a curve at the given parameter values, returning one string token per
/// segment in the form `"<curve>[<t_i>..<t_{i+1}]"`.
///
/// Equivalent to constructing a [`SplitCurve3d`], calling
/// [`SplitCurve3d::set_split_values`] and [`SplitCurve3d::perform`], then
/// collecting the result strings.
///
/// If `params` is empty or `curve` is empty, a single segment spanning the
/// inferred range is returned (or an empty `Vec` for an empty curve name).
pub fn split_curve_at_params(curve: &str, params: &[f64]) -> Vec<String> {
    if curve.is_empty() {
        return Vec::new();
    }

    // Derive a parameter range from the supplied split points, defaulting to
    // [0.0, 1.0] when none are given.
    let first = params
        .iter()
        .copied()
        .reduce(f64::min)
        .map(|v| v - 1.0)
        .unwrap_or(0.0);
    let last = params
        .iter()
        .copied()
        .reduce(f64::max)
        .map(|v| v + 1.0)
        .unwrap_or(1.0);

    let mut splitter = SplitCurve3d::new(curve);
    splitter.init(curve, first, last);
    splitter.set_split_values(params.to_vec());
    splitter.perform(false);

    // Expose the internal results via nb_results + the repr we built.
    // SplitCurve3d keeps results private, so re-derive them here identically.
    let mut breaks: Vec<f64> = Vec::with_capacity(params.len() + 2);
    breaks.push(first);
    let mut interior: Vec<f64> = params
        .iter()
        .copied()
        .filter(|&v| v > first && v < last)
        .collect();
    interior.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    interior.dedup_by(|a, b| (*a - *b).abs() < 1.0e-12);
    breaks.extend(interior);
    breaks.push(last);

    breaks
        .windows(2)
        .map(|w| format!("{}[{}..{}]", curve, w[0], w[1]))
        .collect()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_curve3d_basic() {
        let mut sc = SplitCurve3d::new("line");
        sc.init("line", 0.0, 10.0);
        sc.set_split_values(vec![2.5, 5.0, 7.5]);
        sc.perform(false);
        assert_eq!(sc.status(), 1);
        assert_eq!(sc.nb_results(), 4);
    }

    #[test]
    fn split_curve3d_approximate() {
        let mut sc = SplitCurve3d::new("spline");
        sc.init("spline", 0.0, 1.0);
        sc.set_split_values(vec![0.5]);
        sc.perform(true);
        assert_eq!(sc.status(), 2);
        assert_eq!(sc.nb_results(), 2);
    }

    #[test]
    fn split_curve3d_empty_curve_fails() {
        let mut sc = SplitCurve3d::new("");
        sc.perform(false);
        assert_eq!(sc.status(), 3);
        assert_eq!(sc.nb_results(), 0);
    }

    #[test]
    fn split_curve3d_no_splits_yields_one_segment() {
        let mut sc = SplitCurve3d::new("arc");
        sc.init("arc", 0.0, 1.0);
        sc.perform(false);
        assert_eq!(sc.nb_results(), 1);
    }

    #[test]
    fn split_surface_basic() {
        let mut ss = SplitSurface::new("plane");
        ss.set_u_splits(vec![0.33, 0.66]);
        ss.set_v_splits(vec![0.5]);
        ss.perform(false);
        // 3 U intervals x 2 V intervals = 6 patches
        assert_eq!(ss.nb_results(), 6);
    }

    #[test]
    fn split_surface_no_splits() {
        let mut ss = SplitSurface::new("sphere");
        ss.perform(false);
        assert_eq!(ss.nb_results(), 1);
    }

    #[test]
    fn split_surface_empty_name_fails() {
        let mut ss = SplitSurface::new("");
        ss.perform(false);
        assert_eq!(ss.nb_results(), 0);
    }

    #[test]
    fn split_curve_at_params_helper() {
        let segs = split_curve_at_params("bspline", &[1.0, 2.0, 3.0]);
        // params in range (-inf,0) < 1 < 2 < 3 < (inf,4) → 4 segments
        assert_eq!(segs.len(), 4);
        for s in &segs {
            assert!(s.starts_with("bspline["));
        }
    }

    #[test]
    fn split_curve_at_params_empty_curve() {
        let segs = split_curve_at_params("", &[0.5]);
        assert!(segs.is_empty());
    }

    #[test]
    fn split_curve_at_params_no_params() {
        let segs = split_curve_at_params("line", &[]);
        assert_eq!(segs.len(), 1);
    }
}
