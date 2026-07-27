//! `gp` analytic primitives — `gp_Lin`, `gp_Circ`, `gp_Pln`, mirroring
//! OpenCascade's elementary geometry classes. Semantics and method names follow
//! OCCT (snake_case for Rust idiom).

use crate::gp::{Ax1, Ax3, Pnt, Trsf};
use std::f64::consts::PI;

/// `gp_Lin` — an infinite line: a location plus a unit direction.
// occt-ref: gp_Lin
#[derive(Clone, Copy, Debug)]
pub struct Lin {
    loc: Pnt,
    dir: Pnt,
}

impl Lin {
    /// From a `gp_Ax1`.
    pub fn new(axis: Ax1) -> Self {
        Self {
            loc: axis.location,
            dir: axis.direction.normalized(),
        }
    }

    /// From a point and a direction.
    pub fn new_pd(p: Pnt, d: Pnt) -> Self {
        Self {
            loc: p,
            dir: d.normalized(),
        }
    }

    pub fn location(&self) -> Pnt {
        self.loc
    }
    pub fn direction(&self) -> Pnt {
        self.dir
    }

    /// `Distance(Pnt)` — perpendicular distance from a point.
    pub fn distance_point(&self, p: Pnt) -> f64 {
        let rel = p - self.loc;
        (rel - self.dir * rel.dot(self.dir)).norm()
    }

    /// `Distance(Lin)` — shortest distance between two lines.
    pub fn distance_line(&self, o: &Lin) -> f64 {
        let cross = self.dir.cross(o.dir);
        let n = cross.norm();
        if n < 1e-12 {
            self.distance_point(o.loc) // parallel
        } else {
            (o.loc - self.loc).dot(cross).abs() / n
        }
    }

    /// `Contains(Pnt, tol)`.
    pub fn contains(&self, p: Pnt, tol: f64) -> bool {
        self.distance_point(p) <= tol
    }

    /// `Angle(Lin)` — angle between line directions, in `[0, π]`.
    pub fn angle(&self, o: &Lin) -> f64 {
        self.dir.angle(o.dir)
    }

    pub fn translated(&self, v: Pnt) -> Lin {
        Lin {
            loc: self.loc + v,
            dir: self.dir,
        }
    }

    pub fn rotated(&self, axis: Ax1, angle: f64) -> Lin {
        let t = Trsf::rotation(axis, angle);
        Lin {
            loc: t.apply_point(self.loc),
            dir: t.apply_vector(self.dir).normalized(),
        }
    }

    pub fn mirrored_axis(&self, axis: Ax1) -> Lin {
        let a = axis.direction.normalized();
        Lin {
            loc: self.loc.mirrored_axis(axis),
            dir: (a * (2.0 * self.dir.dot(a)) - self.dir).normalized(),
        }
    }

    pub fn transformed(&self, t: &Trsf) -> Lin {
        Lin {
            loc: t.apply_point(self.loc),
            dir: t.apply_vector(self.dir).normalized(),
        }
    }
}

/// `gp_Circ` — a circle: a placement (`gp_Ax2`) plus a radius.
// occt-ref: gp_Circ
#[derive(Clone, Copy, Debug)]
pub struct Circ {
    placement: Ax3,
    radius: f64,
}

