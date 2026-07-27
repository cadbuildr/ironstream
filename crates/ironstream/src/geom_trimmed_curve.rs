//! `Geom_TrimmedCurve` — a trimmed (bounded) 3-D curve, mirroring OpenCascade's
//! `Geom_TrimmedCurve` (`src/ModelingData/TKG3d/Geom/Geom_TrimmedCurve.hxx`).
//!
//! A trimmed curve is a *restriction* of a [`BasisCurve`] to a finite parameter
//! interval `[U1, U2]`.  The basis curve is stored by value inside the
//! [`BasisCurve`] enum variant.  All geometric evaluation (`Value`, `D1`, …,
//! `DN`) simply delegates to the basis curve at the requested parameter, so the
//! implementation is exact and free of approximation.
//!
//! OCCT notes faithfully reproduced here:
//! - The trimmed curve is parameterised as `C(U)` for `U ∈ [U1, U2]`.
//! - The orientation of the trimmed curve is the same as the basis curve unless
//!   the trim parameters are reversed (i.e. `U1 > U2` in the `new()` call with
//!   `sense = true`).
//! - `IsClosed()` returns `true` when `Value(U1)` and `Value(U2)` are within
//!   `Precision::Confusion()` of each other.
//! - `IsPeriodic()` delegates to the basis curve.

use crate::geom_bezier::GeomBezierCurve;
use crate::geom_bspline_curve::GeomBSplineCurve;
use crate::geom_circle::GeomCircle;
use crate::geom_ellipse::GeomEllipse;
use crate::geom_hyperbola::GeomHyperbola;
use crate::geom_line::GeomLine;
use crate::geom_parabola::GeomParabola;
use crate::gp::{Ax3, Pnt, Trsf, Vec3};
use crate::precision::{CONFUSION, INFINITE};

// ─────────────────────────────── BasisCurve ──────────────────────────────────

/// A discriminated union of all 3-D basis-curve kinds that can appear inside a
/// `Geom_TrimmedCurve`.  Mirrors OCCT's `Handle(Geom_Curve)`.
// occt-ref: Geom_Curve // (handle — here by value)
#[derive(Clone, Debug)]
pub enum BasisCurve {
    Line(GeomLine),
    Circle(GeomCircle),
    Ellipse(GeomEllipse),
    Hyperbola(GeomHyperbola),
    Parabola(GeomParabola),
    Bezier(GeomBezierCurve),
    BSpline(GeomBSplineCurve),
}

impl BasisCurve {
    /// `FirstParameter()` — natural first parameter of the basis curve.
    pub fn first_parameter(&self) -> f64 {
        match self {
            BasisCurve::Line(c) => c.first_parameter(),
            BasisCurve::Circle(c) => c.first_parameter(),
            BasisCurve::Ellipse(c) => c.first_parameter(),
            // Hyperbola / Parabola are open: ]-∞, +∞[
            BasisCurve::Hyperbola(_) | BasisCurve::Parabola(_) => -INFINITE,
            BasisCurve::Bezier(c) => c.first_parameter(),
            BasisCurve::BSpline(c) => c.first_parameter(),
        }
    }

    /// `LastParameter()` — natural last parameter of the basis curve.
    pub fn last_parameter(&self) -> f64 {
        match self {
            BasisCurve::Line(c) => c.last_parameter(),
            BasisCurve::Circle(c) => c.last_parameter(),
            BasisCurve::Ellipse(c) => c.last_parameter(),
            // Hyperbola / Parabola are open: ]-∞, +∞[
            BasisCurve::Hyperbola(_) | BasisCurve::Parabola(_) => INFINITE,
            BasisCurve::Bezier(c) => c.last_parameter(),
            BasisCurve::BSpline(c) => c.last_parameter(),
        }
    }

    /// `IsPeriodic()`.
    pub fn is_periodic(&self) -> bool {
        match self {
            BasisCurve::Line(c) => c.is_periodic(),
            BasisCurve::Circle(c) => c.is_periodic(),
            BasisCurve::Ellipse(c) => c.is_periodic(),
            BasisCurve::Hyperbola(c) => c.is_periodic(),
            BasisCurve::Parabola(c) => c.is_periodic(),
            BasisCurve::Bezier(c) => c.is_periodic(),
            BasisCurve::BSpline(c) => c.is_periodic(),
        }
    }

    /// `Period()` — the period; only meaningful when `IsPeriodic()`.
    fn period(&self) -> f64 {
        match self {
            BasisCurve::Circle(c) => c.period(),
            BasisCurve::Ellipse(c) => c.period(),
            BasisCurve::BSpline(c) if c.is_periodic() => {
                c.last_parameter() - c.first_parameter()
            }
            _ => f64::INFINITY,
        }
    }

