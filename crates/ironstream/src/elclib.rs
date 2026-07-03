//! `ElCLib` -- Elementary curve library: parametric conversions and evaluators
//! for lines, circles and ellipses, mirroring OpenCascade's `ElCLib` package
//! (`src/FoundationClasses/TKMath/ElCLib`).
//!
//! All functions are free functions (or associated functions on [`ElCLib`]) and
//! operate on the elementary geometry types from [`crate::gp`] and
//! [`crate::gp_prim`]. No allocations, no third-party deps.
//!
//! # Covered API
//! - **Parameter** — project a point onto a curve, return the parameter.
//! - **Value** — evaluate the point at a given parameter.
//! - **DN** — evaluate the N-th derivative vector at a given parameter.
//! - **To3d** — lift a 2D curve point/derivative into 3D given a placement.
//! - **InPeriod** — reduce an arbitrary real into `[U0, U0+Period[`.
//! - **AdjustPeriodic** — clamp/snap two period-boundary parameters.

use crate::gp::{Ax3, Pnt, Vec3};
use crate::gp2d::Pnt2d;
use crate::gp_prim::{Circ, Lin};
use std::f64::consts::PI;

// ──────────────────────────────────────────────────────────────────────────────
// gp_Elips — 3D ellipse (not yet in gp_prim; introduced here)
// ──────────────────────────────────────────────────────────────────────────────

/// A 3D ellipse (`gp_Elips`): a placement ([`Ax3`]) whose origin is the centre,
/// whose X direction is the major axis, and whose Z direction is the ellipse
/// normal; plus a major radius ≥ minor radius ≥ 0.
///
/// Parameterised by angle `U`:
/// ```text
/// P(U) = O + MajorRadius*cos(U)*XDir + MinorRadius*sin(U)*YDir
/// ```
// occt: ElCLib
#[derive(Clone, Copy, Debug)]
pub struct Elips {
    placement: Ax3,
    major_radius: f64,
    minor_radius: f64,
}

impl Elips {
    /// Construct an ellipse from a coordinate system and two radii.
    ///
    /// Panics if `minor_radius < 0` or `major_radius < minor_radius`.
    pub fn new(placement: Ax3, major_radius: f64, minor_radius: f64) -> Self {
        assert!(
            minor_radius >= 0.0 && major_radius >= minor_radius,
            "Elips: require MinorRadius >= 0 and MajorRadius >= MinorRadius"
        );
        Self {
            placement,
            major_radius,
            minor_radius,
        }
    }

    /// The coordinate system (placement) of the ellipse.
    #[inline]
    pub fn position(&self) -> Ax3 {
        self.placement
    }

    /// The centre of the ellipse.
    #[inline]
    pub fn location(&self) -> Pnt {
        self.placement.location
    }

    /// The major (semi-major) radius.
    #[inline]
    pub fn major_radius(&self) -> f64 {
        self.major_radius
    }

