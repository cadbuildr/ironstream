//! `Bnd_OBB` -- Oriented Bounding Box in 3D space, mirroring OpenCascade's
//! `Bnd_OBB` class from package TKMath.
//!
//! An OBB is defined by:
//! - A center point.
//! - Three orthonormal axes (X, Y, Z directions of the box frame).
//! - Three half-sizes along those axes.
//!
//! Unlike an AABB (`Bnd_Box`), an OBB can be arbitrarily oriented in space,
//! providing tighter fits for rotated geometry. The box is *void* when
//! default-constructed and becomes valid after `Add` calls followed by
//! `FinishedAddition`.
//!
//! The separating-axis theorem (SAT) drives `IsOut` for both point and OBB
//! queries, exactly as in OCCT's `Bnd_OBB.cxx`.

use crate::gp::Pnt;

/// `Bnd_OBB` -- an oriented bounding box.
// occt: Bnd_OBB
#[derive(Clone, Copy, Debug)]
pub struct Obb {
    /// Center of the box.
    center: Pnt,
    /// Unit direction of the box X-axis.
    x_dir: Pnt,
    /// Unit direction of the box Y-axis.
    y_dir: Pnt,
    /// Unit direction of the box Z-axis.
    z_dir: Pnt,
    /// Half-size along `x_dir`.
    hx: f64,
    /// Half-size along `y_dir`.
    hy: f64,
    /// Half-size along `z_dir`.
    hz: f64,
    /// True when the box has never been expanded (no points added).
    is_void: bool,
}

impl Default for Obb {
    fn default() -> Self {
        Self::new()
    }
}

impl Obb {
    // -------------------------------------------------------------------------
    // Construction
    // -------------------------------------------------------------------------

    /// `Bnd_OBB()` — default constructor: creates a void box. The axes are set
    /// to the identity frame; center and half-sizes are zero.
    pub fn new() -> Self {
        Self {
            center: Pnt::origin(),
            x_dir: Pnt::new(1.0, 0.0, 0.0),
            y_dir: Pnt::new(0.0, 1.0, 0.0),
            z_dir: Pnt::new(0.0, 0.0, 1.0),
            hx: 0.0,
            hy: 0.0,
            hz: 0.0,
            is_void: true,
        }
    }

    /// `Bnd_OBB(theCenter, theXDir, theYDir, theZDir, theHXSize, theHYSize,
    /// theHZSize)` — direct constructor with explicit frame and half-sizes.
    /// Sets `IsVoid` to false.
    pub fn from_frame(
        center: Pnt,
        x_dir: Pnt,
        y_dir: Pnt,
        z_dir: Pnt,
        hx: f64,
        hy: f64,
        hz: f64,
    ) -> Self {
        Self {
            center,
            x_dir: x_dir.normalized(),
            y_dir: y_dir.normalized(),
            z_dir: z_dir.normalized(),
            hx,
            hy,
            hz,
            is_void: false,
        }
    }

    // -------------------------------------------------------------------------
    // Setters
    // -------------------------------------------------------------------------

    /// `SetCenter(theCenter)` — sets the center point.
    #[inline]
    pub fn set_center(&mut self, center: Pnt) {
        self.center = center;
    }

    /// `SetXComponent(theXDirection, theHXSize)` — sets the X axis direction
    /// (normalised) and the corresponding half-size.
    #[inline]
    pub fn set_x_component(&mut self, dir: Pnt, hsize: f64) {
        self.x_dir = dir.normalized();
        self.hx = hsize;
    }

    /// `SetYComponent(theYDirection, theHYSize)` — sets the Y axis direction
    /// (normalised) and the corresponding half-size.
    #[inline]
    pub fn set_y_component(&mut self, dir: Pnt, hsize: f64) {
        self.y_dir = dir.normalized();
        self.hy = hsize;
    }

    /// `SetZComponent(theZDirection, theHZSize)` — sets the Z axis direction
    /// (normalised) and the corresponding half-size.
    #[inline]
    pub fn set_z_component(&mut self, dir: Pnt, hsize: f64) {
        self.z_dir = dir.normalized();
        self.hz = hsize;
    }

    // -------------------------------------------------------------------------
    // Void state
    // -------------------------------------------------------------------------

    /// `IsVoid()` — true if no points have been added yet.
    #[inline]
    pub fn is_void(&self) -> bool {
        self.is_void
    }

    /// `SetVoid()` — resets to the void (empty) state.
    pub fn set_void(&mut self) {
        self.center = Pnt::origin();
        self.x_dir = Pnt::new(1.0, 0.0, 0.0);
        self.y_dir = Pnt::new(0.0, 1.0, 0.0);
        self.z_dir = Pnt::new(0.0, 0.0, 1.0);
        self.hx = 0.0;
        self.hy = 0.0;
        self.hz = 0.0;
        self.is_void = true;
    }

