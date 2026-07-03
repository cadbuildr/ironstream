//! `gp_Hypr2d` -- 2-D hyperbola primitive, mirroring OpenCascade's
//! `gp_Hypr2d` from the `TKMath` package.
//!
//! A 2-D hyperbola is determined by a local coordinate system (`gp_Ax22d`),
//! whose origin is the centre, whose "X Direction" points toward the first
//! vertex (along the real/major axis), and a major radius `a > 0` and minor
//! radius `b > 0`.  The two branches satisfy the implicit equation
//!
//! ```text
//! (x / a)^2 - (y / b)^2 = 1
//! ```
//!
//! in the local frame.
//!
//! The focal distance is `c = sqrt(a^2 + b^2)` and the eccentricity is
//! `e = c / a > 1`.
//!
//! Asymptotes pass through the centre with slopes `±(b/a)` relative to the
//! major axis.

use crate::geom2d_circle::Ax22d2;
use crate::gp2d::{Ax2d, Pnt2d, Trsf2d};

// occt: gp_Hypr2d

/// A 2-D hyperbola (`gp_Hypr2d`): a local coordinate system plus major and
/// minor radii.
///
/// The local coordinate system (`Ax22d2`) positions the centre at its origin
/// and the major ("X") axis along its X direction.  The frame is direct
/// (counter-clockwise) when `sense = true`.
#[derive(Clone, Copy, Debug)]
pub struct Hypr2d {
    position: Ax22d2,
    major_radius: f64,
    minor_radius: f64,
}

impl Hypr2d {
    /// Construct from a major axis (`gp_Ax2d`), major radius, minor radius and
    /// orientation sense.
    ///
    /// Panics (OCCT raises `Standard_ConstructionError`) if either radius is
    /// not strictly positive.
    pub fn new(major_axis: Ax2d, major_radius: f64, minor_radius: f64, sense: bool) -> Self {
        assert!(
            major_radius > 0.0 && minor_radius > 0.0,
            "gp_Hypr2d: MajorRadius and MinorRadius must be strictly positive"
        );
        Self {
            position: Ax22d2::from_x_axis(major_axis.location, major_axis.direction, sense),
            major_radius,
            minor_radius,
        }
    }

    /// Construct from a full local coordinate system (`gp_Ax22d`).
    ///
    /// Panics if either radius is not strictly positive.
    pub fn from_axis(position: Ax22d2, major_radius: f64, minor_radius: f64) -> Self {
        assert!(
            major_radius > 0.0 && minor_radius > 0.0,
            "gp_Hypr2d: MajorRadius and MinorRadius must be strictly positive"
        );
        Self {
            position,
            major_radius,
            minor_radius,
        }
    }

    // -----------------------------------------------------------------------
    // Accessors
    // -----------------------------------------------------------------------

    /// The major radius `a`.
    #[inline]
    pub fn major_radius(&self) -> f64 {
        self.major_radius
    }

    /// The minor radius `b`.
    #[inline]
    pub fn minor_radius(&self) -> f64 {
        self.minor_radius
    }

    /// The centre (origin of the local frame).
    #[inline]
    pub fn location(&self) -> Pnt2d {
        self.position.location()
    }

    /// The full local coordinate system.
    #[inline]
    pub fn position(&self) -> Ax22d2 {
        self.position
    }

    /// The "X Axis" of the local frame (major axis direction through the
    /// centre).
    #[inline]
    pub fn x_axis(&self) -> Ax2d {
        Ax2d::new(self.position.location(), self.position.x_direction())
    }

    /// The "Y Axis" of the local frame (minor axis direction through the
    /// centre).
    #[inline]
    pub fn y_axis(&self) -> Ax2d {
        Ax2d::new(self.position.location(), self.position.y_direction())
    }

    // -----------------------------------------------------------------------
    // Geometric properties
    // -----------------------------------------------------------------------

    /// Focal distance from the centre: `c = sqrt(a^2 + b^2)`.
    #[inline]
    pub fn focal(&self) -> f64 {
        (self.major_radius * self.major_radius + self.minor_radius * self.minor_radius).sqrt()
    }

    /// Eccentricity `e = c / a > 1`.
    #[inline]
    pub fn eccentricity(&self) -> f64 {
        self.focal() / self.major_radius
    }

