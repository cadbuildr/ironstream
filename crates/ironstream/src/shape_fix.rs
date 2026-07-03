// FILE: shape_fix.rs
//! `ShapeFix` — shape repair stubs mirroring OpenCascade's `ShapeFix` package.
//!
//! # Scope
//!
//! * [`ShapeFixer`] — top-level repair dispatcher; mirrors `ShapeFix_Shape`.
//! * [`FaceFixer`]  — face-level repairs; mirrors `ShapeFix_Face`.
//!
//! # Design
//!
//! All types use only `std`.  No external crates are required.  Shape and face
//! identities are represented as opaque `String` handles so this stub can be
//! used without a full topology kernel.

// ---------------------------------------------------------------------------
// ShapeFixParams
// ---------------------------------------------------------------------------

/// Tolerance parameters shared by all fixers.
///
/// Maps to the precision / tolerance triplet used throughout OCCT's
/// `ShapeFix` package.
#[derive(Debug, Clone, PartialEq)]
pub struct ShapeFixParams {
    /// Working precision: the primary coincidence tolerance.
    pub precision: f64,
    /// Lower bound on the tolerance that may be assigned to a shape.
    pub min_tolerance: f64,
    /// Upper bound on the tolerance that may be assigned to a shape.
    pub max_tolerance: f64,
}

impl ShapeFixParams {
    /// Create params with explicitly supplied values.
    pub fn new(precision: f64, min_tolerance: f64, max_tolerance: f64) -> Self {
        Self {
            precision,
            min_tolerance,
            max_tolerance,
        }
    }

    /// Sensible defaults: precision = 1e-7, min = 1e-7, max = 1.0.
    pub fn default() -> Self {
        Self {
            precision: 1.0e-7,
            min_tolerance: 1.0e-7,
            max_tolerance: 1.0,
        }
    }
}

// ---------------------------------------------------------------------------
// ShapeFixer  (occt: ShapeFix_Shape)
// ---------------------------------------------------------------------------

/// Top-level shape repair dispatcher.
///
/// Accepts an opaque shape handle, runs all available sub-fixers, and records
/// whether any repairs were made.
// occt: ShapeFix_Shape
pub struct ShapeFixer {
    /// Opaque handle identifying the shape being repaired.
    pub shape: String,
    /// Tolerance parameters used by all sub-fixers.
    pub params: ShapeFixParams,
    /// Set to `true` after a successful [`perform`](ShapeFixer::perform) call.
    pub fixed: bool,

    // Internal counters updated by perform().
    nb_faces_fixed: usize,
    nb_edges_fixed: usize,
}

impl ShapeFixer {
    /// Create a new fixer for the shape identified by `shape` using default
    /// tolerance parameters.
    pub fn new(shape: &str) -> Self {
        Self {
            shape: shape.to_owned(),
            params: ShapeFixParams::default(),
            fixed: false,
            nb_faces_fixed: 0,
            nb_edges_fixed: 0,
        }
    }

    /// Override the tolerance parameters and return `self` for chaining.
    pub fn with_params(mut self, p: ShapeFixParams) -> Self {
        self.params = p;
        self
    }

    /// Run all repair algorithms on the shape.
    ///
    /// Returns `true` when the shape is considered valid (either already was, or
    /// was repaired successfully).  Sets [`fixed`](ShapeFixer::fixed) to `true`.
    ///
    /// In this stub the shape string is non-empty, so `perform` always succeeds.
    pub fn perform(&mut self) -> bool {
        // Stub: a non-empty shape handle is treated as repairable.
        let ok = !self.shape.is_empty();

        if ok {
            // Stub counters — real implementations would traverse sub-shapes.
            self.nb_faces_fixed = 0;
            self.nb_edges_fixed = 0;
            self.fixed = true;
        }

        ok
    }

    /// Return the (possibly repaired) shape handle.
    pub fn shape(&self) -> &str {
        &self.shape
    }

    /// Number of faces that were corrected during the last [`perform`](ShapeFixer::perform).
    pub fn nb_faces_fixed(&self) -> usize {
        self.nb_faces_fixed
    }

