// FILE: chfi3d_fillet.rs
// occt: ChFi3d_FilletAlgo, ChFi3d_Builder, ChFi3d_ChBuilder (chamfer builder)

use std::collections::HashMap;

/// Status after a ChFi3d_Builder operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chfi3dStatus {
    NotDone,
    Done,
    FailTopology,
    FailNoFace,
    FailSmallRadius,
    FailContiguity,
    Error,
}

impl Default for Chfi3dStatus {
    fn default() -> Self { Self::NotDone }
}

impl Chfi3dStatus {
    pub fn is_done(&self) -> bool { *self == Chfi3dStatus::Done }
    pub fn is_fail(&self) -> bool { matches!(self, Self::FailTopology | Self::FailNoFace | Self::FailSmallRadius | Self::FailContiguity | Self::Error) }
}

/// Mode for how a chamfer/fillet is constructed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chfi3dMode {
    Rational,  // for fillets
    QuasiAngular,
    Polynomial,
}

impl Default for Chfi3dMode {
    fn default() -> Self { Self::Rational }
}

/// A fillet edge specification (edge_id + radius).
#[derive(Clone, Debug)]
pub struct FilletEdgeSpec {
    pub edge_id: u32,
    pub radius: f64,
    pub radius2: f64,   // for variable or two-sided chamfer
    pub is_variable: bool,
}

impl FilletEdgeSpec {
    pub fn new(edge_id: u32, radius: f64) -> Self {
        Self { edge_id, radius, radius2: radius, is_variable: false }
    }

    pub fn variable(edge_id: u32, r1: f64, r2: f64) -> Self {
        Self { edge_id, radius: r1, radius2: r2, is_variable: true }
    }
}

// occt: ChFi3d_Builder — base class for fillet/chamfer on a solid
#[derive(Clone, Debug)]
pub struct Chfi3dBuilder {
    pub solid_id: u32,
    pub mode: Chfi3dMode,
    pub tolerance: f64,
    pub edge_specs: Vec<FilletEdgeSpec>,
    pub status: Chfi3dStatus,
    pub result_id: u32,
    pub nb_failed: usize,
}

impl Chfi3dBuilder {
    pub fn new(solid_id: u32) -> Self {
        Self {
            solid_id,
            mode: Chfi3dMode::Rational,
            tolerance: 1e-8,
            edge_specs: Vec::new(),
            status: Chfi3dStatus::NotDone,
            result_id: 0,
            nb_failed: 0,
        }
    }

    pub fn set_mode(&mut self, mode: Chfi3dMode) { self.mode = mode; }
    pub fn set_params(&mut self, tolerance: f64) { self.tolerance = tolerance; }

    pub fn add_edge(&mut self, edge_id: u32, radius: f64) {
        if radius > 0.0 && edge_id > 0 {
            self.edge_specs.push(FilletEdgeSpec::new(edge_id, radius));
        }
    }

    pub fn remove_edge(&mut self, edge_id: u32) {
        self.edge_specs.retain(|s| s.edge_id != edge_id);
    }

    pub fn build(&mut self) {
        if self.solid_id == 0 { self.status = Chfi3dStatus::FailTopology; return; }
        if self.edge_specs.is_empty() { self.status = Chfi3dStatus::Done; self.result_id = self.solid_id; return; }
        // Stub: check for zero radii
        let bad = self.edge_specs.iter().any(|s| s.radius < 1e-15);
        if bad { self.status = Chfi3dStatus::FailSmallRadius; return; }
        self.result_id = self.solid_id + 10000;
        self.status = Chfi3dStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn shape(&self) -> u32 { self.result_id }
    pub fn nb_contours(&self) -> usize { self.edge_specs.len() }
    pub fn nb_failed_contours(&self) -> usize { self.nb_failed }
    pub fn contour(&self, i: usize) -> Option<&FilletEdgeSpec> { self.edge_specs.get(i) }
    pub fn status(&self) -> Chfi3dStatus { self.status }
}

// occt: ChFi3d_FilletAlgo — the fillet algorithm on a BRep solid
#[derive(Clone, Debug)]
pub struct Chfi3dFilletAlgo {
    pub builder: Chfi3dBuilder,
    pub is_parallel: bool,
}

impl Chfi3dFilletAlgo {
    pub fn new(solid_id: u32) -> Self {
        Self { builder: Chfi3dBuilder::new(solid_id), is_parallel: false }
    }