    /// `ReversedParameter(U)`.
    fn reversed_parameter(&self, u: f64) -> f64 {
        match self {
            BasisCurve::Line(c) => c.reversed_parameter(u),
            BasisCurve::Circle(c) => c.reversed_parameter(u),
            BasisCurve::Ellipse(c) => c.reversed_parameter(u),
            BasisCurve::Hyperbola(c) => c.reversed_parameter(u),
            BasisCurve::Parabola(c) => c.reversed_parameter(u),
            BasisCurve::Bezier(c) => c.reversed_parameter(u),
            BasisCurve::BSpline(c) => c.reversed_parameter(u),
        }
    }

    /// `D0(U)` — point on the basis curve.
    pub fn value(&self, u: f64) -> Pnt {
        match self {
            BasisCurve::Line(c) => c.d0(u),
            BasisCurve::Circle(c) => c.d0(u),
            BasisCurve::Ellipse(c) => c.d0(u),
            BasisCurve::Hyperbola(c) => c.value(u),
            BasisCurve::Parabola(c) => c.d0(u),
            BasisCurve::Bezier(c) => c.value(u),
            BasisCurve::BSpline(c) => c.value(u),
        }
    }

    /// `D1(U)` — point and first derivative.
    pub fn d1(&self, u: f64) -> (Pnt, Vec3) {
        match self {
            BasisCurve::Line(c) => c.d1(u),
            BasisCurve::Circle(c) => c.d1(u),
            BasisCurve::Ellipse(c) => c.d1(u),
            BasisCurve::Hyperbola(c) => c.d1(u),
            BasisCurve::Parabola(c) => c.d1(u),
            BasisCurve::Bezier(c) => c.d1(u),
            BasisCurve::BSpline(c) => c.d1(u),
        }
    }

    /// `D2(U)` — point and first two derivatives.
    pub fn d2(&self, u: f64) -> (Pnt, Vec3, Vec3) {
        match self {
            BasisCurve::Line(c) => c.d2(u),
            BasisCurve::Circle(c) => c.d2(u),
            BasisCurve::Ellipse(c) => c.d2(u),
            BasisCurve::Hyperbola(c) => c.d2(u),
            BasisCurve::Parabola(c) => c.d2(u),
            BasisCurve::Bezier(c) => c.d2(u),
            BasisCurve::BSpline(c) => c.d2(u),
        }
    }

    /// `D3(U)` — point and first three derivatives.
    pub fn d3(&self, u: f64) -> (Pnt, Vec3, Vec3, Vec3) {
        match self {
            BasisCurve::Line(c) => c.d3(u),
            BasisCurve::Circle(c) => c.d3(u),
            BasisCurve::Ellipse(c) => c.d3(u),
            BasisCurve::Hyperbola(c) => c.d3(u),
            BasisCurve::Parabola(c) => c.d3(u),
            BasisCurve::Bezier(c) => c.d3(u),
            BasisCurve::BSpline(c) => c.d3(u),
        }
    }

    /// `DN(U, N)` — the N-th derivative vector (`N >= 1`).
    pub fn dn(&self, u: f64, n: i32) -> Vec3 {
        match self {
            BasisCurve::Line(c) => c.dn(u, n),
            BasisCurve::Circle(c) => c.dn(u, n),
            BasisCurve::Ellipse(c) => c.dn(u, n),
            BasisCurve::Hyperbola(c) => c.dn(u, n),
            BasisCurve::Parabola(c) => c.dn(u, n),
            BasisCurve::Bezier(c) => c.dn(u, n),
            BasisCurve::BSpline(c) => c.dn(u, n),
        }
    }

    /// `Reverse()` — reverse the direction of the basis curve in place.
    ///
    /// For curves that supply their own `reverse()` (line, parabola, b-spline,
    /// Bezier) we call it directly. For conics (circle, ellipse, hyperbola) the
    /// OCCT `Geom_Conic::Reverse()` implementation negates the Y direction of the
    /// local frame (which in turn negates Z to keep the frame right-handed), thus
    /// reversing the parametric direction.
    pub fn reverse(&mut self) {
        match self {
            BasisCurve::Line(c) => c.reverse(),
            BasisCurve::Circle(c) => {
                // Geom_Conic::Reverse(): negate YDir, flip ZDir sign.
                let pos = c.position();
                let new_pos = reverse_conic_frame(pos);
                *c = GeomCircle::from_ax3(new_pos, c.radius());
            }
            BasisCurve::Ellipse(c) => {
                let pos = c.position();
                let new_pos = reverse_conic_frame(pos);
                *c = GeomEllipse::from_ax3(new_pos, c.major_radius(), c.minor_radius());
            }
            BasisCurve::Hyperbola(c) => {
                let pos = c.position();
                let new_pos = reverse_conic_frame(pos);
                let major = c.major_radius();
                let minor = c.minor_radius();
                *c = GeomHyperbola::new(new_pos, major, minor);
            }
            BasisCurve::Parabola(c) => c.reverse(),
            BasisCurve::Bezier(c) => c.reverse(),
            BasisCurve::BSpline(c) => c.reverse(),
        }
    }

