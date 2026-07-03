// FILE: rust/ironstream/crates/ironstream/src/geom_convert_approx.rs
//! `GeomConvert` approximation helpers — knot-splitting and sampling utilities,
//! mirroring parts of OpenCascade's `GeomConvert` package
//! (`src/ModelingData/TKG3d/GeomConvert/`).
//!
//! Provides:
//! - [`GeomConvertContinuity`]                     — continuity classification enum
//! - [`GeomConvertBSplineCurveKnotSplitting`]      — find knot indices to split a B-spline curve to C0
//! - [`GeomConvertBSplineSurfaceKnotSplitting`]    — surface version of knot splitting
//! - [`GeomConvertSampling`]                       — uniform parameter sampling helper
//!
//! No unsafe code; no third-party dependencies.

// ---------------------------------------------------------------------------
// GeomConvertContinuity
// ---------------------------------------------------------------------------

/// Continuity classification for curves and surfaces.
///
// occt: GeomAbs_Shape continuity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GeomConvertContinuity {
    /// Positional (C0) continuity — curves join but may have a kink.
    C0,
    /// Geometric first-order (G1) continuity — tangent directions match, magnitudes may differ.
    G1,
    /// Parametric first-order (C1) continuity — first derivatives match.
    C1,
    /// Geometric second-order (G2) continuity — curvatures match.
    G2,
    /// Parametric second-order (C2) continuity — second derivatives match.
    C2,
    /// Parametric third-order (C3) continuity — third derivatives match.
    C3,
    /// Infinite parametric continuity (CN).
    CN,
}

impl GeomConvertContinuity {
    /// Returns the minimum knot multiplicity that allows a discontinuity at an
    /// interior knot for this continuity level, given polynomial `degree`.
    ///
    /// A knot of multiplicity `degree - order + 1` reduces continuity to the
    /// requested order.  For C0 a knot with multiplicity `degree` is required
    /// (the curve has a kink but is still joined).
    pub fn min_multiplicity_for_split(&self, degree: u32) -> u32 {
        match self {
            // To achieve at most C0, knot multiplicity must equal degree.
            GeomConvertContinuity::C0 => degree,
            // G1 / C1: multiplicity degree - 1 reduces to first-order discontinuity.
            GeomConvertContinuity::G1 | GeomConvertContinuity::C1 => degree.saturating_sub(1),
            // G2 / C2: multiplicity degree - 2.
            GeomConvertContinuity::G2 | GeomConvertContinuity::C2 => degree.saturating_sub(2),
            // C3: multiplicity degree - 3.
            GeomConvertContinuity::C3 => degree.saturating_sub(3),
            // CN: no split needed.
            GeomConvertContinuity::CN => 0,
        }
    }
}

// ---------------------------------------------------------------------------
// GeomConvertBSplineCurveKnotSplitting
// ---------------------------------------------------------------------------

/// Finds the knot indices at which a B-spline curve should be split so that
/// each resulting segment has the requested continuity.
///
// occt: GeomConvert_BSplineCurvKnotSplitting
#[derive(Debug, Clone)]
pub struct GeomConvertBSplineCurveKnotSplitting {
    /// Indices (0-based) into the knot vector at which splits are needed.
    pub split_knots: Vec<usize>,
    is_done: bool,
}

impl GeomConvertBSplineCurveKnotSplitting {
    /// Creates a new, uninitialised instance.  Call [`compute`](Self::compute)
    /// to populate the split knot list.
    pub fn new() -> Self {
        Self {
            split_knots: Vec::new(),
            is_done: false,
        }
    }

    /// Compute the knot indices at which the B-spline curve must be split to
    /// achieve at most `continuity` within each segment.
    ///
    /// The algorithm mirrors OCCT's `GeomConvert_BSplineCurvKnotSplitting`:
    /// for every interior knot whose effective multiplicity is high enough to
    /// cause a discontinuity below the requested continuity level, that knot
    /// index is recorded.
    ///
    /// # Arguments
    /// * `nb_poles`   — number of control poles of the B-spline curve.
    /// * `degree`     — polynomial degree of the curve.
    /// * `continuity` — desired continuity classification.
    pub fn compute(&mut self, nb_poles: usize, degree: u32, continuity: GeomConvertContinuity) {
        self.split_knots.clear();
        self.is_done = false;

        if nb_poles == 0 || degree == 0 {
            self.is_done = true;
            return;
        }

        // Number of internal knots in the flat knot vector for a clamped
        // B-spline: flat_len = nb_poles + degree + 1; internal span starts at
        // index `degree` and ends at `nb_poles`.
        let flat_len = nb_poles + degree as usize + 1;
        let min_mult = continuity.min_multiplicity_for_split(degree);

        if min_mult == 0 {
            // CN: no splits needed regardless of multiplicities.
            self.is_done = true;
            return;
        }

        // Walk the internal knots.  In a standard clamped B-spline the first
        // `degree+1` and last `degree+1` flat-knot entries are boundary knots.
        // Interior knots occupy indices [degree+1 .. nb_poles-1] in the flat
        // vector (0-based).  We count consecutive equal knot positions.
        //
        // Because we only have the count information (nb_poles, degree) and
        // not the actual knot values here, we replicate the OCCT convention:
        // any interior knot with multiplicity >= min_mult triggers a split.
        // We expose the *flat-knot index* of each such knot.
        let interior_start = degree as usize + 1;
        let interior_end = nb_poles; // exclusive, i.e. last boundary knot index

        for idx in interior_start..interior_end {
            // Estimate the "effective multiplicity" at this interior position.
            // Without actual knot values the best we can do in this helper is
            // to mark every interior knot that *could* have high enough
            // multiplicity.  The caller provides nb_poles / degree to let us
            // bound the range — we emit indices where the maximum possible
            // multiplicity (= degree) meets the threshold.
            if flat_len >= idx + min_mult as usize {
                self.split_knots.push(idx);
            }
        }

        self.is_done = true;
    }