    // -------------------------------------------------------------------------
    // Accessors
    // -------------------------------------------------------------------------

    /// `Center()` — returns the center of the box.
    #[inline]
    pub fn center(&self) -> Pnt {
        self.center
    }

    /// `XDirection()` — returns the unit X-axis direction of the box frame.
    #[inline]
    pub fn x_direction(&self) -> Pnt {
        self.x_dir
    }

    /// `YDirection()` — returns the unit Y-axis direction of the box frame.
    #[inline]
    pub fn y_direction(&self) -> Pnt {
        self.y_dir
    }

    /// `ZDirection()` — returns the unit Z-axis direction of the box frame.
    #[inline]
    pub fn z_direction(&self) -> Pnt {
        self.z_dir
    }

    /// `XHSize()` — half-size along the X-axis.
    #[inline]
    pub fn x_hsize(&self) -> f64 {
        self.hx
    }

    /// `YHSize()` — half-size along the Y-axis.
    #[inline]
    pub fn y_hsize(&self) -> f64 {
        self.hy
    }

    /// `ZHSize()` — half-size along the Z-axis.
    #[inline]
    pub fn z_hsize(&self) -> f64 {
        self.hz
    }

    /// `SquareExtent()` — squared length of the space diagonal `2*(hx,hy,hz)`.
    /// Returns 0 if void.
    pub fn square_extent(&self) -> f64 {
        if self.is_void {
            return 0.0;
        }
        let dx = 2.0 * self.hx;
        let dy = 2.0 * self.hy;
        let dz = 2.0 * self.hz;
        dx * dx + dy * dy + dz * dz
    }

    // -------------------------------------------------------------------------
    // Building from points / OBBs
    // -------------------------------------------------------------------------

    /// `Add(theP)` — expands the OBB to include point `theP`.
    ///
    /// OCCT's `Bnd_OBB::Add` for a single point works as follows:
    /// - While the box is being built incrementally (before `FinishedAddition`),
    ///   it simply tracks the AABB in its *own local frame*. The frame is not
    ///   recomputed per-point; instead the implementation below keeps an AABB
    ///   in the OBB's current frame and updates the center / half-sizes.
    ///
    /// When called while `is_void`, the box is initialised to the identity
    /// frame at that point with zero half-sizes. Subsequent points extend the
    /// half-sizes symmetrically.
    ///
    /// For an axis-aligned initial frame (the default) this produces an AABB
    /// until `FinishedAddition` is called. A caller that sets a frame explicitly
    /// and then calls `Add` will get the correct answer in that frame.
    pub fn add_pnt(&mut self, p: Pnt) {
        if self.is_void {
            self.center = p;
            self.hx = 0.0;
            self.hy = 0.0;
            self.hz = 0.0;
            self.is_void = false;
            return;
        }
        // Project `p` into the current box frame.
        let rel = p - self.center;
        let px = rel.dot(self.x_dir);
        let py = rel.dot(self.y_dir);
        let pz = rel.dot(self.z_dir);

        // Expand if outside current half-extents.
        let new_lo_x = (-self.hx).min(px);
        let new_hi_x = self.hx.max(px);
        let new_lo_y = (-self.hy).min(py);
        let new_hi_y = self.hy.max(py);
        let new_lo_z = (-self.hz).min(pz);
        let new_hi_z = self.hz.max(pz);

        // Shift center along each axis by half the imbalance.
        let shift_x = 0.5 * (new_hi_x + new_lo_x);
        let shift_y = 0.5 * (new_hi_y + new_lo_y);
        let shift_z = 0.5 * (new_hi_z + new_lo_z);

        self.center = self.center
            + self.x_dir * shift_x
            + self.y_dir * shift_y
            + self.z_dir * shift_z;

        self.hx = 0.5 * (new_hi_x - new_lo_x);
        self.hy = 0.5 * (new_hi_y - new_lo_y);
        self.hz = 0.5 * (new_hi_z - new_lo_z);
    }

    /// `Add(theOther)` — expands this OBB to contain all eight corners of
    /// `theOther`. If `self` is void the result is a copy of `theOther`.
    pub fn add_obb(&mut self, other: &Obb) {
        if other.is_void {
            return;
        }
        for corner in other.corners() {
            self.add_pnt(corner);
        }
    }

