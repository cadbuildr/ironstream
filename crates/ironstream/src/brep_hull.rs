// FILE: rust/ironstream/crates/ironstream/src/brep_hull.rs
//! Convex hull utilities mirroring OCCT's BRepMesh hull interface,
//! implemented in pure Rust with zero external dependencies.

// ---------------------------------------------------------------------------
// HullVertex
// ---------------------------------------------------------------------------

/// A single vertex in a convex hull result.
///
// occt: BRepMesh_Vertex
#[derive(Debug, Clone, PartialEq)]
pub struct HullVertex {
    point: [f64; 3],
    index: usize,
}

impl HullVertex {
    /// Construct a hull vertex from a 3-D point and an index into the input cloud.
    pub fn new(point: [f64; 3], index: usize) -> Self {
        Self { point, index }
    }

    /// Return the 3-D coordinates of this vertex.
    pub fn point(&self) -> [f64; 3] {
        self.point
    }

    /// Return the index of this vertex in the original input point cloud.
    pub fn index(&self) -> usize {
        self.index
    }
}

// ---------------------------------------------------------------------------
// ConvexHull3D
// ---------------------------------------------------------------------------

/// 3-D convex hull stub mirroring OCCT's BRepMesh convex-hull interface.
///
/// `perform()` stores every input point as a `HullVertex` (stub: no actual
/// incremental-gift-wrap or quickhull is performed).  `faces` is left empty.
///
// occt-ref: BRepMesh_ConvexHull
#[derive(Debug, Clone, PartialEq)]
pub struct ConvexHull3D {
    vertices: Vec<HullVertex>,
    faces: Vec<[usize; 3]>,
    is_done: bool,
}

impl ConvexHull3D {
    /// Construct an empty, not-yet-performed hull.
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            faces: Vec::new(),
            is_done: false,
        }
    }

    /// Run the hull computation over the given point cloud.
    ///
    /// Stub: every input point is stored as a `HullVertex`; `faces` remains empty.
    pub fn perform(&mut self, points: Vec<[f64; 3]>) {
        self.vertices = points
            .into_iter()
            .enumerate()
            .map(|(i, p)| HullVertex::new(p, i))
            .collect();
        self.faces.clear();
        self.is_done = true;
    }

    /// Return `true` once `perform` has been called.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Return the number of hull vertices.
    pub fn nb_vertices(&self) -> usize {
        self.vertices.len()
    }

    /// Return the number of hull faces (stub: always 0).
    pub fn nb_faces(&self) -> usize {
        self.faces.len()
    }

    /// Return a reference to the hull vertex at position `i`.
    ///
    /// # Panics
    /// Panics if `i >= nb_vertices()`.
    pub fn vertex(&self, i: usize) -> &HullVertex {
        &self.vertices[i]
    }

    /// Return the axis-aligned bounding box `(min, max)` of all hull vertices.
    ///
    /// Returns `([0,0,0], [0,0,0])` when the hull is empty.
    pub fn bounding_box(&self) -> ([f64; 3], [f64; 3]) {
        if self.vertices.is_empty() {
            return ([0.0; 3], [0.0; 3]);
        }
        let first = self.vertices[0].point;
        let mut lo = first;
        let mut hi = first;
        for v in &self.vertices[1..] {
            for k in 0..3 {
                if v.point[k] < lo[k] {
                    lo[k] = v.point[k];
                }
                if v.point[k] > hi[k] {
                    hi[k] = v.point[k];
                }
            }
        }
        (lo, hi)
    }
}

impl Default for ConvexHull3D {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// ConvexHull2D
// ---------------------------------------------------------------------------

/// 2-D convex hull stub.
///
/// `perform()` stores every input point (stub: no actual Graham scan).
/// `area_approx()` applies the shoelace formula over the stored points.
///
// occt-note: BRepMesh_ConvexHull (2-D variant)
#[derive(Debug, Clone, PartialEq)]
pub struct ConvexHull2D {
    points: Vec<[f64; 2]>,
    is_done: bool,
}

impl ConvexHull2D {
    /// Construct an empty, not-yet-performed hull.
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            is_done: false,
        }
    }

    /// Run the hull computation over the given 2-D point cloud.
    ///
    /// Stub: every input point is stored as-is; no actual convex hull is computed.
    pub fn perform(&mut self, points: Vec<[f64; 2]>) {
        self.points = points;
        self.is_done = true;
    }

    /// Return `true` once `perform` has been called.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Return the number of stored vertices.
    pub fn nb_vertices(&self) -> usize {
        self.points.len()
    }

    /// Return the 2-D point at position `i`.
    ///
    /// # Panics
    /// Panics if `i >= nb_vertices()`.
    pub fn vertex(&self, i: usize) -> [f64; 2] {
        self.points[i]
    }

    /// Approximate the hull area using the shoelace formula applied to the
    /// stored points in their original order.
    ///
    /// Returns `0.0` for fewer than 3 points.
    pub fn area_approx(&self) -> f64 {
        let n = self.points.len();
        if n < 3 {
            return 0.0;
        }
        let mut sum = 0.0_f64;
        for i in 0..n {
            let j = (i + 1) % n;
            sum += self.points[i][0] * self.points[j][1];
            sum -= self.points[j][0] * self.points[i][1];
        }
        sum.abs() * 0.5
    }
}

impl Default for ConvexHull2D {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hull_vertex_roundtrip() {
        let v = HullVertex::new([1.0, 2.0, 3.0], 5);
        assert_eq!(v.point(), [1.0, 2.0, 3.0]);
        assert_eq!(v.index(), 5);
    }

    #[test]
    fn hull3d_new_not_done() {
        let h = ConvexHull3D::new();
        assert!(!h.is_done());
        assert_eq!(h.nb_vertices(), 0);
        assert_eq!(h.nb_faces(), 0);
    }

    #[test]
    fn hull3d_perform_and_bounding_box() {
        let mut h = ConvexHull3D::new();
        h.perform(vec![[0.0, 0.0, 0.0], [1.0, 2.0, 3.0]]);
        assert!(h.is_done());
        assert_eq!(h.nb_vertices(), 2);
        let (lo, hi) = h.bounding_box();
        assert_eq!(lo, [0.0, 0.0, 0.0]);
        assert_eq!(hi, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn hull2d_area_unit_square() {
        let mut h = ConvexHull2D::new();
        h.perform(vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
        assert!((h.area_approx() - 1.0).abs() < 1e-12);
    }
}
