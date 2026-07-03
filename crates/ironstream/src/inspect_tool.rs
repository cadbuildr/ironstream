// FILE: inspect_tool.rs
// occt: BRep_Tool (curve/surface/point access utilities), BRepTools inspector

/// Classification of a curve's geometric type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurveGeomType {
    Line,
    Circle,
    Ellipse,
    Hyperbola,
    Parabola,
    BezierCurve,
    BSplineCurve,
    OffsetCurve,
    OtherCurve,
}

impl Default for CurveGeomType {
    fn default() -> Self { Self::OtherCurve }
}

/// Classification of a surface's geometric type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceGeomType {
    Plane,
    Cylinder,
    Cone,
    Sphere,
    Torus,
    BezierSurface,
    BSplineSurface,
    RevolutionSurface,
    ExtrusionSurface,
    OffsetSurface,
    OtherSurface,
}

impl Default for SurfaceGeomType {
    fn default() -> Self { Self::OtherSurface }
}

// occt: BRep_Tool face data — surface + tolerance
#[derive(Clone, Debug, Default)]
pub struct BrepToolFace {
    pub face_id: u32,
    pub surface_id: u32,
    pub surface_type: SurfaceGeomType,
    pub tolerance: f64,
    pub natural_restriction: bool,
    pub u_range: [f64; 2],
    pub v_range: [f64; 2],
}

impl BrepToolFace {
    pub fn new(face_id: u32, surface_id: u32, surface_type: SurfaceGeomType) -> Self {
        Self {
            face_id,
            surface_id,
            surface_type,
            tolerance: 1e-7,
            natural_restriction: false,
            u_range: [0.0, 1.0],
            v_range: [0.0, 1.0],
        }
    }

    pub fn surface(&self) -> u32 { self.surface_id }
    pub fn tolerance(&self) -> f64 { self.tolerance }
    pub fn natural_restriction(&self) -> bool { self.natural_restriction }
    pub fn parameter_range(&self) -> ([f64; 2], [f64; 2]) { (self.u_range, self.v_range) }
    pub fn set_tolerance(&mut self, tol: f64) { self.tolerance = tol; }
}

// occt: BRep_Tool edge data — 3D curve + tolerance + flags
#[derive(Clone, Debug, Default)]
pub struct BrepToolEdge {
    pub edge_id: u32,
    pub curve_id: u32,
    pub curve_type: CurveGeomType,
    pub tolerance: f64,
    pub is_degenerated: bool,
    pub same_parameter: bool,
    pub same_range: bool,
    pub first_param: f64,
    pub last_param: f64,
    pub curve_on_surface: Option<(u32, u32)>,   // (pcurve_id, face_id)
}

impl BrepToolEdge {
    pub fn new(edge_id: u32, curve_id: u32, curve_type: CurveGeomType) -> Self {
        Self {
            edge_id,
            curve_id,
            curve_type,
            tolerance: 1e-7,
            same_parameter: true,
            same_range: true,
            first_param: 0.0,
            last_param: 1.0,
            ..Default::default()
        }
    }

    pub fn curve(&self) -> u32 { self.curve_id }
    pub fn curve_type(&self) -> CurveGeomType { self.curve_type }
    pub fn tolerance(&self) -> f64 { self.tolerance }
    pub fn is_degenerated(&self) -> bool { self.is_degenerated }
    pub fn same_parameter(&self) -> bool { self.same_parameter }
    pub fn same_range(&self) -> bool { self.same_range }
    pub fn range(&self) -> (f64, f64) { (self.first_param, self.last_param) }
    pub fn length_estimate(&self) -> f64 { (self.last_param - self.first_param).abs() }

    pub fn curve_on_surface(&self) -> Option<(u32, u32)> { self.curve_on_surface }
    pub fn has_curve_on_surface(&self) -> bool { self.curve_on_surface.is_some() }
    pub fn set_tolerance(&mut self, tol: f64) { self.tolerance = tol; }
}

// occt: BRep_Tool vertex data — 3D point + tolerance
#[derive(Clone, Debug, Default)]
pub struct BrepToolVertex {
    pub vertex_id: u32,
    pub point: [f64; 3],
    pub tolerance: f64,
}

impl BrepToolVertex {
    pub fn new(vertex_id: u32, point: [f64; 3]) -> Self {
        Self { vertex_id, point, tolerance: 1e-7 }
    }

    pub fn pnt(&self) -> [f64; 3] { self.point }
    pub fn tolerance(&self) -> f64 { self.tolerance }

    pub fn distance_to(&self, other: &BrepToolVertex) -> f64 {
        let d = [
            self.point[0]-other.point[0],
            self.point[1]-other.point[1],
            self.point[2]-other.point[2],
        ];
        (d[0]*d[0]+d[1]*d[1]+d[2]*d[2]).sqrt()
    }
}

