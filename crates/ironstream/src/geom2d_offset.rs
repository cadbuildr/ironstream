// FILE: rust/ironstream/crates/ironstream/src/geom2d_offset.rs

//! `Geom2d_OffsetCurve` — lightweight stub: offset of a 2D curve by a scalar.
//!
//! This module provides a self-contained, zero-dependency representation of the
//! key metadata that OCCT's `Geom2d_OffsetCurve` carries: the offset distance,
//! the degree and pole-count of the basis curve, the parameter range, and the
//! periodicity flag.
//!
//! Evaluation methods (`evaluate`, `d1`, `continuity`) are stub implementations
//! that capture the structural contract without requiring a concrete basis-curve
//! type.  They are replaced by accurate implementations once the basis curve is
//! wired in.
//!
//! This module is zero-dependency (only `std`).

// occt-ref: Geom2d_OffsetCurve // — offset of a 2D curve by a scalar
/// `Geom2d_OffsetCurve` — offset of a 2D curve by a signed scalar distance.
///
/// Stores the essential metadata of OCCT's `Geom2d_OffsetCurve`: the offset
/// value, the degree and number of poles of the underlying basis curve, the
/// parameter range `[first_param, last_param]`, and whether the curve is
/// periodic.
///
/// Evaluation methods are stubs: `evaluate(t)` returns `[t, offset]`
/// (offset in the y-direction), and `d1(t)` returns the value together with
/// the constant tangent `[1, 0]`.
// occt-ref: Geom2d_OffsetCurve
#[derive(Clone, Debug, PartialEq)]
pub struct Geom2dOffsetCurve {
    /// Signed offset distance from the basis curve.
    offset: f64,
    /// Degree of the basis curve polynomial.
    basis_curve_degree: u32,
    /// Number of poles (control points) of the basis curve.
    basis_curve_nb_poles: usize,
    /// Whether the offset curve is periodic.
    is_periodic: bool,
    /// First parameter of the parameter range.
    first_param: f64,
    /// Last parameter of the parameter range.
    last_param: f64,
}

impl Geom2dOffsetCurve {
    // ──────────────────────────────────── Constructor ─────────────────────────

    /// `Geom2d_OffsetCurve` — create a non-periodic offset curve.
    ///
    /// # Parameters
    /// - `offset`   — signed offset distance.
    /// - `degree`   — polynomial degree of the basis curve.
    /// - `nb_poles` — number of poles (control points) of the basis curve.
    /// - `first`    — first parameter value.
    /// - `last`     — last parameter value.
    pub fn new(offset: f64, degree: u32, nb_poles: usize, first: f64, last: f64) -> Self {
        Self {
            offset,
            basis_curve_degree: degree,
            basis_curve_nb_poles: nb_poles,
            is_periodic: false,
            first_param: first,
            last_param: last,
        }
    }

    // ──────────────────────────────────── Offset ──────────────────────────────

    /// `Offset()` — the signed offset distance.
    pub fn offset(&self) -> f64 {
        self.offset
    }

    /// `SetOffsetValue(d)` — update the signed offset distance.
    pub fn set_offset(&mut self, d: f64) {
        self.offset = d;
    }

    // ──────────────────────────────────── Basis curve metadata ────────────────

    /// `Degree()` — polynomial degree of the basis curve.
    pub fn degree(&self) -> u32 {
        self.basis_curve_degree
    }

    /// `NbPoles()` — number of poles of the basis curve.
    pub fn nb_poles(&self) -> usize {
        self.basis_curve_nb_poles
    }

    // ──────────────────────────────────── Parameter range ─────────────────────

    /// `FirstParameter()` — first parameter of the curve's range.
    pub fn first_param(&self) -> f64 {
        self.first_param
    }

    /// `LastParameter()` — last parameter of the curve's range.
    pub fn last_param(&self) -> f64 {
        self.last_param
    }

    /// `SetRange(first, last)` — update the parameter range.
    pub fn set_range(&mut self, first: f64, last: f64) {
        self.first_param = first;
        self.last_param = last;
    }

    // ──────────────────────────────────── Topology ────────────────────────────

    /// `IsPeriodic()` — whether the offset curve is periodic.
    pub fn is_periodic(&self) -> bool {
        self.is_periodic
    }

    // ──────────────────────────────────── Evaluation (stubs) ─────────────────

    /// `Value(t)` — stub evaluation: returns `[t, offset]`.
    ///
    /// The offset is applied in the y-direction; the x-coordinate tracks the
    /// parameter directly.  A real implementation would evaluate the basis
    /// curve and apply the true offset normal.
    pub fn evaluate(&self, t: f64) -> [f64; 2] {
        [t, self.offset]
    }

    /// `D1(t)` — stub: returns (value, tangent).
    ///
    /// - value   = `[t, offset]` (same as `evaluate`).
    /// - tangent = `[1.0, 0.0]` (constant horizontal unit tangent).
    ///
    /// A real implementation would compute the exact first derivative of the
    /// offset curve from the basis curve's first and second derivatives.
    pub fn d1(&self, t: f64) -> ([f64; 2], [f64; 2]) {
        let value = self.evaluate(t);
        let tangent = [1.0, 0.0];
        (value, tangent)
    }

    /// `Continuity()` — stub: returns `min(degree, 2)` as a `u8`.
    ///
    /// OCCT's `Geom2d_OffsetCurve::Continuity()` returns the continuity class
    /// of the offset curve, which is one order lower than the basis curve.
    /// This stub approximates that by capping the degree at 2.
    pub fn continuity(&self) -> u8 {
        self.basis_curve_degree.min(2) as u8
    }
}
