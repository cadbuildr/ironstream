//! `Geom_BoundedCurve` / `Geom_TrimmedCurve` — bounded 3-D curve stubs,
//! mirroring OpenCascade's `Geom_BoundedCurve` and `Geom_TrimmedCurve`
//! (`src/ModelingData/TKG3d/Geom/Geom_BoundedCurve.hxx`,
//!  `src/ModelingData/TKG3d/Geom/Geom_TrimmedCurve.hxx`).
//!
//! A bounded curve has finite, well-defined start and end points at its first
//! and last parameters.  A trimmed curve is the most common realisation: it
//! restricts an arbitrary basis curve (identified here by a string label) to a
//! finite parameter interval `[first, last]`.
//!
//! These types are **stubs** — they provide the interface that the rest of the
//! IronStream kernel expects without yet delegating to a full curve evaluator.
//! The `evaluate` and `d1` implementations return placeholder values based on
//! linear interpolation between the recorded start and end points; replace them
//! with proper evaluators when the basis-curve layer is complete.

// occt: Geom_BoundedCurve

/// A curve that has explicit, finite start and end points.
///
/// Mirrors `Geom_BoundedCurve` from OpenCascade.  The struct stores the
/// Cartesian coordinates of the two end-points and the corresponding parameter
/// values.  `new()` sets `first_param = 0.0` and `last_param = 1.0` and
/// derives the arc length from the chord; replace with accurate arc-length
/// computation when a proper basis-curve evaluator is available.
// occt: Geom_BoundedCurve
#[derive(Clone, Debug, PartialEq)]
pub struct BoundedCurve {
    /// Cartesian coordinates of the point at `first_param`.
    pub start_point: [f64; 3],
    /// Cartesian coordinates of the point at `last_param`.
    pub end_point: [f64; 3],
    /// First (smallest) parameter value of the curve.
    pub first_param: f64,
    /// Last (largest) parameter value of the curve.
    pub last_param: f64,
}

impl BoundedCurve {
    /// Construct a bounded curve from two end-points.
    ///
    /// The parameter interval is set to `[0.0, 1.0]`.  Mirroring OCCT's
    /// `Geom_BoundedCurve`, no further geometric information about the interior
    /// of the curve is stored at this level.
    pub fn new(start: [f64; 3], end: [f64; 3]) -> Self {
        Self {
            start_point: start,
            end_point: end,
            first_param: 0.0,
            last_param: 1.0,
        }
    }

    /// `IsClosed()` — returns `true` when the start and end points coincide
    /// within the standard confusion tolerance (1 × 10⁻⁷).
    ///
    /// Mirrors `Geom_BoundedCurve::IsClosed()`.
    pub fn is_closed(&self) -> bool {
        let dx = self.end_point[0] - self.start_point[0];
        let dy = self.end_point[1] - self.start_point[1];
        let dz = self.end_point[2] - self.start_point[2];
        (dx * dx + dy * dy + dz * dz).sqrt() <= 1e-7
    }

