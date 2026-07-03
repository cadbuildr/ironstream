//! `Bnd_Box` — an axis-aligned bounding box in 3D space, mirroring
//! OpenCascade's `Bnd_Box` class.
//!
//! A bounding box is parallel to the axes of the coordinate system. If finite
//! it is described by three intervals `[Xmin,Xmax]`, `[Ymin,Ymax]`,
//! `[Zmin,Zmax]`. It may also be *open* (infinite) in one or more of the six
//! directions, *whole* (infinite in all six), or *void* (empty).
//!
//! Each box additionally carries a `gap` which is included on both sides in any
//! direction when consulting the finite bounds.
//!
//! This is a faithful, dependency-free port of OCCT's `Bnd_Box.cxx`/`.hxx`,
//! built on the existing IronStream `gp` API. Semantics, bit flags and numeric
//! constants follow OCCT exactly; method names are snake_case for Rust idiom.

use crate::gp::{Pnt, Trsf};
use crate::gp_prim::{Lin, Pln};

// OCCT uses `RealLast()` (the largest finite double) for the raw void bounds.
const REAL_LAST: f64 = f64::MAX;
// OCCT's `Bnd_Box.cxx` `THE_BND_PRECISION_INFINITE` — the value returned for
// open (infinite) directions. NB: this is *not* `Precision::Infinite()`.
const BND_INFINITE: f64 = 1.0e100;
// OCCT `RealEpsilon()` — relative spacing of doubles around 1.0.
const REAL_EPSILON: f64 = f64::EPSILON;
// OCCT `RealSmall()` — the smallest positive normalized double.
const REAL_SMALL: f64 = f64::MIN_POSITIVE;

// Bit flags, identical to OCCT's `Bnd_Box::MaskFlags`.
const VOID_MASK: i32 = 0x01;
const XMIN_MASK: i32 = 0x02;
const XMAX_MASK: i32 = 0x04;
const YMIN_MASK: i32 = 0x08;
const YMAX_MASK: i32 = 0x10;
const ZMIN_MASK: i32 = 0x20;
const ZMAX_MASK: i32 = 0x40;
const WHOLE_MASK: i32 = 0x7e;

#[inline]
fn square(v: f64) -> f64 {
    v * v
}

/// The six bounds of a box as returned by [`BndBox::get_limits`]. Mirrors
/// OCCT's `Bnd_Box::Limits` struct. The gap is included and open directions
/// account for the infinite value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Limits {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
    pub zmin: f64,
    pub zmax: f64,
}

/// `Bnd_Box` — an axis-aligned bounding box.
// occt: Bnd_Box
#[derive(Clone, Copy, Debug)]
pub struct BndBox {
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
    zmin: f64,
    zmax: f64,
    gap: f64,
    flags: i32,
}

impl Default for BndBox {
    fn default() -> Self {
        Self::new()
    }
}

impl BndBox {
    /// Creates an empty (Void) box with a null gap. Mirrors the default
    /// constructor `Bnd_Box()`.
    pub const fn new() -> Self {
        Self {
            xmin: REAL_LAST,
            xmax: -REAL_LAST,
            ymin: REAL_LAST,
            ymax: -REAL_LAST,
            zmin: REAL_LAST,
            zmax: -REAL_LAST,
            gap: 0.0,
            flags: VOID_MASK,
        }
    }

    /// Creates a finite box spanning from `min` to `max`. Mirrors the
    /// `Bnd_Box(theMin, theMax)` constructor: bounds are taken verbatim, gap is
    /// null, and the box is *not* void.
    pub fn from_corners(min: Pnt, max: Pnt) -> Self {
        Self {
            xmin: min.x,
            xmax: max.x,
            ymin: min.y,
            ymax: max.y,
            zmin: min.z,
            zmax: max.z,
            gap: 0.0,
            flags: 0,
        }
    }

    // ---- whole / void state --------------------------------------------------

    /// `SetWhole()` — makes this box cover all of 3D space.
    pub fn set_whole(&mut self) {
        self.flags = WHOLE_MASK;
    }

    /// `SetVoid()` — makes this box empty.
    pub fn set_void(&mut self) {
        self.xmin = REAL_LAST;
        self.xmax = -REAL_LAST;
        self.ymin = REAL_LAST;
        self.ymax = -REAL_LAST;
        self.zmin = REAL_LAST;
        self.zmax = -REAL_LAST;
        self.gap = 0.0;
        self.flags = VOID_MASK;
    }

    // ---- Set -----------------------------------------------------------------

    /// `Set(P)` — voids then bounds the single point `P`.
    pub fn set_point(&mut self, p: Pnt) {
        self.set_void();
        self.add_point(p);
    }

    /// `Set(P, D)` — voids then bounds the half-line from `P` in direction `D`.
    pub fn set_point_dir(&mut self, p: Pnt, d: Pnt) {
        self.set_void();
        self.add_point_dir(p, d);
    }

    // ---- Update --------------------------------------------------------------

