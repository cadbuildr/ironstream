// FILE: brep_face.rs
// occt-ref: TopoDsFace, BRepAdaptor_Face, BRepBuilderAPI_MakeFace

/// Face orientation and type info.
// occt-ref: TopoDsFace
#[derive(Clone, Debug)]
pub struct TopoDsFace {
    pub face_id: u32,
    pub surface_id: u32,
    pub tolerance: f64,
    pub orientation: u8,  // 0=Forward 1=Reversed
}

impl TopoDsFace {
    pub fn new(id: u32, surf_id: u32) -> Self {
        Self {
            face_id: id,
            surface_id: surf_id,
            tolerance: 1e-7,
            orientation: 0,
        }
    }

    pub fn is_forward(&self) -> bool { self.orientation == 0 }
    pub fn is_reversed(&self) -> bool { self.orientation == 1 }
    pub fn set_orientation(&mut self, o: u8) { self.orientation = o.min(1); }
}

impl Default for TopoDsFace {
    fn default() -> Self {
        Self {
            face_id: 0,
            surface_id: 0,
            tolerance: 1e-7,
            orientation: 0,
        }
    }
}

/// Face adaptor: parameter space information.
// occt-ref: BRepAdaptor_Face
#[derive(Clone, Debug)]
pub struct BRepAdaptorFace {
    pub face_id: u32,
    pub u_min: f64,
    pub u_max: f64,
    pub v_min: f64,
    pub v_max: f64,
    pub u_period: f64,
    pub v_period: f64,
}

impl BRepAdaptorFace {
    pub fn new(face_id: u32) -> Self {
        Self {
            face_id,
            u_min: 0.0,
            u_max: 1.0,
            v_min: 0.0,
            v_max: 1.0,
            u_period: 0.0,
            v_period: 0.0,
        }
    }

    pub fn u_range(&self) -> (f64, f64) { (self.u_min, self.u_max) }
    pub fn v_range(&self) -> (f64, f64) { (self.v_min, self.v_max) }

    pub fn is_u_periodic(&self) -> bool { self.u_period > 0.0 }
    pub fn is_v_periodic(&self) -> bool { self.v_period > 0.0 }

    pub fn set_u_range(&mut self, min: f64, max: f64) {
        self.u_min = min;
        self.u_max = max;
    }

    pub fn set_v_range(&mut self, min: f64, max: f64) {
        self.v_min = min;
        self.v_max = max;
    }
}

impl Default for BRepAdaptorFace {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Wire explorer: iterate over edges of a face.
// occt-ref: BRepTools_WireExplorer
#[derive(Clone, Debug)]
pub struct BRepToolsWireExplorer {
    pub face_id: u32,
    pub edges: Vec<u32>,
    pub current_index: usize,
}

impl BRepToolsWireExplorer {
    pub fn new(face_id: u32) -> Self {
        Self {
            face_id,
            edges: Vec::new(),
            current_index: 0,
        }
    }

    pub fn add_edge(&mut self, edge_id: u32) {
        self.edges.push(edge_id);
    }

    pub fn current(&self) -> Option<u32> {
        if self.current_index < self.edges.len() {
            Some(self.edges[self.current_index])
        } else {
            None
        }
    }

    pub fn more(&self) -> bool { self.current_index < self.edges.len() }

    pub fn next(&mut self) {
        if self.current_index < self.edges.len() {
            self.current_index += 1;
        }
    }

    pub fn init(&mut self) { self.current_index = 0; }

    pub fn nb_edges(&self) -> usize { self.edges.len() }
}

impl Default for BRepToolsWireExplorer {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Make face from wire(s).
// occt-ref: BRepBuilderAPI_MakeFace
#[derive(Clone, Debug)]
pub struct BRepBuilderApiMakeFace {
    pub surface_id: u32,
    pub outer_wire_id: u32,
    pub inner_wires: Vec<u32>,
    pub face_id: Option<u32>,
}

impl BRepBuilderApiMakeFace {
    pub fn new(surf_id: u32, wire_id: u32) -> Self {
        Self {
            surface_id: surf_id,
            outer_wire_id: wire_id,
            inner_wires: Vec::new(),
            face_id: None,
        }
    }

    pub fn add_inner_wire(&mut self, wire_id: u32) {
        self.inner_wires.push(wire_id);
    }

    pub fn build(&mut self) -> bool {
        if self.face_id.is_none() {
            let id = (self.surface_id * 1000 + self.outer_wire_id * 10 + self.inner_wires.len() as u32) % 100000;
            self.face_id = Some(id);
            true
        } else {
            false
        }
    }

    pub fn face(&self) -> Option<u32> { self.face_id }
    pub fn is_done(&self) -> bool { self.face_id.is_some() }
}

impl Default for BRepBuilderApiMakeFace {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn topods_face() {
        let mut f = TopoDsFace::new(1, 10);
        assert!(f.is_forward());
        f.set_orientation(1);
        assert!(f.is_reversed());
    }

    #[test]
    fn brep_adaptor_face() {
        let mut af = BRepAdaptorFace::new(5);
        af.set_u_range(0.0, 2.0);
        af.set_v_range(0.0, 3.0);
        assert_eq!(af.u_range(), (0.0, 2.0));
        assert_eq!(af.v_range(), (0.0, 3.0));
    }

    #[test]
    fn wire_explorer() {
        let mut we = BRepToolsWireExplorer::new(1);
        we.add_edge(10);
        we.add_edge(20);
        assert_eq!(we.nb_edges(), 2);
        assert_eq!(we.current(), Some(10));
        we.next();
        assert_eq!(we.current(), Some(20));
    }

    #[test]
    fn make_face() {
        let mut mf = BRepBuilderApiMakeFace::new(5, 3);
        mf.add_inner_wire(7);
        assert!(!mf.is_done());
        mf.build();
        assert!(mf.is_done());
        assert!(mf.face().is_some());
    }

    #[test]
    fn wire_explorer_iteration() {
        let mut we = BRepToolsWireExplorer::new(2);
        for i in 1..=5 {
            we.add_edge(i * 10);
        }
        we.init();
        let mut count = 0;
        while we.more() {
            count += 1;
            we.next();
        }
        assert_eq!(count, 5);
    }
}
