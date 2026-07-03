// FILE: rust/ironstream/crates/ironstream/src/gp_ax3_ext.rs

// occt: gp_Dir — unit direction vector (zero-dependency port)
/// A 3D unit direction vector.
///
/// Always stored normalized so that `x²+y²+z² == 1`.
// occt: gp_Dir
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpDir {
    x: f64,
    y: f64,
    z: f64,
}

/// Smallest magnitude accepted for normalization.
const RESOLUTION: f64 = 1.0e-15;

impl GpDir {
    /// Construct from raw coordinates, normalizing immediately.
    ///
    /// # Panics
    /// Panics if the input vector's magnitude is ≤ `RESOLUTION`.
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let n = (x * x + y * y + z * z).sqrt();
        assert!(
            n > RESOLUTION,
            "GpDir: cannot build a direction from the null vector"
        );
        let inv = 1.0 / n;
        Self {
            x: x * inv,
            y: y * inv,
            z: z * inv,
        }
    }

    /// Return the X component.
    #[inline]
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Return the Y component.
    #[inline]
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Return the Z component.
    #[inline]
    pub fn z(&self) -> f64 {
        self.z
    }

    /// Return the direction with all components negated.
    // occt: gp_Dir::Reversed()
    #[inline]
    pub fn reversed(&self) -> GpDir {
        GpDir {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    /// Dot product; equals `cos(angle(self, other))` for unit directions.
    // occt: gp_Dir::Dot(Other)
    #[inline]
    pub fn dot(&self, other: &GpDir) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Cross product `self × other` returned as a new `GpDir`.
    ///
    /// # Panics
    /// Panics if the result is the null vector (parallel or anti-parallel input).
    // occt: gp_Dir::Crossed(Other)
    #[inline]
    pub fn cross(&self, other: &GpDir) -> GpDir {
        let cx = self.y * other.z - self.z * other.y;
        let cy = self.z * other.x - self.x * other.z;
        let cz = self.x * other.y - self.y * other.x;
        GpDir::new(cx, cy, cz)
    }

    /// Unsigned angle in `[0, π]` radians between `self` and `other`.
    // occt: gp_Dir::Angle(Other)
    #[inline]
    pub fn angle(&self, other: &GpDir) -> f64 {
        self.dot(other).clamp(-1.0, 1.0).acos()
    }

    /// True when `self` and `other` are parallel or anti-parallel within `tol` radians.
    // occt: gp_Dir::IsParallel(Other, AngularTolerance)
    #[inline]
    pub fn is_parallel(&self, other: &GpDir, tol: f64) -> bool {
        let a = self.angle(other);
        a <= tol || (std::f64::consts::PI - a) <= tol
    }

    /// True when `self` and `other` are perpendicular within `tol` radians
    /// (i.e. `|angle - π/2| ≤ tol`).
    // occt: gp_Dir::IsNormal(Other, AngularTolerance)
    #[inline]
    pub fn is_normal(&self, other: &GpDir, tol: f64) -> bool {
        (self.angle(other) - std::f64::consts::FRAC_PI_2).abs() <= tol
    }
}

// ─────────────────────────────────────────────────────────────────────────────

// occt: gp_Ax1 — axis defined by a location point and a unit direction
/// An axis in 3D space: a location (point) and a unit direction.
// occt: gp_Ax1
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpAx1Ext {
    location: [f64; 3],
    direction: GpDir,
}

impl GpAx1Ext {
    /// Construct from an origin point and a direction vector (which will be normalized).
    ///
    /// # Panics
    /// Panics if `direction` is the null vector.
    pub fn new(location: [f64; 3], direction: [f64; 3]) -> Self {
        Self {
            location,
            direction: GpDir::new(direction[0], direction[1], direction[2]),
        }
    }

    /// Return the location (origin point) of the axis.
    #[inline]
    pub fn location(&self) -> [f64; 3] {
        self.location
    }

    /// Return a reference to the unit direction of the axis.
    #[inline]
    pub fn direction(&self) -> &GpDir {
        &self.direction
    }

    /// Set the location (origin point) of the axis.
    #[inline]
    pub fn set_location(&mut self, location: [f64; 3]) {
        self.location = location;
    }

    /// Set the direction, normalizing the supplied vector.
    ///
    /// # Panics
    /// Panics if `direction` is the null vector.
    #[inline]
    pub fn set_direction(&mut self, direction: [f64; 3]) {
        self.direction = GpDir::new(direction[0], direction[1], direction[2]);
    }

    /// Return a new axis with the direction reversed (location unchanged).
    // occt: gp_Ax1::Reversed()
    #[inline]
    pub fn reversed(&self) -> GpAx1Ext {
        GpAx1Ext {
            location: self.location,
            direction: self.direction.reversed(),
        }
    }

    /// True when `self` and `other` share the same line (collinear within tolerances).
    ///
    /// `ang_tol`: angular tolerance for direction comparison (radians).
    /// `lin_tol`: linear tolerance for distance from `other`'s origin to `self`'s line.
    // occt: gp_Ax1::IsCoaxial(Other, AngularTolerance, LinearTolerance)
    pub fn is_coaxial(&self, other: &GpAx1Ext, ang_tol: f64, lin_tol: f64) -> bool {
        if !self.direction.is_parallel(&other.direction, ang_tol) {
            return false;
        }
        // Distance from other.location to the infinite line through self.location along self.direction
        let dx = other.location[0] - self.location[0];
        let dy = other.location[1] - self.location[1];
        let dz = other.location[2] - self.location[2];
        let d = &self.direction;
        // cross product of (other.loc - self.loc) with self.direction
        let cx = dy * d.z - dz * d.y;
        let cy = dz * d.x - dx * d.z;
        let cz = dx * d.y - dy * d.x;
        let dist = (cx * cx + cy * cy + cz * cz).sqrt();
        dist <= lin_tol
    }

    /// True when `self` and `other` are parallel or anti-parallel within `tol` radians.
    // occt: gp_Ax1::IsParallel(Other, AngularTolerance)
    #[inline]
    pub fn is_parallel(&self, other: &GpAx1Ext, tol: f64) -> bool {
        self.direction.is_parallel(&other.direction, tol)
    }

    /// Unsigned angle in `[0, π]` radians between the directions of `self` and `other`.
    // occt: gp_Ax1::Angle(Other)
    #[inline]
    pub fn angle(&self, other: &GpAx1Ext) -> f64 {
        self.direction.angle(&other.direction)
    }
}

// ─────────────────────────────────────────────────────────────────────────────

// occt: gp_Ax3 — right-handed coordinate frame (origin + X/Y/Z directions)
/// A right-handed 3D coordinate frame: origin, X direction, Y direction, Z direction.
///
/// On construction `y_direction` is computed as `z × x` so the frame is always
/// right-handed.
// occt: gp_Ax3
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpAx3Ext {
    origin: [f64; 3],
    x_direction: GpDir,
    y_direction: GpDir,
    z_direction: GpDir,
}

impl GpAx3Ext {
    /// Construct a right-handed coordinate frame.
    ///
    /// `z_dir` becomes the Z axis; `x_dir` is re-orthogonalized against `z_dir`
    /// and normalized to form the X axis; the Y axis is `z × x`.
    ///
    /// # Panics
    /// Panics if `z_dir` or the component of `x_dir` perpendicular to `z_dir` is null.
    pub fn new(origin: [f64; 3], z_dir: [f64; 3], x_dir: [f64; 3]) -> Self {
        let z = GpDir::new(z_dir[0], z_dir[1], z_dir[2]);

        // Orthogonalize x_dir against z: x_orth = x_dir - (x_dir · z) * z
        let xr = [x_dir[0], x_dir[1], x_dir[2]];
        let dot_xz = xr[0] * z.x + xr[1] * z.y + xr[2] * z.z;
        let ox = xr[0] - dot_xz * z.x;
        let oy = xr[1] - dot_xz * z.y;
        let oz = xr[2] - dot_xz * z.z;
        let x = GpDir::new(ox, oy, oz);

        // y = z × x  (right-handed)
        let y = z.cross(&x);

        Self {
            origin,
            x_direction: x,
            y_direction: y,
            z_direction: z,
        }
    }

    /// Return the origin of the frame.
    #[inline]
    pub fn origin(&self) -> [f64; 3] {
        self.origin
    }

    /// Return a reference to the X direction.
    #[inline]
    pub fn x_direction(&self) -> &GpDir {
        &self.x_direction
    }

    /// Return a reference to the Y direction.
    #[inline]
    pub fn y_direction(&self) -> &GpDir {
        &self.y_direction
    }

    /// Return a reference to the Z direction.
    #[inline]
    pub fn z_direction(&self) -> &GpDir {
        &self.z_direction
    }

    /// Always returns `true`: this frame is always right-handed by construction.
    // occt: gp_Ax3::Direct()
    #[inline]
    pub fn is_direct(&self) -> bool {
        true
    }

    /// Unsigned angle in `[0, π]` radians between the Z axes of `self` and `other`.
    // occt: gp_Ax3::Angle(Other) — angle between Z directions
    #[inline]
    pub fn angle(&self, other: &GpAx3Ext) -> f64 {
        self.z_direction.angle(&other.z_direction)
    }
}