    /// `Transform(T)` — apply transformation in place.
    pub fn transform(&mut self, t: &Trsf) {
        match self {
            BasisCurve::Line(c) => c.transform(t),
            BasisCurve::Circle(c) => c.transform(t),
            BasisCurve::Ellipse(c) => c.transform(t),
            BasisCurve::Hyperbola(c) => c.transform(t),
            BasisCurve::Parabola(c) => c.transform(t),
            BasisCurve::Bezier(c) => c.transform(t),
            BasisCurve::BSpline(c) => c.transform(t),
        }
    }
}

/// Reverse a conic local frame by negating the Y direction and updating Z.
///
/// OCCT `Geom_Conic::Reverse()` flips the YDir so that the curve runs in the
/// opposite angular direction; ZDir = XDir × YDir is updated accordingly.
fn reverse_conic_frame(pos: Ax3) -> Ax3 {
    let new_y = -pos.y_dir;
    let new_z = pos.x_dir.cross(new_y).normalized();
    Ax3 {
        location: pos.location,
        x_dir: pos.x_dir,
        y_dir: new_y,
        z_dir: new_z,
    }
}

// ─────────────────────────────── GeomTrimmedCurve ────────────────────────────

/// `Geom_TrimmedCurve` — a restriction of a 3-D `Geom_Curve` to the interval
/// `[U1, U2]`.
///
/// All evaluation delegates to the stored [`BasisCurve`]; the trim bounds are
/// preserved verbatim so callers get back exactly the parameters they supplied.
// occt: Geom_TrimmedCurve
#[derive(Clone, Debug)]
pub struct GeomTrimmedCurve {
    /// The basis curve.
    basis: BasisCurve,
    /// The first trim parameter (always ≤ `u2`).
    u1: f64,
    /// The last trim parameter (always ≥ `u1`).
    u2: f64,
}

impl GeomTrimmedCurve {
    // ─────────────────────────────── Construction ────────────────────────────

    /// `Geom_TrimmedCurve(C, U1, U2, Sense=true, theAdjustPeriodic=true)`.
    ///
    /// Constructs a trimmed curve that is the restriction of `basis` to the
    /// parameter interval `[min(U1,U2), max(U1,U2)]`.
    ///
    /// When the basis curve is periodic and `adjust_periodic` is `true`, the
    /// trim parameters are adjusted so the interval fits within one period.
    ///
    /// `sense` controls the orientation: when `sense = false` the basis curve
    /// is reversed and the trim parameters are mapped accordingly, so the curve
    /// runs from `U2` to `U1` on the original basis.
    ///
    /// # Panics
    /// - If `U1 == U2` (degenerate interval).
    /// - If the basis curve is not periodic and the interval exceeds its domain.
    pub fn new(basis: BasisCurve, u1: f64, u2: f64) -> Self {
        Self::with_sense(basis, u1, u2, true, true)
    }

    /// Full constructor mirroring `Geom_TrimmedCurve(C, U1, U2, Sense,
    /// theAdjustPeriodic)`.
    pub fn with_sense(
        mut basis: BasisCurve,
        u1: f64,
        u2: f64,
        sense: bool,
        adjust_periodic: bool,
    ) -> Self {
        assert!(
            (u2 - u1).abs() > 0.0,
            "Geom_TrimmedCurve: U1 and U2 must be different"
        );

        // Ordered pair.
        let (mut first, mut last) = if u1 <= u2 { (u1, u2) } else { (u2, u1) };

        if basis.is_periodic() && adjust_periodic {
            let period = basis.period();
            let natural_first = basis.first_parameter();
            // Bring `first` into [natural_first, natural_first + period[
            let shift = ((first - natural_first) / period).floor() * period;
            first -= shift;
            last -= shift;
            // Clamp to at most one period.
            if (last - first) > period {
                last = first + period;
            }
        } else if !basis.is_periodic() {
            let bf = basis.first_parameter();
            let bl = basis.last_parameter();
            assert!(
                first >= bf - CONFUSION && last <= bl + CONFUSION,
                "Geom_TrimmedCurve: trim interval [{first}, {last}] exceeds basis curve domain [{bf}, {bl}]"
            );
            first = first.max(bf);
            last = last.min(bl);
        }

        // When sense = false, reverse the basis curve so the trimmed curve runs
        // from `last` toward `first` on the original basis.  Map the trim params
        // through `reversed_parameter` before reversing the curve.
        let (u1_stored, u2_stored) = if !sense {
            let r1 = basis.reversed_parameter(last);
            let r2 = basis.reversed_parameter(first);
            basis.reverse();
            (r1.min(r2), r1.max(r2))
        } else {
            (first, last)
        };

        Self {
            basis,
            u1: u1_stored,
            u2: u2_stored,
        }
    }

