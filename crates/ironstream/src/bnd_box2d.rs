//! `Bnd_Box2d` — an axis-aligned bounding box in 2D space, mirroring
//! OpenCascade's `Bnd_Box2d` class.
//!
//! A bounding box is parallel to the axes of the coordinate system. If finite
//! it is described by two intervals `[Xmin,Xmax]`, `[Ymin,Ymax]`. It may also
//! be *open* (infinite) in one or more of the four directions, *whole*
//! (infinite in all four), or *void* (empty).
//!
//! Each box additionally carries a `gap` which is included on both sides in any
//! direction when consulting the finite bounds.
//!
//! This is a faithful, dependency-free port of OCCT's `Bnd_Box2d.cxx`/`.hxx`,
//! built on the existing IronStream `gp2d` API. Semantics, bit flags and
//! numeric constants follow OCCT exactly; method names are snake_case for Rust
//! idiom.

use crate::gp2d::{Pnt2d, Trsf2d};

// OCCT uses `RealLast()` (the largest finite double) for the raw void bounds.
const REAL_LAST: f64 = f64::MAX;
// OCCT's `Bnd_Box2d.cxx` `THE_BND_PRECISION_INFINITE` — the value returned for
// open (infinite) directions. NB: this is *not* `Precision::Infinite()`.
const BND_INFINITE: f64 = 1.0e100;
// OCCT `RealEpsilon()` — relative spacing of doubles around 1.0.
const REAL_EPSILON: f64 = f64::EPSILON;

// Bit flags, identical to OCCT's `Bnd_Box2d::MaskFlags`.
const VOID_MASK: i32 = 0x01;
const XMIN_MASK: i32 = 0x02;
const XMAX_MASK: i32 = 0x04;
const YMIN_MASK: i32 = 0x08;
const YMAX_MASK: i32 = 0x10;
const WHOLE_MASK: i32 = 0x1e;

/// The four bounds of a 2D box as returned by [`BndBox2d::get_limits`]. Mirrors
/// OCCT's `Bnd_Box2d::Limits` struct (field order `Xmin, Xmax, Ymin, Ymax`).
/// The gap is included and open directions account for the infinite value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Limits {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
}

/// `Bnd_Box2d` — an axis-aligned bounding box in 2D space.
// occt: Bnd_Box2d
#[derive(Clone, Copy, Debug)]
pub struct BndBox2d {
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
    gap: f64,
    flags: i32,
}

impl Default for BndBox2d {
    fn default() -> Self {
        Self::new()
    }
}

impl BndBox2d {
    /// Creates an empty (Void) box with a null gap. Mirrors the default
    /// constructor `Bnd_Box2d()`.
    pub const fn new() -> Self {
        Self {
            xmin: REAL_LAST,
            xmax: -REAL_LAST,
            ymin: REAL_LAST,
            ymax: -REAL_LAST,
            gap: 0.0,
            flags: VOID_MASK,
        }
    }

    // ---- whole / void state --------------------------------------------------

    /// `SetWhole()` — makes this box cover all of 2D space.
    pub fn set_whole(&mut self) {
        self.flags = WHOLE_MASK;
    }

    /// `SetVoid()` — makes this box empty.
    pub fn set_void(&mut self) {
        self.xmin = REAL_LAST;
        self.xmax = -REAL_LAST;
        self.ymin = REAL_LAST;
        self.ymax = -REAL_LAST;
        self.gap = 0.0;
        self.flags = VOID_MASK;
    }

    // ---- Set -----------------------------------------------------------------

    /// `Set(P)` — voids then bounds the single point `P`.
    pub fn set_point(&mut self, p: Pnt2d) {
        self.flags = VOID_MASK;
        self.gap = 0.0;
        self.add_point(p);
    }

    /// `Set(P, D)` — voids then bounds the half-line from `P` in direction `D`.
    pub fn set_point_dir(&mut self, p: Pnt2d, d: Pnt2d) {
        self.flags = VOID_MASK;
        self.gap = 0.0;
        self.add_point_dir(p, d);
    }

    // ---- Update --------------------------------------------------------------