    /// `Update(aXmin, aYmin, aZmin, aXmax, aYmax, aZmax)` — enlarges the box so
    /// that it contains at least the given intervals.
    pub fn update_bounds(&mut self, x: f64, y: f64, z: f64, big_x: f64, big_y: f64, big_z: f64) {
        if self.is_void() {
            self.xmin = x;
            self.ymin = y;
            self.zmin = z;
            self.xmax = big_x;
            self.ymax = big_y;
            self.zmax = big_z;
            self.flags &= !VOID_MASK;
        } else {
            self.xmin = self.xmin.min(x);
            self.xmax = self.xmax.max(big_x);
            self.ymin = self.ymin.min(y);
            self.ymax = self.ymax.max(big_y);
            self.zmin = self.zmin.min(z);
            self.zmax = self.zmax.max(big_z);
        }
    }

    /// `Update(X, Y, Z)` — adds a point of given coordinates.
    pub fn update_point(&mut self, x: f64, y: f64, z: f64) {
        if self.is_void() {
            self.xmin = x;
            self.ymin = y;
            self.zmin = z;
            self.xmax = x;
            self.ymax = y;
            self.zmax = z;
            self.flags &= !VOID_MASK;
        } else {
            self.xmin = self.xmin.min(x);
            self.xmax = self.xmax.max(x);
            self.ymin = self.ymin.min(y);
            self.ymax = self.ymax.max(y);
            self.zmin = self.zmin.min(z);
            self.zmax = self.zmax.max(z);
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

    /// `Get(...)` — returns the six bounds with gap included. Returns `None`
    /// (mirroring OCCT's `Standard_ConstructionError`) if the box is void.
    pub fn get(&self) -> Option<Limits> {
        if self.is_void() {
            return None;
        }
        Some(self.get_limits())
    }

    /// `Get()` (the `Limits` overload) — returns the six bounds. If the box is
    /// void this returns the raw internal values (as OCCT does).
    pub fn get_limits(&self) -> Limits {
        Limits {
            xmin: self.get_x_min(),
            xmax: self.get_x_max(),
            ymin: self.get_y_min(),
            ymax: self.get_y_max(),
            zmin: self.get_z_min(),
            zmax: self.get_z_max(),
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
    /// `GetZMin()`.
    pub fn get_z_min(&self) -> f64 {
        if self.is_open_zmin() {
            -BND_INFINITE
        } else {
            self.zmin - self.gap
        }
    }
    /// `GetZMax()`.
    pub fn get_z_max(&self) -> f64 {
        if self.is_open_zmax() {
            BND_INFINITE
        } else {
            self.zmax + self.gap
        }
    }

    /// `CornerMin()` — the lower corner (gap included). `None` if void.
    pub fn corner_min(&self) -> Option<Pnt> {
        if self.is_void() {
            return None;
        }
        Some(Pnt::new(
            if self.is_open_xmin() {
                -BND_INFINITE
            } else {
                self.xmin - self.gap
            },
            if self.is_open_ymin() {
                -BND_INFINITE
            } else {
                self.ymin - self.gap
            },
            if self.is_open_zmin() {
                -BND_INFINITE
            } else {
                self.zmin - self.gap
            },
        ))
    }

    /// `CornerMax()` — the upper corner (gap included). `None` if void.
    pub fn corner_max(&self) -> Option<Pnt> {
        if self.is_void() {
            return None;
        }
        Some(Pnt::new(
            if self.is_open_xmax() {
                BND_INFINITE
            } else {
                self.xmax + self.gap
            },
            if self.is_open_ymax() {
                BND_INFINITE
            } else {
                self.ymax + self.gap
            },
            if self.is_open_zmax() {
                BND_INFINITE
            } else {
                self.zmax + self.gap
            },
        ))
    }

    /// `Center()` — midpoint of the box (gap included). `None` if void.
    pub fn center(&self) -> Option<Pnt> {
        if self.is_void() {
            return None;
        }
        Some(Pnt::new(
            0.5 * (self.get_x_min() + self.get_x_max()),
            0.5 * (self.get_y_min() + self.get_y_max()),
            0.5 * (self.get_z_min() + self.get_z_max()),
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
    /// `OpenZmin()`.
    pub fn open_zmin(&mut self) {
        self.flags |= ZMIN_MASK;
    }
    /// `OpenZmax()`.
    pub fn open_zmax(&mut self) {
        self.flags |= ZMAX_MASK;
    }

    /// `IsOpen()` — true if at least one direction is open.
    pub fn is_open(&self) -> bool {
        (self.flags & WHOLE_MASK) != 0
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
    /// `IsOpenZmin()`.
    pub fn is_open_zmin(&self) -> bool {
        (self.flags & ZMIN_MASK) != 0
    }
    /// `IsOpenZmax()`.
    pub fn is_open_zmax(&self) -> bool {
        (self.flags & ZMAX_MASK) != 0
    }

    /// `IsWhole()` — infinite in all six directions.
    pub fn is_whole(&self) -> bool {
        (self.flags & WHOLE_MASK) == WHOLE_MASK
    }
    /// `IsVoid()` — empty.
    pub fn is_void(&self) -> bool {
        (self.flags & VOID_MASK) != 0
    }

    // ---- thinness ------------------------------------------------------------

    /// `IsXThin(tol)`.
    pub fn is_x_thin(&self, tol: f64) -> bool {
        if self.is_whole() {
            return false;
        }
        if self.is_void() {
            return true;
        }
        if self.is_open_xmin() || self.is_open_xmax() {
            return false;
        }
        self.xmax - self.xmin < tol
    }
    /// `IsYThin(tol)`.
    pub fn is_y_thin(&self, tol: f64) -> bool {
        if self.is_whole() {
            return false;
        }
        if self.is_void() {
            return true;
        }
        if self.is_open_ymin() || self.is_open_ymax() {
            return false;
        }
        self.ymax - self.ymin < tol
    }
    /// `IsZThin(tol)`.
    pub fn is_z_thin(&self, tol: f64) -> bool {
        if self.is_whole() {
            return false;
        }
        if self.is_void() {
            return true;
        }
        if self.is_open_zmin() || self.is_open_zmax() {
            return false;
        }
        self.zmax - self.zmin < tol
    }
    /// `IsThin(tol)` — thin in all three dimensions.
    pub fn is_thin(&self, tol: f64) -> bool {
        if self.is_whole() {
            return false;
        }
        if self.is_void() {
            return true;
        }
        if self.is_open_xmin() || self.is_open_xmax() || self.xmax - self.xmin >= tol {
            return false;
        }
        if self.is_open_ymin() || self.is_open_ymax() || self.ymax - self.ymin >= tol {
            return false;
        }
        if self.is_open_zmin() || self.is_open_zmax() || self.zmax - self.zmin >= tol {
            return false;
        }
        true
    }

    // ---- transform -----------------------------------------------------------

    /// `Transformed(T)` — the box covering the transform of this box. Note that
    /// a non-trivial transform (e.g. a rotation) generally enlarges the box.
    pub fn transformed(&self, t: &Trsf) -> BndBox {
        if self.is_void() {
            return BndBox::new();
        }
        match trsf_form(t) {
            TrsfForm::Identity => *self,
            TrsfForm::Translation => {
                if !self.has_finite_part() {
                    return *self;
                }
                let (dx, dy, dz) = (t.t[0], t.t[1], t.t[2]);
                let mut nb = *self;
                nb.xmin += dx;
                nb.xmax += dx;
                nb.ymin += dy;
                nb.ymax += dy;
                nb.zmin += dz;
                nb.zmax += dz;
                nb
            }
            TrsfForm::Other => {
                let mut nb = BndBox::new();
                if self.has_finite_part() {
                    let corners = [
                        Pnt::new(self.xmin, self.ymin, self.zmin),
                        Pnt::new(self.xmax, self.ymin, self.zmin),
                        Pnt::new(self.xmin, self.ymax, self.zmin),
                        Pnt::new(self.xmax, self.ymax, self.zmin),
                        Pnt::new(self.xmin, self.ymin, self.zmax),
                        Pnt::new(self.xmax, self.ymin, self.zmax),
                        Pnt::new(self.xmin, self.ymax, self.zmax),
                        Pnt::new(self.xmax, self.ymax, self.zmax),
                    ];
                    for c in corners {
                        nb.add_point(t.apply_point(c));
                    }
                }
                nb.gap = self.gap;
                if !self.is_open() {
                    return nb;
                }

                // Carry over open directions: transform each open unit
                // direction and re-open the box accordingly.
                if self.is_open_xmin() {
                    nb.add_dir(t.apply_vector(Pnt::new(-1.0, 0.0, 0.0)));
                }
                if self.is_open_xmax() {
                    nb.add_dir(t.apply_vector(Pnt::new(1.0, 0.0, 0.0)));
                }
                if self.is_open_ymin() {
                    nb.add_dir(t.apply_vector(Pnt::new(0.0, -1.0, 0.0)));
                }
                if self.is_open_ymax() {
                    nb.add_dir(t.apply_vector(Pnt::new(0.0, 1.0, 0.0)));
                }
                if self.is_open_zmin() {
                    nb.add_dir(t.apply_vector(Pnt::new(0.0, 0.0, -1.0)));
                }
                if self.is_open_zmax() {
                    nb.add_dir(t.apply_vector(Pnt::new(0.0, 0.0, 1.0)));
                }
                nb
            }
        }
    }

    // ---- Add -----------------------------------------------------------------

    /// `Add(Other)` — unions another box into this one.
    pub fn add(&mut self, other: &BndBox) {
        if other.is_void() {
            return;
        }
        if self.is_void() {
            *self = *other;
            return;
        }

        self.gap = self.gap.max(other.gap);

        if self.is_whole() {
            return;
        }
        if other.is_whole() {
            self.set_whole();
            return;
        }

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
        if !self.is_open_zmin() {
            if other.is_open_zmin() {
                self.open_zmin();
            } else if self.zmin > other.zmin {
                self.zmin = other.zmin;
            }
        }
        if !self.is_open_zmax() {
            if other.is_open_zmax() {
                self.open_zmax();
            } else if self.zmax < other.zmax {
                self.zmax = other.zmax;
            }
        }
    }

    /// `Add(P)` — extends the box to contain point `P`.
    pub fn add_point(&mut self, p: Pnt) {
        self.update_point(p.x, p.y, p.z);
    }

    /// `Add(P, D)` — adds the half-line from `P` in direction `D`.
    pub fn add_point_dir(&mut self, p: Pnt, d: Pnt) {
        self.add_point(p);
        self.add_dir(d);
    }

    /// `Add(D)` — opens the box along the half-line direction `D`.
    pub fn add_dir(&mut self, d: Pnt) {
        let (dx, dy, dz) = (d.x, d.y, d.z);
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
        if dz < -REAL_EPSILON {
            self.open_zmin();
        } else if dz > REAL_EPSILON {
            self.open_zmax();
        }
    }

    // ---- IsOut ---------------------------------------------------------------

    /// `IsOut(P)` — true if point `P` is outside the box.
    pub fn is_out_point(&self, p: Pnt) -> bool {
        if self.is_whole() {
            return false;
        }
        if self.is_void() {
            return true;
        }
        let (x, y, z) = (p.x, p.y, p.z);
        if !self.is_open_xmin() && (x < (self.xmin - self.gap)) {
            return true;
        }
        if !self.is_open_xmax() && (x > (self.xmax + self.gap)) {
            return true;
        }
        if !self.is_open_ymin() && (y < (self.ymin - self.gap)) {
            return true;
        }
        if !self.is_open_ymax() && (y > (self.ymax + self.gap)) {
            return true;
        }
        if !self.is_open_zmin() && (z < (self.zmin - self.gap)) {
            return true;
        }
        if !self.is_open_zmax() && (z > (self.zmax + self.gap)) {
            return true;
        }
        false
    }

    /// `IsOut(Pln)` — false if the plane intersects the box.
    pub fn is_out_plane(&self, pln: &Pln) -> bool {
        if self.is_whole() {
            return false;
        }
        if self.is_void() {
            return true;
        }
        let (a, b, c, d) = pln.coefficients();
        let lim = self.get_limits();
        let (a_xmin, a_xmax) = (lim.xmin, lim.xmax);
        let (a_ymin, a_ymax) = (lim.ymin, lim.ymax);
        let (a_zmin, a_zmax) = (lim.zmin, lim.zmax);

        let dd = a * a_xmin + b * a_ymin + c * a_zmin + d;
        let plus = dd > 0.0;
        if plus != ((a * a_xmin + b * a_ymin + c * a_zmax + d) > 0.0) {
            return false;
        }
        if plus != ((a * a_xmin + b * a_ymax + c * a_zmin + d) > 0.0) {
            return false;
        }
        if plus != ((a * a_xmin + b * a_ymax + c * a_zmax + d) > 0.0) {
            return false;
        }
        if plus != ((a * a_xmax + b * a_ymin + c * a_zmin + d) > 0.0) {
            return false;
        }
        if plus != ((a * a_xmax + b * a_ymin + c * a_zmax + d) > 0.0) {
            return false;
        }
        if plus != ((a * a_xmax + b * a_ymax + c * a_zmin + d) > 0.0) {
            return false;
        }
        plus == ((a * a_xmax + b * a_ymax + c * a_zmax + d) > 0.0)
    }

    /// `IsOut(Lin)` — false if the (infinite) line intersects the box.
    pub fn is_out_line(&self, l: &Lin) -> bool {
        if self.is_whole() {
            return false;
        }
        if self.is_void() {
            return true;
        }

        let lim = self.get_limits();
        let (my_xmin, my_ymin, my_zmin) = (lim.xmin, lim.ymin, lim.zmin);
        let (my_xmax, my_ymax, my_zmax) = (lim.xmax, lim.ymax, lim.zmax);

        let loc = l.location();
        let dir = l.direction();

        let (mut xmin, mut xmax) = (0.0_f64, 0.0_f64);
        let (mut ymin, mut ymax) = (0.0_f64, 0.0_f64);
        let zmin;
        let zmax;

        let mut parmin;
        let mut parmax;
        let mut par1;
        let mut par2;
        let x_to_set;
        let y_to_set;

        let a_dir_x = dir.x.abs();
        let a_dir_y = dir.y.abs();
        let a_dir_z = dir.z.abs();

        if a_dir_x > 0.0 {
            par1 = (my_xmin - loc.x) / dir.x;
            par2 = (my_xmax - loc.x) / dir.x;
            parmin = par1.min(par2);
            parmax = par1.max(par2);
            x_to_set = true;
        } else {
            if loc.x < my_xmin || my_xmax < loc.x {
                return true;
            }
            xmin = loc.x;
            xmax = loc.x;
            parmin = -BND_INFINITE;
            parmax = BND_INFINITE;
            x_to_set = false;
        }

        if a_dir_y > 0.0 {
            par1 = (my_ymin - loc.y) / dir.y;
            par2 = (my_ymax - loc.y) / dir.y;
            if parmax < par1.min(par2) || parmin > par1.max(par2) {
                return true;
            }
            parmin = parmin.max(par1.min(par2));
            parmax = parmax.min(par1.max(par2));
            y_to_set = true;
        } else {
            if loc.y < my_ymin || my_ymax < loc.y {
                return true;
            }
            ymin = loc.y;
            ymax = loc.y;
            y_to_set = false;
        }

        if a_dir_z > 0.0 {
            par1 = (my_zmin - loc.z) / dir.z;
            par2 = (my_zmax - loc.z) / dir.z;
            if parmax < par1.min(par2) || parmin > par1.max(par2) {
                return true;
            }
            parmin = parmin.max(par1.min(par2));
            parmax = parmax.min(par1.max(par2));
            par1 = loc.z + parmin * dir.z;
            par2 = loc.z + parmax * dir.z;
            zmin = par1.min(par2);
            zmax = par1.max(par2);
        } else {
            if loc.z < my_zmin || my_zmax < loc.z {
                return true;
            }
            zmin = loc.z;
            zmax = loc.z;
        }
        if zmax < my_zmin || my_zmax < zmin {
            return true;
        }

        if x_to_set {
            par1 = loc.x + parmin * dir.x;
            par2 = loc.x + parmax * dir.x;
            xmin = par1.min(par2);
            xmax = par1.max(par2);
        }
        if xmax < my_xmin || my_xmax < xmin {
            return true;
        }

        if y_to_set {
            par1 = loc.y + parmin * dir.y;
            par2 = loc.y + parmax * dir.y;
            ymin = par1.min(par2);
            ymax = par1.max(par2);
        }
        if ymax < my_ymin || my_ymax < ymin {
            return true;
        }

        false
    }

    /// `IsOut(Other)` — false if the other box intersects or is inside this one.
    pub fn is_out_box(&self, other: &BndBox) -> bool {
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
            if self.zmin - other.zmax > delta {
                return true;
            }
            if other.zmin - self.zmax > delta {
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

        let delta = other.gap + self.gap;

        if !self.is_open_xmin() && !other.is_open_xmax() && self.xmin - other.xmax > delta {
            return true;
        }
        if !self.is_open_xmax() && !other.is_open_xmin() && other.xmin - self.xmax > delta {
            return true;
        }
        if !self.is_open_ymin() && !other.is_open_ymax() && self.ymin - other.ymax > delta {
            return true;
        }
        if !self.is_open_ymax() && !other.is_open_ymin() && other.ymin - self.ymax > delta {
            return true;
        }
        if !self.is_open_zmin() && !other.is_open_zmax() && self.zmin - other.zmax > delta {
            return true;
        }
        if !self.is_open_zmax() && !other.is_open_zmin() && other.zmin - self.zmax > delta {
            return true;
        }

        false
    }

    /// `IsOut(Other, T)` — false if the transformed other box overlaps this one.
    pub fn is_out_box_trsf(&self, other: &BndBox, t: &Trsf) -> bool {
        self.is_out_box(&other.transformed(t))
    }

    /// `IsOut(T1, Other, T2)` — both boxes transformed independently.
    pub fn is_out_box_trsf2(&self, t1: &Trsf, other: &BndBox, t2: &Trsf) -> bool {
        self.transformed(t1).is_out_box(&other.transformed(t2))
    }

    /// `IsOut(P1, P2, D)` — false if the flat band between the two parallel
    /// lines through `P1` and `P2` with direction `D` intersects the box.
    pub fn is_out_band(&self, p1: Pnt, p2: Pnt, d: Pnt) -> bool {
        if self.is_whole() {
            return false;
        }
        if self.is_void() {
            return true;
        }

        let eps = REAL_SMALL;
        let lim = self.get_limits();
        let (my_xmin, my_ymin, my_zmin) = (lim.xmin, lim.ymin, lim.zmin);
        let (my_xmax, my_ymax, my_zmax) = (lim.xmax, lim.ymax, lim.zmax);

        if d.x.abs() < eps && d.y.abs() < eps {
            return is_segment_out(my_xmin, my_ymin, my_xmax, my_ymax, p1.x, p1.y, p2.x, p2.y);
        }

        if d.x.abs() < eps && d.z.abs() < eps {
            return is_segment_out(my_xmin, my_zmin, my_xmax, my_zmax, p1.x, p1.z, p2.x, p2.z);
        }

        if d.y.abs() < eps && d.z.abs() < eps {
            return is_segment_out(my_ymin, my_zmin, my_ymax, my_zmax, p1.y, p1.z, p2.y, p2.z);
        }

        if d.x.abs() < eps {
            if !is_segment_out(
                my_xmin,
                my_zmin,
                my_xmax,
                my_zmax,
                p1.x,
                (my_ymin - p1.y) * d.z / d.y + p1.z,
                p2.x,
                (my_ymin - p2.y) * d.z / d.y + p2.z,
            ) {
                return false;
            }
            if !is_segment_out(
                my_xmin,
                my_zmin,
                my_xmax,
                my_zmax,
                p1.x,
                (my_ymax - p1.y) * d.z / d.y + p1.z,
                p2.x,
                (my_ymax - p2.y) * d.z / d.y + p2.z,
            ) {
                return false;
            }
            if !is_segment_out(
                my_xmin,
                my_ymin,
                my_xmax,
                my_ymax,
                p1.x,
                (my_zmin - p1.z) * d.y / d.z + p1.y,
                p2.x,
                (my_zmin - p2.z) * d.y / d.z + p2.y,
            ) {
                return false;
            }
            if !is_segment_out(
                my_xmin,
                my_ymin,
                my_xmax,
                my_ymax,
                p1.x,
                (my_zmax - p1.z) * d.y / d.z + p1.y,
                p2.x,
                (my_zmax - p2.z) * d.y / d.z + p2.y,
            ) {
                return false;
            }
            return true;
        }

        if d.y.abs() < eps {
            if !is_segment_out(
                my_ymin,
                my_zmin,
                my_ymax,
                my_zmax,
                p1.y,
                (my_xmin - p1.x) * d.z / d.x + p1.z,
                p2.y,
                (my_xmin - p2.x) * d.z / d.x + p2.z,
            ) {
                return false;
            }
            if !is_segment_out(
                my_ymin,
                my_zmin,
                my_ymax,
                my_zmax,
                p1.y,
                (my_xmax - p1.x) * d.z / d.x + p1.z,
                p2.y,
                (my_xmax - p2.x) * d.z / d.x + p2.z,
            ) {
                return false;
            }
            if !is_segment_out(
                my_ymin,
                my_xmin,
                my_ymax,
                my_xmax,
                p1.y,
                (my_zmin - p1.z) * d.x / d.z + p1.x,
                p2.y,
                (my_zmin - p2.z) * d.x / d.z + p2.x,
            ) {
                return false;
            }
            if !is_segment_out(
                my_ymin,
                my_xmin,
                my_ymax,
                my_xmax,
                p1.y,
                (my_zmax - p1.z) * d.x / d.z + p1.x,
                p2.y,
                (my_zmax - p2.z) * d.x / d.z + p2.x,
            ) {
                return false;
            }
            return true;
        }

        if d.z.abs() < eps {
            if !is_segment_out(
                my_zmin,
                my_xmin,
                my_zmax,
                my_xmax,
                p1.z,
                (my_ymax - p1.y) * d.x / d.y + p1.x,
                p2.z,
                (my_ymax - p2.y) * d.x / d.y + p2.x,
            ) {
                return false;
            }
            if !is_segment_out(
                my_zmin,
                my_xmin,
                my_zmax,
                my_xmax,
                p1.z,
                (my_ymin - p1.y) * d.x / d.y + p1.x,
                p2.z,
                (my_ymin - p2.y) * d.x / d.y + p2.x,
            ) {
                return false;
            }
            if !is_segment_out(
                my_zmin,
                my_ymin,
                my_zmax,
                my_ymax,
                p1.z,
                (my_xmax - p1.x) * d.y / d.x + p1.y,
                p2.z,
                (my_xmax - p2.x) * d.y / d.x + p2.y,
            ) {
                return false;
            }
            if !is_segment_out(
                my_zmin,
                my_ymin,
                my_zmax,
                my_ymax,
                p1.z,
                (my_xmin - p1.x) * d.y / d.x + p1.y,
                p2.z,
                (my_xmin - p2.x) * d.y / d.x + p2.y,
            ) {
                return false;
            }
            return true;
        }

        if !is_segment_out(
            my_xmin,
            my_zmin,
            my_xmax,
            my_zmax,
            (my_ymin - p1.y) / d.y * d.x + p1.x,
            (my_ymin - p1.y) / d.y * d.z + p1.z,
            (my_ymin - p2.y) / d.y * d.x + p2.x,
            (my_ymin - p2.y) / d.y * d.z + p2.z,
        ) {
            return false;
        }
        if !is_segment_out(
            my_xmin,
            my_zmin,
            my_xmax,
            my_zmax,
            (my_ymax - p1.y) / d.y * d.x + p1.x,
            (my_ymax - p1.y) / d.y * d.z + p1.z,
            (my_ymax - p2.y) / d.y * d.x + p2.x,
            (my_ymax - p2.y) / d.y * d.z + p2.z,
        ) {
            return false;
        }
        if !is_segment_out(
            my_xmin,
            my_ymin,
            my_xmax,
            my_ymax,
            (my_zmin - p1.z) / d.z * d.x + p1.x,
            (my_zmin - p1.z) / d.z * d.y + p1.y,
            (my_zmin - p2.z) / d.z * d.x + p2.x,
            (my_zmin - p2.z) / d.z * d.y + p2.y,
        ) {
            return false;
        }
        if !is_segment_out(
            my_xmin,
            my_ymin,
            my_xmax,
            my_ymax,
            (my_zmax - p1.z) / d.z * d.x + p1.x,
            (my_zmax - p1.z) / d.z * d.y + p1.y,
            (my_zmax - p2.z) / d.z * d.x + p2.x,
            (my_zmax - p2.z) / d.z * d.y + p2.y,
        ) {
            return false;
        }
        if !is_segment_out(
            my_zmin,
            my_ymin,
            my_zmax,
            my_ymax,
            (my_xmin - p1.x) / d.x * d.z + p1.z,
            (my_xmin - p1.x) / d.x * d.y + p1.y,
            (my_xmin - p2.x) / d.x * d.z + p2.z,
            (my_xmin - p2.x) / d.x * d.y + p2.y,
        ) {
            return false;
        }
        if !is_segment_out(
            my_zmin,
            my_ymin,
            my_zmax,
            my_ymax,
            (my_xmax - p1.x) / d.x * d.z + p1.z,
            (my_xmax - p1.x) / d.x * d.y + p1.y,
            (my_xmax - p2.x) / d.x * d.z + p2.z,
            (my_xmax - p2.x) / d.x * d.y + p2.y,
        ) {
            return false;
        }

        true
    }

    /// `Contains(P)` — true if the point is inside or on the boundary.
    pub fn contains(&self, p: Pnt) -> bool {
        !self.is_out_point(p)
    }

    /// `Intersects(Other)` — true if the other box overlaps this one.
    pub fn intersects(&self, other: &BndBox) -> bool {
        !self.is_out_box(other)
    }

    // ---- distance ------------------------------------------------------------

    /// `Distance(Other)` — minimum distance between two boxes (0 if void).
    pub fn distance(&self, other: &BndBox) -> f64 {
        if self.is_void() || other.is_void() {
            return 0.0;
        }
        let a = self.get_limits();
        let b = other.get_limits();

        let dist_x = distance_in_dimension(a.xmin, a.xmax, b.xmin, b.xmax);
        let dist_y = distance_in_dimension(a.ymin, a.ymax, b.ymin, b.ymax);
        let dist_z = distance_in_dimension(a.zmin, a.zmax, b.zmin, b.zmax);

        (dist_x + dist_y + dist_z).sqrt()
    }

    // ---- extent / finite part ------------------------------------------------

    /// `SquareExtent()` — squared length of the diagonal (0 if void).
    pub fn square_extent(&self) -> f64 {
        if self.is_void() {
            return 0.0;
        }
        let dx = self.xmax - self.xmin + self.gap + self.gap;
        let dy = self.ymax - self.ymin + self.gap + self.gap;
        let dz = self.zmax - self.zmin + self.gap + self.gap;
        dx * dx + dy * dy + dz * dz
    }

    /// `FinitePart()` — the finite portion of a (possibly open) box.
    pub fn finite_part(&self) -> BndBox {
        if !self.has_finite_part() {
            return BndBox::new();
        }
        let mut nb = BndBox::new();
        nb.update_bounds(
            self.xmin, self.ymin, self.zmin, self.xmax, self.ymax, self.zmax,
        );
        nb.set_gap(self.gap);
        nb
    }

    /// `HasFinitePart()` — true if the box has a finite extent.
    pub fn has_finite_part(&self) -> bool {
        !self.is_void() && self.xmax >= self.xmin
    }

    // ---- JSON round-trip -----------------------------------------------------

    /// `DumpJson(...)` — serialize to OCCT's `Standard_Dump` JSON format.
    pub fn dump_json(&self) -> String {
        format!(
            "\"Bnd_Box\": {{\"CornerMin\": [{}, {}, {}], \"CornerMax\": [{}, {}, {}], \"Gap\": {}, \"Flags\": {}}}",
            fmt(self.xmin),
            fmt(self.ymin),
            fmt(self.zmin),
            fmt(self.xmax),
            fmt(self.ymax),
            fmt(self.zmax),
            fmt(self.gap),
            self.flags
        )
    }

    /// `InitFromJson(...)` — deserialize from the JSON produced by
    /// [`dump_json`](Self::dump_json). Returns `false` on a parse failure.
    pub fn init_from_json(&mut self, json: &str) -> bool {
        let cmin = match parse_vec3(json, "CornerMin") {
            Some(v) => v,
            None => return false,
        };
        let cmax = match parse_vec3(json, "CornerMax") {
            Some(v) => v,
            None => return false,
        };
        let gap = match parse_scalar(json, "Gap") {
            Some(v) => v,
            None => return false,
        };
        let flags = match parse_scalar(json, "Flags") {
            Some(v) => v as i32,
            None => return false,
        };
        self.xmin = cmin[0];
        self.ymin = cmin[1];
        self.zmin = cmin[2];
        self.xmax = cmax[0];
        self.ymax = cmax[1];
        self.zmax = cmax[2];
        self.gap = gap;
        self.flags = flags;
        true
    }
}

// ---- free helpers (private) -------------------------------------------------

/// The transform "form" classification used by OCCT's `gp_Trsf::Form()`, as far
/// as `Transformed` needs to distinguish it.
enum TrsfForm {
    Identity,
    Translation,
    Other,
}

fn trsf_form(t: &Trsf) -> TrsfForm {
    let identity_linear = t.m[0][0] == 1.0
        && t.m[0][1] == 0.0
        && t.m[0][2] == 0.0
        && t.m[1][0] == 0.0
        && t.m[1][1] == 1.0
        && t.m[1][2] == 0.0
        && t.m[2][0] == 0.0
        && t.m[2][1] == 0.0
        && t.m[2][2] == 1.0;
    if !identity_linear {
        return TrsfForm::Other;
    }
    if t.t[0] == 0.0 && t.t[1] == 0.0 && t.t[2] == 0.0 {
        TrsfForm::Identity
    } else {
        TrsfForm::Translation
    }
}

/// Minimum squared distance between two 1D intervals (OCCT `DistMini2Box`).
fn dist_mini_2box(r1min: f64, r1max: f64, r2min: f64, r2max: f64) -> f64 {
    let a = square(r1min - r2max);
    let b = square(r1max - r2min);
    a.min(b)
}

/// Squared distance in one dimension; 0 if the intervals overlap
/// (OCCT `DistanceInDimension`).
fn distance_in_dimension(min1: f64, max1: f64, min2: f64, max2: f64) -> f64 {
    if (min1 <= min2 && min2 <= max1) || (min2 <= min1 && min1 <= max2) {
        return 0.0;
    }
    dist_mini_2box(min1, max1, min2, max2)
}

/// Tests if a 2D segment is outside a 2D box (OCCT anonymous-namespace
/// `IsSegmentOut`).
#[allow(clippy::too_many_arguments)]
fn is_segment_out(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    xs1: f64,
    ys1: f64,
    xs2: f64,
    ys2: f64,
) -> bool {
    let eps = REAL_SMALL;
    let xs_min = xs1.min(xs2);
    let xs_max = xs1.max(xs2);
    let ys_min = ys1.min(ys2);
    let ys_max = ys1.max(ys2);

    if ys_max - ys_min < eps
        && (y1 - ys1 < eps && ys1 - y2 < eps)
        && ((xs_min - x1 < eps && x1 - xs_max < eps)
            || (xs_min - x2 < eps && x2 - xs_max < eps)
            || (x1 - xs1 < eps && xs1 - x2 < eps))
    {
        return false;
    }
    if xs_max - xs_min < eps
        && (x1 - xs1 < eps && xs1 - x2 < eps)
        && ((ys_min - y1 < eps && y1 - ys_max < eps)
            || (ys_min - y2 < eps && y2 - ys_max < eps)
            || (y1 - ys1 < eps && ys1 - y2 < eps))
    {
        return false;
    }

    if (xs1 < x1 && xs2 < x1)
        || (xs1 > x2 && xs2 > x2)
        || (ys1 < y1 && ys2 < y1)
        || (ys1 > y2 && ys2 > y2)
    {
        return true;
    }

    if (xs2 - xs1).abs() > eps {
        let ya = (x1.min(x2) - xs1) * (ys2 - ys1) / (xs2 - xs1) + ys1;
        let yb = (x1.max(x2) - xs1) * (ys2 - ys1) / (xs2 - xs1) + ys1;
        if (ya < y1 && yb < y1) || (ya > y2 && yb > y2) {
            return true;
        }
    } else if (ys2 - ys1).abs() > eps {
        let xa = (y1.min(y2) - ys1) * (xs2 - xs1) / (ys2 - ys1) + xs1;
        let xb = (y1.max(y2) - ys1) * (xs2 - xs1) / (ys2 - ys1) + xs1;
        if (xa < x1 && xb < x1) || (xa > x2 && xb > x2) {
            return true;
        }
    } else {
        return true;
    }

    false
}

/// Formats a double the way the JSON dump expects (round-trippable).
fn fmt(v: f64) -> String {
    // `{:?}` on f64 keeps full precision and round-trips.
    format!("{v:?}")
}

/// Parses a 3-element numeric array following the `"<name>": [a, b, c]` key.
fn parse_vec3(json: &str, name: &str) -> Option<[f64; 3]> {
    let key = format!("\"{name}\"");
    let start = json.find(&key)?;
    let bracket = json[start..].find('[')? + start;
    let close = json[bracket..].find(']')? + bracket;
    let inner = &json[bracket + 1..close];
    let mut nums = inner.split(',').map(|s| s.trim().parse::<f64>());
    let a = nums.next()?.ok()?;
    let b = nums.next()?.ok()?;
    let c = nums.next()?.ok()?;
    Some([a, b, c])
}

/// Parses a scalar value following the `"<name>": <value>` key.
fn parse_scalar(json: &str, name: &str) -> Option<f64> {
    let key = format!("\"{name}\"");
    let start = json.find(&key)?;
    let after = &json[start + key.len()..];
    let colon = after.find(':')?;
    let rest = &after[colon + 1..];
    // Read until the next `,` or `}`.
    let end = rest.find([',', '}']).unwrap_or(rest.len());
    rest[..end].trim().parse::<f64>().ok()
}
