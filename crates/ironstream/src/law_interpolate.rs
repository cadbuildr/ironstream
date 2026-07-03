// FILE: rust/ironstream/crates/ironstream/src/law_interpolate.rs
//! Scalar interpolation laws mirroring OpenCascade's `Law_Interpolate` and
//! `Law_S` packages.

// occt: Law_Interpolate
/// Interpolates a scalar law through given (parameter, value) pairs.
///
/// Points are sorted by parameter on construction.  After calling
/// [`LawInterpolate::perform`] the law is ready for evaluation via
/// [`LawInterpolate::evaluate`], which uses piecewise linear interpolation
/// (or extrapolation outside the range).
// occt: Law_Interpolate
#[derive(Clone, Debug)]
pub struct LawInterpolate {
    /// Sorted (parameter, value) pairs.
    points: Vec<(f64, f64)>,
    /// Whether the law has been finalised.
    is_done: bool,
    /// Whether the interpolation is periodic.
    is_periodic: bool,
    /// Optional tangent at each knot (same length as `points` when set).
    tangents: Option<Vec<f64>>,
}

impl LawInterpolate {
    /// Construct a new interpolation law from the given (parameter, value)
    /// pairs.  The pairs are sorted by parameter in ascending order.
    pub fn new(mut points: Vec<(f64, f64)>, is_periodic: bool) -> Self {
        points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        Self {
            points,
            is_done: false,
            is_periodic,
            tangents: None,
        }
    }

    /// Set the tangent at each knot.  The length of `tangents` must equal
    /// the number of interpolation points.  Tangents are stored but not used
    /// during linear evaluation.
    pub fn load(&mut self, tangents: Vec<f64>) {
        self.tangents = Some(tangents);
    }

    /// Finalise the interpolation.  Must be called before [`Self::evaluate`].
    pub fn perform(&mut self) {
        self.is_done = true;
    }

    /// Returns `true` if [`Self::perform`] has been called successfully.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Evaluate the law at parameter `t` using piecewise linear interpolation.
    ///
    /// If `t` lies outside the range of stored parameters the result is
    /// linearly extrapolated from the nearest segment.
    ///
    /// # Panics
    /// Panics if the law has fewer than two points or [`Self::perform`] has
    /// not been called.
    pub fn evaluate(&self, t: f64) -> f64 {
        assert!(self.is_done, "LawInterpolate::perform() must be called first");
        let pts = &self.points;
        assert!(pts.len() >= 2, "at least two points are required");

        // Extrapolate below the first knot using the first segment.
        if t <= pts[0].0 {
            let (t0, v0) = pts[0];
            let (t1, v1) = pts[1];
            let dt = t1 - t0;
            if dt == 0.0 {
                return v0;
            }
            return v0 + (t - t0) / dt * (v1 - v0);
        }

        // Extrapolate above the last knot using the last segment.
        let last = pts.len() - 1;
        if t >= pts[last].0 {
            let (t0, v0) = pts[last - 1];
            let (t1, v1) = pts[last];
            let dt = t1 - t0;
            if dt == 0.0 {
                return v1;
            }
            return v0 + (t - t0) / dt * (v1 - v0);
        }

        // Binary search for the segment containing `t`.
        let mut lo = 0usize;
        let mut hi = last;
        while hi - lo > 1 {
            let mid = (lo + hi) / 2;
            if pts[mid].0 <= t {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        let (t0, v0) = pts[lo];
        let (t1, v1) = pts[hi];
        let dt = t1 - t0;
        if dt == 0.0 {
            return v0;
        }
        v0 + (t - t0) / dt * (v1 - v0)
    }

    /// Number of interpolation points.
    pub fn nb_points(&self) -> usize {
        self.points.len()
    }

    /// Return the interpolation point at 0-based index `i`.
    ///
    /// # Panics
    /// Panics if `i` is out of bounds.
    pub fn point(&self, i: usize) -> (f64, f64) {
        self.points[i]
    }
}

// occt: Law_S
/// S-shaped ramp law that maps the interval `[first, last]` smoothly from 0
/// to 1 using the smooth-step polynomial 3t² − 2t³.
///
/// Outside the interval the law clamps to 0 (below `first`) or 1 (above
/// `last`).
// occt: Law_S
#[derive(Clone, Debug)]
pub struct LawS {
    first: f64,
    last: f64,
}

impl LawS {
    /// Construct an S-shaped law over `[first, last]`.
    pub fn new(first: f64, last: f64) -> Self {
        Self { first, last }
    }

    /// Evaluate the law at parameter `t`.
    ///
    /// Returns a value in `[0, 1]` using the smooth-step formula 3t²−2t³
    /// mapped onto `[first, last]`.  Clamps to 0 below `first` and to 1
    /// above `last`.
    pub fn evaluate(&self, t: f64) -> f64 {
        let span = self.last - self.first;
        if span == 0.0 {
            return if t < self.first { 0.0 } else { 1.0 };
        }
        let u = (t - self.first) / span;
        let u = u.clamp(0.0, 1.0);
        u * u * (3.0 - 2.0 * u)
    }

    /// The start of the law's parameter interval.
    pub fn first(&self) -> f64 {
        self.first
    }

    /// The end of the law's parameter interval.
    pub fn last(&self) -> f64 {
        self.last
    }
}
