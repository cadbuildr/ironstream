//! `Geom2d` — analytic 2D curves (pcurves), mirroring OpenCascade's
//! `Geom2d_Curve` hierarchy. These live in a surface's parametric space and are
//! attached to edges as pcurves, or used directly as sketch geometry.

use crate::bsplines::BSplineCurve;
use crate::gp2d::Pnt2d;
use std::f64::consts::PI;

/// An analytic or freeform 2D curve (`Geom2d_Curve`).
#[derive(Clone, Debug)]
// occt: Geom2d_Line, Geom2d_Circle, Geom2d_Ellipse, Geom2d_Curve
pub enum Curve2d {
    /// Line `origin + t*dir`.
    Line { origin: Pnt2d, dir: Pnt2d },
    /// Circle of `radius` centered at `center`.
    Circle { center: Pnt2d, radius: f64 },
    /// Ellipse axis-aligned to `(x_dir, perp)` with `major`/`minor` radii.
    Ellipse {
        center: Pnt2d,
        x_dir: Pnt2d,
        major: f64,
        minor: f64,
    },
    /// Freeform B-spline / NURBS pcurve.
    BSpline(BSplineCurve<Pnt2d>),
}

impl Curve2d {
    pub fn range(&self) -> (f64, f64) {
        match self {
            Curve2d::Line { .. } => (0.0, 1.0),
            Curve2d::Circle { .. } | Curve2d::Ellipse { .. } => (0.0, 2.0 * PI),
            Curve2d::BSpline(c) => (c.first_param(), c.last_param()),
        }
    }

    pub fn value(&self, t: f64) -> Pnt2d {
        match self {
            Curve2d::Line { origin, dir } => *origin + *dir * t,
            Curve2d::Circle { center, radius } => {
                *center + Pnt2d::new(radius * t.cos(), radius * t.sin())
            }
            Curve2d::Ellipse {
                center,
                x_dir,
                major,
                minor,
            } => {
                let y_dir = x_dir.perp();
                *center + *x_dir * (major * t.cos()) + y_dir * (minor * t.sin())
            }
            Curve2d::BSpline(c) => c.value(t),
        }
    }

    pub fn sample(&self, t0: f64, t1: f64, segments: usize) -> Vec<Pnt2d> {
        (0..=segments)
            .map(|i| self.value(t0 + (t1 - t0) * i as f64 / segments as f64))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle2d_quarter() {
        let c = Curve2d::Circle {
            center: Pnt2d::origin(),
            radius: 1.0,
        };
        let p = c.value(PI / 2.0);
        assert!(p.distance(Pnt2d::new(0.0, 1.0)) < 1e-12, "{p:?}");
    }
}
