// FILE: brep_fillet.rs

// occt: BRepFilletAPI_MakeFillet / BRepFilletAPI_MakeChamfer

/// Error codes for fillet and chamfer operations.
// occt: BRepFilletAPI error codes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FilletError {
    NotDone,
    BadInput,
    FilletFailed,
    NegativeRadius,
    ZeroDistance,
}

/// Records a single edge and its fillet or chamfer parameter.
pub struct FilletEdgeEntry {
    pub edge_id: u32,
    pub radius: f64,
}

/// 3D fillet builder for BRep solids.
// occt: BRepFilletAPI_MakeFillet
pub struct BRepFilletApiMakeFillet {
    edges: Vec<FilletEdgeEntry>,
    is_done: bool,
    nb_faulted: u32,
}

impl BRepFilletApiMakeFillet {
    pub fn new() -> Self {
        Self {
            edges: Vec::new(),
            is_done: false,
            nb_faulted: 0,
        }
    }

    pub fn add(&mut self, edge_id: u32, radius: f64) -> Result<(), FilletError> {
        if radius < 0.0 {
            return Err(FilletError::NegativeRadius);
        }
        self.edges.push(FilletEdgeEntry { edge_id, radius });
        Ok(())
    }

    pub fn build(&mut self) {
        self.is_done = !self.edges.is_empty();
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn nb_edges(&self) -> usize {
        self.edges.len()
    }

    pub fn nb_faulted_contours(&self) -> u32 {
        self.nb_faulted
    }

    pub fn radius(&self, edge_id: u32) -> Option<f64> {
        self.edges
            .iter()
            .find(|e| e.edge_id == edge_id)
            .map(|e| e.radius)
    }
}

impl Default for BRepFilletApiMakeFillet {
    fn default() -> Self {
        Self::new()
    }
}

/// 3D chamfer builder for BRep solids.
// occt: BRepFilletAPI_MakeChamfer
pub struct BRepFilletApiMakeChamfer {
    edges: Vec<FilletEdgeEntry>,
    is_done: bool,
}

impl BRepFilletApiMakeChamfer {
    pub fn new() -> Self {
        Self {
            edges: Vec::new(),
            is_done: false,
        }
    }

    pub fn add(&mut self, edge_id: u32, dist: f64) -> Result<(), FilletError> {
        if dist <= 0.0 {
            return Err(FilletError::ZeroDistance);
        }
        self.edges.push(FilletEdgeEntry { edge_id, radius: dist });
        Ok(())
    }

    pub fn build(&mut self) {
        self.is_done = !self.edges.is_empty();
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn nb_edges(&self) -> usize {
        self.edges.len()
    }
}

impl Default for BRepFilletApiMakeChamfer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fillet_new_is_not_done() {
        let f = BRepFilletApiMakeFillet::new();
        assert!(!f.is_done());
        assert_eq!(f.nb_edges(), 0);
    }

    #[test]
    fn fillet_add_and_build() {
        let mut f = BRepFilletApiMakeFillet::new();
        f.add(1, 0.5).unwrap();
        f.add(2, 1.0).unwrap();
        f.build();
        assert!(f.is_done());
        assert_eq!(f.nb_edges(), 2);
    }

    #[test]
    fn fillet_negative_radius_rejected() {
        let mut f = BRepFilletApiMakeFillet::new();
        assert_eq!(f.add(1, -1.0), Err(FilletError::NegativeRadius));
        assert_eq!(f.nb_edges(), 0);
    }

    #[test]
    fn fillet_zero_radius_accepted() {
        let mut f = BRepFilletApiMakeFillet::new();
        assert!(f.add(1, 0.0).is_ok());
    }

    #[test]
    fn fillet_build_empty_not_done() {
        let mut f = BRepFilletApiMakeFillet::new();
        f.build();
        assert!(!f.is_done());
    }

    #[test]
    fn fillet_radius_lookup() {
        let mut f = BRepFilletApiMakeFillet::new();
        f.add(10, 2.5).unwrap();
        f.add(20, 5.0).unwrap();
        assert_eq!(f.radius(10), Some(2.5));
        assert_eq!(f.radius(20), Some(5.0));
        assert_eq!(f.radius(99), None);
    }

    #[test]
    fn fillet_nb_faulted_starts_zero() {
        let f = BRepFilletApiMakeFillet::new();
        assert_eq!(f.nb_faulted_contours(), 0);
    }

    #[test]
    fn chamfer_new_is_not_done() {
        let c = BRepFilletApiMakeChamfer::new();
        assert!(!c.is_done());
        assert_eq!(c.nb_edges(), 0);
    }

    #[test]
    fn chamfer_add_and_build() {
        let mut c = BRepFilletApiMakeChamfer::new();
        c.add(1, 0.5).unwrap();
        c.build();
        assert!(c.is_done());
        assert_eq!(c.nb_edges(), 1);
    }

    #[test]
    fn chamfer_zero_dist_rejected() {
        let mut c = BRepFilletApiMakeChamfer::new();
        assert_eq!(c.add(1, 0.0), Err(FilletError::ZeroDistance));
    }

    #[test]
    fn chamfer_negative_dist_rejected() {
        let mut c = BRepFilletApiMakeChamfer::new();
        assert_eq!(c.add(1, -3.0), Err(FilletError::ZeroDistance));
    }

    #[test]
    fn chamfer_build_empty_not_done() {
        let mut c = BRepFilletApiMakeChamfer::new();
        c.build();
        assert!(!c.is_done());
    }
}