    /// Number of edges that were corrected during the last [`perform`](ShapeFixer::perform).
    pub fn nb_edges_fixed(&self) -> usize {
        self.nb_edges_fixed
    }
}

// ---------------------------------------------------------------------------
// FaceFixer  (occt: ShapeFix_Face)
// ---------------------------------------------------------------------------

/// Repair algorithms scoped to a single topological face.
///
/// Mirrors `ShapeFix_Face` from OCCT, providing orientation correction, wire
/// repair, and a combined `perform` entry point.
// occt: ShapeFix_Face
pub struct FaceFixer {
    /// Opaque handle identifying the face being repaired.
    pub face: String,
}

impl FaceFixer {
    /// Create a new face fixer for the face identified by `face`.
    pub fn new(face: &str) -> Self {
        Self {
            face: face.to_owned(),
        }
    }

    /// Correct the orientation (winding order) of the face's outer wire.
    ///
    /// Returns `true` when an orientation fix was applied or when the face is
    /// already correctly oriented.  Stub always returns `true` for non-empty handles.
    pub fn fix_orientation(&mut self) -> bool {
        !self.face.is_empty()
    }

    /// Repair the outer (and any inner) wires of the face.
    ///
    /// Returns `true` when all wires are valid after repair.  Stub always returns
    /// `true` for non-empty handles.
    pub fn fix_wire(&mut self) -> bool {
        !self.face.is_empty()
    }

    /// Run all face repair algorithms in the standard OCCT order:
    /// wire repair → orientation fix.
    ///
    /// Returns `true` when the face is considered valid after all repairs.
    pub fn perform(&mut self) -> bool {
        self.fix_wire() && self.fix_orientation()
    }
}

// ---------------------------------------------------------------------------
// Convenience function
// ---------------------------------------------------------------------------

/// Apply a standard repair pass to `shape` using `tol` as the working
/// precision and return the (possibly modified) shape handle.
///
/// This is a thin wrapper around [`ShapeFixer`] and mirrors the convenience
/// helpers found in the `ShapeFix` package.
pub fn fix_shape(shape: &str, tol: f64) -> String {
    let params = ShapeFixParams::new(tol, tol * 0.01, tol * 100.0);
    let mut fixer = ShapeFixer::new(shape).with_params(params);
    fixer.perform();
    fixer.shape().to_owned()
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ---- ShapeFixParams ----

    #[test]
    fn params_new_stores_values() {
        let p = ShapeFixParams::new(1e-4, 1e-6, 1e-2);
        assert!((p.precision - 1e-4).abs() < 1e-15);
        assert!((p.min_tolerance - 1e-6).abs() < 1e-15);
        assert!((p.max_tolerance - 1e-2).abs() < 1e-15);
    }

    #[test]
    fn params_default_values() {
        let p = ShapeFixParams::default();
        assert!((p.precision - 1e-7).abs() < 1e-15);
        assert!((p.min_tolerance - 1e-7).abs() < 1e-15);
        assert!((p.max_tolerance - 1.0).abs() < 1e-15);
    }

    // ---- ShapeFixer ----

    #[test]
    fn shape_fixer_new_sets_defaults() {
        let sf = ShapeFixer::new("box");
        assert_eq!(sf.shape, "box");
        assert!(!sf.fixed);
        assert_eq!(sf.nb_faces_fixed(), 0);
        assert_eq!(sf.nb_edges_fixed(), 0);
    }

    #[test]
    fn shape_fixer_with_params_overrides() {
        let p = ShapeFixParams::new(1e-3, 1e-5, 0.1);
        let sf = ShapeFixer::new("sphere").with_params(p.clone());
        assert_eq!(sf.params, p);
    }

    #[test]
    fn shape_fixer_perform_returns_true_for_valid_shape() {
        let mut sf = ShapeFixer::new("cone");
        assert!(sf.perform());
        assert!(sf.fixed);
    }

    #[test]
    fn shape_fixer_perform_returns_false_for_empty_shape() {
        let mut sf = ShapeFixer::new("");
        assert!(!sf.perform());
        assert!(!sf.fixed);
    }

    #[test]
    fn shape_fixer_shape_accessor() {
        let sf = ShapeFixer::new("torus");
        assert_eq!(sf.shape(), "torus");
    }

    #[test]
    fn shape_fixer_counters_start_at_zero() {
        let mut sf = ShapeFixer::new("cube");
        sf.perform();
        assert_eq!(sf.nb_faces_fixed(), 0);
        assert_eq!(sf.nb_edges_fixed(), 0);
    }

    // ---- FaceFixer ----

    #[test]
    fn face_fixer_new_stores_face() {
        let ff = FaceFixer::new("face_42");
        assert_eq!(ff.face, "face_42");
    }

    #[test]
    fn face_fixer_fix_orientation_valid() {
        let mut ff = FaceFixer::new("f1");
        assert!(ff.fix_orientation());
    }

    #[test]
    fn face_fixer_fix_orientation_empty() {
        let mut ff = FaceFixer::new("");
        assert!(!ff.fix_orientation());
    }

    #[test]
    fn face_fixer_fix_wire_valid() {
        let mut ff = FaceFixer::new("f2");
        assert!(ff.fix_wire());
    }

    #[test]
    fn face_fixer_fix_wire_empty() {
        let mut ff = FaceFixer::new("");
        assert!(!ff.fix_wire());
    }

    #[test]
    fn face_fixer_perform_valid() {
        let mut ff = FaceFixer::new("f3");
        assert!(ff.perform());
    }

    #[test]
    fn face_fixer_perform_empty() {
        let mut ff = FaceFixer::new("");
        assert!(!ff.perform());
    }

    // ---- fix_shape ----

    #[test]
    fn fix_shape_returns_same_handle() {
        let out = fix_shape("my_shape", 1e-6);
        assert_eq!(out, "my_shape");
    }

    #[test]
    fn fix_shape_empty_handle_passthrough() {
        let out = fix_shape("", 1e-6);
        assert_eq!(out, "");
    }
}

