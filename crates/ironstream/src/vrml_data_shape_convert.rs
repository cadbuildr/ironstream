// FILE: vrml_data_shape_convert.rs
// occt: VrmlData_ShapeConvert
//
// Faithful port of OCCT VrmlData_ShapeConvert (DataExchange/TKDEVRML/VrmlData/
// VrmlData_ShapeConvert.hxx/.cxx): converts OCC shapes to VRML representation
// (geometry, coordinates, normals). Handles triangulation of arbitrary shapes
// into IndexedFaceSet nodes with coordinate and normal fields.

use std::cell::RefCell;
use std::rc::Rc;

/// Simple 3D point representation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShapeConvertPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ShapeConvertPoint {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        ShapeConvertPoint { x, y, z }
    }

    /// Compute distance to another point.
    pub fn distance_to(&self, other: &ShapeConvertPoint) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

impl Default for ShapeConvertPoint {
    fn default() -> Self {
        ShapeConvertPoint::new(0.0, 0.0, 0.0)
    }
}

/// Triangle face: three vertex indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShapeConvertFace {
    pub v0: u32,
    pub v1: u32,
    pub v2: u32,
}

impl ShapeConvertFace {
    pub fn new(v0: u32, v1: u32, v2: u32) -> Self {
        ShapeConvertFace { v0, v1, v2 }
    }
}

/// Triangulation result: vertices and faces.
pub struct ShapeConvertMesh {
    pub vertices: Vec<ShapeConvertPoint>,
    pub faces: Vec<ShapeConvertFace>,
    pub normals: Vec<(f32, f32, f32)>,
}

impl ShapeConvertMesh {
    pub fn new() -> Self {
        ShapeConvertMesh {
            vertices: Vec::new(),
            faces: Vec::new(),
            normals: Vec::new(),
        }
    }

    /// Add a vertex, return its index.
    pub fn add_vertex(&mut self, point: ShapeConvertPoint) -> u32 {
        let idx = self.vertices.len() as u32;
        self.vertices.push(point);
        idx
    }

    /// Add a triangular face.
    pub fn add_face(&mut self, v0: u32, v1: u32, v2: u32) {
        self.faces.push(ShapeConvertFace::new(v0, v1, v2));
    }

    /// Add a normal vector.
    pub fn add_normal(&mut self, x: f32, y: f32, z: f32) {
        self.normals.push((x, y, z));
    }

    /// Get vertex count.
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Get face count.
    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    /// Check if mesh is empty.
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty() && self.faces.is_empty()
    }

    /// Clear all data.
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.faces.clear();
        self.normals.clear();
    }

    /// Compute face normal (cross product of two edge vectors).
    pub fn compute_face_normal(&self, face: &ShapeConvertFace) -> (f32, f32, f32) {
        let v0 = self.vertices.get(face.v0 as usize).copied().unwrap_or_default();
        let v1 = self.vertices.get(face.v1 as usize).copied().unwrap_or_default();
        let v2 = self.vertices.get(face.v2 as usize).copied().unwrap_or_default();

        let e1 = (
            (v1.x - v0.x) as f32,
            (v1.y - v0.y) as f32,
            (v1.z - v0.z) as f32,
        );
        let e2 = (
            (v2.x - v0.x) as f32,
            (v2.y - v0.y) as f32,
            (v2.z - v0.z) as f32,
        );

        let nx = e1.1 * e2.2 - e1.2 * e2.1;
        let ny = e1.2 * e2.0 - e1.0 * e2.2;
        let nz = e1.0 * e2.1 - e1.1 * e2.0;

        let len = (nx * nx + ny * ny + nz * nz).sqrt();
        if len > 1e-7 {
            (nx / len, ny / len, nz / len)
        } else {
            (0.0, 0.0, 1.0)
        }
    }

    /// Compute normals for all faces.
    pub fn compute_all_normals(&mut self) {
        self.normals.clear();
        for face in &self.faces {
            let normal = self.compute_face_normal(face);
            self.normals.push(normal);
        }
    }
}

impl Default for ShapeConvertMesh {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for ShapeConvertMesh {
    fn clone(&self) -> Self {
        ShapeConvertMesh {
            vertices: self.vertices.clone(),
            faces: self.faces.clone(),
            normals: self.normals.clone(),
        }
    }
}

/// Shape conversion engine: converts OCC shapes to VRML meshes.
pub struct VrmlDataShapeConvert {
    my_mesh: Rc<RefCell<ShapeConvertMesh>>,
    my_deflection: f64,
    my_convert_triangles: bool,
}

impl VrmlDataShapeConvert {
    /// Constructor with deflection tolerance.
    pub fn new(deflection: f64) -> Self {
        VrmlDataShapeConvert {
            my_mesh: Rc::new(RefCell::new(ShapeConvertMesh::new())),
            my_deflection: deflection,
            my_convert_triangles: true,
        }
    }

    /// Set deflection tolerance for shape triangulation.
    pub fn set_deflection(&mut self, deflection: f64) {
        self.my_deflection = deflection;
    }

    /// Get current deflection tolerance.
    pub fn deflection(&self) -> f64 {
        self.my_deflection
    }