impl Circ {
    pub fn new(placement: Ax3, radius: f64) -> Self {
        Self { placement, radius }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
    pub fn location(&self) -> Pnt {
        self.placement.location
    }
    pub fn axis(&self) -> Pnt {
        self.placement.z_dir
    }
    /// `Position()` — the full placement (`gp_Ax2`) of the circle.
    pub fn position(&self) -> Ax3 {
        self.placement
    }

    /// `Area()` = π r².
    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    /// `Length()` = 2π r.
    pub fn length(&self) -> f64 {
        2.0 * PI * self.radius
    }

    pub fn translated(&self, v: Pnt) -> Circ {
        Circ {
            placement: self.placement.transformed(&Trsf::translation(v)),
            radius: self.radius,
        }
    }

    pub fn rotated(&self, axis: Ax1, angle: f64) -> Circ {
        Circ {
            placement: self.placement.transformed(&Trsf::rotation(axis, angle)),
            radius: self.radius,
        }
    }

    pub fn mirrored_point(&self, center: Pnt) -> Circ {
        let mut p = self.placement;
        p.location = p.location.mirrored_point(center);
        Circ {
            placement: p,
            radius: self.radius,
        }
    }

    pub fn transformed(&self, t: &Trsf) -> Circ {
        Circ {
            placement: self.placement.transformed(t),
            radius: self.radius * t.scale_factor(),
        }
    }
}

/// `gp_Pln` — an infinite plane defined by a placement (`gp_Ax3`).
// occt-ref: gp_Pln
#[derive(Clone, Copy, Debug)]
pub struct Pln {
    placement: Ax3,
}

impl Pln {
    pub fn new(placement: Ax3) -> Self {
        Self { placement }
    }

    /// From a point and a normal.
    pub fn new_pd(p: Pnt, normal: Pnt) -> Self {
        Self {
            placement: Ax3::from_origin_normal(p, normal, normal.any_perpendicular()),
        }
    }

    /// From the implicit-form coefficients `a x + b y + c z + d = 0`.
    pub fn new_coeffs(a: f64, b: f64, c: f64, d: f64) -> Self {
        let normal = Pnt::new(a, b, c);
        let n2 = normal.dot(normal);
        let loc = if n2 > 1e-18 {
            normal * (-d / n2)
        } else {
            Pnt::origin()
        };
        Self::new_pd(loc, normal)
    }

    pub fn location(&self) -> Pnt {
        self.placement.location
    }

    /// `Position()` — the full local coordinate system (`gp_Ax3`) of the plane.
    pub fn position(&self) -> Ax3 {
        self.placement
    }

    /// `Axis().Direction()` — the plane normal.
    pub fn axis_direction(&self) -> Pnt {
        self.placement.z_dir
    }

    /// `Coefficients(a, b, c, d)` of `a x + b y + c z + d = 0`.
    pub fn coefficients(&self) -> (f64, f64, f64, f64) {
        let n = self.placement.z_dir;
        let d = -n.dot(self.placement.location);
        (n.x, n.y, n.z, d)
    }

    /// `Distance(Pnt)` — signed-magnitude distance to a point.
    pub fn distance_point(&self, p: Pnt) -> f64 {
        (p - self.placement.location)
            .dot(self.placement.z_dir)
            .abs()
    }

    pub fn contains(&self, p: Pnt, tol: f64) -> bool {
        self.distance_point(p) <= tol
    }

    pub fn translated(&self, v: Pnt) -> Pln {
        Pln {
            placement: self.placement.transformed(&Trsf::translation(v)),
        }
    }

    pub fn rotated(&self, axis: Ax1, angle: f64) -> Pln {
        Pln {
            placement: self.placement.transformed(&Trsf::rotation(axis, angle)),
        }
    }

    pub fn scaled(&self, center: Pnt, factor: f64) -> Pln {
        let mut t = Trsf::identity();
        t.set_scale(center, factor);
        Pln {
            placement: self.placement.transformed(&t),
        }
    }

    pub fn transformed(&self, t: &Trsf) -> Pln {
        Pln {
            placement: self.placement.transformed(t),
        }
    }

    /// `Mirrored(Ax2)` — reflect across the XY plane of `mirror`.
    pub fn mirrored_plane(&self, mirror: Ax3) -> Pln {
        let mut p = self.placement;
        p.location = p.location.mirrored_plane(mirror);
        let n = mirror.z_dir.normalized();
        p.z_dir = p.z_dir - n * (2.0 * p.z_dir.dot(n));
        Pln { placement: p }
    }
}
