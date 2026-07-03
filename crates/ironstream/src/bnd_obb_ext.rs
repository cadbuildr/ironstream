// FILE: bnd_obb_ext.rs
// occt: Bnd_OBB (extended builder), Bnd_Sphere (extended),
//       Bnd_Range

/// An Oriented Bounding Box (OBB) defined by center, axes, and half-extents.
/// occt: Bnd_OBB
#[derive(Clone, Debug)]
pub struct BndObbExt {
    pub center: [f64; 3],
    pub x_direction: [f64; 3],
    pub y_direction: [f64; 3],
    pub z_direction: [f64; 3],
    pub half_size_x: f64,
    pub half_size_y: f64,
    pub half_size_z: f64,
    pub is_void: bool,
}

impl Default for BndObbExt {
    fn default() -> Self {
        Self {
            center: [0.0; 3],
            x_direction: [1.0, 0.0, 0.0],
            y_direction: [0.0, 1.0, 0.0],
            z_direction: [0.0, 0.0, 1.0],
            half_size_x: 0.0, half_size_y: 0.0, half_size_z: 0.0,
            is_void: true,
        }
    }
}

impl BndObbExt {
    pub fn new(center: [f64; 3], hx: f64, hy: f64, hz: f64) -> Self {
        Self {
            center, half_size_x: hx.abs(), half_size_y: hy.abs(), half_size_z: hz.abs(),
            is_void: hx == 0.0 && hy == 0.0 && hz == 0.0,
            ..Self::default()
        }
    }

    pub fn set_void(&mut self) { self.is_void = true; }
    pub fn is_void(&self) -> bool { self.is_void }

    pub fn volume(&self) -> f64 {
        8.0 * self.half_size_x * self.half_size_y * self.half_size_z
    }

    pub fn surface_area(&self) -> f64 {
        let (x, y, z) = (self.half_size_x, self.half_size_y, self.half_size_z);
        8.0 * (x*y + y*z + z*x)
    }

    /// AABB half-diagonal length (approximation for non-axis-aligned OBB).
    pub fn diagonal_size(&self) -> f64 {
        let d = [self.half_size_x * 2.0, self.half_size_y * 2.0, self.half_size_z * 2.0];
        (d[0]*d[0]+d[1]*d[1]+d[2]*d[2]).sqrt()
    }

    /// Enlarge each half-extent by gap.
    pub fn enlarge(&mut self, gap: f64) {
        self.half_size_x += gap.abs();
        self.half_size_y += gap.abs();
        self.half_size_z += gap.abs();
        self.is_void = false;
    }

    /// Conservative OBB-OBB intersection test (axis-aligned approximation).
    pub fn intersects_obb(&self, other: &BndObbExt) -> bool {
        if self.is_void || other.is_void { return false; }
        for k in 0..3 {
            let d = (self.center[k] - other.center[k]).abs();
            let combined = match k {
                0 => self.half_size_x + other.half_size_x,
                1 => self.half_size_y + other.half_size_y,
                _ => self.half_size_z + other.half_size_z,
            };
            if d > combined { return false; }
        }
        true
    }
}

/// A bounding sphere.
/// occt: Bnd_Sphere
#[derive(Clone, Debug)]
pub struct BndSphereExt {
    pub center: [f64; 3],
    pub radius: f64,
    pub u: f64,   // min parameter on generating curve
    pub v: f64,   // max parameter on generating curve
    pub is_valid: bool,
}

impl Default for BndSphereExt {
    fn default() -> Self { Self { center: [0.0;3], radius: 0.0, u: 0.0, v: 0.0, is_valid: false } }
}

impl BndSphereExt {
    pub fn new(center: [f64; 3], radius: f64) -> Self {
        Self { center, radius: radius.max(0.0), u: 0.0, v: 0.0, is_valid: radius >= 0.0 }
    }

    pub fn is_valid(&self) -> bool { self.is_valid }
    pub fn radius(&self) -> f64 { self.radius }
    pub fn center(&self) -> [f64; 3] { self.center }

    pub fn enlarge(&mut self, gap: f64) { self.radius += gap.abs(); }

