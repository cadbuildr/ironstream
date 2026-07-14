// FILE: rust/ironstream/crates/ironstream/src/geom_axis_placement.rs
//! Axis placement objects in 3D space — axis-1 and axis-2 placements.
//!
//! Ports `Geom_Axis1Placement` and `Geom_Axis2Placement` from OpenCascade
//! (`src/ModelingData/TKG3d/Geom/Geom_Axis1Placement.hxx` and
//! `Geom_Axis2Placement.hxx`).

// ── internal helpers (no external deps) ─────────────────────────────────────

#[inline]
fn normalize(v: [f64; 3]) -> [f64; 3] {
    let mag = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    assert!(mag > 1e-300, "cannot normalize a zero-length vector");
    [v[0] / mag, v[1] / mag, v[2] / mag]
}

#[inline]
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

// ── GeomAxis1Pl ──────────────────────────────────────────────────────────────

// occt-ref: Geom_Axis1Placement
/// An axis placement in 3D space: a location point and a unit direction.
///
/// Corresponds to OCCT's `Geom_Axis1Placement`, which wraps `gp_Ax1`.
/// The direction is always normalized on construction and when set.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeomAxis1Pl {
    location: [f64; 3],
    direction: [f64; 3],
}

impl GeomAxis1Pl {
    /// Create a new axis placement.
    ///
    /// `direction` is normalized; panics on a zero-length direction.
    pub fn new(location: [f64; 3], direction: [f64; 3]) -> Self {
        Self {
            location,
            direction: normalize(direction),
        }
    }

    /// Return the location (origin) of the axis.
    pub fn location(&self) -> [f64; 3] {
        self.location
    }

    /// Return the unit direction of the axis.
    pub fn direction(&self) -> [f64; 3] {
        self.direction
    }

    /// Set a new location point.
    pub fn set_location(&mut self, location: [f64; 3]) {
        self.location = location;
    }

    /// Set a new direction (normalized); panics on a zero-length vector.
    pub fn set_direction(&mut self, direction: [f64; 3]) {
        self.direction = normalize(direction);
    }

    /// Return a copy of this axis with the direction reversed.
    ///
    /// Mirrors `Geom_Axis1Placement::Reversed()`.
    pub fn reversed(&self) -> Self {
        Self {
            location: self.location,
            direction: [-self.direction[0], -self.direction[1], -self.direction[2]],
        }
    }

    /// Returns `true` when `self` and `other` are parallel (or anti-parallel)
    /// within `tol` radians.
    ///
    /// Checks whether `|dot(d1, d2)|` is within `tol` of 1.0, i.e. the angle
    /// between the two unit directions is within `tol` of 0 or π.
    ///
    /// Mirrors `Geom_Axis1Placement::IsParallel(other, tol)`.
    pub fn is_parallel(&self, other: &GeomAxis1Pl, tol: f64) -> bool {
        let d = dot(self.direction, other.direction).abs();
        d >= (1.0 - tol).max(0.0)
    }

    /// Returns the angle between this axis and `other` in radians, in `[0, π]`.
    ///
    /// Mirrors `Geom_Axis1Placement::Angle(other)`.
    pub fn angle(&self, other: &GeomAxis1Pl) -> f64 {
        dot(self.direction, other.direction).clamp(-1.0, 1.0).acos()
    }
}

// ── GeomAxis2Pl ──────────────────────────────────────────────────────────────

// occt-ref: Geom_Axis2Placement
/// A right-handed 3D coordinate system: location, X axis, Y axis, Z axis.
///
/// Corresponds to OCCT's `Geom_Axis2Placement`, which wraps `gp_Ax2`.
/// The Z direction is the "main" (normal) direction. The X direction is
/// orthogonalized against Z on construction. Y is always derived as `z × x`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeomAxis2Pl {
    location: [f64; 3],
    /// Main direction (Z axis).
    x_dir: [f64; 3],
    y_dir: [f64; 3],
    z_dir: [f64; 3],
}

impl GeomAxis2Pl {
    /// Create a right-handed coordinate system.
    ///
    /// `z_dir` is the main (normal) direction; `x_dir` is orthogonalized
    /// against `z_dir` via Gram–Schmidt before being stored. `y_dir` is
    /// computed as `z × x`.
    ///
    /// Panics if `z_dir` or the projection of `x_dir` onto the plane normal
    /// to `z_dir` is zero-length.
    pub fn new(location: [f64; 3], z_dir: [f64; 3], x_dir: [f64; 3]) -> Self {
        let z = normalize(z_dir);
        // Gram–Schmidt: remove the z component from x_dir.
        let x_raw = [
            x_dir[0] - dot(x_dir, z) * z[0],
            x_dir[1] - dot(x_dir, z) * z[1],
            x_dir[2] - dot(x_dir, z) * z[2],
        ];
        let x = normalize(x_raw);
        let y = cross(z, x);
        Self {
            location,
            x_dir: x,
            y_dir: y,
            z_dir: z,
        }
    }

    /// Return the location (origin) of the coordinate system.
    pub fn location(&self) -> [f64; 3] {
        self.location
    }

    /// Return the X direction.
    pub fn x_dir(&self) -> [f64; 3] {
        self.x_dir
    }

    /// Return the Y direction (always `z × x`).
    pub fn y_dir(&self) -> [f64; 3] {
        self.y_dir
    }

    /// Return the Z (main) direction.
    pub fn z_dir(&self) -> [f64; 3] {
        self.z_dir
    }

    /// Set a new location point.
    pub fn set_location(&mut self, location: [f64; 3]) {
        self.location = location;
    }

    /// Set a new Z direction, recomputing Y as `z × x` with the stored X.
    ///
    /// The stored `x_dir` is re-orthogonalized against the new `z_dir`.
    /// Mirrors `Geom_Axis2Placement::SetDirection(v)`.
    pub fn set_z_dir(&mut self, z_dir: [f64; 3]) {
        let z = normalize(z_dir);
        let x_raw = [
            self.x_dir[0] - dot(self.x_dir, z) * z[0],
            self.x_dir[1] - dot(self.x_dir, z) * z[1],
            self.x_dir[2] - dot(self.x_dir, z) * z[2],
        ];
        let x = normalize(x_raw);
        let y = cross(z, x);
        self.z_dir = z;
        self.x_dir = x;
        self.y_dir = y;
    }

    /// Returns `true` — a `Geom_Axis2Placement` is always a right-handed system.
    ///
    /// Mirrors `Geom_Axis2Placement::IsDirect()`.
    pub fn is_direct(&self) -> bool {
        true
    }
}