    /// `Update(aXmin, aYmin, aXmax, aYmax)` — enlarges the box so that it
    /// contains at least the given intervals. Open directions are preserved.
    pub fn update_bounds(&mut self, x: f64, y: f64, big_x: f64, big_y: f64) {
        if self.flags & VOID_MASK != 0 {
            self.xmin = x;
            self.ymin = y;
            self.xmax = big_x;
            self.ymax = big_y;
            self.flags &= !VOID_MASK;
        } else {
            if self.flags & XMIN_MASK == 0 {
                self.xmin = self.xmin.min(x);
            }
            if self.flags & XMAX_MASK == 0 {
                self.xmax = self.xmax.max(big_x);
            }
            if self.flags & YMIN_MASK == 0 {
                self.ymin = self.ymin.min(y);
            }
            if self.flags & YMAX_MASK == 0 {
                self.ymax = self.ymax.max(big_y);
            }
        }
    }

    /// `Update(X, Y)` — adds a point of given coordinates.
    pub fn update_point(&mut self, x: f64, y: f64) {
        if self.flags & VOID_MASK != 0 {
            self.xmin = x;
            self.ymin = y;
            self.xmax = x;
            self.ymax = y;
            self.flags &= !VOID_MASK;
        } else {
            if self.flags & XMIN_MASK == 0 {
                self.xmin = self.xmin.min(x);
            }
            if self.flags & XMAX_MASK == 0 {
                self.xmax = self.xmax.max(x);
            }
            if self.flags & YMIN_MASK == 0 {
                self.ymin = self.ymin.min(y);
            }
            if self.flags & YMAX_MASK == 0 {
                self.ymax = self.ymax.max(y);
            }
        }
    }

    // ---- gap -----------------------------------------------------------------

    /// `GetGap()` — the current gap.
    pub fn gap(&self) -> f64 {
        self.gap
    }

    /// `SetGap(Tol)` — sets the gap to `abs(Tol)`.
    pub fn set_gap(&mut self, tol: f64) {
        self.gap = tol.abs();
    }

    /// `Enlarge(Tol)` — grows the gap to `max(gap, abs(Tol))`.
    pub fn enlarge(&mut self, tol: f64) {
        self.gap = self.gap.max(tol.abs());
    }

    // ---- bound accessors -----------------------------------------------------

    /// `Get(...)` — returns the four bounds with gap included. Returns `None`
    /// (mirroring OCCT's `Standard_ConstructionError`) if the box is void.
    pub fn get(&self) -> Option<Limits> {
        if self.is_void() {
            return None;
        }
        Some(self.get_limits())
    }

    /// `Get()` (the `Limits` overload) — returns the four bounds. If the box is
    /// void this returns the raw internal values (as OCCT does).
    pub fn get_limits(&self) -> Limits {
        Limits {
            xmin: self.get_x_min(),
            xmax: self.get_x_max(),
            ymin: self.get_y_min(),
            ymax: self.get_y_max(),
        }
    }

    /// `GetXMin()`.
    pub fn get_x_min(&self) -> f64 {
        if self.is_open_xmin() {
            -BND_INFINITE
        } else {
            self.xmin - self.gap
        }
    }
    /// `GetXMax()`.
    pub fn get_x_max(&self) -> f64 {
        if self.is_open_xmax() {
            BND_INFINITE
        } else {
            self.xmax + self.gap
        }
    }
    /// `GetYMin()`.
    pub fn get_y_min(&self) -> f64 {
        if self.is_open_ymin() {
            -BND_INFINITE
        } else {
            self.ymin - self.gap
        }
    }
    /// `GetYMax()`.
    pub fn get_y_max(&self) -> f64 {
        if self.is_open_ymax() {
            BND_INFINITE
        } else {
            self.ymax + self.gap
        }
    }

    /// `Center()` — midpoint of the box (gap included). `None` if void.
    pub fn center(&self) -> Option<Pnt2d> {
        if self.is_void() {
            return None;
        }
        Some(Pnt2d::new(
            0.5 * (self.get_x_min() + self.get_x_max()),
            0.5 * (self.get_y_min() + self.get_y_max()),
        ))
    }

    // ---- open in a direction -------------------------------------------------

    /// `OpenXmin()`.
    pub fn open_xmin(&mut self) {
        self.flags |= XMIN_MASK;
    }
    /// `OpenXmax()`.
    pub fn open_xmax(&mut self) {
        self.flags |= XMAX_MASK;
    }
    /// `OpenYmin()`.
    pub fn open_ymin(&mut self) {
        self.flags |= YMIN_MASK;
    }
    /// `OpenYmax()`.
    pub fn open_ymax(&mut self) {
        self.flags |= YMAX_MASK;
    }