    /// First focus: `Centre + c * XDirection`.
    pub fn focus1(&self) -> Pnt2d {
        let c = self.focal();
        self.position.location() + self.position.x_direction() * c
    }

    /// Second focus: `Centre - c * XDirection`.
    pub fn focus2(&self) -> Pnt2d {
        let c = self.focal();
        self.position.location() - self.position.x_direction() * c
    }

    /// Semi-latus rectum (also called the "parameter" in OCCT):
    /// `p = b^2 / a`.
    #[inline]
    pub fn parameter(&self) -> f64 {
        self.minor_radius * self.minor_radius / self.major_radius
    }

    /// First asymptote through the centre with direction
    /// `a * XDirection + b * YDirection` (normalised).
    ///
    /// In the local frame this is the line `y = (b/a) x`.
    pub fn asymptote1(&self) -> Ax2d {
        let dir = (self.position.x_direction() * self.major_radius
            + self.position.y_direction() * self.minor_radius)
            .normalized();
        Ax2d::new(self.position.location(), dir)
    }

    /// Second asymptote through the centre with direction
    /// `a * XDirection - b * YDirection` (normalised).
    ///
    /// In the local frame this is the line `y = -(b/a) x`.
    pub fn asymptote2(&self) -> Ax2d {
        let dir = (self.position.x_direction() * self.major_radius
            - self.position.y_direction() * self.minor_radius)
            .normalized();
        Ax2d::new(self.position.location(), dir)
    }

    // -----------------------------------------------------------------------
    // Conjugate / auxiliary / opposite hyperbola
    // -----------------------------------------------------------------------

    /// The conjugate hyperbola: major and minor radii are swapped, and the
    /// frame is rotated 90° so that the transverse axis becomes the conjugate
    /// axis.  In OCCT this is `gp_Hypr2d::ConjugateHypr2d()`.
    pub fn conjugate(&self) -> Hypr2d {
        // Swap X/Y directions: new X = old Y, new Y = -old X (or old X,
        // depending on sense).  We rebuild via from_axis with the Y direction
        // as the new X axis.
        let new_pos = Ax22d2::from_x_axis(
            self.position.location(),
            self.position.y_direction(),
            true,
        );
        Hypr2d {
            position: new_pos,
            major_radius: self.minor_radius,
            minor_radius: self.major_radius,
        }
    }

    /// The opposite branch (mirrored in the centre).  The position is the same
    /// but with the X direction reversed.
    pub fn opposite(&self) -> Hypr2d {
        let new_pos = Ax22d2::from_x_axis(
            self.position.location(),
            -self.position.x_direction(),
            self.is_direct(),
        );
        Hypr2d {
            position: new_pos,
            major_radius: self.major_radius,
            minor_radius: self.minor_radius,
        }
    }

    /// Whether the frame is direct (counter-clockwise / positive sense).
    ///
    /// A frame is direct when the cross product XDirection × YDirection > 0.
    #[inline]
    pub fn is_direct(&self) -> bool {
        self.position
            .x_direction()
            .cross(self.position.y_direction())
            > 0.0
    }

    // -----------------------------------------------------------------------
    // Transforms
    // -----------------------------------------------------------------------

    /// Translate by a vector `v`.
    pub fn translated(&self, v: Pnt2d) -> Hypr2d {
        let new_pos = Ax22d2::from_x_axis(
            self.position.location() + v,
            self.position.x_direction(),
            self.is_direct(),
        );
        Hypr2d {
            position: new_pos,
            major_radius: self.major_radius,
            minor_radius: self.minor_radius,
        }
    }

    /// Rotate about a point by `angle` radians (CCW for a direct frame).
    pub fn rotated(&self, centre: Pnt2d, angle: f64) -> Hypr2d {
        let t = Trsf2d::rotation(centre, angle);
        self.transformed(&t)
    }

    /// Mirror about a point (central symmetry).
    pub fn mirrored_point(&self, centre: Pnt2d) -> Hypr2d {
        let new_loc = Pnt2d::new(
            2.0 * centre.x - self.position.location().x,
            2.0 * centre.y - self.position.location().y,
        );
        let new_pos = Ax22d2::new(
            new_loc,
            -self.position.x_direction(),
            -self.position.y_direction(),
        );
        Hypr2d {
            position: new_pos,
            major_radius: self.major_radius,
            minor_radius: self.minor_radius,
        }
    }

