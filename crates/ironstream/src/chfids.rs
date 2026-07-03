// FILE: chfids.rs
// occt: ChFiDS_Stripe, ChFiDS_HData, ChFiDS_Spine, ChFiDS_Map

/// Stripe: fillet/chamfer surface between two faces.
/// occt: ChFiDS_Stripe
#[derive(Clone, Debug)]
pub struct ChFiDsStripe {
    pub stripe_id: u32,
    pub spine_id: u32,
    pub left_face_id: u32,
    pub right_face_id: u32,
    pub surface_id: u32,
}

impl ChFiDsStripe {
    pub fn new(id: u32, spine: u32, left: u32, right: u32) -> Self {
        Self {
            stripe_id: id,
            spine_id: spine,
            left_face_id: left,
            right_face_id: right,
            surface_id: 0,
        }
    }

    pub fn set_surface(&mut self, surf_id: u32) { self.surface_id = surf_id; }
    pub fn has_surface(&self) -> bool { self.surface_id > 0 }
}

impl Default for ChFiDsStripe {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

/// Spine: edge sequence for fillet/chamfer construction.
/// occt: ChFiDS_Spine
#[derive(Clone, Debug)]
pub struct ChFiDsSpine {
    pub spine_id: u32,
    pub edges: Vec<u32>,
    pub parameters: Vec<f64>,
}

impl ChFiDsSpine {
    pub fn new(id: u32) -> Self {
        Self {
            spine_id: id,
            edges: Vec::new(),
            parameters: Vec::new(),
        }
    }

    pub fn add_edge(&mut self, edge_id: u32) {
        self.edges.push(edge_id);
    }

    pub fn add_parameter(&mut self, param: f64) {
        self.parameters.push(param);
    }

    pub fn nb_edges(&self) -> usize { self.edges.len() }
}

impl Default for ChFiDsSpine {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Fillet/chamfer data set.
/// occt: ChFiDS_HData
#[derive(Clone, Debug)]
pub struct ChFiDsHData {
    pub stripes: Vec<ChFiDsStripe>,
    pub spines: Vec<ChFiDsSpine>,
}

impl ChFiDsHData {
    pub fn new() -> Self {
        Self {
            stripes: Vec::new(),
            spines: Vec::new(),
        }
    }

    pub fn add_stripe(&mut self, stripe: ChFiDsStripe) {
        self.stripes.push(stripe);
    }

    pub fn add_spine(&mut self, spine: ChFiDsSpine) {
        self.spines.push(spine);
    }

    pub fn nb_stripes(&self) -> usize { self.stripes.len() }
    pub fn nb_spines(&self) -> usize { self.spines.len() }
}

impl Default for ChFiDsHData {
    fn default() -> Self {
        Self::new()
    }
}

/// Map of fillet elements.
/// occt: ChFiDS_Map
#[derive(Clone, Debug)]
pub struct ChFiDsMap {
    pub edges_to_stripes: Vec<(u32, u32)>,  // (edge_id, stripe_id)
}

impl ChFiDsMap {
    pub fn new() -> Self {
        Self {
            edges_to_stripes: Vec::new(),
        }
    }

    pub fn add_edge_stripe(&mut self, edge_id: u32, stripe_id: u32) {
        self.edges_to_stripes.push((edge_id, stripe_id));
    }

    pub fn stripes_for_edge(&self, edge_id: u32) -> Vec<u32> {
        self.edges_to_stripes
            .iter()
            .filter(|(e, _)| *e == edge_id)
            .map(|(_, s)| *s)
            .collect()
    }
}

impl Default for ChFiDsMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chfids_stripe() {
        let mut s = ChFiDsStripe::new(1, 2, 3, 4);
        assert!(!s.has_surface());
        s.set_surface(10);
        assert!(s.has_surface());
    }

    #[test]
    fn chfids_spine() {
        let mut sp = ChFiDsSpine::new(1);
        sp.add_edge(10);
        sp.add_edge(20);
        sp.add_parameter(0.0);
        sp.add_parameter(1.0);
        assert_eq!(sp.nb_edges(), 2);
    }

    #[test]
    fn chfids_hdata() {
        let mut hd = ChFiDsHData::new();
        hd.add_stripe(ChFiDsStripe::new(1, 1, 2, 3));
        hd.add_spine(ChFiDsSpine::new(1));
        assert_eq!(hd.nb_stripes(), 1);
        assert_eq!(hd.nb_spines(), 1);
    }

    #[test]
    fn chfids_map() {
        let mut m = ChFiDsMap::new();
        m.add_edge_stripe(5, 1);
        m.add_edge_stripe(5, 2);
        m.add_edge_stripe(6, 3);
        let stripes = m.stripes_for_edge(5);
        assert_eq!(stripes.len(), 2);
    }

    #[test]
    fn chfids_stripe_default() {
        let s = ChFiDsStripe::default();
        assert!(!s.has_surface());
    }
}
