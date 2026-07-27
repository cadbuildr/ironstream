// FILE: bndlib_ext.rs
// occt: BndLib_Add2dCurve
// occt-ref: BndLib_Add3dCurve, BndLib_AddSurface

/// A 3D axis-aligned bounding box.
#[derive(Clone, Debug)]
pub struct Bbox3d {
    pub min: [f64; 3],
    pub max: [f64; 3],
    pub is_void: bool,
}

impl Default for Bbox3d {
    fn default() -> Self {
        Self { min: [f64::MAX, f64::MAX, f64::MAX], max: [f64::MIN, f64::MIN, f64::MIN], is_void: true }
    }
}

impl Bbox3d {
    pub fn new() -> Self { Self::default() }

    pub fn add_point(&mut self, p: [f64; 3]) {
        if self.is_void {
            self.min = p;
            self.max = p;
            self.is_void = false;
        } else {
            for i in 0..3 {
                if p[i] < self.min[i] { self.min[i] = p[i]; }
                if p[i] > self.max[i] { self.max[i] = p[i]; }
            }
        }
    }

    pub fn enlarge(&mut self, tol: f64) {
        if !self.is_void {
            for i in 0..3 { self.min[i] -= tol; self.max[i] += tol; }
        }
    }

    pub fn is_void(&self) -> bool { self.is_void }

    pub fn diagonal(&self) -> f64 {
        if self.is_void { return 0.0; }
        let d = [self.max[0]-self.min[0], self.max[1]-self.min[1], self.max[2]-self.min[2]];
        (d[0]*d[0]+d[1]*d[1]+d[2]*d[2]).sqrt()
    }

    pub fn center(&self) -> [f64; 3] {
        [(self.min[0]+self.max[0])/2.0, (self.min[1]+self.max[1])/2.0, (self.min[2]+self.max[2])/2.0]
    }

    pub fn contains(&self, p: [f64; 3]) -> bool {
        !self.is_void &&
        p[0] >= self.min[0] && p[0] <= self.max[0] &&
        p[1] >= self.min[1] && p[1] <= self.max[1] &&
        p[2] >= self.min[2] && p[2] <= self.max[2]
    }

    pub fn intersects(&self, other: &Bbox3d) -> bool {
        if self.is_void || other.is_void { return false; }
        for i in 0..3 {
            if self.min[i] > other.max[i] || other.min[i] > self.max[i] { return false; }
        }
        true
    }

    pub fn merged_with(&self, other: &Bbox3d) -> Bbox3d {
        let mut out = self.clone();
        if !other.is_void {
            out.add_point(other.min);
            out.add_point(other.max);
        }
        out
    }
}

/// A 2D axis-aligned bounding box.
#[derive(Clone, Debug)]
pub struct Bbox2d {
    pub min: [f64; 2],
    pub max: [f64; 2],
    pub is_void: bool,
}

impl Default for Bbox2d {
    fn default() -> Self {
        Self { min: [f64::MAX, f64::MAX], max: [f64::MIN, f64::MIN], is_void: true }
    }
}

impl Bbox2d {
    pub fn new() -> Self { Self::default() }

    pub fn add_point(&mut self, p: [f64; 2]) {
        if self.is_void {
            self.min = p; self.max = p; self.is_void = false;
        } else {
            for i in 0..2 {
                if p[i] < self.min[i] { self.min[i] = p[i]; }
                if p[i] > self.max[i] { self.max[i] = p[i]; }
            }
        }
    }

    pub fn enlarge(&mut self, tol: f64) {
        if !self.is_void {
            for i in 0..2 { self.min[i] -= tol; self.max[i] += tol; }
        }
    }

    pub fn is_void(&self) -> bool { self.is_void }
    pub fn area(&self) -> f64 {
        if self.is_void { 0.0 } else { (self.max[0]-self.min[0]) * (self.max[1]-self.min[1]) }
    }
}

// occt-ref: BndLib_Add3dCurve // — computes bounding box of a 3D curve
#[derive(Clone, Debug, Default)]
pub struct BndLibAdd3dCurve;

impl BndLibAdd3dCurve {
    pub fn new() -> Self { Self }

    /// Compute bbox for a parametric curve given sample points.
    pub fn add_samples(samples: &[[f64; 3]], tolerance: f64) -> Bbox3d {
        let mut bbox = Bbox3d::new();
        for &pt in samples { bbox.add_point(pt); }
        bbox.enlarge(tolerance);
        bbox
    }

    /// Compute bbox for a line segment.
    pub fn add_line_segment(p1: [f64; 3], p2: [f64; 3], tol: f64) -> Bbox3d {
        let mut bbox = Bbox3d::new();
        bbox.add_point(p1);
        bbox.add_point(p2);
        bbox.enlarge(tol);
        bbox
    }

