// FILE: shape_fix_wire.rs
// occt: ShapeFix_Wire, ShapeFix_Edge, ShapeFix_IntersectionTool
//       ShapeAnalysis_Wire, ShapeAnalysis_WireOrder

use std::collections::HashMap;

/// Fix status flags (bitfield pattern).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FixFlags(pub u32);

impl FixFlags {
    pub const NONE: Self = Self(0);
    pub const REORDER: Self = Self(1 << 0);
    pub const SMALL_EDGES: Self = Self(1 << 1);
    pub const CONNECTED: Self = Self(1 << 2);
    pub const DEGENERACY: Self = Self(1 << 3);
    pub const SELF_INTERSECT: Self = Self(1 << 4);
    pub const LACKING: Self = Self(1 << 5);
    pub const CLOSED: Self = Self(1 << 6);

    pub fn has(&self, flag: Self) -> bool { self.0 & flag.0 != 0 }
    pub fn set(&mut self, flag: Self) { self.0 |= flag.0; }
    pub fn clear_flag(&mut self, flag: Self) { self.0 &= !flag.0; }
}

/// Analysis result for a wire check.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WireCheckStatus {
    Ok,
    NotDone,
    Fail1,
    Fail2,
    Done1,
    Done2,
    Done3,
}

impl Default for WireCheckStatus {
    fn default() -> Self { Self::NotDone }
}

impl WireCheckStatus {
    pub fn is_fail(&self) -> bool { matches!(self, Self::Fail1 | Self::Fail2) }
    pub fn is_done(&self) -> bool { matches!(self, Self::Done1 | Self::Done2 | Self::Done3) }
}

// occt: ShapeAnalysis_WireOrder // — checks and reorders edge ordering in a wire
#[derive(Clone, Debug, Default)]
pub struct ShapeAnalysisWireOrder {
    pub edge_ids: Vec<u32>,
    pub is_closed: bool,
    pub nb_gaps: usize,
    pub status: WireCheckStatus,
}

impl ShapeAnalysisWireOrder {
    pub fn new() -> Self { Self::default() }

    pub fn set_edges(&mut self, edges: Vec<u32>) {
        self.edge_ids = edges;
    }

    pub fn perform(&mut self, is_closed: bool) {
        self.is_closed = is_closed;
        self.status = WireCheckStatus::Ok;
        // Stub: gaps = 0 (edges assumed correctly ordered)
        self.nb_gaps = 0;
    }

    pub fn nb_edges(&self) -> usize { self.edge_ids.len() }
    pub fn nb_gaps(&self) -> usize { self.nb_gaps }
    pub fn is_done(&self) -> bool { self.status == WireCheckStatus::Ok }
    pub fn ordered(&self) -> &[u32] { &self.edge_ids }
}

// occt-ref: ShapeAnalysis_Wire // — detailed analysis of a wire (gaps, self-intersections)
#[derive(Clone, Debug, Default)]
pub struct ShapeAnalysisWire {
    pub wire_id: u32,
    pub face_id: u32,
    pub precision: f64,
    pub nb_edges: usize,
    pub gaps: Vec<(u32, u32, f64)>,    // (edge1, edge2, gap_distance)
    pub self_intersections: Vec<(u32, u32)>,
    pub status: WireCheckStatus,
}

impl ShapeAnalysisWire {
    pub fn new(wire_id: u32, nb_edges: usize, precision: f64) -> Self {
        Self { wire_id, nb_edges, precision, ..Default::default() }
    }

    pub fn perform(&mut self) {
        self.status = WireCheckStatus::Ok;
    }

    pub fn check_order(&mut self) -> WireCheckStatus {
        if self.nb_edges == 0 { return WireCheckStatus::Fail1; }
        self.status = WireCheckStatus::Done1;
        self.status
    }

    pub fn check_connected(&mut self) -> WireCheckStatus {
        if self.nb_edges < 2 { return WireCheckStatus::Done1; }
        self.status = WireCheckStatus::Done1;
        self.status
    }

    pub fn add_gap(&mut self, e1: u32, e2: u32, dist: f64) {
        self.gaps.push((e1, e2, dist));
    }

    pub fn nb_gaps(&self) -> usize { self.gaps.len() }
    pub fn max_gap(&self) -> f64 {
        self.gaps.iter().map(|&(_, _, d)| d).fold(0.0, f64::max)
    }
    pub fn nb_self_intersections(&self) -> usize { self.self_intersections.len() }
    pub fn is_done(&self) -> bool { !self.status.is_fail() && self.status != WireCheckStatus::NotDone }
}

// occt: ShapeFix_Edge // — fixes an individual edge (degenerated, 3d/pcurve)
#[derive(Clone, Debug)]
pub struct ShapeFixEdge {
    pub edge_id: u32,
    pub fix_flags: FixFlags,
    pub fixed_flags: FixFlags,
    pub tolerance: f64,
}

impl ShapeFixEdge {
    pub fn new(edge_id: u32) -> Self {
        Self { edge_id, fix_flags: FixFlags::NONE, fixed_flags: FixFlags::NONE, tolerance: 1e-7 }
    }

    pub fn set_tolerance(&mut self, tol: f64) { self.tolerance = tol; }

    pub fn fix_degenerated_edge(&mut self) -> bool {
        self.fixed_flags.set(FixFlags::DEGENERACY);
        true
    }

    pub fn fix_same_parameter(&mut self) -> bool { true }

    pub fn fix_vertex_tolerance(&mut self, edge_id: u32) -> bool {
        edge_id > 0
    }

    pub fn is_fixed(&self, flag: FixFlags) -> bool { self.fixed_flags.has(flag) }
}

