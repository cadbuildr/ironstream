// FILE: brep_gap_fill.rs
// occt: BRepOffsetAPI_FindContigousEdges, ShapeAnalysis_FreeBounds

use std::collections::HashMap;

/// Status of a gap-fill or free-bounds operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GapFillStatus {
    Ok,
    NotDone,
    NoEdges,
    ToleranceError,
    Error,
}

impl Default for GapFillStatus {
    fn default() -> Self { Self::NotDone }
}

impl GapFillStatus {
    pub fn is_ok(&self) -> bool { *self == GapFillStatus::Ok }
}

/// An edge that is on the boundary (open edge, not shared by two faces).
#[derive(Clone, Debug)]
pub struct FreeEdge {
    pub edge_id: u32,
    pub face_id: u32,
    pub first_vertex: u32,
    pub last_vertex: u32,
    pub is_degenerated: bool,
    pub tolerance: f64,
}

impl FreeEdge {
    pub fn new(edge_id: u32, face_id: u32, first: u32, last: u32) -> Self {
        Self { edge_id, face_id, first_vertex: first, last_vertex: last, is_degenerated: false, tolerance: 1e-7 }
    }
}

// occt: ShapeAnalysis_FreeBounds — finds free (boundary) edges in a shape
#[derive(Clone, Debug, Default)]
pub struct ShapeAnalysisFreeBouns {
    pub shape_id: u32,
    pub free_edges: Vec<FreeEdge>,
    pub closed_wires: Vec<Vec<u32>>,  // each Vec<u32> = edge_ids forming a closed wire
    pub open_wires: Vec<Vec<u32>>,    // open wire sequences
    pub tolerance: f64,
    pub is_done: bool,
}

impl ShapeAnalysisFreeBouns {
    pub fn new(shape_id: u32, tolerance: f64) -> Self {
        Self { shape_id, tolerance, ..Default::default() }
    }

    pub fn perform(&mut self, edges: Vec<FreeEdge>) {
        self.free_edges = edges;
        // Stub: group all edges into one open wire
        let wire: Vec<u32> = self.free_edges.iter().map(|e| e.edge_id).collect();
        if !wire.is_empty() {
            if self.free_edges.first().map(|e| e.first_vertex) == self.free_edges.last().map(|e| e.last_vertex) {
                self.closed_wires.push(wire);
            } else {
                self.open_wires.push(wire);
            }
        }
        self.is_done = true;
    }

    pub fn nb_free_edges(&self) -> usize { self.free_edges.len() }
    pub fn nb_closed_wires(&self) -> usize { self.closed_wires.len() }
    pub fn nb_open_wires(&self) -> usize { self.open_wires.len() }
    pub fn is_done(&self) -> bool { self.is_done }

    pub fn free_edge(&self, i: usize) -> Option<&FreeEdge> { self.free_edges.get(i) }
    pub fn closed_wire(&self, i: usize) -> Option<&Vec<u32>> { self.closed_wires.get(i) }
    pub fn open_wire(&self, i: usize) -> Option<&Vec<u32>> { self.open_wires.get(i) }
}

/// A contiguous edge pairing (edges that should be stitched).
#[derive(Clone, Debug)]
pub struct ContiguousPair {
    pub edge1_id: u32,
    pub edge2_id: u32,
    pub shape1_id: u32,
    pub shape2_id: u32,
    pub distance: f64,
}

// occt: BRepOffsetAPI_FindContigousEdges — matches free edges between shapes within tolerance
#[derive(Clone, Debug, Default)]
pub struct FindContigousEdges {
    pub tolerance: f64,
    pub shapes: Vec<u32>,
    pub free_edges: Vec<FreeEdge>,
    pub contiguous_pairs: Vec<ContiguousPair>,
    pub status: GapFillStatus,
}

impl FindContigousEdges {
    pub fn new(tolerance: f64) -> Self {
        Self { tolerance, status: GapFillStatus::NotDone, ..Default::default() }
    }

    pub fn add(&mut self, shape_id: u32) { self.shapes.push(shape_id); }
    pub fn add_free_edge(&mut self, edge: FreeEdge) { self.free_edges.push(edge); }

