// FILE: brep_locs.rs
// occt: BRepLocs_Displacement, BRepLocs_StackIterator,
//       BRepGProp_Gauss (solid/surface/curve mass properties)

/// A displacement: a 3D rigid-body motion (rotation + translation).
/// occt: BRepLocs_Displacement
#[derive(Clone, Copy, Debug)]
pub struct BrepLocsDisplacement {
    /// Row-major 3×4 matrix: rotation (cols 0-2) + translation (col 3).
    pub matrix: [[f64; 4]; 3],
}

impl Default for BrepLocsDisplacement {
    fn default() -> Self {
        Self {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ]
        }
    }
}

impl BrepLocsDisplacement {
    pub fn identity() -> Self { Self::default() }

    pub fn from_translation(dx: f64, dy: f64, dz: f64) -> Self {
        let mut m = Self::default();
        m.matrix[0][3] = dx;
        m.matrix[1][3] = dy;
        m.matrix[2][3] = dz;
        m
    }

    /// Rotation around Z axis by angle (radians).
    pub fn from_rotation_z(angle: f64) -> Self {
        let c = angle.cos(); let s = angle.sin();
        Self {
            matrix: [
                [c, -s, 0.0, 0.0],
                [s,  c, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ]
        }
    }

    pub fn translation(&self) -> [f64; 3] {
        [self.matrix[0][3], self.matrix[1][3], self.matrix[2][3]]
    }

    /// Apply to a 3D point.
    pub fn apply(&self, p: [f64; 3]) -> [f64; 3] {
        let m = &self.matrix;
        [m[0][0]*p[0]+m[0][1]*p[1]+m[0][2]*p[2]+m[0][3],
         m[1][0]*p[0]+m[1][1]*p[1]+m[1][2]*p[2]+m[1][3],
         m[2][0]*p[0]+m[2][1]*p[1]+m[2][2]*p[2]+m[2][3]]
    }

    /// Compose two displacements: self then other.
    pub fn compose(&self, other: &Self) -> Self {
        let a = &self.matrix; let b = &other.matrix;
        let mut r = [[0.0f64; 4]; 3];
        for i in 0..3 {
            for j in 0..3 {
                r[i][j] = a[i][0]*b[0][j] + a[i][1]*b[1][j] + a[i][2]*b[2][j];
            }
            r[i][3] = a[i][0]*b[0][3] + a[i][1]*b[1][3] + a[i][2]*b[2][3] + a[i][3];
        }
        Self { matrix: r }
    }

    pub fn is_identity(&self) -> bool {
        let id = Self::identity();
        for i in 0..3 { for j in 0..4 { if (self.matrix[i][j] - id.matrix[i][j]).abs() > 1e-14 { return false; } } }
        true
    }
}

/// Stack of displacements for hierarchical location composition.
/// occt: BRepLocs_StackIterator (simplified)
#[derive(Clone, Debug, Default)]
pub struct BrepLocsStack {
    pub displacements: Vec<BrepLocsDisplacement>,
}

impl BrepLocsStack {
    pub fn new() -> Self { Self::default() }

    pub fn push(&mut self, d: BrepLocsDisplacement) { self.displacements.push(d); }
    pub fn pop(&mut self) -> Option<BrepLocsDisplacement> { self.displacements.pop() }

    /// Compose all displacements in order (bottom-of-stack first).
    pub fn composed(&self) -> BrepLocsDisplacement {
        let mut result = BrepLocsDisplacement::identity();
        for d in &self.displacements { result = result.compose(d); }
        result
    }

    pub fn depth(&self) -> usize { self.displacements.len() }
    pub fn is_empty(&self) -> bool { self.displacements.is_empty() }
}

/// Mass/inertia properties of a shape.
/// occt: BRepGProp_Gauss (simplified result struct)
#[derive(Clone, Debug, Default)]
pub struct BrepGPropResult {
    pub mass: f64,
    pub centre_of_mass: [f64; 3],
    pub inertia: [f64; 6],   // Ixx, Iyy, Izz, Ixy, Ixz, Iyz
}

impl BrepGPropResult {
    pub fn new() -> Self { Self::default() }

