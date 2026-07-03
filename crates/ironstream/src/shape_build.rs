// FILE: rust/ironstream/crates/ironstream/src/shape_build.rs

use std::collections::HashMap;

// ---------------------------------------------------------------------------
// ShapeBuildEdgeError
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShapeBuildEdgeError {
    NoCurve3d,
    DegeneratedEdge,
    ClosedEdge,
    NoPCurve,
}

// ---------------------------------------------------------------------------
// ShapeBuildEdge
// ---------------------------------------------------------------------------

// occt: ShapeBuild_Edge
pub struct ShapeBuildEdge {
    tolerance: f64,
    same_parameter: bool,
    has_3d_curve: bool,
    nb_pcurves: u32,
}

impl ShapeBuildEdge {
    pub fn new() -> Self {
        Self {
            tolerance: 1e-7,
            same_parameter: true,
            has_3d_curve: false,
            nb_pcurves: 0,
        }
    }

    pub fn set_tolerance(&mut self, t: f64) {
        self.tolerance = t;
    }

    pub fn set_same_parameter(&mut self, v: bool) {
        self.same_parameter = v;
    }

    pub fn add_pcurve(&mut self) {
        self.nb_pcurves += 1;
    }

    pub fn nb_pcurves(&self) -> u32 {
        self.nb_pcurves
    }

    pub fn has_3d_curve(&self) -> bool {
        self.has_3d_curve
    }

    pub fn set_has_3d_curve(&mut self, v: bool) {
        self.has_3d_curve = v;
    }

    pub fn validate(&self) -> Result<(), ShapeBuildEdgeError> {
        if self.nb_pcurves == 0 && !self.has_3d_curve {
            Err(ShapeBuildEdgeError::NoCurve3d)
        } else {
            Ok(())
        }
    }
}

impl Default for ShapeBuildEdge {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// ShapeBuildVertex
// ---------------------------------------------------------------------------

// occt: ShapeBuild_Vertex
pub struct ShapeBuildVertex {
    pub tolerance: f64,
    pub point: [f64; 3],
}

impl ShapeBuildVertex {
    pub fn new(x: f64, y: f64, z: f64, tolerance: f64) -> Self {
        Self {
            tolerance,
            point: [x, y, z],
        }
    }

    pub fn set_tolerance(&mut self, t: f64) {
        self.tolerance = t;
    }

    pub fn set_point(&mut self, x: f64, y: f64, z: f64) {
        self.point = [x, y, z];
    }

    pub fn distance(&self, other: &ShapeBuildVertex) -> f64 {
        let dx = self.point[0] - other.point[0];
        let dy = self.point[1] - other.point[1];
        let dz = self.point[2] - other.point[2];
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

// ---------------------------------------------------------------------------
// ShapeBuildWire
// ---------------------------------------------------------------------------

// occt: ShapeBuild_Wire
pub struct ShapeBuildWire {
    edges: Vec<u32>,
    is_closed: bool,
    tolerance: f64,
}

impl ShapeBuildWire {
    pub fn new(tolerance: f64) -> Self {
        Self {
            edges: Vec::new(),
            is_closed: false,
            tolerance,
        }
    }

    pub fn add_edge(&mut self, edge_id: u32) {
        self.edges.push(edge_id);
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    pub fn nb_edges(&self) -> usize {
        self.edges.len()
    }

    pub fn edge(&self, idx: usize) -> Option<u32> {
        self.edges.get(idx).copied()
    }

    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }
}

// ---------------------------------------------------------------------------
// ShapeBuildReShape
// ---------------------------------------------------------------------------

// occt: ShapeBuild_ReShape
pub struct ShapeBuildReShape {
    replacements: HashMap<u32, u32>,
}

impl ShapeBuildReShape {
    pub fn new() -> Self {
        Self {
            replacements: HashMap::new(),
        }
    }

    pub fn replace(&mut self, old_id: u32, new_id: u32) {
        self.replacements.insert(old_id, new_id);
    }

    pub fn remove(&mut self, id: u32) {
        self.replacements.insert(id, u32::MAX);
    }

    pub fn apply(&self, id: u32) -> Option<u32> {
        match self.replacements.get(&id) {
            Some(&u32::MAX) => None,
            Some(&new_id) => Some(new_id),
            None => Some(id),
        }
    }

    pub fn nb_replacements(&self) -> usize {
        self.replacements.len()
    }

    pub fn is_removed(&self, id: u32) -> bool {
        self.replacements.get(&id) == Some(&u32::MAX)
    }
}

impl Default for ShapeBuildReShape {
    fn default() -> Self {
        Self::new()
    }
}