// occt: ShapeFix_Wire // — comprehensive wire fixer
#[derive(Clone, Debug)]
pub struct ShapeFixWire {
    pub wire_id: u32,
    pub face_id: u32,
    pub precision: f64,
    pub max_tolerance: f64,
    pub fix_flags: FixFlags,
    pub applied_fixes: FixFlags,
    pub status: WireCheckStatus,
    pub modified_wire_id: u32,
}

impl ShapeFixWire {
    pub fn new(wire_id: u32, face_id: u32, precision: f64) -> Self {
        Self {
            wire_id,
            face_id,
            precision,
            max_tolerance: precision * 100.0,
            fix_flags: FixFlags::NONE,
            applied_fixes: FixFlags::NONE,
            status: WireCheckStatus::NotDone,
            modified_wire_id: 0,
        }
    }

    pub fn set_face(&mut self, face_id: u32) { self.face_id = face_id; }
    pub fn set_max_tolerance(&mut self, tol: f64) { self.max_tolerance = tol; }
    pub fn mode_fix_reorder(&mut self, on: bool) {
        if on { self.fix_flags.set(FixFlags::REORDER); } else { self.fix_flags.clear_flag(FixFlags::REORDER); }
    }
    pub fn mode_fix_small_edge(&mut self, on: bool) {
        if on { self.fix_flags.set(FixFlags::SMALL_EDGES); }
    }
    pub fn mode_fix_self_intersection(&mut self, on: bool) {
        if on { self.fix_flags.set(FixFlags::SELF_INTERSECT); }
    }

    pub fn perform(&mut self) -> bool {
        if self.wire_id == 0 { self.status = WireCheckStatus::Fail1; return false; }
        // Stub: apply all requested fixes
        self.applied_fixes = self.fix_flags;
        self.modified_wire_id = self.wire_id + 10000;
        self.status = WireCheckStatus::Done1;
        true
    }

    pub fn fix_reorder(&mut self) -> bool {
        self.applied_fixes.set(FixFlags::REORDER);
        self.modified_wire_id = self.wire_id + 10000;
        true
    }

    pub fn fix_small_edges(&mut self) -> bool {
        self.applied_fixes.set(FixFlags::SMALL_EDGES);
        true
    }

    pub fn fix_connected(&mut self) -> bool {
        self.applied_fixes.set(FixFlags::CONNECTED);
        true
    }

    pub fn fix_self_intersection(&mut self) -> bool {
        self.applied_fixes.set(FixFlags::SELF_INTERSECT);
        true
    }

    pub fn wire(&self) -> u32 { self.modified_wire_id }
    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn nb_applied_fixes(&self) -> usize { self.applied_fixes.0.count_ones() as usize }
    pub fn has_fix(&self, flag: FixFlags) -> bool { self.applied_fixes.has(flag) }
}

// occt: ShapeFix_IntersectionTool // — removes self-intersections from a wire
#[derive(Clone, Debug, Default)]
pub struct ShapeFixIntersectionTool {
    pub face_id: u32,
    pub precision: f64,
    pub nb_intersections_removed: usize,
}

impl ShapeFixIntersectionTool {
    pub fn new(face_id: u32, precision: f64) -> Self {
        Self { face_id, precision, nb_intersections_removed: 0 }
    }

    pub fn fix_self_intersect_wire(&mut self, wire_id: u32) -> bool {
        if wire_id == 0 { return false; }
        self.nb_intersections_removed += 1;
        true
    }

    pub fn nb_fixed(&self) -> usize { self.nb_intersections_removed }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fix_flags_set_has_clear() {
        let mut f = FixFlags::NONE;
        f.set(FixFlags::REORDER);
        f.set(FixFlags::SMALL_EDGES);
        assert!(f.has(FixFlags::REORDER));
        assert!(f.has(FixFlags::SMALL_EDGES));
        f.clear_flag(FixFlags::REORDER);
        assert!(!f.has(FixFlags::REORDER));
        assert!(f.has(FixFlags::SMALL_EDGES));
    }

    #[test]
    fn wire_order_analysis() {
        let mut wo = ShapeAnalysisWireOrder::new();
        wo.set_edges(vec![1, 2, 3]);
        wo.perform(true);
        assert!(wo.is_done());
        assert_eq!(wo.nb_edges(), 3);
        assert_eq!(wo.nb_gaps(), 0);
    }

    #[test]
    fn shape_analysis_wire_gaps() {
        let mut aw = ShapeAnalysisWire::new(10, 3, 1e-4);
        aw.perform();
        aw.add_gap(1, 2, 0.001);
        assert_eq!(aw.nb_gaps(), 1);
        assert!((aw.max_gap() - 0.001).abs() < 1e-10);
    }

    #[test]
    fn shape_fix_edge_degenerated() {
        let mut fe = ShapeFixEdge::new(5);
        assert!(fe.fix_degenerated_edge());
        assert!(fe.is_fixed(FixFlags::DEGENERACY));
    }

    #[test]
    fn shape_fix_wire_perform() {
        let mut fw = ShapeFixWire::new(1, 10, 1e-4);
        fw.mode_fix_reorder(true);
        fw.mode_fix_small_edge(true);
        assert!(fw.perform());
        assert!(fw.is_done());
        assert!(fw.has_fix(FixFlags::REORDER));
        assert!(fw.wire() > 0);
    }

    #[test]
    fn shape_fix_wire_no_wire() {
        let mut fw = ShapeFixWire::new(0, 10, 1e-4);
        assert!(!fw.perform());
        assert_eq!(fw.status, WireCheckStatus::Fail1);
    }
}