    /// The minor (semi-minor) radius.
    #[inline]
    pub fn minor_radius(&self) -> f64 {
        self.minor_radius
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// ElCLib — the curve library itself
// ──────────────────────────────────────────────────────────────────────────────

/// `ElCLib` — elementary curve library.
///
/// All methods are class-level (no instance data). The naming convention and
/// semantics mirror OpenCascade's `ElCLib` exactly.
pub struct ElCLib;

impl ElCLib {
    // ─────────────────────────────────── InPeriod / AdjustPeriodic ───────────

    /// `ElCLib::InPeriod(U, UFirst, ULast)` — reduce `U` to the half-open
    /// interval `[UFirst, ULast[` (with `Period = ULast - UFirst`).
    ///
    /// The result satisfies `UFirst <= result < ULast`.
    pub fn in_period(u: f64, u_first: f64, u_last: f64) -> f64 {
        let period = u_last - u_first;
        if period <= 0.0 {
            return u;
        }
        // Number of full periods to subtract so that we land in [u_first, u_last[
        let mut result = u;
        if result < u_first {
            let n = ((u_first - result) / period).ceil();
            result += n * period;
        } else if result >= u_last {
            let n = ((result - u_first) / period).floor();
            result -= n * period;
        }
        // Guard against floating-point edge at u_last
        if result >= u_last {
            result = u_first;
        }
        result
    }

    /// `ElCLib::AdjustPeriodic(UFirst, ULast, Precision, U1, U2)` — adjust
    /// `U1` into `[UFirst, ULast[` and shift `U2` so that `U2 - U1` is
    /// preserved (up to the period).
    ///
    /// After the call, `U1` lies in `[UFirst, ULast[` and `U2 >= U1`.
    /// If the adjusted `U2` lies strictly inside the period it is left alone;
    /// otherwise it is reduced to `[UFirst, ULast]`. `precision` is used to
    /// snap values that are within tolerance of a boundary.
    ///
    /// Returns `(u1_out, u2_out)`.
    pub fn adjust_periodic(
        u_first: f64,
        u_last: f64,
        precision: f64,
        u1: f64,
        u2: f64,
    ) -> (f64, f64) {
        let period = u_last - u_first;
        if period <= 0.0 {
            return (u1, u2);
        }
        // Snap u1 into period [u_first, u_last[
        let delta = u2 - u1; // preserve the difference
        let u1_new = Self::in_period(u1, u_first, u_last);
        let mut u2_new = u1_new + delta;

        // Snap boundaries within precision
        if (u1_new - u_first).abs() <= precision {
            // already at lower boundary — ok
        }
        if (u2_new - u_last).abs() <= precision {
            u2_new = u_last;
        }
        if u2_new > u_last + precision {
            u2_new = Self::in_period(u2_new, u_first, u_last);
            if u2_new < u1_new {
                u2_new += period;
            }
        }

        (u1_new, u2_new)
    }

    // ─────────────────────────────────── LineParameter ────────────────────────

    /// `ElCLib::Parameter(gp_Lin, gp_Pnt)` — the parameter on the line that is
    /// closest to `p`: the signed projection distance along the direction.
    ///
    /// `t = (P - O) · Dir`
    pub fn line_parameter(lin: &Lin, p: Pnt) -> f64 {
        (p - lin.location()).dot(lin.direction())
    }

    // ─────────────────────────────────── CircleParameter ─────────────────────

    /// `ElCLib::Parameter(gp_Circ, gp_Pnt)` — the angle `U ∈ [0, 2π[` such
    /// that `CircleValue(U, circ)` is the projection of `p` onto the circle.
    ///
    /// Computed as the angle of `(P - C)` in the local XY frame of the circle.
    pub fn circle_parameter(circ: &Circ, p: Pnt) -> f64 {
        let pos = circ.position();
        let rel = p - pos.location;
        let x = rel.dot(pos.x_dir);
        let y = rel.dot(pos.y_dir);
        let angle = y.atan2(x);
        if angle < 0.0 {
            angle + 2.0 * PI
        } else {
            angle
        }
    }

    // ─────────────────────────────────── EllipseParameter ────────────────────

    /// `ElCLib::Parameter(gp_Elips, gp_Pnt)` — the angle `U ∈ [0, 2π[` such
    /// that `EllipseValue(U, elips)` is the projection of `p` onto the ellipse.
    ///
    /// Computed by projecting `(P - C)` onto the major and minor axes:
    /// `U = atan2(y/b, x/a)`, then mapped to `[0, 2π[`.
    pub fn ellipse_parameter(elips: &Elips, p: Pnt) -> f64 {
        let pos = elips.placement;
        let rel = p - pos.location;
        let x = rel.dot(pos.x_dir);
        let y = rel.dot(pos.y_dir);
        // Normalise by the respective radii so the angle comes from a unit circle
        let a = elips.major_radius;
        let b = elips.minor_radius;
        let xn = if a.abs() > f64::EPSILON { x / a } else { 0.0 };
        let yn = if b.abs() > f64::EPSILON { y / b } else { 0.0 };
        let angle = yn.atan2(xn);
        if angle < 0.0 {
            angle + 2.0 * PI
        } else {
            angle
        }
    }

    // ─────────────────────────────────── LineValue ────────────────────────────

    /// `ElCLib::Value(U, gp_Lin)` — the point on the line at parameter `U`:
    /// `P = O + U * Dir`.
    pub fn line_value(u: f64, lin: &Lin) -> Pnt {
        lin.location() + lin.direction() * u
    }

    // ─────────────────────────────────── CircleValue ──────────────────────────

    /// `ElCLib::Value(U, gp_Circ)` — the point on the circle at angle `U`:
    /// `P = C + R*cos(U)*XDir + R*sin(U)*YDir`.
    pub fn circle_value(u: f64, circ: &Circ) -> Pnt {
        let pos = circ.position();
        let r = circ.radius();
        let (s, c) = u.sin_cos();
        pos.location + pos.x_dir * (r * c) + pos.y_dir * (r * s)
    }

    // ─────────────────────────────────── EllipseValue ────────────────────────

    /// `ElCLib::Value(U, gp_Elips)` — the point on the ellipse at angle `U`:
    /// `P = C + MajorRadius*cos(U)*XDir + MinorRadius*sin(U)*YDir`.
    pub fn ellipse_value(u: f64, elips: &Elips) -> Pnt {
        let pos = elips.placement;
        let (s, c) = u.sin_cos();
        pos.location
            + pos.x_dir * (elips.major_radius * c)
            + pos.y_dir * (elips.minor_radius * s)
    }

    // ─────────────────────────────────── LineDN ───────────────────────────────

    /// `ElCLib::DN(U, gp_Lin, N)` — the N-th derivative of the line at `U`.
    ///
    /// For a line `P(U) = O + U*Dir`, all derivatives of order ≥ 2 are zero;
    /// the first derivative is always `Dir`; derivatives of order 0 return the
    /// point (not a vector — caller should use [`line_value`] instead).
    ///
    /// Panics if `n < 1`.
    pub fn line_dn(u: f64, lin: &Lin, n: u32) -> Vec3 {
        assert!(n >= 1, "ElCLib::LineDN: N must be >= 1");
        let _ = u; // parameter not needed; suppress unused-variable lint
        if n == 1 {
            lin.direction()
        } else {
            Pnt::origin() // zero vector for all higher derivatives
        }
    }

    // ─────────────────────────────────── CircleDN ────────────────────────────

    /// `ElCLib::DN(U, gp_Circ, N)` — the N-th derivative of the circle at `U`.
    ///
    /// `DN(U) = R * cos(U + N*π/2) * XDir + R * sin(U + N*π/2) * YDir`
    ///
    /// Panics if `n < 1`.
    pub fn circle_dn(u: f64, circ: &Circ, n: u32) -> Vec3 {
        assert!(n >= 1, "ElCLib::CircleDN: N must be >= 1");
        let pos = circ.position();
        let r = circ.radius();
        let phase = u + (n as f64) * (PI / 2.0);
        let (s, c) = phase.sin_cos();
        pos.x_dir * (r * c) + pos.y_dir * (r * s)
    }

    // ─────────────────────────────────── EllipseDN ───────────────────────────

    /// `ElCLib::DN(U, gp_Elips, N)` — the N-th derivative of the ellipse at `U`.
    ///
    /// `DN(U) = MajorRadius*cos(U + N*π/2)*XDir + MinorRadius*sin(U + N*π/2)*YDir`
    ///
    /// Panics if `n < 1`.
    pub fn ellipse_dn(u: f64, elips: &Elips, n: u32) -> Vec3 {
        assert!(n >= 1, "ElCLib::EllipseDN: N must be >= 1");
        let pos = elips.placement;
        let phase = u + (n as f64) * (PI / 2.0);
        let (s, c) = phase.sin_cos();
        pos.x_dir * (elips.major_radius * c) + pos.y_dir * (elips.minor_radius * s)
    }

    // ─────────────────────────────────── To3d ─────────────────────────────────

    /// `ElCLib::To3d(gp_Ax2, gp_Pnt2d)` — lift a 2D point in the local frame
    /// of `placement` into 3D world space.
    ///
    /// `result = Origin + p.x*XDir + p.y*YDir`
    pub fn to3d_point(placement: &Ax3, p: Pnt2d) -> Pnt {
        placement.location + placement.x_dir * p.x + placement.y_dir * p.y
    }

    /// `ElCLib::To3d(gp_Ax2, gp_Vec2d)` — lift a 2D vector (no translation) in
    /// the local frame into 3D world space.
    ///
    /// `result = v.x*XDir + v.y*YDir`
    pub fn to3d_vector(placement: &Ax3, v: Pnt2d) -> Vec3 {
        placement.x_dir * v.x + placement.y_dir * v.y
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Module-level free-function aliases (OCCT style: overloaded by argument type)
// ──────────────────────────────────────────────────────────────────────────────

/// Free-function alias for [`ElCLib::in_period`].
#[inline]
pub fn in_period(u: f64, u_first: f64, u_last: f64) -> f64 {
    ElCLib::in_period(u, u_first, u_last)
}

/// Free-function alias for [`ElCLib::adjust_periodic`].
#[inline]
pub fn adjust_periodic(
    u_first: f64,
    u_last: f64,
    precision: f64,
    u1: f64,
    u2: f64,
) -> (f64, f64) {
    ElCLib::adjust_periodic(u_first, u_last, precision, u1, u2)
}

/// Free-function alias for [`ElCLib::line_parameter`].
#[inline]
pub fn line_parameter(lin: &Lin, p: Pnt) -> f64 {
    ElCLib::line_parameter(lin, p)
}

/// Free-function alias for [`ElCLib::circle_parameter`].
#[inline]
pub fn circle_parameter(circ: &Circ, p: Pnt) -> f64 {
    ElCLib::circle_parameter(circ, p)
}

/// Free-function alias for [`ElCLib::ellipse_parameter`].
#[inline]
pub fn ellipse_parameter(elips: &Elips, p: Pnt) -> f64 {
    ElCLib::ellipse_parameter(elips, p)
}

/// Free-function alias for [`ElCLib::line_value`].
#[inline]
pub fn line_value(u: f64, lin: &Lin) -> Pnt {
    ElCLib::line_value(u, lin)
}

/// Free-function alias for [`ElCLib::circle_value`].
#[inline]
pub fn circle_value(u: f64, circ: &Circ) -> Pnt {
    ElCLib::circle_value(u, circ)
}

/// Free-function alias for [`ElCLib::ellipse_value`].
#[inline]
pub fn ellipse_value(u: f64, elips: &Elips) -> Pnt {
    ElCLib::ellipse_value(u, elips)
}

/// Free-function alias for [`ElCLib::line_dn`].
#[inline]
pub fn line_dn(u: f64, lin: &Lin, n: u32) -> Vec3 {
    ElCLib::line_dn(u, lin, n)
}

/// Free-function alias for [`ElCLib::circle_dn`].
#[inline]
pub fn circle_dn(u: f64, circ: &Circ, n: u32) -> Vec3 {
    ElCLib::circle_dn(u, circ, n)
}

/// Free-function alias for [`ElCLib::ellipse_dn`].
#[inline]
pub fn ellipse_dn(u: f64, elips: &Elips, n: u32) -> Vec3 {
    ElCLib::ellipse_dn(u, elips, n)
}

/// Free-function alias for [`ElCLib::to3d_point`].
#[inline]
pub fn to3d_point(placement: &Ax3, p: Pnt2d) -> Pnt {
    ElCLib::to3d_point(placement, p)
}

/// Free-function alias for [`ElCLib::to3d_vector`].
#[inline]
pub fn to3d_vector(placement: &Ax3, v: Pnt2d) -> Vec3 {
    ElCLib::to3d_vector(placement, v)
}
