// FILE: rust/ironstream/crates/ironstream/src/bnd_tools_ext.rs
//
// Pure-Rust, zero-dependency stubs modelled after OCCT's BndLib_AddSurface and
// BndLib_Add3dCurve / BndLib_Add2dCurve families.  No external crates are used;
// only `std` is available.

// ---------------------------------------------------------------------------
// BndAddCurveResult — 3-D AABB accumulator for curve bounds
// ---------------------------------------------------------------------------

/// Axis-aligned bounding box accumulator for 3-D curve bounding-box queries.
///
/// A newly created result is *void* (empty).  Points are added one at a time via
/// [`add_point`](BndAddCurveResult::add_point); the box expands to contain each
/// point.  `enlarge` pads every face outward by a uniform gap, mirroring
/// OCCT's `Bnd_Box::Enlarge`.
///
// occt: BndLib_Add3dCurve
#[derive(Clone, Copy, Debug)]
pub struct BndAddCurveResult {
    min: [f64; 3],
    max: [f64; 3],
    void: bool,
}

impl BndAddCurveResult {
    /// Create a void (empty) bounding box.
    pub fn new() -> Self {
        Self {
            min: [f64::MAX, f64::MAX, f64::MAX],
            max: [f64::MIN, f64::MIN, f64::MIN],
            void: true,
        }
    }

    /// Extend the box to contain point `p`.
    pub fn add_point(&mut self, p: [f64; 3]) {
        if self.void {
            self.min = p;
            self.max = p;
            self.void = false;
        } else {
            for i in 0..3 {
                if p[i] < self.min[i] {
                    self.min[i] = p[i];
                }
                if p[i] > self.max[i] {
                    self.max[i] = p[i];
                }
            }
        }
    }

    /// Pad every face of the box outward by `gap`. No-op when void.
    pub fn enlarge(&mut self, gap: f64) {
        if self.void {
            return;
        }
        for i in 0..3 {
            self.min[i] -= gap;
            self.max[i] += gap;
        }
    }

    /// Returns `true` when no points have been added yet.
    pub fn is_void(&self) -> bool {
        self.void
    }

    /// Return the minimum corner of the bounding box.
    pub fn min(&self) -> [f64; 3] {
        self.min
    }

    /// Return the maximum corner of the bounding box.
    pub fn max(&self) -> [f64; 3] {
        self.max
    }

    /// Return the size `(max - min)` in each axis.
    ///
    /// Returns `[0, 0, 0]` when void.
    pub fn size(&self) -> [f64; 3] {
        if self.void {
            return [0.0; 3];
        }
        [
            self.max[0] - self.min[0],
            self.max[1] - self.min[1],
            self.max[2] - self.min[2],
        ]
    }

    /// Return the center of the bounding box.
    ///
    /// Returns `[0, 0, 0]` when void.
    pub fn center(&self) -> [f64; 3] {
        if self.void {
            return [0.0; 3];
        }
        [
            (self.min[0] + self.max[0]) * 0.5,
            (self.min[1] + self.max[1]) * 0.5,
            (self.min[2] + self.max[2]) * 0.5,
        ]
    }

    /// Length of the space diagonal `||max - min||`.
    ///
    /// Returns `0.0` for a void box.
    pub fn diagonal(&self) -> f64 {
        if self.void {
            return 0.0;
        }
        let s = self.size();
        (s[0] * s[0] + s[1] * s[1] + s[2] * s[2]).sqrt()
    }
}

impl Default for BndAddCurveResult {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// BndAddSurfaceResult — 3-D AABB accumulator for surface bounds
// ---------------------------------------------------------------------------

/// Axis-aligned bounding box accumulator for parametric surface bounding-box queries.
///
/// Carries UV sample counts (`u_nb_samples`, `v_nb_samples`) used by OCCT's
/// `BndLib_AddSurface::Add` to control sampling density.
///
// occt: BndLib_AddSurface
#[derive(Clone, Copy, Debug)]
pub struct BndAddSurfaceResult {
    /// Number of samples along the U parameter direction.
    pub u_nb_samples: usize,
    /// Number of samples along the V parameter direction.
    pub v_nb_samples: usize,
    min: [f64; 3],
    max: [f64; 3],
    void: bool,
}

impl BndAddSurfaceResult {
    /// Construct a void result with the given UV sample counts.
    pub fn new(u_nb_samples: usize, v_nb_samples: usize) -> Self {
        Self {
            u_nb_samples,
            v_nb_samples,
            min: [f64::MAX, f64::MAX, f64::MAX],
            max: [f64::MIN, f64::MIN, f64::MIN],
            void: true,
        }
    }

    /// Extend the box to contain point `p`.
    pub fn add_point(&mut self, p: [f64; 3]) {
        if self.void {
            self.min = p;
            self.max = p;
            self.void = false;
        } else {
            for i in 0..3 {
                if p[i] < self.min[i] {
                    self.min[i] = p[i];
                }
                if p[i] > self.max[i] {
                    self.max[i] = p[i];
                }
            }
        }
    }

