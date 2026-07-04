// FILE: t_data_xtd_triangulation.rs
// occt: TDataXtd_Triangulation

use std::collections::VecDeque;

/// GUID for TDataXtd_Triangulation attribute.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StandardGUID {
    data: [u8; 16],
}

impl StandardGUID {
    pub fn new(data: [u8; 16]) -> Self {
        StandardGUID { data }
    }

    pub fn get_id() -> Self {
        // "27AE2C44-60B0-41AE-AC18-BA3FDA538D03"
        let bytes: [u8; 16] = [0x27, 0xae, 0x2c, 0x44, 0x60, 0xb0, 0x41, 0xae,
                               0xac, 0x18, 0xba, 0x3f, 0xda, 0x53, 0x8d, 0x03];
        StandardGUID { data: bytes }
    }
}

/// 3D point representation (gp_Pnt placeholder).
#[derive(Clone, Debug, Copy, Default, PartialEq)]
pub struct GpPnt {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl GpPnt {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        GpPnt { x, y, z }
    }
}

/// 2D point representation (gp_Pnt2d placeholder).
#[derive(Clone, Debug, Copy, Default, PartialEq)]
pub struct GpPnt2d {
    pub x: f64,
    pub y: f64,
}

impl GpPnt2d {
    pub fn new(x: f64, y: f64) -> Self {
        GpPnt2d { x, y }
    }
}

/// Direction vector (gp_Dir placeholder).
#[derive(Clone, Debug, Copy, Default, PartialEq)]
pub struct GpDir {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl GpDir {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        GpDir { x, y, z }
    }
}

/// Triangle representation (Poly_Triangle placeholder).
#[derive(Clone, Debug, Copy, Default, PartialEq)]
pub struct PolyTriangle {
    pub v1: i32,
    pub v2: i32,
    pub v3: i32,
}

impl PolyTriangle {
    pub fn new(v1: i32, v2: i32, v3: i32) -> Self {
        PolyTriangle { v1, v2, v3 }
    }
}

/// A triangulation (mesh) data structure.
/// Mirrors OCCT's Poly_Triangulation.
#[derive(Clone, Debug)]
pub struct PolyTriangulation {
    nodes: Vec<GpPnt>,
    uv_nodes: Vec<GpPnt2d>,
    triangles: Vec<PolyTriangle>,
    normals: Vec<GpDir>,
    has_uv_nodes: bool,
    has_normals: bool,
    deflection: f64,
}

impl Default for PolyTriangulation {
    fn default() -> Self {
        PolyTriangulation {
            nodes: Vec::new(),
            uv_nodes: Vec::new(),
            triangles: Vec::new(),
            normals: Vec::new(),
            has_uv_nodes: false,
            has_normals: false,
            deflection: 0.0,
        }
    }
}

impl PolyTriangulation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_size(nb_nodes: usize, nb_triangles: usize) -> Self {
        PolyTriangulation {
            nodes: vec![GpPnt::default(); nb_nodes],
            uv_nodes: Vec::new(),
            triangles: vec![PolyTriangle::default(); nb_triangles],
            normals: Vec::new(),
            has_uv_nodes: false,
            has_normals: false,
            deflection: 0.0,
        }
    }

    pub fn nb_nodes(&self) -> i32 {
        self.nodes.len() as i32
    }

    pub fn nb_triangles(&self) -> i32 {
        self.triangles.len() as i32
    }

    pub fn node(&self, index: usize) -> Option<GpPnt> {
        if index > 0 && index <= self.nodes.len() {
            Some(self.nodes[index - 1])
        } else {
            None
        }
    }

    pub fn set_node(&mut self, index: usize, point: GpPnt) {
        if index > 0 && index <= self.nodes.len() {
            self.nodes[index - 1] = point;
        }
    }

    pub fn has_uv_nodes(&self) -> bool {
        self.has_uv_nodes
    }

    pub fn uv_node(&self, index: usize) -> Option<GpPnt2d> {
        if index > 0 && index <= self.uv_nodes.len() {
            Some(self.uv_nodes[index - 1])
        } else {
            None
        }
    }

    pub fn set_uv_node(&mut self, index: usize, point: GpPnt2d) {
        if index > 0 && index <= self.uv_nodes.len() {
            self.uv_nodes[index - 1] = point;
        }
    }

    pub fn remove_uv_nodes(&mut self) {
        self.uv_nodes.clear();
        self.has_uv_nodes = false;
    }

    pub fn triangle(&self, index: usize) -> Option<PolyTriangle> {
        if index > 0 && index <= self.triangles.len() {
            Some(self.triangles[index - 1])
        } else {
            None
        }
    }

    pub fn set_triangle(&mut self, index: usize, tri: PolyTriangle) {
        if index > 0 && index <= self.triangles.len() {
            self.triangles[index - 1] = tri;
        }
    }

    pub fn has_normals(&self) -> bool {
        self.has_normals
    }

    pub fn normal(&self, index: usize) -> Option<GpDir> {
        if index > 0 && index <= self.normals.len() {
            Some(self.normals[index - 1])
        } else {
            None
        }
    }

    pub fn set_normal(&mut self, index: usize, dir: GpDir) {
        if index > 0 && index <= self.normals.len() {
            self.normals[index - 1] = dir;
        }
    }

    pub fn deflection(&self) -> f64 {
        self.deflection
    }

    pub fn set_deflection(&mut self, deflection: f64) {
        self.deflection = deflection;
    }

    pub fn copy(&self) -> Self {
        self.clone()
    }
}