// ---------------------------------------------------------------------------
// OCCT-named types for integration tests
// ---------------------------------------------------------------------------

use crate::top_exp::{TopoDS_Edge, TopoDS_Face, TopoDS_Shape, TopoDS_Vertex, TopoDS_Wire};

/// Default tolerance used by all fixers (mirrors OCCT Precision::Confusion).
pub const FIX_TOLERANCE: f64 = 1.0e-7;

// ---------------------------------------------------------------------------
// FixReport
// ---------------------------------------------------------------------------

/// Summary of repairs applied by a single fixer pass.
#[derive(Debug, Clone, Default)]
pub struct FixReport {
    pub total_fixes: usize,
    pub edge_fixes: usize,
    pub wire_fixes: usize,
    pub face_fixes: usize,
}

impl FixReport {
    /// Returns true when no repairs were needed.
    pub fn is_clean(&self) -> bool {
        self.total_fixes == 0
    }
}

// ---------------------------------------------------------------------------
// ShapeFix_Edge  (occt: ShapeFix_Edge)
// ---------------------------------------------------------------------------

/// Edge-level repair fixer.
pub struct ShapeFix_Edge {
    edge: TopoDS_Edge,
    tol: f64,
    /// True when the edge has zero length (coincident endpoints).
    pub is_degenerate: bool,
    /// Distance between first and last vertex (set after perform()).
    pub endpoint_gap: f64,
}

impl ShapeFix_Edge {
    pub fn new(e: TopoDS_Edge) -> Self {
        Self {
            edge: e,
            tol: FIX_TOLERANCE,
            is_degenerate: false,
            endpoint_gap: 0.0,
        }
    }

    pub fn set_tolerance(&mut self, t: f64) { self.tol = t; }
    pub fn tolerance(&self) -> f64 { self.tol }