// occt: BRepTools inspector — aggregates face/edge/vertex info for a shape
#[derive(Clone, Debug, Default)]
pub struct BrepInspector {
    pub shape_id: u32,
    pub faces: Vec<BrepToolFace>,
    pub edges: Vec<BrepToolEdge>,
    pub vertices: Vec<BrepToolVertex>,
}

impl BrepInspector {
    pub fn new(shape_id: u32) -> Self { Self { shape_id, ..Default::default() } }

    pub fn add_face(&mut self, f: BrepToolFace) { self.faces.push(f); }
    pub fn add_edge(&mut self, e: BrepToolEdge) { self.edges.push(e); }
    pub fn add_vertex(&mut self, v: BrepToolVertex) { self.vertices.push(v); }

    pub fn face(&self, id: u32) -> Option<&BrepToolFace> { self.faces.iter().find(|f| f.face_id == id) }
    pub fn edge(&self, id: u32) -> Option<&BrepToolEdge> { self.edges.iter().find(|e| e.edge_id == id) }
    pub fn vertex(&self, id: u32) -> Option<&BrepToolVertex> { self.vertices.iter().find(|v| v.vertex_id == id) }

    pub fn face_count(&self) -> usize { self.faces.len() }
    pub fn edge_count(&self) -> usize { self.edges.len() }
    pub fn vertex_count(&self) -> usize { self.vertices.len() }

    pub fn degenerated_edges(&self) -> Vec<u32> {
        self.edges.iter().filter(|e| e.is_degenerated).map(|e| e.edge_id).collect()
    }

    pub fn max_tolerance(&self) -> f64 {
        let ft = self.faces.iter().map(|f| f.tolerance).fold(0.0_f64, f64::max);
        let et = self.edges.iter().map(|e| e.tolerance).fold(0.0_f64, f64::max);
        let vt = self.vertices.iter().map(|v| v.tolerance).fold(0.0_f64, f64::max);
        ft.max(et).max(vt)
    }

    pub fn faces_of_type(&self, surface_type: SurfaceGeomType) -> Vec<u32> {
        self.faces.iter().filter(|f| f.surface_type == surface_type).map(|f| f.face_id).collect()
    }

    pub fn euler_characteristic(&self) -> i64 {
        self.vertex_count() as i64 - self.edge_count() as i64 + self.face_count() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brep_tool_face() {
        let mut f = BrepToolFace::new(1, 10, SurfaceGeomType::Plane);
        f.set_tolerance(1e-6);
        assert_eq!(f.surface(), 10);
        assert!((f.tolerance() - 1e-6).abs() < 1e-10);
        assert!(!f.natural_restriction());
    }

    #[test]
    fn brep_tool_edge() {
        let mut e = BrepToolEdge::new(2, 20, CurveGeomType::Line);
        e.is_degenerated = true;
        assert!(e.is_degenerated());
        assert!(e.same_parameter());
        assert!((e.length_estimate() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn brep_tool_vertex_distance() {
        let v1 = BrepToolVertex::new(1, [0.0, 0.0, 0.0]);
        let v2 = BrepToolVertex::new(2, [3.0, 4.0, 0.0]);
        assert!((v1.distance_to(&v2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn brep_inspector_counts() {
        let mut insp = BrepInspector::new(1);
        insp.add_face(BrepToolFace::new(1, 10, SurfaceGeomType::Plane));
        insp.add_face(BrepToolFace::new(2, 11, SurfaceGeomType::Cylinder));
        insp.add_edge(BrepToolEdge::new(1, 20, CurveGeomType::Line));
        insp.add_vertex(BrepToolVertex::new(1, [0.0, 0.0, 0.0]));
        assert_eq!(insp.face_count(), 2);
        assert_eq!(insp.edge_count(), 1);
        assert_eq!(insp.vertex_count(), 1);
        assert_eq!(insp.euler_characteristic(), 2); // 1 - 1 + 2
    }

    #[test]
    fn brep_inspector_max_tolerance() {
        let mut insp = BrepInspector::new(1);
        let mut f = BrepToolFace::new(1, 10, SurfaceGeomType::Sphere);
        f.set_tolerance(1e-3);
        insp.add_face(f);
        insp.add_vertex(BrepToolVertex::new(1, [0.0, 0.0, 0.0]));
        assert!((insp.max_tolerance() - 1e-3).abs() < 1e-10);
    }

    #[test]
    fn brep_inspector_faces_of_type() {
        let mut insp = BrepInspector::new(1);
        insp.add_face(BrepToolFace::new(1, 10, SurfaceGeomType::Plane));
        insp.add_face(BrepToolFace::new(2, 11, SurfaceGeomType::Plane));
        insp.add_face(BrepToolFace::new(3, 12, SurfaceGeomType::Sphere));
        let planes = insp.faces_of_type(SurfaceGeomType::Plane);
        assert_eq!(planes.len(), 2);
    }
}