    // ─────────────────────────────── Trim mutation ───────────────────────────

    /// `SetTrim(U1, U2, Sense=true, theAdjustPeriodic=true)` — change the trim
    /// interval of this curve without changing the basis curve.
    pub fn set_trim(&mut self, u1: f64, u2: f64, sense: bool, adjust_periodic: bool) {
        let new = Self::with_sense(self.basis.clone(), u1, u2, sense, adjust_periodic);
        self.u1 = new.u1;
        self.u2 = new.u2;
    }

    // ─────────────────────────────── Basis curve ─────────────────────────────

    /// `BasisCurve()` — the basis (un-trimmed) curve.
    pub fn basis_curve(&self) -> &BasisCurve {
        &self.basis
    }

    // ─────────────────────────────── Parameters ──────────────────────────────

    /// `FirstParameter()` — the start of the trim interval.
    pub fn first_parameter(&self) -> f64 {
        self.u1
    }

    /// `LastParameter()` — the end of the trim interval.
    pub fn last_parameter(&self) -> f64 {
        self.u2
    }

    // ─────────────────────────────── Topology ────────────────────────────────

    /// `IsClosed()` — `true` when `Value(U1)` ≈ `Value(U2)`.
    pub fn is_closed(&self) -> bool {
        let p1 = self.basis.value(self.u1);
        let p2 = self.basis.value(self.u2);
        p1.distance(p2) <= CONFUSION
    }

    /// `IsPeriodic()` — delegates to the basis curve.
    pub fn is_periodic(&self) -> bool {
        self.basis.is_periodic()
    }

    // ─────────────────────────────── Evaluation ──────────────────────────────

    /// `Value(U)` / `D0(U)` — point on the trimmed curve at parameter `U`.
    pub fn value(&self, u: f64) -> Pnt {
        self.basis.value(u)
    }

    /// `D0(U)` — alias for `value`.
    pub fn d0(&self, u: f64) -> Pnt {
        self.basis.value(u)
    }

    /// `D1(U)` — point and first derivative.
    pub fn d1(&self, u: f64) -> (Pnt, Vec3) {
        self.basis.d1(u)
    }

    /// `D2(U)` — point and first two derivatives.
    pub fn d2(&self, u: f64) -> (Pnt, Vec3, Vec3) {
        self.basis.d2(u)
    }

    /// `D3(U)` — point and first three derivatives.
    pub fn d3(&self, u: f64) -> (Pnt, Vec3, Vec3, Vec3) {
        self.basis.d3(u)
    }

    /// `DN(U, N)` — N-th derivative vector (`N >= 1`).
    pub fn dn(&self, u: f64, n: i32) -> Vec3 {
        self.basis.dn(u, n)
    }

    // ─────────────────────────────── Orientation ─────────────────────────────

    /// `Reverse()` — reverse the direction of parameterisation in place.
    ///
    /// The underlying basis curve is reversed and the trim parameters are
    /// updated via `ReversedParameter` so the same geometric interval is covered.
    pub fn reverse(&mut self) {
        let new_u1 = self.basis.reversed_parameter(self.u2);
        let new_u2 = self.basis.reversed_parameter(self.u1);
        self.basis.reverse();
        // Ensure u1 <= u2 after the flip.
        self.u1 = new_u1.min(new_u2);
        self.u2 = new_u1.max(new_u2);
    }

    /// `Reversed()` — returns a reversed copy.
    pub fn reversed(&self) -> GeomTrimmedCurve {
        let mut c = self.clone();
        c.reverse();
        c
    }

    /// `ReversedParameter(U)` — the parameter on the reversed curve corresponding
    /// to `U` on this curve.
    pub fn reversed_parameter(&self, u: f64) -> f64 {
        self.basis.reversed_parameter(u)
    }

    // ─────────────────────────────── Transform ───────────────────────────────

    /// `Transform(T)` — apply transformation `T` to this trimmed curve in place.
    ///
    /// The basis curve is transformed; for scaling transforms the trim parameters
    /// scale by the transform's scale factor (a line's parameter is arc-length
    /// for unit direction).
    pub fn transform(&mut self, t: &Trsf) {
        let s = t.scale_factor();
        self.basis.transform(t);
        self.u1 *= s;
        self.u2 *= s;
    }

    /// `Transformed(T)` — returns a transformed copy.
    pub fn transformed(&self, t: &Trsf) -> GeomTrimmedCurve {
        let mut c = self.clone();
        c.transform(t);
        c
    }
}