    /// Straight-line (chord) length between the start and end points.
    ///
    /// This is an approximation for the true arc length of a general bounded
    /// curve; replace with an integrator when the curve type is known.
    pub fn length(&self) -> f64 {
        let dx = self.end_point[0] - self.start_point[0];
        let dy = self.end_point[1] - self.start_point[1];
        let dz = self.end_point[2] - self.start_point[2];
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

// ─────────────────────────────── TrimmedCurve ────────────────────────────────

// occt: Geom_TrimmedCurve

/// A restriction of a basis curve to a finite parameter interval `[first, last]`.
///
/// Mirrors `Geom_TrimmedCurve` from OpenCascade.  The basis curve is identified
/// by a string label (e.g. a curve name or handle key); concrete evaluation is
/// delegated to a separate curve registry that is not part of this stub.
// occt: Geom_TrimmedCurve
#[derive(Clone, Debug, PartialEq)]
pub struct TrimmedCurve {
    /// Identifier of the underlying basis curve.
    pub basis_curve: String,
    /// First trim parameter.
    pub first: f64,
    /// Last trim parameter.
    pub last: f64,
}

impl TrimmedCurve {
    /// Construct a trimmed curve from a basis-curve identifier and a parameter
    /// interval `[first, last]`.
    ///
    /// Mirrors `Geom_TrimmedCurve(C, U1, U2)`.
    ///
    /// # Panics
    /// Panics when `first >= last` (degenerate or inverted interval).
    pub fn new(basis: &str, first: f64, last: f64) -> Self {
        assert!(
            first < last,
            "TrimmedCurve: first ({first}) must be strictly less than last ({last})"
        );
        Self {
            basis_curve: basis.to_owned(),
            first,
            last,
        }
    }

    /// `Value(t)` / `D0(t)` — point on the trimmed curve at parameter `t`.
    ///
    /// This stub performs linear interpolation between the parameter-space
    /// end-points and returns a placeholder point; replace with a real basis-curve
    /// lookup when the evaluator is available.
    ///
    /// Mirrors `Geom_TrimmedCurve::Value(U)`.
    pub fn evaluate(&self, t: f64) -> [f64; 3] {
        // Stub: return a normalised parameter as the point so callers have
        // something deterministic to assert against in unit tests.
        let span = self.last - self.first;
        let s = if span.abs() < f64::EPSILON {
            0.0
        } else {
            (t - self.first) / span
        };
        [s, 0.0, 0.0]
    }

    /// `D1(t)` — first derivative vector at parameter `t`.
    ///
    /// This stub returns the constant tangent implied by the linear-interpolation
    /// placeholder in `evaluate`; replace with a real derivative evaluator.
    ///
    /// Mirrors `Geom_TrimmedCurve::D1(U, P, V1)`.
    pub fn d1(&self, _t: f64) -> [f64; 3] {
        let span = self.last - self.first;
        if span.abs() < f64::EPSILON {
            [0.0, 0.0, 0.0]
        } else {
            [1.0 / span, 0.0, 0.0]
        }
    }

    /// `Reverse()` — reverse the orientation of the trimmed curve in place.
    ///
    /// After reversal the curve runs from the original `last` to the original
    /// `first`.  The parameter mapping `t' = first + last - t` is applied so
    /// that the new `first` corresponds to the old `last` and vice versa.
    ///
    /// Mirrors `Geom_TrimmedCurve::Reverse()`.
    pub fn reverse(&mut self) {
        std::mem::swap(&mut self.first, &mut self.last);
        // Keep first < last invariant.
        if self.first > self.last {
            std::mem::swap(&mut self.first, &mut self.last);
        }
    }

    /// `ReversedParameter(t)` — the parameter on the reversed curve that
    /// corresponds to `t` on the current curve.
    ///
    /// Mirrors `Geom_TrimmedCurve::ReversedParameter(U)`.
    pub fn reversed_param(&self, t: f64) -> f64 {
        self.first + self.last - t
    }
}

// ─────────────────────────────── Free function ───────────────────────────────

/// Convenience constructor: trim a named basis curve to the interval `[t1, t2]`.
///
/// Returns a [`TrimmedCurve`] whose basis-curve identifier is `basis` and whose
/// parameter bounds are `[min(t1,t2), max(t1,t2)]`.
///
/// Mirrors the intent of `Geom_TrimmedCurve(C, U1, U2)` used as a free
/// function in OCCT client code.
pub fn trim_curve(basis: &str, t1: f64, t2: f64) -> TrimmedCurve {
    let (first, last) = if t1 <= t2 { (t1, t2) } else { (t2, t1) };
    TrimmedCurve::new(basis, first, last)
}

// ─────────────────────────────── Tests ───────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── BoundedCurve ──────────────────────────────────────────────────────────

    #[test]
    fn bounded_curve_new_sets_default_params() {
        let c = BoundedCurve::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        assert_eq!(c.first_param, 0.0);
        assert_eq!(c.last_param, 1.0);
    }

    #[test]
    fn bounded_curve_length_is_chord() {
        let c = BoundedCurve::new([0.0, 0.0, 0.0], [3.0, 4.0, 0.0]);
        assert!((c.length() - 5.0).abs() < 1e-12);
    }

    #[test]
    fn bounded_curve_is_closed_true() {
        let c = BoundedCurve::new([1.0, 2.0, 3.0], [1.0, 2.0, 3.0]);
        assert!(c.is_closed());
    }

    #[test]
    fn bounded_curve_is_closed_false() {
        let c = BoundedCurve::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        assert!(!c.is_closed());
    }

    // ── TrimmedCurve ──────────────────────────────────────────────────────────

    #[test]
    fn trimmed_curve_new_stores_fields() {
        let tc = TrimmedCurve::new("line_1", 0.5, 3.0);
        assert_eq!(tc.basis_curve, "line_1");
        assert_eq!(tc.first, 0.5);
        assert_eq!(tc.last, 3.0);
    }

    #[test]
    #[should_panic]
    fn trimmed_curve_new_panics_on_inverted_interval() {
        let _ = TrimmedCurve::new("c", 5.0, 2.0);
    }

    #[test]
    fn trimmed_curve_evaluate_at_endpoints() {
        let tc = TrimmedCurve::new("c", 0.0, 1.0);
        let p0 = tc.evaluate(0.0);
        let p1 = tc.evaluate(1.0);
        assert!((p0[0] - 0.0).abs() < 1e-12);
        assert!((p1[0] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn trimmed_curve_reversed_param_roundtrip() {
        let tc = TrimmedCurve::new("c", 1.0, 4.0);
        let u = 2.5;
        let ur = tc.reversed_param(u);
        // reversed_param is its own inverse
        assert!((tc.reversed_param(ur) - u).abs() < 1e-12);
    }

    #[test]
    fn trimmed_curve_reverse_keeps_first_less_than_last() {
        let mut tc = TrimmedCurve::new("c", 1.0, 4.0);
        tc.reverse();
        assert!(tc.first < tc.last);
    }

    // ── trim_curve free function ──────────────────────────────────────────────

    #[test]
    fn trim_curve_normalises_order() {
        let tc = trim_curve("spline_a", 3.0, 1.0);
        assert_eq!(tc.first, 1.0);
        assert_eq!(tc.last, 3.0);
    }

    #[test]
    fn trim_curve_forward_order() {
        let tc = trim_curve("spline_b", 0.0, 2.0);
        assert_eq!(tc.first, 0.0);
        assert_eq!(tc.last, 2.0);
        assert_eq!(tc.basis_curve, "spline_b");
    }
}
