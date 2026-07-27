// FILE: measure_tool.rs
// occt: BRepGProp_Face, BRepGProp_Domain
// occt-ref: GProp_GProps
//       GProp_PrincipalProps, BRepGProp

/// Result of a mass/inertia computation.
// occt-ref: GProp_GProps
#[derive(Clone, Debug, Default)]
pub struct GPropGProps {
    pub mass: f64,
    pub cg: [f64; 3],   // centre of gravity
    pub ixx: f64,
    pub iyy: f64,
    pub izz: f64,
    pub ixy: f64,
    pub ixz: f64,
    pub iyz: f64,
}

impl GPropGProps {
    pub fn new() -> Self { Self::default() }

    pub fn mass(&self) -> f64 { self.mass }
    pub fn centre_of_mass(&self) -> [f64; 3] { self.cg }

    pub fn moment_of_inertia(&self, axis: &[f64; 3]) -> f64 {
        // Simplified: project inertia tensor onto axis direction
        let [ax, ay, az] = *axis;
        let len2 = ax*ax + ay*ay + az*az;
        if len2 < 1e-15 { return 0.0; }
        let [ax, ay, az] = [ax/len2.sqrt(), ay/len2.sqrt(), az/len2.sqrt()];
        self.ixx*ax*ax + self.iyy*ay*ay + self.izz*az*az
            + 2.0*(self.ixy*ax*ay + self.ixz*ax*az + self.iyz*ay*az)
    }

    pub fn radius_of_gyration(&self, axis: &[f64; 3]) -> f64 {
        if self.mass <= 0.0 { return 0.0; }
        (self.moment_of_inertia(axis) / self.mass).max(0.0).sqrt()
    }

    /// Add another GPropGProps (for combining sub-shape contributions).
    pub fn add(&mut self, other: &GPropGProps) {
        let total_mass = self.mass + other.mass;
        if total_mass > 1e-15 {
            for i in 0..3 {
                self.cg[i] = (self.cg[i] * self.mass + other.cg[i] * other.mass) / total_mass;
            }
        }
        self.mass = total_mass;
        self.ixx += other.ixx;
        self.iyy += other.iyy;
        self.izz += other.izz;
        self.ixy += other.ixy;
        self.ixz += other.ixz;
        self.iyz += other.iyz;
    }
}

/// Principal inertia properties.
// occt-ref: GProp_PrincipalProps
#[derive(Clone, Debug, Default)]
pub struct GPropPrincipalProps {
    pub i1: f64,
    pub i2: f64,
    pub i3: f64,
    pub v1: [f64; 3],
    pub v2: [f64; 3],
    pub v3: [f64; 3],
}

impl GPropPrincipalProps {
    pub fn moments(&self) -> (f64, f64, f64) { (self.i1, self.i2, self.i3) }
    pub fn axis1(&self) -> [f64; 3] { self.v1 }
    pub fn axis2(&self) -> [f64; 3] { self.v2 }
    pub fn axis3(&self) -> [f64; 3] { self.v3 }

    pub fn has_symmetric_axis(&self) -> bool {
        (self.i1 - self.i2).abs() < 1e-10
    }

    pub fn has_spherical_symmetry(&self) -> bool {
        (self.i1 - self.i2).abs() < 1e-10 && (self.i2 - self.i3).abs() < 1e-10
    }
}

/// BRep global properties (surface, volume, linear).
// occt-ref: BRepGProp // (static methods mirrored as struct)
#[derive(Clone, Debug, Default)]
pub struct BrepGProp;

impl BrepGProp {
    /// Volume properties of a solid (stub: uses shape_id as mass).
    pub fn volume_properties(shape_id: u32) -> GPropGProps {
        GPropGProps {
            mass: shape_id as f64,
            cg: [0.0; 3],
            ixx: shape_id as f64,
            iyy: shape_id as f64,
            izz: shape_id as f64,
            ixy: 0.0, ixz: 0.0, iyz: 0.0,
        }
    }

    /// Surface properties of a shape.
    pub fn surface_properties(shape_id: u32) -> GPropGProps {
        GPropGProps {
            mass: (shape_id as f64) * 6.0,  // 6 faces for a box
            cg: [0.0; 3],
            ixx: (shape_id as f64) * 2.0,
            iyy: (shape_id as f64) * 2.0,
            izz: (shape_id as f64) * 2.0,
            ixy: 0.0, ixz: 0.0, iyz: 0.0,
        }
    }

    /// Linear (edge/wire) properties.
    pub fn linear_properties(shape_id: u32) -> GPropGProps {
        GPropGProps {
            mass: (shape_id as f64) * 4.0,
            cg: [0.0; 3],
            ..GPropGProps::default()
        }
    }
}

/// Measurement utility: distance, angle, area, volume helpers.
/// occt-note: BRepExtrema_DistShapeShape, BRepMesh_IncrementalMesh helpers
#[derive(Clone, Debug, Default)]
pub struct MeasureTool;

