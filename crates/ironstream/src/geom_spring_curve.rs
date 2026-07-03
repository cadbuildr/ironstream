// FILE: rust/ironstream/crates/ironstream/src/geom_spring_curve.rs
//! `Geom_TrimmedCurve`-based spring / composite curve.
//!
//! A [`SpringCurve`] is an ordered sequence of [`SpringCurveSegment`]s, each of
//! which records a parameter interval `[u_start, u_end]` and a human-readable
//! label, mirroring the way OCCT builds a composite spring geometry from a chain
//! of `Geom_TrimmedCurve` segments.

// occt: Geom_TrimmedCurve segment
/// One trimmed-curve segment of a spring: a closed parameter interval
/// `[u_start, u_end]` with an identifying label.
///
/// Mirrors a single `Geom_TrimmedCurve` node inside a spring assembly.
// occt: Geom_TrimmedCurve
#[derive(Clone, Debug)]
pub struct SpringCurveSegment {
    /// Parameter at the start of the trim.
    // occt: Geom_TrimmedCurve::FirstParameter
    u_start: f64,
    /// Parameter at the end of the trim.
    // occt: Geom_TrimmedCurve::LastParameter
    u_end: f64,
    /// Human-readable identifier for this segment.
    // occt: label / user-data field on a shape
    label: String,
}

impl SpringCurveSegment {
    /// Create a new trimmed-curve segment.
    ///
    /// # Arguments
    /// * `u_start` — start parameter of the trim interval.
    /// * `u_end`   — end parameter of the trim interval.
    /// * `label`   — identifying name for this segment.
    // occt: Geom_TrimmedCurve::new (constructor)
    pub fn new(u_start: f64, u_end: f64, label: &str) -> Self {
        Self {
            u_start,
            u_end,
            label: label.to_owned(),
        }
    }

    /// Start parameter of the trim interval.
    // occt: Geom_TrimmedCurve::FirstParameter
    pub fn u_start(&self) -> f64 {
        self.u_start
    }

    /// End parameter of the trim interval.
    // occt: Geom_TrimmedCurve::LastParameter
    pub fn u_end(&self) -> f64 {
        self.u_end
    }

    /// Identifying label for this segment.
    // occt: user-data / name field
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Parametric length of this segment (`u_end − u_start`).
    ///
    /// Mirrors `Geom_TrimmedCurve::LastParameter() - FirstParameter()`.
    // occt: Geom_TrimmedCurve parametric length
    pub fn length(&self) -> f64 {
        self.u_end - self.u_start
    }

    /// Parameter at the midpoint of the trim interval.
    // occt: (FirstParameter + LastParameter) / 2.0
    pub fn mid_param(&self) -> f64 {
        (self.u_start + self.u_end) * 0.5
    }
}

// ─────────────────────────────── free function ───────────────────────────────

/// Convenience constructor — mirrors `Geom_TrimmedCurve::new`.
///
/// Equivalent to `SpringCurveSegment::new(u_start, u_end, label)`.
// occt: Geom_TrimmedCurve::new
pub fn trim_curve(u_start: f64, u_end: f64, label: &str) -> SpringCurveSegment {
    SpringCurveSegment::new(u_start, u_end, label)
}

// ─────────────────────────────── SpringCurve ─────────────────────────────────

/// Composite spring curve composed of an ordered sequence of
/// [`SpringCurveSegment`]s.
///
/// Models the OCCT pattern of stitching multiple `Geom_TrimmedCurve` objects
/// end-to-end to describe a spring coil path.  Call [`SpringCurve::add_segment`]
/// to append segments in order, then [`SpringCurve::build`] to finalise
/// `total_length` and `is_closed`.
// occt: Geom_TrimmedCurve composite (spring assembly)
#[derive(Clone, Debug)]
pub struct SpringCurve {
    /// Ordered list of trimmed-curve segments.
    // occt: sequence of Geom_TrimmedCurve handles
    segments: Vec<SpringCurveSegment>,
    /// True when the curve forms a closed loop
    /// (`first.u_start ≈ last.u_end` within floating-point tolerance).
    // occt: Geom_Curve::IsClosed
    is_closed: bool,
    /// Sum of all segment parametric lengths; set by [`SpringCurve::build`].
    // occt: total arc-length accumulator
    total_length: f64,
}

/// Tolerance used to decide whether the curve is closed.
// occt: Precision::Confusion() analogue
const CLOSURE_TOL: f64 = 1e-7;

impl SpringCurve {
    /// Create an empty spring curve.
    // occt: constructor with empty segment list
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
            is_closed: false,
            total_length: 0.0,
        }
    }

    /// Append a segment to the end of the curve.
    // occt: append Geom_TrimmedCurve to composite
    pub fn add_segment(&mut self, s: SpringCurveSegment) {
        self.segments.push(s);
    }

    /// Finalise the curve: compute `total_length` and determine `is_closed`.
    ///
    /// `is_closed` is set to `true` when the `u_start` of the first segment is
    /// within [`CLOSURE_TOL`] of the `u_end` of the last segment.
    // occt: Geom_TrimmedCurve composite build / IsClosed evaluation
    pub fn build(&mut self) {
        self.total_length = self.segments.iter().map(|s| s.length()).sum();

        self.is_closed = if let (Some(first), Some(last)) =
            (self.segments.first(), self.segments.last())
        {
            (first.u_start - last.u_end).abs() < CLOSURE_TOL
        } else {
            false
        };
    }

    /// Whether the composite curve is closed.
    // occt: Geom_Curve::IsClosed
    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    /// Total parametric length (sum of all segment lengths), set by [`build`](SpringCurve::build).
    // occt: total arc-length of composite Geom_TrimmedCurve
    pub fn total_length(&self) -> f64 {
        self.total_length
    }

    /// Number of segments in the composite curve.
    // occt: number of Geom_TrimmedCurve nodes
    pub fn nb_segments(&self) -> usize {
        self.segments.len()
    }

    /// Return a reference to segment `i` (0-based).
    ///
    /// # Panics
    /// Panics if `i >= nb_segments()`.
    // occt: access individual Geom_TrimmedCurve by index
    pub fn segment(&self, i: usize) -> &SpringCurveSegment {
        &self.segments[i]
    }

    /// Convert an arc-length offset from the start of the curve into a curve
    /// parameter.
    ///
    /// Walks the segments in order, accumulating parametric length, until the
    /// requested `arc_length` is consumed.  If `arc_length` exceeds
    /// `total_length`, the parameter of the last segment's end is returned.
    // occt: arc-length → parameter walk over Geom_TrimmedCurve chain
    pub fn parameter_at_length(&self, arc_length: f64) -> f64 {
        let mut remaining = arc_length;

        for seg in &self.segments {
            let seg_len = seg.length();
            if remaining <= seg_len {
                // Linear interpolation within this segment.
                return seg.u_start + remaining;
            }
            remaining -= seg_len;
        }

        // arc_length >= total_length: clamp to the end.
        match self.segments.last() {
            Some(last) => last.u_end,
            None => 0.0,
        }
    }
}

impl Default for SpringCurve {
    fn default() -> Self {
        Self::new()
    }
}