    /// `IsOpenXmin()`.
    pub fn is_open_xmin(&self) -> bool {
        (self.flags & XMIN_MASK) != 0
    }
    /// `IsOpenXmax()`.
    pub fn is_open_xmax(&self) -> bool {
        (self.flags & XMAX_MASK) != 0
    }
    /// `IsOpenYmin()`.
    pub fn is_open_ymin(&self) -> bool {
        (self.flags & YMIN_MASK) != 0
    }
    /// `IsOpenYmax()`.
    pub fn is_open_ymax(&self) -> bool {
        (self.flags & YMAX_MASK) != 0
    }

    /// `IsWhole()` — infinite in all four directions.
    pub fn is_whole(&self) -> bool {
        (self.flags & WHOLE_MASK) == WHOLE_MASK
    }
    /// `IsVoid()` — empty.
    pub fn is_void(&self) -> bool {
        (self.flags & VOID_MASK) != 0
    }

    // ---- transform -----------------------------------------------------------

    /// `Transformed(T)` — the box covering the transform of this box. Note that
    /// a non-trivial transform (e.g. a rotation) generally enlarges the box.
    pub fn transformed(&self, t: &Trsf2d) -> BndBox2d {
        if self.is_void() {
            return *self;
        }
        match trsf_form(t) {
            TrsfForm::Identity => *self,
            TrsfForm::Translation => {
                let dx = t.t[0];
                let dy = t.t[1];
                let mut nb = *self;
                if self.flags & XMIN_MASK == 0 {
                    nb.xmin += dx;
                }
                if self.flags & XMAX_MASK == 0 {
                    nb.xmax += dx;
                }
                if self.flags & YMIN_MASK == 0 {
                    nb.ymin += dy;
                }
                if self.flags & YMAX_MASK == 0 {
                    nb.ymax += dy;
                }
                nb
            }
            TrsfForm::Other => {
                // Mirror OCCT's general path: re-open transformed directions and
                // add the finite corners that remain bounded in both axes.
                let mut vertex = [true, true, true, true];
                let mut dirs: Vec<Pnt2d> = Vec::new();

                if self.flags & XMIN_MASK != 0 {
                    dirs.push(Pnt2d::new(-1.0, 0.0));
                    vertex[0] = false;
                    vertex[2] = false;
                }
                if self.flags & XMAX_MASK != 0 {
                    dirs.push(Pnt2d::new(1.0, 0.0));
                    vertex[1] = false;
                    vertex[3] = false;
                }
                if self.flags & YMIN_MASK != 0 {
                    dirs.push(Pnt2d::new(0.0, -1.0));
                    vertex[0] = false;
                    vertex[1] = false;
                }
                if self.flags & YMAX_MASK != 0 {
                    dirs.push(Pnt2d::new(0.0, 1.0));
                    vertex[2] = false;
                    vertex[3] = false;
                }

                let mut nb = BndBox2d::new();

                for d in dirs {
                    nb.add_dir(apply_vector(t, d));
                }

                let p = [
                    Pnt2d::new(self.xmin, self.ymin),
                    Pnt2d::new(self.xmax, self.ymin),
                    Pnt2d::new(self.xmin, self.ymax),
                    Pnt2d::new(self.xmax, self.ymax),
                ];
                if vertex[0] {
                    nb.add_point(t.apply(p[0]));
                }
                if vertex[1] {
                    nb.add_point(t.apply(p[1]));
                }
                if vertex[2] {
                    nb.add_point(t.apply(p[2]));
                }
                if vertex[3] {
                    nb.add_point(t.apply(p[3]));
                }
                nb.gap = self.gap;
                nb
            }
        }
    }

    // ---- Add -----------------------------------------------------------------

    /// `Add(Other)` — unions another box into this one.
    pub fn add(&mut self, other: &BndBox2d) {
        if self.is_whole() {
            // already covers all space
        } else if other.is_void() {
            // nothing to add
        } else if other.is_whole() {
            self.set_whole();
        } else if self.is_void() {
            *self = *other;
        } else {
            if !self.is_open_xmin() {
                if other.is_open_xmin() {
                    self.open_xmin();
                } else if self.xmin > other.xmin {
                    self.xmin = other.xmin;
                }
            }
            if !self.is_open_xmax() {
                if other.is_open_xmax() {
                    self.open_xmax();
                } else if self.xmax < other.xmax {
                    self.xmax = other.xmax;
                }
            }
            if !self.is_open_ymin() {
                if other.is_open_ymin() {
                    self.open_ymin();
                } else if self.ymin > other.ymin {
                    self.ymin = other.ymin;
                }
            }
            if !self.is_open_ymax() {
                if other.is_open_ymax() {
                    self.open_ymax();
                } else if self.ymax < other.ymax {
                    self.ymax = other.ymax;
                }
            }
            self.gap = self.gap.max(other.gap);
        }
    }