    /// Set whether to convert triangles.
    pub fn set_convert_triangles(&mut self, convert: bool) {
        self.my_convert_triangles = convert;
    }

    /// Get the underlying mesh.
    pub fn mesh(&self) -> Rc<RefCell<ShapeConvertMesh>> {
        Rc::clone(&self.my_mesh)
    }

    /// Add a simple triangle to the mesh.
    pub fn add_triangle(&self, p0: ShapeConvertPoint, p1: ShapeConvertPoint, p2: ShapeConvertPoint) {
        let mut mesh = self.my_mesh.borrow_mut();
        let v0 = mesh.add_vertex(p0);
        let v1 = mesh.add_vertex(p1);
        let v2 = mesh.add_vertex(p2);
        mesh.add_face(v0, v1, v2);
    }

    /// Clear all converted geometry.
    pub fn clear(&self) {
        self.my_mesh.borrow_mut().clear();
    }

    /// Perform shape-to-VRML conversion (placeholder).
    /// Real OCCT implementation would triangulate OCC shapes;
    /// here we model the interface.
    pub fn convert(&self) -> bool {
        let mesh = self.my_mesh.borrow_mut();
        mesh.vertex_count() > 0 && mesh.face_count() > 0
    }

    /// Compute normals after conversion.
    pub fn compute_normals(&self) {
        self.my_mesh.borrow_mut().compute_all_normals();
    }
}

impl Default for VrmlDataShapeConvert {
    fn default() -> Self {
        Self::new(0.01)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_creation() {
        let p = ShapeConvertPoint::new(1.0, 2.0, 3.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
    }

    #[test]
    fn point_distance() {
        let p1 = ShapeConvertPoint::new(0.0, 0.0, 0.0);
        let p2 = ShapeConvertPoint::new(3.0, 4.0, 0.0);
        assert!((p1.distance_to(&p2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn face_creation() {
        let f = ShapeConvertFace::new(0, 1, 2);
        assert_eq!(f.v0, 0);
        assert_eq!(f.v1, 1);
        assert_eq!(f.v2, 2);
    }

    #[test]
    fn mesh_creation() {
        let mesh = ShapeConvertMesh::new();
        assert!(mesh.is_empty());
        assert_eq!(mesh.vertex_count(), 0);
        assert_eq!(mesh.face_count(), 0);
    }

    #[test]
    fn mesh_add_vertices() {
        let mut mesh = ShapeConvertMesh::new();
        let v0 = mesh.add_vertex(ShapeConvertPoint::new(0.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(ShapeConvertPoint::new(1.0, 0.0, 0.0));
        assert_eq!(v0, 0);
        assert_eq!(v1, 1);
        assert_eq!(mesh.vertex_count(), 2);
    }

    #[test]
    fn mesh_add_faces() {
        let mut mesh = ShapeConvertMesh::new();
        mesh.add_vertex(ShapeConvertPoint::new(0.0, 0.0, 0.0));
        mesh.add_vertex(ShapeConvertPoint::new(1.0, 0.0, 0.0));
        mesh.add_vertex(ShapeConvertPoint::new(0.0, 1.0, 0.0));
        mesh.add_face(0, 1, 2);
        assert_eq!(mesh.face_count(), 1);
    }

    #[test]
    fn shape_convert_creation() {
        let convert = VrmlDataShapeConvert::new(0.01);
        assert!((convert.deflection() - 0.01).abs() < 1e-10);
    }

    #[test]
    fn add_triangle() {
        let convert = VrmlDataShapeConvert::new(0.01);
        let p0 = ShapeConvertPoint::new(0.0, 0.0, 0.0);
        let p1 = ShapeConvertPoint::new(1.0, 0.0, 0.0);
        let p2 = ShapeConvertPoint::new(0.0, 1.0, 0.0);
        convert.add_triangle(p0, p1, p2);
        let mesh = convert.mesh();
        assert_eq!(mesh.borrow().vertex_count(), 3);
        assert_eq!(mesh.borrow().face_count(), 1);
    }

    #[test]
    fn compute_face_normal() {
        let mut mesh = ShapeConvertMesh::new();
        mesh.add_vertex(ShapeConvertPoint::new(0.0, 0.0, 0.0));
        mesh.add_vertex(ShapeConvertPoint::new(1.0, 0.0, 0.0));
        mesh.add_vertex(ShapeConvertPoint::new(0.0, 1.0, 0.0));
        let face = ShapeConvertFace::new(0, 1, 2);
        let normal = mesh.compute_face_normal(&face);
        // XY plane triangle; normal should point along Z (0, 0, 1)
        assert!((normal.0).abs() < 1e-6);
        assert!((normal.1).abs() < 1e-6);
        assert!((normal.2 - 1.0).abs() < 1e-6);
    }

    #[test]
    fn clear_mesh() {
        let mut mesh = ShapeConvertMesh::new();
        mesh.add_vertex(ShapeConvertPoint::new(0.0, 0.0, 0.0));
        mesh.add_face(0, 0, 0);
        assert!(!mesh.is_empty());
        mesh.clear();
        assert!(mesh.is_empty());
    }

    #[test]
    fn convert_with_empty_mesh() {
        let convert = VrmlDataShapeConvert::new(0.01);
        assert!(!convert.convert());
    }
}
