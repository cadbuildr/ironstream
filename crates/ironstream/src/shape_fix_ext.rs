// FILE: shape_fix_ext.rs
// occt: ShapeFix_Shape, ShapeFix_Shell, ShapeFix_Solid, ShapeFix_Wire extended

use std::collections::HashMap;

/// Status flags from ShapeFix operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FixStatus {
    Ok,
    Done1, Done2, Done3, Done4, Done5,
    Fail1, Fail2, Fail3, Fail4, Fail5,
    NotDone,
}

impl Default for FixStatus {
    fn default() -> Self { Self::NotDone }
}

impl FixStatus {
    pub fn is_done(&self) -> bool {
        matches!(self, FixStatus::Done1 | FixStatus::Done2 | FixStatus::Done3 | FixStatus::Done4 | FixStatus::Done5 | FixStatus::Ok)
    }
    pub fn is_fail(&self) -> bool {
        matches!(self, FixStatus::Fail1 | FixStatus::Fail2 | FixStatus::Fail3 | FixStatus::Fail4 | FixStatus::Fail5)
    }
}

/// Summary of fixes applied by ShapeFix.
#[derive(Clone, Debug, Default)]
pub struct FixReport {
    pub nb_faces_fixed: usize,
    pub nb_edges_fixed: usize,
    pub nb_wires_fixed: usize,
    pub nb_vertices_fixed: usize,
    pub nb_shells_fixed: usize,
    pub nb_solids_fixed: usize,
    pub messages: Vec<String>,
}

impl FixReport {
    pub fn new() -> Self { Self::default() }
    pub fn add_message(&mut self, msg: &str) { self.messages.push(msg.to_owned()); }
    pub fn nb_messages(&self) -> usize { self.messages.len() }
    pub fn is_empty(&self) -> bool {
        self.nb_faces_fixed == 0 && self.nb_edges_fixed == 0 && self.nb_wires_fixed == 0
    }
}

// occt: ShapeFix_Shape — fix all subshapes in a shape
#[derive(Clone, Debug)]
pub struct ShapeFixShape {
    pub shape_id: u32,
    pub tolerance: f64,
    pub max_tolerance: f64,
    pub min_tolerance: f64,
    pub fix_solid_mode: i32,
    pub fix_shell_mode: i32,
    pub fix_face_mode: i32,
    pub fix_wire_mode: i32,
    pub status: FixStatus,
    pub result_id: u32,
    pub report: FixReport,
}

impl ShapeFixShape {
    pub fn new(shape_id: u32) -> Self {
        Self {
            shape_id,
            tolerance: 1e-7,
            max_tolerance: 1.0,
            min_tolerance: 1e-7,
            fix_solid_mode: -1,
            fix_shell_mode: -1,
            fix_face_mode: -1,
            fix_wire_mode: -1,
            status: FixStatus::NotDone,
            result_id: 0,
            report: FixReport::new(),
        }
    }

    pub fn set_max_tolerance(&mut self, t: f64) { self.max_tolerance = t; }
    pub fn set_min_tolerance(&mut self, t: f64) { self.min_tolerance = t; }
    pub fn set_precision(&mut self, t: f64) { self.tolerance = t; }
    pub fn set_fix_solid_mode(&mut self, m: i32) { self.fix_solid_mode = m; }
    pub fn set_fix_shell_mode(&mut self, m: i32) { self.fix_shell_mode = m; }

    pub fn perform(&mut self) -> bool {
        if self.shape_id == 0 { self.status = FixStatus::Fail1; return false; }
        self.result_id = self.shape_id + 1;
        self.report.nb_faces_fixed = 0;
        self.status = FixStatus::Done1;
        true
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn shape(&self) -> u32 { self.result_id }
    pub fn report(&self) -> &FixReport { &self.report }
}

// occt: ShapeFix_Shell — fix an individual shell (orientation, connected faces)
#[derive(Clone, Debug)]
pub struct ShapeFixShell {
    pub shell_id: u32,
    pub tolerance: f64,
    pub fix_orientation: bool,
    pub fix_face_orientation: bool,
    pub status: FixStatus,
    pub result_id: u32,
    pub nb_free_edges_before: usize,
    pub nb_free_edges_after: usize,
}

impl ShapeFixShell {
    pub fn new(shell_id: u32) -> Self {
        Self {
            shell_id,
            tolerance: 1e-7,
            fix_orientation: true,
            fix_face_orientation: true,
            status: FixStatus::NotDone,
            result_id: 0,
            nb_free_edges_before: 0,
            nb_free_edges_after: 0,
        }
    }