    /// Run edge repairs. Returns the number of fixes applied.
    pub fn perform(&mut self) -> usize {
        let mut fixes = 0usize;

        // Pad to 2 vertices if fewer.
        while self.edge.vertices.len() < 2 {
            let last = self.edge.vertices.last().cloned().unwrap_or(TopoDS_Vertex { id: 0, x: 0.0, y: 0.0, z: 0.0 });
            self.edge.vertices.push(TopoDS_Vertex { id: last.id + 1, x: last.x, y: last.y, z: last.z });
            fixes += 1;
        }

        // Repair NaN / Inf coordinates.
        for v in &mut self.edge.vertices {
            if !v.x.is_finite() { v.x = 0.0; fixes += 1; }
            if !v.y.is_finite() { v.y = 0.0; fixes += 1; }
            if !v.z.is_finite() { v.z = 0.0; fixes += 1; }
        }

        // Compute endpoint gap.
        let a = &self.edge.vertices[0];
        let b = self.edge.vertices.last().unwrap();
        let dx = b.x - a.x;
        let dy = b.y - a.y;
        let dz = b.z - a.z;
        self.endpoint_gap = (dx * dx + dy * dy + dz * dz).sqrt();
        self.is_degenerate = self.endpoint_gap < self.tol;

        fixes
    }

    /// Return a reference to the (possibly repaired) edge.
    pub fn edge(&self) -> &TopoDS_Edge { &self.edge }
}

// ---------------------------------------------------------------------------
// ShapeFix_Wire  (occt: ShapeFix_Wire)
// ---------------------------------------------------------------------------

/// Wire-level repair fixer.
pub struct ShapeFix_Wire {
    wire: TopoDS_Wire,
    tol: f64,
}

impl ShapeFix_Wire {
    pub fn new(w: TopoDS_Wire) -> Self {
        Self { wire: w, tol: FIX_TOLERANCE }
    }

    pub fn set_tolerance(&mut self, t: f64) { self.tol = t; }

    /// Run wire repairs. Returns the number of fixes applied.
    pub fn perform(&mut self) -> usize {
        let mut fixes = 0usize;

        // 1. Remove degenerate (zero-length) edges.
        let tol = self.tol;
        let before = self.wire.edges.len();
        self.wire.edges.retain(|e| {
            if e.vertices.len() < 2 { return true; }
            let a = &e.vertices[0];
            let b = e.vertices.last().unwrap();
            let dx = b.x - a.x;
            let dy = b.y - a.y;
            let dz = b.z - a.z;
            (dx * dx + dy * dy + dz * dz).sqrt() >= tol
        });
        fixes += before - self.wire.edges.len();

        // 2. Close small gaps between consecutive edges (snap end to next start).
        let n = self.wire.edges.len();
        for i in 0..n.saturating_sub(1) {
            let end = {
                let e = &self.wire.edges[i];
                if e.vertices.is_empty() { continue; }
                e.vertices.last().unwrap().clone()
            };
            let start = {
                let e = &self.wire.edges[i + 1];
                if e.vertices.is_empty() { continue; }
                e.vertices[0].clone()
            };
            let dx = start.x - end.x;
            let dy = start.y - end.y;
            let dz = start.z - end.z;
            let gap = (dx * dx + dy * dy + dz * dz).sqrt();
            if gap > 0.0 && gap < tol * 2.0 {
                // Snap start of next edge to end of current.
                self.wire.edges[i + 1].vertices[0].x = end.x;
                self.wire.edges[i + 1].vertices[0].y = end.y;
                self.wire.edges[i + 1].vertices[0].z = end.z;
                fixes += 1;
            }
        }

        // 3. Greedy reorder: build a chain starting from edges[0].
        if self.wire.edges.len() > 1 {
            let mut ordered: Vec<TopoDS_Edge> = Vec::with_capacity(self.wire.edges.len());
            let mut remaining: Vec<TopoDS_Edge> = self.wire.edges.drain(..).collect();
            ordered.push(remaining.remove(0));

            while !remaining.is_empty() {
                let tail_x = ordered.last().unwrap().vertices.last().map(|v| v.x).unwrap_or(0.0);
                let tail_y = ordered.last().unwrap().vertices.last().map(|v| v.y).unwrap_or(0.0);
                let tail_z = ordered.last().unwrap().vertices.last().map(|v| v.z).unwrap_or(0.0);

                // Find the remaining edge whose start is closest to the tail.
                let mut best_idx = 0;
                let mut best_dist = f64::MAX;
                for (idx, e) in remaining.iter().enumerate() {
                    if let Some(v) = e.vertices.first() {
                        let d = (v.x - tail_x).powi(2) + (v.y - tail_y).powi(2) + (v.z - tail_z).powi(2);
                        if d < best_dist { best_dist = d; best_idx = idx; }
                    }
                }
                ordered.push(remaining.remove(best_idx));
            }

            // Count how many moved from original position.
            self.wire.edges = ordered;
        }

        fixes
    }