    /// Compute bbox for a circle in 3D.
    pub fn add_circle(center: [f64; 3], radius: f64, tol: f64) -> Bbox3d {
        let mut bbox = Bbox3d::new();
        let r = radius + tol;
        bbox.min = [center[0]-r, center[1]-r, center[2]-r];
        bbox.max = [center[0]+r, center[1]+r, center[2]+r];
        bbox.is_void = false;
        bbox
    }
}

// occt: BndLib_Add2dCurve // — computes bounding box of a 2D curve
#[derive(Clone, Debug, Default)]
pub struct BndLibAdd2dCurve;

impl BndLibAdd2dCurve {
    pub fn new() -> Self { Self }

    pub fn add_samples(samples: &[[f64; 2]], tolerance: f64) -> Bbox2d {
        let mut bbox = Bbox2d::new();
        for &pt in samples { bbox.add_point(pt); }
        bbox.enlarge(tolerance);
        bbox
    }

    pub fn add_line_segment_2d(p1: [f64; 2], p2: [f64; 2], tol: f64) -> Bbox2d {
        let mut bbox = Bbox2d::new();
        bbox.add_point(p1);
        bbox.add_point(p2);
        bbox.enlarge(tol);
        bbox
    }
}

// occt-ref: BndLib_AddSurface // — computes bounding box of a surface
#[derive(Clone, Debug, Default)]
pub struct BndLibAddSurface;

impl BndLibAddSurface {
    pub fn new() -> Self { Self }

    /// Add samples on a parametric surface to a bounding box.
    pub fn add_samples(samples: &[[f64; 3]], tolerance: f64) -> Bbox3d {
        BndLibAdd3dCurve::add_samples(samples, tolerance)
    }

    /// Add a planar surface (defined by corner points) to a bounding box.
    pub fn add_plane_patch(corners: &[[f64; 3]], tol: f64) -> Bbox3d {
        let mut bbox = Bbox3d::new();
        for &c in corners { bbox.add_point(c); }
        bbox.enlarge(tol);
        bbox
    }

    /// Add a sphere surface to a bounding box.
    pub fn add_sphere(center: [f64; 3], radius: f64, tol: f64) -> Bbox3d {
        BndLibAdd3dCurve::add_circle(center, radius, tol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bbox3d_add_points() {
        let mut b = Bbox3d::new();
        assert!(b.is_void());
        b.add_point([1.0, 2.0, 3.0]);
        b.add_point([-1.0, 0.0, 5.0]);
        assert!(!b.is_void());
        assert_eq!(b.min, [-1.0, 0.0, 3.0]);
        assert_eq!(b.max, [1.0, 2.0, 5.0]);
    }

    #[test]
    fn bbox3d_contains_intersects() {
        let mut b = Bbox3d::new();
        b.add_point([0.0, 0.0, 0.0]);
        b.add_point([2.0, 2.0, 2.0]);
        assert!(b.contains([1.0, 1.0, 1.0]));
        assert!(!b.contains([3.0, 1.0, 1.0]));

        let mut b2 = Bbox3d::new();
        b2.add_point([1.0, 1.0, 1.0]);
        b2.add_point([3.0, 3.0, 3.0]);
        assert!(b.intersects(&b2));
    }

    #[test]
    fn bbox3d_diagonal_center() {
        let mut b = Bbox3d::new();
        b.add_point([0.0, 0.0, 0.0]);
        b.add_point([3.0, 4.0, 0.0]);
        assert!((b.diagonal() - 5.0).abs() < 1e-10);
        let c = b.center();
        assert!((c[0] - 1.5).abs() < 1e-10);
    }

    #[test]
    fn bndlib_add_line_segment() {
        let b = BndLibAdd3dCurve::add_line_segment([0.0,0.0,0.0], [1.0,1.0,1.0], 0.0);
        assert!(!b.is_void());
        assert!((b.diagonal() - 3_f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn bndlib_add_circle() {
        let b = BndLibAdd3dCurve::add_circle([0.0,0.0,0.0], 5.0, 0.0);
        assert!(!b.is_void());
        assert_eq!(b.min, [-5.0, -5.0, -5.0]);
        assert_eq!(b.max, [5.0, 5.0, 5.0]);
    }

    #[test]
    fn bndlib_2d_curve() {
        let b = BndLibAdd2dCurve::add_line_segment_2d([0.0, 0.0], [3.0, 4.0], 0.1);
        assert!(b.area() > 0.0);
    }

    #[test]
    fn bndlib_add_sphere() {
        let b = BndLibAddSurface::add_sphere([1.0, 2.0, 3.0], 2.0, 0.0);
        assert!(b.contains([1.0, 2.0, 3.0]));
    }
}