    /// Mirror about a line given by an `Ax2d`.
    pub fn mirrored_axis(&self, axis: &Ax2d) -> Hypr2d {
        let t = mirror_trsf2d(axis);
        self.transformed(&t)
    }

    /// Apply an arbitrary 2-D affine transform.
    ///
    /// Radii are scaled by the absolute scale factor of `t`.
    pub fn transformed(&self, t: &Trsf2d) -> Hypr2d {
        let scale = trsf2d_scale_factor(t).abs();
        let new_loc = t.apply(self.position.location());
        let new_x = apply_vector_2d(t, self.position.x_direction());
        let new_y = apply_vector_2d(t, self.position.y_direction());
        let new_pos = Ax22d2::new(new_loc, new_x, new_y);
        Hypr2d {
            position: new_pos,
            major_radius: self.major_radius * scale,
            minor_radius: self.minor_radius * scale,
        }
    }
}

// -----------------------------------------------------------------------
// Internal helpers (mirrors of helpers in geom2d_circle / geom2d_ellipse)
// -----------------------------------------------------------------------

/// Apply the linear part of a 2-D transform to a vector (no translation).
#[inline]
fn apply_vector_2d(t: &Trsf2d, v: Pnt2d) -> Pnt2d {
    Pnt2d::new(
        t.m[0][0] * v.x + t.m[0][1] * v.y,
        t.m[1][0] * v.x + t.m[1][1] * v.y,
    )
}

/// Uniform scale factor of a 2-D transform: `sqrt(|det(linear)|)`.
#[inline]
fn trsf2d_scale_factor(t: &Trsf2d) -> f64 {
    let det = t.m[0][0] * t.m[1][1] - t.m[0][1] * t.m[1][0];
    det.abs().sqrt()
}

/// Build a 2-D reflection transform about the line through `axis.location`
/// in direction `axis.direction`.
///
/// For a unit direction `(dx, dy)` the reflection matrix is:
/// ```text
/// [ 2dx^2-1   2dx*dy ]
/// [ 2dx*dy    2dy^2-1 ]
/// ```
fn mirror_trsf2d(axis: &Ax2d) -> Trsf2d {
    let d = axis.direction.normalized();
    let (dx, dy) = (d.x, d.y);
    let m = [
        [2.0 * dx * dx - 1.0, 2.0 * dx * dy],
        [2.0 * dx * dy, 2.0 * dy * dy - 1.0],
    ];
    // The translation compensates for non-origin axis location:
    // t = p - M*p  where p = axis.location.
    let p = axis.location;
    let mx = m[0][0] * p.x + m[0][1] * p.y;
    let my = m[1][0] * p.x + m[1][1] * p.y;
    Trsf2d {
        m,
        t: [p.x - mx, p.y - my],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp2d::Pnt2d;

    fn default_hypr() -> Hypr2d {
        Hypr2d::new(
            Ax2d::new(Pnt2d::origin(), Pnt2d::new(1.0, 0.0)),
            5.0,
            3.0,
            true,
        )
    }

    #[test]
    fn radii() {
        let h = default_hypr();
        assert!((h.major_radius() - 5.0).abs() < 1e-12);
        assert!((h.minor_radius() - 3.0).abs() < 1e-12);
    }

    #[test]
    fn focal_and_eccentricity() {
        let h = default_hypr();
        let c = (25.0_f64 + 9.0).sqrt(); // sqrt(34)
        assert!((h.focal() - c).abs() < 1e-12);
        assert!((h.eccentricity() - c / 5.0).abs() < 1e-12);
    }

    #[test]
    fn foci_positions() {
        let h = default_hypr();
        let c = h.focal();
        assert!((h.focus1().x - c).abs() < 1e-12);
        assert!((h.focus2().x + c).abs() < 1e-12);
    }

    #[test]
    fn translate() {
        let h = default_hypr().translated(Pnt2d::new(2.0, 3.0));
        assert!((h.location().x - 2.0).abs() < 1e-12);
        assert!((h.location().y - 3.0).abs() < 1e-12);
        assert!((h.major_radius() - 5.0).abs() < 1e-12);
    }

    #[test]
    fn parameter() {
        let h = default_hypr();
        let expected = 9.0 / 5.0; // b^2/a
        assert!((h.parameter() - expected).abs() < 1e-12);
    }
}