    /// `Add(P)` — extends the box to contain point `P`.
    pub fn add_point(&mut self, p: Pnt2d) {
        self.update_point(p.x, p.y);
    }

    /// `Add(P, D)` — adds the half-line from `P` in direction `D`.
    pub fn add_point_dir(&mut self, p: Pnt2d, d: Pnt2d) {
        self.add_point(p);
        self.add_dir(d);
    }

    /// `Add(D)` — opens the box along the half-line direction `D`.
    pub fn add_dir(&mut self, d: Pnt2d) {
        let dx = d.x;
        let dy = d.y;
        if dx < -REAL_EPSILON {
            self.open_xmin();
        } else if dx > REAL_EPSILON {
            self.open_xmax();
        }
        if dy < -REAL_EPSILON {
            self.open_ymin();
        } else if dy > REAL_EPSILON {
            self.open_ymax();
        }
    }

    // ---- IsOut ---------------------------------------------------------------

    /// `IsOut(P)` — true if point `P` is outside the box.
    pub fn is_out_point(&self, p: Pnt2d) -> bool {
        if self.is_whole() {
            return false;
        }
        if self.is_void() {
            return true;
        }
        let x = p.x;
        let y = p.y;
        if self.flags & XMIN_MASK == 0 && x < (self.xmin - self.gap) {
            return true;
        }
        if self.flags & XMAX_MASK == 0 && x > (self.xmax + self.gap) {
            return true;
        }
        if self.flags & YMIN_MASK == 0 && y < (self.ymin - self.gap) {
            return true;
        }
        if self.flags & YMAX_MASK == 0 && y > (self.ymax + self.gap) {
            return true;
        }
        false
    }

    /// `IsOut(L)` — true if the (infinite) line does not intersect the box.
    /// `loc` is a point on the line, `dir` its direction (need not be unit).
    pub fn is_out_line(&self, loc: Pnt2d, dir: Pnt2d) -> bool {
        if self.is_whole() {
            return false;
        }
        if self.is_void() {
            return true;
        }
        let lim = self.get_limits();
        let center = Pnt2d::new(
            (lim.xmin + lim.xmax) / 2.0,
            (lim.ymin + lim.ymax) / 2.0,
        );
        let heigh = Pnt2d::new(
            (lim.xmax - center.x).abs(),
            (lim.ymax - center.y).abs(),
        );

        // dir ^ (center - loc): 2D cross product (signed area).
        let prod0 = dir.cross(center - loc);
        let prod1 = dir.x * heigh.y;
        let prod2 = dir.y * heigh.x;
        prod0.abs() > (prod1.abs() + prod2.abs())
    }

    /// `IsOut(P0, P1)` — true if the segment `[P0,P1]` does not intersect the
    /// box.
    pub fn is_out_segment(&self, p0: Pnt2d, p1: Pnt2d) -> bool {
        if self.is_whole() {
            return false;
        }
        if self.is_void() {
            return true;
        }

        let lim = self.get_limits();
        let seg_delta = p1 - p0;

        let center = Pnt2d::new(
            (lim.xmin + lim.xmax) / 2.0,
            (lim.ymin + lim.ymax) / 2.0,
        );
        let heigh = Pnt2d::new(
            (lim.xmax - center.x).abs(),
            (lim.ymax - center.y).abs(),
        );

        let prod0 = seg_delta.cross(center - p0);
        let prod1 = seg_delta.x * heigh.y;
        let prod2 = seg_delta.y * heigh.x;

        let mut status = true;
        if prod0.abs() <= (prod1.abs() + prod2.abs()) {
            // Intersection with the line detected; check the segment as a
            // bounding box (separating-axis on the two world axes).
            let hseg = Pnt2d::new(0.5 * seg_delta.x, 0.5 * seg_delta.y);
            let hseg_abs = Pnt2d::new(hseg.x.abs(), hseg.y.abs());
            let mid = p0 + hseg - center;
            status = mid.x.abs() > (heigh.x + hseg_abs.x) || mid.y.abs() > (heigh.y + hseg_abs.y);
        }
        status
    }

