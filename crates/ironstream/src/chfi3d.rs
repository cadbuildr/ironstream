// FILE: chfi3d.rs

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChFi3dError {
    NotDone,
    NegativeRadius,
    EmptyShape,
    FilletFailed,
    NoSuchEdge,
}

pub struct ChFi3dFilletEntry {
    pub edge_id: u32,
    pub radius: f64,
}

pub struct ChFi3dChamferEntry {
    pub edge_id: u32,
    pub dist1: f64,
    pub dist2: f64,
}

// occt: ChFi3d_Builder
pub struct ChFi3dBuilder {
    fillets: Vec<ChFi3dFilletEntry>,
    chamfers: Vec<ChFi3dChamferEntry>,
    tolerance: f64,
    is_done: bool,
    nb_faulted: u32,
}

impl ChFi3dBuilder {
    pub fn new(tolerance: f64) -> Self {
        Self {
            fillets: Vec::new(),
            chamfers: Vec::new(),
            tolerance,
            is_done: false,
            nb_faulted: 0,
        }
    }

    pub fn add_fillet(&mut self, edge_id: u32, radius: f64) -> Result<(), ChFi3dError> {
        if radius < 0.0 {
            return Err(ChFi3dError::NegativeRadius);
        }
        self.fillets.push(ChFi3dFilletEntry { edge_id, radius });
        Ok(())
    }

    pub fn add_chamfer(
        &mut self,
        edge_id: u32,
        dist1: f64,
        dist2: f64,
    ) -> Result<(), ChFi3dError> {
        if dist1 < 0.0 || dist2 < 0.0 {
            return Err(ChFi3dError::NegativeRadius);
        }
        self.chamfers
            .push(ChFi3dChamferEntry { edge_id, dist1, dist2 });
        Ok(())
    }

    pub fn build(&mut self) {
        self.is_done = !self.fillets.is_empty() || !self.chamfers.is_empty();
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn nb_contours(&self) -> usize {
        self.fillets.len() + self.chamfers.len()
    }

    pub fn nb_faulted_contours(&self) -> u32 {
        self.nb_faulted
    }

    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    pub fn fillet_radius(&self, edge_id: u32) -> Option<f64> {
        self.fillets
            .iter()
            .find(|e| e.edge_id == edge_id)
            .map(|e| e.radius)
    }

    pub fn remove_contour(&mut self, edge_id: u32) {
        if let Some(pos) = self.fillets.iter().position(|e| e.edge_id == edge_id) {
            self.fillets.remove(pos);
            return;
        }
        if let Some(pos) = self.chamfers.iter().position(|e| e.edge_id == edge_id) {
            self.chamfers.remove(pos);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_not_done() {
        let b = ChFi3dBuilder::new(1e-6);
        assert!(!b.is_done());
        assert_eq!(b.nb_contours(), 0);
        assert_eq!(b.nb_faulted_contours(), 0);
    }

    #[test]
    fn tolerance_stored() {
        let b = ChFi3dBuilder::new(0.001);
        assert!((b.tolerance() - 0.001).abs() < 1e-15);
    }

    #[test]
    fn add_fillet_increments_contours() {
        let mut b = ChFi3dBuilder::new(1e-6);
        b.add_fillet(1, 2.0).unwrap();
        b.add_fillet(2, 3.0).unwrap();
        assert_eq!(b.nb_contours(), 2);
    }

    #[test]
    fn add_fillet_negative_radius_errors() {
        let mut b = ChFi3dBuilder::new(1e-6);
        assert_eq!(b.add_fillet(1, -1.0), Err(ChFi3dError::NegativeRadius));
        assert_eq!(b.nb_contours(), 0);
    }

    #[test]
    fn add_chamfer_increments_contours() {
        let mut b = ChFi3dBuilder::new(1e-6);
        b.add_chamfer(5, 1.0, 2.0).unwrap();
        assert_eq!(b.nb_contours(), 1);
    }

    #[test]
    fn add_chamfer_negative_dist_errors() {
        let mut b = ChFi3dBuilder::new(1e-6);
        assert_eq!(b.add_chamfer(1, -0.1, 1.0), Err(ChFi3dError::NegativeRadius));
        assert_eq!(b.add_chamfer(1, 1.0, -0.1), Err(ChFi3dError::NegativeRadius));
        assert_eq!(b.nb_contours(), 0);
    }

    #[test]
    fn build_sets_done_when_entries_present() {
        let mut b = ChFi3dBuilder::new(1e-6);
        b.add_fillet(10, 1.5).unwrap();
        b.build();
        assert!(b.is_done());
    }

    #[test]
    fn build_not_done_when_empty() {
        let mut b = ChFi3dBuilder::new(1e-6);
        b.build();
        assert!(!b.is_done());
    }

    #[test]
    fn fillet_radius_lookup() {
        let mut b = ChFi3dBuilder::new(1e-6);
        b.add_fillet(3, 4.5).unwrap();
        assert_eq!(b.fillet_radius(3), Some(4.5));
        assert_eq!(b.fillet_radius(99), None);
    }

    #[test]
    fn remove_fillet_contour() {
        let mut b = ChFi3dBuilder::new(1e-6);
        b.add_fillet(1, 1.0).unwrap();
        b.add_fillet(2, 2.0).unwrap();
        b.remove_contour(1);
        assert_eq!(b.nb_contours(), 1);
        assert_eq!(b.fillet_radius(1), None);
        assert_eq!(b.fillet_radius(2), Some(2.0));
    }

    #[test]
    fn remove_chamfer_contour() {
        let mut b = ChFi3dBuilder::new(1e-6);
        b.add_chamfer(7, 1.0, 2.0).unwrap();
        b.remove_contour(7);
        assert_eq!(b.nb_contours(), 0);
    }

    #[test]
    fn remove_nonexistent_is_noop() {
        let mut b = ChFi3dBuilder::new(1e-6);
        b.add_fillet(1, 1.0).unwrap();
        b.remove_contour(99);
        assert_eq!(b.nb_contours(), 1);
    }
}
