// FILE: shape_heal.rs
// occt: ShapeFix_FixSmallFace
// occt-ref: ShapeHealing, ShapeFix_RemoveSmallEdges

/// Category of shape healing fix.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HealFixType {
    SmallFace,
    SmallEdge,
    SameParameter,
    ReorderFaces,
    SewEdges,
    FixContinuity,
    FixTolerance,
}

/// Outcome of a heal operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HealStatus {
    NotDone,
    Done,
    Failed,
    NoFixes,
}

impl Default for HealStatus {
    fn default() -> Self { Self::NotDone }
}

impl HealStatus {
    pub fn is_done(&self) -> bool { *self == HealStatus::Done }
}

/// Record of one applied fix.
#[derive(Clone, Debug)]
pub struct HealFix {
    pub fix_type: HealFixType,
    pub shape_id: u32,
    pub result_id: u32,
    pub tolerance_used: f64,
}

// occt: ShapeFix_FixSmallFace // — removes degenerate/tiny faces from a shape
#[derive(Clone, Debug)]
pub struct FixSmallFace {
    pub shape_id: u32,
    pub result_id: u32,
    pub tolerance: f64,
    pub status: HealStatus,
    pub nb_removed: usize,
}

impl FixSmallFace {
    pub fn new(shape_id: u32) -> Self {
        Self { shape_id, result_id: 0, tolerance: 1e-7, status: HealStatus::NotDone, nb_removed: 0 }
    }

    pub fn set_tolerance(&mut self, t: f64) { self.tolerance = t; }

    pub fn perform(&mut self, small_face_ids: Vec<u32>) {
        if self.shape_id == 0 { self.status = HealStatus::Failed; return; }
        self.nb_removed = small_face_ids.len();
        if self.nb_removed == 0 { self.status = HealStatus::NoFixes; return; }
        self.result_id = self.shape_id + 40000;
        self.status = HealStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn shape(&self) -> u32 { if self.is_done() { self.result_id } else { self.shape_id } }
    pub fn nb_removed_faces(&self) -> usize { self.nb_removed }
}

// occt-note: ShapeFix_RemoveSmallEdges — removes degenerate/tiny edges
#[derive(Clone, Debug)]
pub struct FixRemoveSmallEdges {
    pub shape_id: u32,
    pub result_id: u32,
    pub tolerance: f64,
    pub status: HealStatus,
    pub removed_edges: Vec<u32>,
}

impl FixRemoveSmallEdges {
    pub fn new(shape_id: u32) -> Self {
        Self { shape_id, result_id: 0, tolerance: 1e-7, status: HealStatus::NotDone, removed_edges: Vec::new() }
    }

    pub fn set_tolerance(&mut self, t: f64) { self.tolerance = t; }

    pub fn remove_small_edges(&mut self, edge_ids: Vec<u32>) -> bool {
        if self.shape_id == 0 { return false; }
        self.removed_edges = edge_ids;
        if self.removed_edges.is_empty() {
            self.status = HealStatus::NoFixes;
            return false;
        }
        self.result_id = self.shape_id + 41000;
        self.status = HealStatus::Done;
        true
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn shape(&self) -> u32 { if self.is_done() { self.result_id } else { self.shape_id } }
    pub fn nb_removed(&self) -> usize { self.removed_edges.len() }
}

// occt-note: ShapeHealing — combined healer for import workflows
#[derive(Clone, Debug)]
pub struct ShapeHealing {
    pub shape_id: u32,
    pub result_id: u32,
    pub tolerance_3d: f64,
    pub tolerance_angular: f64,
    pub fixes: Vec<HealFix>,
    pub status: HealStatus,
}

impl ShapeHealing {
    pub fn new(shape_id: u32) -> Self {
        Self {
            shape_id,
            result_id: 0,
            tolerance_3d: 1e-6,
            tolerance_angular: 1e-5,
            fixes: Vec::new(),
            status: HealStatus::NotDone,
        }
    }

    pub fn set_tolerance_3d(&mut self, t: f64) { self.tolerance_3d = t; }
    pub fn set_tolerance_angular(&mut self, t: f64) { self.tolerance_angular = t; }

    pub fn apply_fix(&mut self, fix_type: HealFixType, affected_ids: Vec<u32>) {
        let result_id = self.shape_id + 42000 + self.fixes.len() as u32;
        for id in &affected_ids {
            self.fixes.push(HealFix {
                fix_type,
                shape_id: *id,
                result_id,
                tolerance_used: self.tolerance_3d,
            });
        }
    }

    pub fn perform(&mut self) {
        if self.shape_id == 0 { self.status = HealStatus::Failed; return; }
        if self.fixes.is_empty() { self.status = HealStatus::NoFixes; return; }
        self.result_id = self.shape_id + 50000;
        self.status = HealStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn shape(&self) -> u32 { if self.is_done() { self.result_id } else { self.shape_id } }
    pub fn nb_fixes(&self) -> usize { self.fixes.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fix_small_face_removes() {
        let mut f = FixSmallFace::new(10);
        f.perform(vec![1, 2, 3]);
        assert!(f.is_done());
        assert_eq!(f.nb_removed_faces(), 3);
        assert!(f.shape() > 0);
    }

    #[test]
    fn fix_small_face_no_fixes() {
        let mut f = FixSmallFace::new(10);
        f.perform(vec![]);
        assert_eq!(f.status, HealStatus::NoFixes);
        assert_eq!(f.shape(), 10); // returns original
    }

    #[test]
    fn fix_remove_small_edges_removes() {
        let mut f = FixRemoveSmallEdges::new(20);
        let ok = f.remove_small_edges(vec![5, 6]);
        assert!(ok);
        assert!(f.is_done());
        assert_eq!(f.nb_removed(), 2);
    }

    #[test]
    fn fix_remove_small_edges_empty() {
        let mut f = FixRemoveSmallEdges::new(20);
        let ok = f.remove_small_edges(vec![]);
        assert!(!ok);
        assert_eq!(f.status, HealStatus::NoFixes);
    }

    #[test]
    fn shape_healing_applies_fixes() {
        let mut h = ShapeHealing::new(100);
        h.apply_fix(HealFixType::SmallEdge, vec![1, 2]);
        h.apply_fix(HealFixType::SewEdges, vec![3]);
        h.perform();
        assert!(h.is_done());
        assert_eq!(h.nb_fixes(), 3);
    }

    #[test]
    fn shape_healing_no_fixes() {
        let mut h = ShapeHealing::new(100);
        h.perform();
        assert_eq!(h.status, HealStatus::NoFixes);
        assert_eq!(h.shape(), 100);
    }
}