    pub fn contains_point(&self, p: [f64; 3]) -> bool {
        let d = [p[0]-self.center[0], p[1]-self.center[1], p[2]-self.center[2]];
        (d[0]*d[0]+d[1]*d[1]+d[2]*d[2]).sqrt() <= self.radius + 1e-14
    }

    pub fn distance_to_point(&self, p: [f64; 3]) -> f64 {
        let d = [p[0]-self.center[0], p[1]-self.center[1], p[2]-self.center[2]];
        let dist = (d[0]*d[0]+d[1]*d[1]+d[2]*d[2]).sqrt();
        (dist - self.radius).max(0.0)
    }

    pub fn intersects_sphere(&self, other: &BndSphereExt) -> bool {
        let d = [self.center[0]-other.center[0], self.center[1]-other.center[1], self.center[2]-other.center[2]];
        let dist = (d[0]*d[0]+d[1]*d[1]+d[2]*d[2]).sqrt();
        dist <= self.radius + other.radius + 1e-14
    }
}

/// A 1D range [first, last].
/// occt: Bnd_Range
#[derive(Clone, Copy, Debug)]
pub struct BndRange {
    pub first: f64,
    pub last: f64,
    pub is_void: bool,
}

impl Default for BndRange {
    fn default() -> Self { Self { first: f64::INFINITY, last: f64::NEG_INFINITY, is_void: true } }
}

impl BndRange {
    pub fn new(first: f64, last: f64) -> Self {
        if last >= first { Self { first, last, is_void: false } }
        else { Self::default() }
    }

    pub fn is_void(&self) -> bool { self.is_void }
    pub fn first(&self) -> f64 { self.first }
    pub fn last(&self) -> f64 { self.last }
    pub fn delta(&self) -> f64 { if self.is_void { 0.0 } else { self.last - self.first } }

    pub fn add(&mut self, val: f64) {
        if self.is_void { self.first = val; self.last = val; self.is_void = false; }
        else { if val < self.first { self.first = val; } if val > self.last { self.last = val; } }
    }

    pub fn intersect(&mut self, other: BndRange) {
        if self.is_void || other.is_void { self.is_void = true; return; }
        let f = self.first.max(other.first);
        let l = self.last.min(other.last);
        if f <= l { self.first = f; self.last = l; }
        else { self.is_void = true; }
    }

    pub fn contains(&self, val: f64) -> bool {
        !self.is_void && val >= self.first - 1e-14 && val <= self.last + 1e-14
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn obb_volume() {
        let o = BndObbExt::new([0.0;3], 1.0, 2.0, 3.0);
        assert!((o.volume() - 48.0).abs() < 1e-10);
        assert!(!o.is_void());
    }

    #[test]
    fn obb_intersects() {
        let a = BndObbExt::new([0.0;3], 1.0, 1.0, 1.0);
        let b = BndObbExt::new([1.5, 0.0, 0.0], 1.0, 1.0, 1.0);
        assert!(a.intersects_obb(&b)); // overlapping
        let c = BndObbExt::new([5.0, 0.0, 0.0], 1.0, 1.0, 1.0);
        assert!(!a.intersects_obb(&c)); // separated
    }

    #[test]
    fn sphere_contains_point() {
        let s = BndSphereExt::new([0.0;3], 2.0);
        assert!(s.contains_point([1.0, 1.0, 0.0]));
        assert!(!s.contains_point([2.0, 2.0, 0.0]));
    }

    #[test]
    fn sphere_distance() {
        let s = BndSphereExt::new([0.0;3], 1.0);
        assert!((s.distance_to_point([3.0, 0.0, 0.0]) - 2.0).abs() < 1e-10);
        assert!((s.distance_to_point([0.5, 0.0, 0.0])).abs() < 1e-10);
    }

    #[test]
    fn bnd_range_operations() {
        let mut r = BndRange::new(1.0, 5.0);
        assert!((r.delta() - 4.0).abs() < 1e-10);
        assert!(r.contains(3.0));
        assert!(!r.contains(6.0));
        r.intersect(BndRange::new(3.0, 10.0));
        assert!((r.first() - 3.0).abs() < 1e-10);
        assert!((r.last() - 5.0).abs() < 1e-10);
    }
}