    pub fn set_radius(&mut self, edge_id: u32, radius: f64) {
        self.builder.add_edge(edge_id, radius);
    }

    pub fn set_radius_variable(&mut self, edge_id: u32, r1: f64, r2: f64) {
        if edge_id > 0 && r1 > 0.0 && r2 > 0.0 {
            self.builder.edge_specs.push(FilletEdgeSpec::variable(edge_id, r1, r2));
        }
    }

    pub fn build(&mut self) { self.builder.build(); }
    pub fn is_done(&self) -> bool { self.builder.is_done() }
    pub fn shape(&self) -> u32 { self.builder.shape() }
    pub fn nb_contours(&self) -> usize { self.builder.nb_contours() }
}

// occt: ChFi3d_ChBuilder — chamfer builder on a BRep solid
#[derive(Clone, Debug)]
pub struct Chfi3dChBuilder {
    pub builder: Chfi3dBuilder,
    pub mode: ChamferMode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChamferMode {
    ClassicChamfer,
    ConstThroat,
    ConstThroatWithPenetration,
    EqualDist,
    DistAngle,
}

impl Default for ChamferMode {
    fn default() -> Self { Self::ClassicChamfer }
}

impl Chfi3dChBuilder {
    pub fn new(solid_id: u32) -> Self {
        Self {
            builder: Chfi3dBuilder::new(solid_id),
            mode: ChamferMode::ClassicChamfer,
        }
    }

    pub fn set_chamfer_mode(&mut self, mode: ChamferMode) { self.mode = mode; }

    pub fn add_edge_dist_angle(&mut self, edge_id: u32, dist: f64, angle: f64) {
        if edge_id > 0 && dist > 0.0 {
            self.builder.edge_specs.push(FilletEdgeSpec::new(edge_id, dist));
        }
    }

    pub fn add_edge_equal(&mut self, edge_id: u32, dist: f64) {
        self.builder.add_edge(edge_id, dist);
    }

    pub fn build(&mut self) { self.builder.build(); }
    pub fn is_done(&self) -> bool { self.builder.is_done() }
    pub fn shape(&self) -> u32 { self.builder.shape() }
    pub fn nb_contours(&self) -> usize { self.builder.nb_contours() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fillet_algo_basic() {
        let mut f = Chfi3dFilletAlgo::new(1);
        f.set_radius(10, 0.5);
        f.build();
        assert!(f.is_done());
        assert!(f.shape() > 0);
        assert_eq!(f.nb_contours(), 1);
    }

    #[test]
    fn fillet_algo_zero_radius() {
        let mut f = Chfi3dFilletAlgo::new(1);
        f.set_radius(10, 0.0);
        f.build();
        // zero radius should not add the edge spec, so build still passes (no specs = identity)
        assert!(f.is_done());
    }

    #[test]
    fn fillet_algo_null_solid() {
        let mut f = Chfi3dFilletAlgo::new(0);
        f.set_radius(10, 0.5);
        f.build();
        assert!(!f.is_done());
        assert_eq!(f.builder.status(), Chfi3dStatus::FailTopology);
    }

    #[test]
    fn chamfer_builder_equal_dist() {
        let mut c = Chfi3dChBuilder::new(1);
        c.add_edge_equal(5, 0.3);
        c.add_edge_equal(6, 0.3);
        c.build();
        assert!(c.is_done());
        assert_eq!(c.nb_contours(), 2);
    }

    #[test]
    fn fillet_variable_radius() {
        let mut f = Chfi3dFilletAlgo::new(1);
        f.set_radius_variable(10, 0.2, 0.5);
        f.build();
        assert!(f.is_done());
        let spec = f.builder.contour(0).unwrap();
        assert!(spec.is_variable);
    }

    #[test]
    fn fillet_remove_edge() {
        let mut b = Chfi3dBuilder::new(1);
        b.add_edge(10, 0.5);
        b.add_edge(11, 0.3);
        assert_eq!(b.nb_contours(), 2);
        b.remove_edge(10);
        assert_eq!(b.nb_contours(), 1);
        assert_eq!(b.edge_specs[0].edge_id, 11);
    }
}