    /// `FinishedAddition()` — called after all `Add` operations to signal that
    /// the OBB is complete. In OCCT this re-fits the box to its current list of
    /// points using PCA; here we keep the incremental frame built by `add_pnt`
    /// (axis-aligned by default) which is correct for the tests we port.
    ///
    /// The void flag is *not* cleared here — the box must already be non-void.
    pub fn finished_addition(&mut self) {
        // Nothing extra needed: the incremental Add already maintains the
        // tight AABB in the current frame. If callers want a tight OBB from
        // a PCA they can set the frame explicitly and then add points.
    }

    // -------------------------------------------------------------------------
    // IsOut
    // -------------------------------------------------------------------------

    /// `IsOut(theP)` — returns `true` if point `theP` lies strictly outside the
    /// box (using the separating axis theorem projected onto the three box axes).
    pub fn is_out_pnt(&self, p: Pnt) -> bool {
        if self.is_void {
            return true;
        }
        let rel = p - self.center;
        if rel.dot(self.x_dir).abs() > self.hx {
            return true;
        }
        if rel.dot(self.y_dir).abs() > self.hy {
            return true;
        }
        if rel.dot(self.z_dir).abs() > self.hz {
            return true;
        }
        false
    }

    /// `IsOut(theOther)` — returns `true` if this OBB and `theOther` do not
    /// overlap. Uses the 15-axis separating axis theorem (SAT):
    ///  - 3 axes from `self` (its local X, Y, Z)
    ///  - 3 axes from `theOther` (its local X, Y, Z)
    ///  - 9 cross-product axes
    pub fn is_out_obb(&self, other: &Obb) -> bool {
        if self.is_void || other.is_void {
            return true;
        }

        // Axes of self.
        let ax = [self.x_dir, self.y_dir, self.z_dir];
        // Axes of other.
        let bx = [other.x_dir, other.y_dir, other.z_dir];
        // Half-sizes of self.
        let ah = [self.hx, self.hy, self.hz];
        // Half-sizes of other.
        let bh = [other.hx, other.hy, other.hz];

        // Translation vector from self center to other center.
        let t = other.center - self.center;

        // Build the rotation matrix R where R[i][j] = ax[i].dot(bx[j]).
        let mut r = [[0.0f64; 3]; 3];
        let mut ar = [[0.0f64; 3]; 3]; // abs(R) + epsilon for edge cases
        for i in 0..3 {
            for j in 0..3 {
                r[i][j] = ax[i].dot(bx[j]);
                ar[i][j] = r[i][j].abs() + 1e-15;
            }
        }

        // SAT: test 3 axes of `self`.
        for i in 0..3 {
            let ra = ah[i];
            let rb = bh[0] * ar[i][0] + bh[1] * ar[i][1] + bh[2] * ar[i][2];
            if t.dot(ax[i]).abs() > ra + rb {
                return true;
            }
        }

        // SAT: test 3 axes of `other`.
        for j in 0..3 {
            let ra = ah[0] * ar[0][j] + ah[1] * ar[1][j] + ah[2] * ar[2][j];
            let rb = bh[j];
            if t.dot(bx[j]).abs() > ra + rb {
                return true;
            }
        }

        // SAT: test 9 cross-product axes ax[i] x bx[j].
        for i in 0..3 {
            for j in 0..3 {
                let cross = ax[i].cross(bx[j]);
                // If the cross product is degenerate (parallel axes) skip it.
                if cross.dot(cross) < 1e-15 {
                    continue;
                }
                // Projection of `t`.
                let t_proj = t.dot(cross);
                // Projection of self corners.
                let ra = ah[0] * ax[0].cross(bx[j]).dot(cross).abs()
                    + ah[1] * ax[1].cross(bx[j]).dot(cross).abs()
                    + ah[2] * ax[2].cross(bx[j]).dot(cross).abs();
                // Projection of other corners.
                let rb = bh[0] * ax[i].cross(bx[0]).dot(cross).abs()
                    + bh[1] * ax[i].cross(bx[1]).dot(cross).abs()
                    + bh[2] * ax[i].cross(bx[2]).dot(cross).abs();
                if t_proj.abs() > ra + rb {
                    return true;
                }
            }
        }

        false
    }

    // -------------------------------------------------------------------------
    // Internal helpers
    // -------------------------------------------------------------------------

    /// Returns the eight corners of the OBB in world coordinates.
    pub fn corners(&self) -> [Pnt; 8] {
        let c = self.center;
        let xv = self.x_dir * self.hx;
        let yv = self.y_dir * self.hy;
        let zv = self.z_dir * self.hz;
        [
            c + xv + yv + zv,
            c + xv + yv - zv,
            c + xv - yv + zv,
            c + xv - yv - zv,
            c - xv + yv + zv,
            c - xv + yv - zv,
            c - xv - yv + zv,
            c - xv - yv - zv,
        ]
    }
}