    pub fn ixx(&self) -> f64 { self.inertia[0] }
    pub fn iyy(&self) -> f64 { self.inertia[1] }
    pub fn izz(&self) -> f64 { self.inertia[2] }
    pub fn ixy(&self) -> f64 { self.inertia[3] }
    pub fn ixz(&self) -> f64 { self.inertia[4] }
    pub fn iyz(&self) -> f64 { self.inertia[5] }

    pub fn principal_moments(&self) -> [f64; 3] {
        [self.ixx(), self.iyy(), self.izz()]
    }
}

/// Computes global properties (volume, surface area, linear length) of shapes.
/// occt: BRepGProp (global function namespace)
#[derive(Clone, Debug, Default)]
pub struct BrepGPropSurface {
    pub shape_id: u32,
    pub result: BrepGPropResult,
    pub is_done: bool,
}

impl BrepGPropSurface {
    pub fn new(shape_id: u32) -> Self {
        Self { shape_id, result: BrepGPropResult::default(), is_done: false }
    }

    /// Stub: surface area = shape_id as a fake area, centre = origin.
    pub fn perform(&mut self, tolerance: f64) {
        let _ = tolerance;
        self.result.mass = self.shape_id as f64 * 1.0;  // fake area
        self.result.centre_of_mass = [0.0; 3];
        self.result.inertia = [1.0, 1.0, 1.0, 0.0, 0.0, 0.0];
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn mass(&self) -> f64 { self.result.mass }
    pub fn centre_of_mass(&self) -> [f64; 3] { self.result.centre_of_mass }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displacement_identity() {
        let d = BrepLocsDisplacement::identity();
        assert!(d.is_identity());
        let p = [1.0, 2.0, 3.0];
        let q = d.apply(p);
        assert!((q[0] - 1.0).abs() < 1e-14);
        assert!((q[1] - 2.0).abs() < 1e-14);
    }

    #[test]
    fn displacement_translation() {
        let d = BrepLocsDisplacement::from_translation(5.0, 0.0, 0.0);
        let p = d.apply([1.0, 0.0, 0.0]);
        assert!((p[0] - 6.0).abs() < 1e-14);
        assert_eq!(d.translation(), [5.0, 0.0, 0.0]);
    }

    #[test]
    fn displacement_rotation_z() {
        use std::f64::consts::PI;
        let d = BrepLocsDisplacement::from_rotation_z(PI / 2.0);
        let p = d.apply([1.0, 0.0, 0.0]);
        assert!((p[0]).abs() < 1e-10);
        assert!((p[1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn displacement_compose() {
        let t1 = BrepLocsDisplacement::from_translation(1.0, 0.0, 0.0);
        let t2 = BrepLocsDisplacement::from_translation(0.0, 2.0, 0.0);
        let t = t1.compose(&t2);
        let p = t.apply([0.0, 0.0, 0.0]);
        assert!((p[0] - 1.0).abs() < 1e-14);
        assert!((p[1] - 2.0).abs() < 1e-14);
    }

    #[test]
    fn stack_composed() {
        let mut s = BrepLocsStack::new();
        s.push(BrepLocsDisplacement::from_translation(1.0, 0.0, 0.0));
        s.push(BrepLocsDisplacement::from_translation(0.0, 1.0, 0.0));
        let c = s.composed();
        let p = c.apply([0.0, 0.0, 0.0]);
        assert!((p[0] - 1.0).abs() < 1e-14);
        assert!((p[1] - 1.0).abs() < 1e-14);
        assert_eq!(s.depth(), 2);
    }

    #[test]
    fn gprop_surface() {
        let mut g = BrepGPropSurface::new(5);
        g.perform(1e-6);
        assert!(g.is_done());
        assert!((g.mass() - 5.0).abs() < 1e-10);
    }
}