    /// Return a reference to the (possibly repaired) wire.
    pub fn wire(&self) -> &TopoDS_Wire { &self.wire }
}

// ---------------------------------------------------------------------------
// ShapeFix_Face  (occt: ShapeFix_Face)
// ---------------------------------------------------------------------------

/// Face-level repair fixer.
pub struct ShapeFix_Face {
    face: TopoDS_Face,
}

impl ShapeFix_Face {
    pub fn new(f: TopoDS_Face) -> Self {
        Self { face: f }
    }

    /// Run face repairs. Returns the number of fixes applied.
    pub fn perform(&mut self) -> usize {
        let mut fixes = 0usize;

        // Remove wires with fewer than 3 edges (too small to be valid).
        // Keep the first wire (outer boundary) unconditionally; only filter inner wires.
        if self.face.wires.len() > 1 {
            let outer = self.face.wires.remove(0);
            let before = self.face.wires.len();
            self.face.wires.retain(|w| w.edges.len() >= 3);
            fixes += before - self.face.wires.len();
            self.face.wires.insert(0, outer);
        }

        // Deduplicate inner wires by id (keep first occurrence).
        if self.face.wires.len() > 1 {
            let outer = self.face.wires.remove(0);
            let mut seen = std::collections::HashSet::new();
            let before = self.face.wires.len();
            self.face.wires.retain(|w| seen.insert(w.id));
            fixes += before - self.face.wires.len();
            self.face.wires.insert(0, outer);
        }

        fixes
    }

    pub fn face(&self) -> &TopoDS_Face { &self.face }
}

// ---------------------------------------------------------------------------
// ShapeFix_Shape  (occt: ShapeFix_Shape)
// ---------------------------------------------------------------------------

/// Top-level shape repair dispatcher (OCCT-named variant).
pub struct ShapeFix_Shape {
    shape: TopoDS_Shape,
}

impl ShapeFix_Shape {
    pub fn new(s: TopoDS_Shape) -> Self {
        Self { shape: s }
    }

    /// Run all repairs. Returns a FixReport summarising changes.
    pub fn perform(&mut self) -> FixReport {
        let mut report = FixReport::default();
        self.fix_shape_recursive(&mut report);
        report
    }

    fn fix_shape_recursive(&mut self, report: &mut FixReport) {
        match &mut self.shape {
            TopoDS_Shape::Vertex(v) => {
                let mut fixed = 0;
                if !v.x.is_finite() { v.x = 0.0; fixed += 1; }
                if !v.y.is_finite() { v.y = 0.0; fixed += 1; }
                if !v.z.is_finite() { v.z = 0.0; fixed += 1; }
                report.total_fixes += fixed;
            }
            TopoDS_Shape::Edge(e) => {
                let mut ef = ShapeFix_Edge::new(e.clone());
                let n = ef.perform();
                *e = ef.edge().clone();
                report.edge_fixes += n;
                report.total_fixes += n;
            }
            TopoDS_Shape::Wire(w) => {
                let mut wf = ShapeFix_Wire::new(w.clone());
                let n = wf.perform();
                *w = wf.wire().clone();
                report.wire_fixes += n;
                report.total_fixes += n;
            }
            TopoDS_Shape::Face(f) => {
                let mut ff = ShapeFix_Face::new(f.clone());
                let n = ff.perform();
                *f = ff.face().clone();
                report.face_fixes += n;
                report.total_fixes += n;
            }
            // Solid / Shell / etc.: recurse into sub-shapes (stub: no fixes at higher level).
            _ => {}
        }
    }

    pub fn shape(&self) -> &TopoDS_Shape { &self.shape }

    pub fn into_shape(self) -> TopoDS_Shape { self.shape }
}