/// A triangulation attribute for OCAF.
/// Wraps a Poly_Triangulation to prevent direct modification and enable undo/redo.
#[derive(Clone, Debug, Default)]
pub struct TDataXtdTriangulation {
    triangulation: Option<PolyTriangulation>,
}

impl TDataXtdTriangulation {
    pub fn new() -> Self {
        TDataXtdTriangulation {
            triangulation: None,
        }
    }

    pub fn get_id() -> StandardGUID {
        StandardGUID::get_id()
    }

    pub fn id(&self) -> StandardGUID {
        Self::get_id()
    }

    pub fn set(&mut self, triangulation: PolyTriangulation) {
        self.triangulation = Some(triangulation);
    }

    pub fn get(&self) -> Option<&PolyTriangulation> {
        self.triangulation.as_ref()
    }

    pub fn get_mut(&mut self) -> Option<&mut PolyTriangulation> {
        self.triangulation.as_mut()
    }

    pub fn deflection(&self) -> f64 {
        self.triangulation
            .as_ref()
            .map(|t| t.deflection())
            .unwrap_or(0.0)
    }

    pub fn set_deflection(&mut self, deflection: f64) {
        if let Some(tri) = &mut self.triangulation {
            tri.set_deflection(deflection);
        }
    }

    pub fn remove_uv_nodes(&mut self) {
        if let Some(tri) = &mut self.triangulation {
            tri.remove_uv_nodes();
        }
    }

    pub fn nb_nodes(&self) -> i32 {
        self.triangulation
            .as_ref()
            .map(|t| t.nb_nodes())
            .unwrap_or(0)
    }

    pub fn nb_triangles(&self) -> i32 {
        self.triangulation
            .as_ref()
            .map(|t| t.nb_triangles())
            .unwrap_or(0)
    }

    pub fn has_uv_nodes(&self) -> bool {
        self.triangulation
            .as_ref()
            .map(|t| t.has_uv_nodes())
            .unwrap_or(false)
    }

    pub fn node(&self, index: usize) -> Option<GpPnt> {
        self.triangulation.as_ref().and_then(|t| t.node(index))
    }

    pub fn set_node(&mut self, index: usize, point: GpPnt) {
        if let Some(tri) = &mut self.triangulation {
            tri.set_node(index, point);
        }
    }

    pub fn uv_node(&self, index: usize) -> Option<GpPnt2d> {
        self.triangulation.as_ref().and_then(|t| t.uv_node(index))
    }

    pub fn set_uv_node(&mut self, index: usize, point: GpPnt2d) {
        if let Some(tri) = &mut self.triangulation {
            tri.set_uv_node(index, point);
        }
    }

    pub fn triangle(&self, index: usize) -> Option<PolyTriangle> {
        self.triangulation.as_ref().and_then(|t| t.triangle(index))
    }

    pub fn set_triangle(&mut self, index: usize, tri: PolyTriangle) {
        if let Some(triangulation) = &mut self.triangulation {
            triangulation.set_triangle(index, tri);
        }
    }

    pub fn has_normals(&self) -> bool {
        self.triangulation
            .as_ref()
            .map(|t| t.has_normals())
            .unwrap_or(false)
    }

    pub fn normal(&self, index: usize) -> Option<GpDir> {
        self.triangulation.as_ref().and_then(|t| t.normal(index))
    }

    pub fn set_normal(&mut self, index: usize, dir: GpDir) {
        if let Some(tri) = &mut self.triangulation {
            tri.set_normal(index, dir);
        }
    }

    pub fn dump(&self) -> String {
        "Triangulation".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangulation_create() {
        let tri = PolyTriangulation::with_size(3, 1);
        assert_eq!(tri.nb_nodes(), 3);
        assert_eq!(tri.nb_triangles(), 1);
    }

    #[test]
    fn test_triangulation_nodes() {
        let mut tri = PolyTriangulation::with_size(3, 1);
        let p = GpPnt::new(1.0, 2.0, 3.0);
        tri.set_node(1, p);
        assert_eq!(tri.node(1), Some(p));
    }

    #[test]
    fn test_triangulation_triangles() {
        let mut tri = PolyTriangulation::with_size(3, 1);
        let t = PolyTriangle::new(1, 2, 3);
        tri.set_triangle(1, t);
        assert_eq!(tri.triangle(1), Some(t));
    }

    #[test]
    fn test_triangulation_deflection() {
        let mut tri = PolyTriangulation::new();
        tri.set_deflection(0.01);
        assert!((tri.deflection() - 0.01).abs() < 1e-10);
    }

    #[test]
    fn test_ocaf_triangulation() {
        let mut attr = TDataXtdTriangulation::new();
        let tri = PolyTriangulation::with_size(3, 1);
        attr.set(tri);
        assert_eq!(attr.nb_nodes(), 3);
        assert_eq!(attr.nb_triangles(), 1);
    }

    #[test]
    fn test_ocaf_get_id() {
        let id = TDataXtdTriangulation::get_id();
        let id2 = TDataXtdTriangulation::get_id();
        assert_eq!(id, id2);
    }

    #[test]
    fn test_ocaf_dump() {
        let attr = TDataXtdTriangulation::new();
        assert_eq!(attr.dump(), "Triangulation");
    }
}