    /// Returns `true` when no points have been added yet.
    pub fn is_void(&self) -> bool {
        self.void
    }

    /// Return the minimum corner of the bounding box.
    pub fn min(&self) -> [f64; 3] {
        self.min
    }

    /// Return the maximum corner of the bounding box.
    pub fn max(&self) -> [f64; 3] {
        self.max
    }

    /// Return the center of the bounding box. Returns `[0,0,0]` when void.
    pub fn center(&self) -> [f64; 3] {
        if self.void {
            return [0.0; 3];
        }
        [
            (self.min[0] + self.max[0]) * 0.5,
            (self.min[1] + self.max[1]) * 0.5,
            (self.min[2] + self.max[2]) * 0.5,
        ]
    }

    /// Length of the space diagonal. Returns `0.0` when void.
    pub fn diagonal(&self) -> f64 {
        if self.void {
            return 0.0;
        }
        let dx = self.max[0] - self.min[0];
        let dy = self.max[1] - self.min[1];
        let dz = self.max[2] - self.min[2];
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

impl Default for BndAddSurfaceResult {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

// ---------------------------------------------------------------------------
// bnd_add_line — bounding box for a parametric line segment
// ---------------------------------------------------------------------------

/// Compute the 3-D bounding box for the line segment defined by
/// `origin + t * direction` for `t` in `[u_start, u_end]`, then expand by `gap`.
///
/// Samples both endpoints and expands outward by `gap`.
///
// occt: BndLib_Add3dCurve::Add (line case)
pub fn bnd_add_line(
    origin: [f64; 3],
    direction: [f64; 3],
    u_start: f64,
    u_end: f64,
    gap: f64,
) -> BndAddCurveResult {
    let mut r = BndAddCurveResult::new();
    let t0 = u_start;
    let t1 = u_end;
    let p0 = [
        origin[0] + t0 * direction[0],
        origin[1] + t0 * direction[1],
        origin[2] + t0 * direction[2],
    ];
    let p1 = [
        origin[0] + t1 * direction[0],
        origin[1] + t1 * direction[1],
        origin[2] + t1 * direction[2],
    ];
    r.add_point(p0);
    r.add_point(p1);
    if gap != 0.0 {
        r.enlarge(gap);
    }
    r
}

// ---------------------------------------------------------------------------
// bnd_add_circle — bounding box for a full 3-D circle
// ---------------------------------------------------------------------------

/// Compute the 3-D bounding box for a full circle with the given `center`
/// and `radius`, expanded outward by `gap`.
///
/// For a full circle the tight AABB is `center ± (radius + gap)` on all axes.
///
// occt: BndLib_Add3dCurve::Add (circle case)
pub fn bnd_add_circle(
    center: [f64; 3],
    radius: f64,
    gap: f64,
) -> BndAddCurveResult {
    let mut r = BndAddCurveResult::new();
    let half = radius + gap;
    r.min = [center[0] - half, center[1] - half, center[2] - half];
    r.max = [center[0] + half, center[1] + half, center[2] + half];
    r.void = false;
    r
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bnd_add_curve_result_void_on_creation() {
        let r = BndAddCurveResult::new();
        assert!(r.is_void());
        assert_eq!(r.diagonal(), 0.0);
        assert_eq!(r.size(), [0.0, 0.0, 0.0]);
        assert_eq!(r.center(), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn bnd_add_curve_result_single_point() {
        let mut r = BndAddCurveResult::new();
        r.add_point([1.0, 2.0, 3.0]);
        assert!(!r.is_void());
        assert_eq!(r.min(), [1.0, 2.0, 3.0]);
        assert_eq!(r.max(), [1.0, 2.0, 3.0]);
        assert_eq!(r.center(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn bnd_add_curve_result_enlarge() {
        let mut r = BndAddCurveResult::new();
        r.add_point([0.0, 0.0, 0.0]);
        r.add_point([2.0, 2.0, 2.0]);
        r.enlarge(1.0);
        assert_eq!(r.min(), [-1.0, -1.0, -1.0]);
        assert_eq!(r.max(), [3.0, 3.0, 3.0]);
    }

    #[test]
    fn bnd_add_surface_result_basic() {
        let mut r = BndAddSurfaceResult::new(4, 4);
        r.add_point([0.0, 0.0, 0.0]);
        r.add_point([1.0, 1.0, 1.0]);
        assert!(!r.is_void());
        assert!((r.diagonal() - 3.0_f64.sqrt()).abs() < 1e-12);
    }

    #[test]
    fn bnd_add_line_x_axis() {
        let r = bnd_add_line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 0.0, 5.0, 0.0);
        assert_eq!(r.min(), [0.0, 0.0, 0.0]);
        assert_eq!(r.max(), [5.0, 0.0, 0.0]);
    }

    #[test]
    fn bnd_add_circle_unit() {
        let r = bnd_add_circle([0.0, 0.0, 0.0], 1.0, 0.0);
        assert_eq!(r.min(), [-1.0, -1.0, -1.0]);
        assert_eq!(r.max(), [1.0, 1.0, 1.0]);
    }
}