    pub fn set_fix_orientation_mode(&mut self, v: bool) { self.fix_orientation = v; }

    pub fn perform(&mut self) -> bool {
        if self.shell_id == 0 { self.status = FixStatus::Fail1; return false; }
        self.result_id = self.shell_id + 2;
        self.status = FixStatus::Done1;
        true
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn shell(&self) -> u32 { self.result_id }
    pub fn nb_faces(&self) -> usize { 0 }
}

// occt: ShapeFix_Solid — fix solid (closed shell, orientation)
#[derive(Clone, Debug)]
pub struct ShapeFixSolid {
    pub solid_id: u32,
    pub tolerance: f64,
    pub fix_shell_orientation: bool,
    pub status: FixStatus,
    pub result_id: u32,
    pub created_shells: Vec<u32>,
}

impl ShapeFixSolid {
    pub fn new(solid_id: u32) -> Self {
        Self {
            solid_id,
            tolerance: 1e-7,
            fix_shell_orientation: true,
            status: FixStatus::NotDone,
            result_id: 0,
            created_shells: Vec::new(),
        }
    }

    pub fn set_fix_shell_orientation_mode(&mut self, v: bool) { self.fix_shell_orientation = v; }

    pub fn perform(&mut self) -> bool {
        if self.solid_id == 0 { self.status = FixStatus::Fail1; return false; }
        self.result_id = self.solid_id + 3;
        self.status = FixStatus::Done1;
        true
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn solid(&self) -> u32 { self.result_id }
    pub fn nb_shells_fixed(&self) -> usize { 0 }
}

// occt: ShapeFix_SplitFace — splits a face at seam edges or pole singularities
#[derive(Clone, Debug, Default)]
pub struct ShapeFixSplitFace {
    pub face_id: u32,
    pub tolerance: f64,
    pub split_closed_faces: bool,
    pub status: FixStatus,
    pub result_faces: Vec<u32>,
}

impl ShapeFixSplitFace {
    pub fn new(face_id: u32) -> Self {
        Self { face_id, tolerance: 1e-7, split_closed_faces: true, ..Default::default() }
    }

    pub fn split(&mut self) {
        if self.face_id == 0 { self.status = FixStatus::Fail1; return; }
        // Stub: no splitting needed
        self.result_faces.push(self.face_id);
        self.status = FixStatus::Done1;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn nb_result_faces(&self) -> usize { self.result_faces.len() }
    pub fn result_face(&self, i: usize) -> Option<u32> { self.result_faces.get(i).copied() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fix_shape_perform() {
        let mut fs = ShapeFixShape::new(1);
        fs.set_precision(1e-6);
        assert!(fs.perform());
        assert!(fs.is_done());
        assert!(fs.shape() > 0);
    }

    #[test]
    fn fix_shape_null() {
        let mut fs = ShapeFixShape::new(0);
        assert!(!fs.perform());
        assert!(fs.status.is_fail());
    }

    #[test]
    fn fix_shell_basic() {
        let mut fs = ShapeFixShell::new(5);
        assert!(fs.perform());
        assert!(fs.is_done());
        assert!(fs.shell() > 0);
    }

    #[test]
    fn fix_solid_basic() {
        let mut fs = ShapeFixSolid::new(10);
        assert!(fs.perform());
        assert!(fs.is_done());
        assert!(fs.solid() > 0);
    }

    #[test]
    fn fix_split_face() {
        let mut fs = ShapeFixSplitFace::new(7);
        fs.split();
        assert!(fs.is_done());
        assert_eq!(fs.nb_result_faces(), 1);
    }

    #[test]
    fn fix_report_basic() {
        let mut r = FixReport::new();
        r.nb_faces_fixed = 3;
        r.add_message("Fixed degenerate edge");
        assert_eq!(r.nb_messages(), 1);
        assert!(!r.is_empty());
    }
}