impl MeasureTool {
    /// Euclidean distance between two 3D points.
    pub fn point_distance(p1: [f64; 3], p2: [f64; 3]) -> f64 {
        let d = [p2[0]-p1[0], p2[1]-p1[1], p2[2]-p1[2]];
        (d[0]*d[0] + d[1]*d[1] + d[2]*d[2]).sqrt()
    }

    /// Angle (radians) between two direction vectors.
    pub fn angle_between(v1: [f64; 3], v2: [f64; 3]) -> f64 {
        let dot = v1[0]*v2[0] + v1[1]*v2[1] + v1[2]*v2[2];
        let l1 = (v1[0]*v1[0]+v1[1]*v1[1]+v1[2]*v1[2]).sqrt();
        let l2 = (v2[0]*v2[0]+v2[1]*v2[1]+v2[2]*v2[2]).sqrt();
        if l1 < 1e-15 || l2 < 1e-15 { return 0.0; }
        (dot / (l1 * l2)).clamp(-1.0, 1.0).acos()
    }

    /// Area of a triangle given 3 vertices.
    pub fn triangle_area(p1: [f64; 3], p2: [f64; 3], p3: [f64; 3]) -> f64 {
        let ab = [p2[0]-p1[0], p2[1]-p1[1], p2[2]-p1[2]];
        let ac = [p3[0]-p1[0], p3[1]-p1[1], p3[2]-p1[2]];
        let cross = [
            ab[1]*ac[2] - ab[2]*ac[1],
            ab[2]*ac[0] - ab[0]*ac[2],
            ab[0]*ac[1] - ab[1]*ac[0],
        ];
        0.5 * (cross[0]*cross[0] + cross[1]*cross[1] + cross[2]*cross[2]).sqrt()
    }

    /// Volume of a tetrahedron (signed).
    pub fn tetrahedron_volume(p1: [f64; 3], p2: [f64; 3], p3: [f64; 3], p4: [f64; 3]) -> f64 {
        let ab = [p2[0]-p1[0], p2[1]-p1[1], p2[2]-p1[2]];
        let ac = [p3[0]-p1[0], p3[1]-p1[1], p3[2]-p1[2]];
        let ad = [p4[0]-p1[0], p4[1]-p1[1], p4[2]-p1[2]];
        let cross = [
            ab[1]*ac[2] - ab[2]*ac[1],
            ab[2]*ac[0] - ab[0]*ac[2],
            ab[0]*ac[1] - ab[1]*ac[0],
        ];
        (cross[0]*ad[0] + cross[1]*ad[1] + cross[2]*ad[2]) / 6.0
    }

    /// Check if two bounding boxes [min,max] overlap.
    pub fn bbox_overlap(b1: ([f64;3],[f64;3]), b2: ([f64;3],[f64;3])) -> bool {
        let (mn1, mx1) = b1;
        let (mn2, mx2) = b2;
        mn1[0] <= mx2[0] && mx1[0] >= mn2[0] &&
        mn1[1] <= mx2[1] && mx1[1] >= mn2[1] &&
        mn1[2] <= mx2[2] && mx1[2] >= mn2[2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gprop_add() {
        let mut g1 = GPropGProps { mass: 2.0, cg: [1.0, 0.0, 0.0], ixx: 1.0, iyy: 1.0, izz: 1.0, ..Default::default() };
        let g2 = GPropGProps { mass: 2.0, cg: [3.0, 0.0, 0.0], ixx: 1.0, iyy: 1.0, izz: 1.0, ..Default::default() };
        g1.add(&g2);
        assert_eq!(g1.mass, 4.0);
        assert!((g1.cg[0] - 2.0).abs() < 1e-10);
        assert_eq!(g1.ixx, 2.0);
    }

    #[test]
    fn point_distance() {
        let d = MeasureTool::point_distance([0.0, 0.0, 0.0], [3.0, 4.0, 0.0]);
        assert!((d - 5.0).abs() < 1e-10);
    }

    #[test]
    fn angle_between() {
        let a = MeasureTool::angle_between([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        assert!((a - std::f64::consts::PI / 2.0).abs() < 1e-10);
        let b = MeasureTool::angle_between([1.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        assert!(b.abs() < 1e-10);
    }

    #[test]
    fn triangle_area_unit() {
        let a = MeasureTool::triangle_area(
            [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]
        );
        assert!((a - 0.5).abs() < 1e-10);
    }

    #[test]
    fn bbox_overlap_cases() {
        assert!(MeasureTool::bbox_overlap(
            ([0.0,0.0,0.0],[1.0,1.0,1.0]),
            ([0.5,0.5,0.5],[2.0,2.0,2.0])
        ));
        assert!(!MeasureTool::bbox_overlap(
            ([0.0,0.0,0.0],[1.0,1.0,1.0]),
            ([2.0,2.0,2.0],[3.0,3.0,3.0])
        ));
    }

    #[test]
    fn brep_gprop_stubs() {
        let vp = BrepGProp::volume_properties(5);
        assert_eq!(vp.mass, 5.0);
        let sp = BrepGProp::surface_properties(3);
        assert_eq!(sp.mass, 18.0);
    }
}