    pub fn perform(&mut self) {
        if self.shapes.is_empty() {
            self.status = GapFillStatus::NoEdges;
            return;
        }
        // Stub: find pairs by tolerance proximity (matching vertex IDs)
        let mut pairs: Vec<ContiguousPair> = Vec::new();
        for i in 0..self.free_edges.len() {
            for j in i+1..self.free_edges.len() {
                let e1 = &self.free_edges[i];
                let e2 = &self.free_edges[j];
                if e1.face_id != e2.face_id &&
                   (e1.first_vertex == e2.first_vertex || e1.first_vertex == e2.last_vertex
                    || e1.last_vertex == e2.first_vertex || e1.last_vertex == e2.last_vertex) {
                    pairs.push(ContiguousPair {
                        edge1_id: e1.edge_id,
                        edge2_id: e2.edge_id,
                        shape1_id: e1.face_id,
                        shape2_id: e2.face_id,
                        distance: 0.0,
                    });
                }
            }
        }
        self.contiguous_pairs = pairs;
        self.status = GapFillStatus::Ok;
    }

    pub fn is_done(&self) -> bool { self.status.is_ok() }
    pub fn nb_contigous_edges(&self) -> usize { self.contiguous_pairs.len() }
    pub fn nb_free_edges(&self) -> usize {
        self.free_edges.len() - self.contiguous_pairs.len() * 2
    }

    pub fn contigous_pair(&self, i: usize) -> Option<&ContiguousPair> { self.contiguous_pairs.get(i) }
    pub fn nb_shapes(&self) -> usize { self.shapes.len() }
}

/// Gap-fill result: edges that were stitched.
#[derive(Clone, Debug, Default)]
pub struct GapFillResult {
    pub stitched_pairs: Vec<(u32, u32)>,
    pub remaining_free: Vec<u32>,
    pub max_gap: f64,
    pub tolerance_used: f64,
}

impl GapFillResult {
    pub fn new() -> Self { Self::default() }

    pub fn add_stitch(&mut self, e1: u32, e2: u32, gap: f64) {
        self.stitched_pairs.push((e1, e2));
        if gap > self.max_gap { self.max_gap = gap; }
    }

    pub fn nb_stitched(&self) -> usize { self.stitched_pairs.len() }
    pub fn nb_remaining_free(&self) -> usize { self.remaining_free.len() }
    pub fn is_fully_stitched(&self) -> bool { self.remaining_free.is_empty() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_contigous_edges_basic() {
        let mut finder = FindContigousEdges::new(1e-4);
        finder.add(1);
        finder.add(2);
        finder.add_free_edge(FreeEdge::new(10, 1, 100, 101));
        finder.add_free_edge(FreeEdge::new(20, 2, 101, 102));  // shares vertex 101
        finder.perform();
        assert!(finder.is_done());
        assert_eq!(finder.nb_contigous_edges(), 1);
    }

    #[test]
    fn find_contigous_edges_no_shapes() {
        let mut finder = FindContigousEdges::new(1e-4);
        finder.perform();
        assert_eq!(finder.status, GapFillStatus::NoEdges);
    }

    #[test]
    fn shape_analysis_free_bounds() {
        let mut fb = ShapeAnalysisFreeBouns::new(1, 1e-4);
        fb.perform(vec![
            FreeEdge::new(1, 10, 100, 101),
            FreeEdge::new(2, 10, 101, 102),
        ]);
        assert!(fb.is_done());
        assert_eq!(fb.nb_free_edges(), 2);
    }

    #[test]
    fn gap_fill_result() {
        let mut r = GapFillResult::new();
        r.add_stitch(1, 2, 0.001);
        r.add_stitch(3, 4, 0.0005);
        assert_eq!(r.nb_stitched(), 2);
        assert!((r.max_gap - 0.001).abs() < 1e-10);
        assert!(r.is_fully_stitched());
    }

    #[test]
    fn free_edge_basic() {
        let e = FreeEdge::new(5, 10, 100, 200);
        assert_eq!(e.edge_id, 5);
        assert_eq!(e.first_vertex, 100);
        assert!(!e.is_degenerated);
    }
}