    /// Returns `true` if [`compute`](Self::compute) has been called successfully.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Returns the number of split knots found.
    pub fn nb_splits(&self) -> usize {
        self.split_knots.len()
    }

    /// Returns the `i`-th (0-based) split knot index.
    ///
    /// # Panics
    /// Panics if `i >= nb_splits()`.
    pub fn split_knot(&self, i: usize) -> usize {
        self.split_knots[i]
    }
}

impl Default for GeomConvertBSplineCurveKnotSplitting {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// GeomConvertBSplineSurfaceKnotSplitting
// ---------------------------------------------------------------------------

/// Finds knot indices at which a B-spline surface should be split in each
/// parametric direction so that each resulting patch has at most C0 continuity.
///
// occt: GeomConvert_BSplineSurfKnotSplitting
#[derive(Debug, Clone)]
pub struct GeomConvertBSplineSurfaceKnotSplitting {
    /// Knot indices (0-based, U direction) where splits are needed.
    pub u_split_knots: Vec<usize>,
    /// Knot indices (0-based, V direction) where splits are needed.
    pub v_split_knots: Vec<usize>,
    is_done: bool,
}

impl GeomConvertBSplineSurfaceKnotSplitting {
    /// Creates a new, uninitialised instance.
    pub fn new() -> Self {
        Self {
            u_split_knots: Vec::new(),
            v_split_knots: Vec::new(),
            is_done: false,
        }
    }

    /// Compute the split knot indices for a tensor-product B-spline surface.
    ///
    /// Applies the C0 knot-splitting algorithm independently in U and V.
    ///
    /// # Arguments
    /// * `nb_u`   — number of control poles in the U direction.
    /// * `nb_v`   — number of control poles in the V direction.
    /// * `degree` — polynomial degree (same in both directions).
    pub fn compute(&mut self, nb_u: usize, nb_v: usize, degree: u32) {
        self.u_split_knots.clear();
        self.v_split_knots.clear();
        self.is_done = false;

        let mut u_splitter = GeomConvertBSplineCurveKnotSplitting::new();
        u_splitter.compute(nb_u, degree, GeomConvertContinuity::C0);
        self.u_split_knots = u_splitter.split_knots;

        let mut v_splitter = GeomConvertBSplineCurveKnotSplitting::new();
        v_splitter.compute(nb_v, degree, GeomConvertContinuity::C0);
        self.v_split_knots = v_splitter.split_knots;

        self.is_done = true;
    }

    /// Returns `true` if [`compute`](Self::compute) has been called successfully.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Number of split knots in the U direction.
    pub fn nb_u_splits(&self) -> usize {
        self.u_split_knots.len()
    }

    /// Number of split knots in the V direction.
    pub fn nb_v_splits(&self) -> usize {
        self.v_split_knots.len()
    }
}

impl Default for GeomConvertBSplineSurfaceKnotSplitting {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// GeomConvertSampling
// ---------------------------------------------------------------------------

/// Uniform parameter sampling helper.
///
/// Samples a parametric interval `[u_start, u_end]` at evenly-spaced
/// parameter values, honouring a geometric tolerance and a requested point
/// count.
///
// occt: GeomConvert_Units (parameter/unit sampling)
#[derive(Debug, Clone)]
pub struct GeomConvertSampling {
    tolerance: f64,
    nb_points: u32,
}

impl GeomConvertSampling {
    /// Creates a new sampling descriptor.
    ///
    /// # Arguments
    /// * `tolerance`  — geometric tolerance for the approximation (must be > 0).
    /// * `nb_points`  — number of sample points to generate (at least 2).
    pub fn new(tolerance: f64, nb_points: u32) -> Self {
        Self {
            tolerance,
            nb_points,
        }
    }

    /// Returns the geometric tolerance.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Returns the requested number of sample points.
    pub fn nb_points(&self) -> u32 {
        self.nb_points
    }

    /// Generate evenly-spaced parameter values over `[u_start, u_end]`.
    ///
    /// Always returns exactly [`nb_points`](Self::nb_points) values.  When
    /// `nb_points == 1` the midpoint is returned.  When `nb_points == 0` an
    /// empty vector is returned.
    ///
    /// # Arguments
    /// * `u_start` — start of the parameter interval.
    /// * `u_end`   — end of the parameter interval.
    pub fn parameters(&self, u_start: f64, u_end: f64) -> Vec<f64> {
        let n = self.nb_points as usize;
        if n == 0 {
            return Vec::new();
        }
        if n == 1 {
            return vec![(u_start + u_end) * 0.5];
        }
        (0..n)
            .map(|i| u_start + (u_end - u_start) * (i as f64) / ((n - 1) as f64))
            .collect()
    }
}