    /// `IsOut(Other)` — true if the other box is disjoint from this one.
    pub fn is_out_box(&self, other: &BndBox2d) -> bool {
        // Fast path: both boxes finite & closed (all flags zero).
        if self.flags == 0 && other.flags == 0 {
            let delta = other.gap + self.gap;
            if self.xmin - other.xmax > delta {
                return true;
            }
            if other.xmin - self.xmax > delta {
                return true;
            }
            if self.ymin - other.ymax > delta {
                return true;
            }
            if other.ymin - self.ymax > delta {
                return true;
            }
            return false;
        }

        if self.is_void() || other.is_void() {
            return true;
        }
        if self.is_whole() || other.is_whole() {
            return false;
        }

        let ob = other.get_limits();
        let (o_xmin, o_xmax, o_ymin, o_ymax) = (ob.xmin, ob.xmax, ob.ymin, ob.ymax);

        if self.flags & XMIN_MASK == 0 && o_xmax < (self.xmin - self.gap) {
            return true;
        }
        if self.flags & XMAX_MASK == 0 && o_xmin > (self.xmax + self.gap) {
            return true;
        }
        if self.flags & YMIN_MASK == 0 && o_ymax < (self.ymin - self.gap) {
            return true;
        }
        if self.flags & YMAX_MASK == 0 && o_ymin > (self.ymax + self.gap) {
            return true;
        }
        false
    }

    /// `IsOut(Other, T)` — true if the transformed other box is disjoint.
    pub fn is_out_box_trsf(&self, other: &BndBox2d, t: &Trsf2d) -> bool {
        self.is_out_box(&other.transformed(t))
    }

    /// `IsOut(T1, Other, T2)` — both boxes transformed independently.
    pub fn is_out_box_trsf2(&self, t1: &Trsf2d, other: &BndBox2d, t2: &Trsf2d) -> bool {
        self.transformed(t1).is_out_box(&other.transformed(t2))
    }

    /// `Contains(P)` — true if the point is inside or on the boundary.
    pub fn contains(&self, p: Pnt2d) -> bool {
        !self.is_out_point(p)
    }

    /// `Intersects(Other)` — true if the other box overlaps or is inside this.
    pub fn intersects(&self, other: &BndBox2d) -> bool {
        !self.is_out_box(other)
    }

    // ---- distance ------------------------------------------------------------

    /// `Distance(Other)` — minimum distance between two boxes (0 if void).
    pub fn distance(&self, other: &BndBox2d) -> f64 {
        if self.is_void() || other.is_void() {
            return 0.0;
        }
        let a = self.get_limits();
        let b = other.get_limits();

        let dx = dist_axis(a.xmin, a.xmax, b.xmin, b.xmax);
        let dy = dist_axis(a.ymin, a.ymax, b.ymin, b.ymax);

        (dx + dy).sqrt()
    }

    // ---- extent --------------------------------------------------------------

    /// `SquareExtent()` — squared length of the diagonal (0 if void).
    pub fn square_extent(&self) -> f64 {
        if self.is_void() {
            return 0.0;
        }
        let dx = self.xmax - self.xmin + self.gap + self.gap;
        let dy = self.ymax - self.ymin + self.gap + self.gap;
        dx * dx + dy * dy
    }
}

// ---- free helpers (private) -------------------------------------------------

/// The transform "form" classification used by OCCT's `gp_Trsf2d::Form()`, as
/// far as `Transformed` needs to distinguish it.
enum TrsfForm {
    Identity,
    Translation,
    Other,
}

fn trsf_form(t: &Trsf2d) -> TrsfForm {
    let identity_linear = t.m[0][0] == 1.0
        && t.m[0][1] == 0.0
        && t.m[1][0] == 0.0
        && t.m[1][1] == 1.0;
    if !identity_linear {
        return TrsfForm::Other;
    }
    if t.t[0] == 0.0 && t.t[1] == 0.0 {
        TrsfForm::Identity
    } else {
        TrsfForm::Translation
    }
}

/// Applies only the linear (2x2) part of a transform to a vector, ignoring the
/// translation — mirroring `gp_Dir2d::Transform` followed by normalization, but
/// only the sign of each component matters for `Add(Dir2d)`.
fn apply_vector(t: &Trsf2d, v: Pnt2d) -> Pnt2d {
    Pnt2d::new(
        t.m[0][0] * v.x + t.m[0][1] * v.y,
        t.m[1][0] * v.x + t.m[1][1] * v.y,
    )
}

/// Squared distance between two 1D intervals; 0 if they overlap (OCCT
/// `distAxis` lambda inside `Bnd_Box2d::Distance`).
fn dist_axis(min1: f64, max1: f64, min2: f64, max2: f64) -> f64 {
    if min1 > max2 {
        let d = min1 - max2;
        return d * d;
    }
    if min2 > max1 {
        let d = min2 - max1;
        return d * d;
    }
    0.0
}
