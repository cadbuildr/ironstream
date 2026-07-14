// FILE: topo_ds_comp.rs
// occt: BRepBuilderAPI_MakeVertex
// occt-ref: TopoDS_Compound, TopoDS_CompSolid, BRep_Builder

/// Topology type tags for shapes.
// occt-ref: TopAbs_ShapeEnum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum TopAbsShapeEnum {
    Compound,
    CompSolid,
    Solid,
    Shell,
    Face,
    Wire,
    Edge,
    Vertex,
    #[default]
    Shape,
}

/// Orientation of a shape in a composite.
// occt-ref: TopAbs_Orientation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TopAbsOrientation {
    #[default]
    Forward,
    Reversed,
    Internal,
    External,
}

impl TopAbsOrientation {
    pub fn reversed(&self) -> Self {
        match self {
            Self::Forward  => Self::Reversed,
            Self::Reversed => Self::Forward,
            other          => *other,
        }
    }

    pub fn is_closed(&self) -> bool {
        matches!(self, Self::Internal | Self::External)
    }
}

/// A single shape entry in a compound.
#[derive(Clone, Debug)]
pub struct TopoDsShapeEntry {
    pub shape_id: u32,
    pub shape_type: TopAbsShapeEnum,
    pub orientation: TopAbsOrientation,
}

impl TopoDsShapeEntry {
    pub fn new(shape_id: u32, shape_type: TopAbsShapeEnum) -> Self {
        Self { shape_id, shape_type, orientation: TopAbsOrientation::Forward }
    }

    pub fn with_orientation(mut self, o: TopAbsOrientation) -> Self { self.orientation = o; self }
}

/// A compound shape containing a list of sub-shapes.
// occt-ref: TopoDS_Compound
#[derive(Clone, Debug, Default)]
pub struct TopoDsCompound {
    pub children: Vec<TopoDsShapeEntry>,
    pub is_free: bool,
}

impl TopoDsCompound {
    pub fn new() -> Self { Self { children: Vec::new(), is_free: true } }

    pub fn add(&mut self, entry: TopoDsShapeEntry) { self.children.push(entry); }

    pub fn remove(&mut self, shape_id: u32) {
        self.children.retain(|e| e.shape_id != shape_id);
    }

    pub fn nb_children(&self) -> usize { self.children.len() }
    pub fn is_empty(&self) -> bool { self.children.is_empty() }

    pub fn children_of_type(&self, t: TopAbsShapeEnum) -> Vec<&TopoDsShapeEntry> {
        self.children.iter().filter(|e| e.shape_type == t).collect()
    }
}

/// A compound solid (connected solid regions).
// occt-ref: TopoDS_CompSolid
#[derive(Clone, Debug, Default)]
pub struct TopoDsCompSolid {
    pub solid_ids: Vec<u32>,
}

impl TopoDsCompSolid {
    pub fn new() -> Self { Self::default() }

    pub fn add_solid(&mut self, solid_id: u32) {
        if !self.solid_ids.contains(&solid_id) {
            self.solid_ids.push(solid_id);
        }
    }

    pub fn nb_solids(&self) -> usize { self.solid_ids.len() }
}

/// Builder for BRep compound shapes.
// occt-ref: BRep_Builder
#[derive(Clone, Debug, Default)]
pub struct BrepBuilder {
    pub next_id: u32,
}

impl BrepBuilder {
    pub fn new() -> Self { Self { next_id: 1 } }

    fn alloc_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn make_compound(&mut self) -> TopoDsCompound {
        let _ = self.alloc_id();
        TopoDsCompound::new()
    }

    pub fn add(&self, compound: &mut TopoDsCompound, entry: TopoDsShapeEntry) {
        compound.add(entry);
    }

    pub fn remove(&self, compound: &mut TopoDsCompound, shape_id: u32) {
        compound.remove(shape_id);
    }

    pub fn make_solid_from_shell(&mut self, shell_id: u32) -> u32 {
        let id = self.alloc_id();
        let _ = shell_id;
        id
    }

    pub fn make_shell(&mut self, nb_faces: usize) -> u32 {
        let _ = nb_faces;
        self.alloc_id()
    }
}

/// Vertex builder convenience wrapper.
/// occt: BRepBuilderAPI_MakeVertex
#[derive(Clone, Debug)]
pub struct BrepBuilderMakeVertex {
    pub point: [f64; 3],
    pub tolerance: f64,
    pub vertex_id: u32,
    pub is_done: bool,
}

impl BrepBuilderMakeVertex {
    pub fn new(point: [f64; 3]) -> Self {
        Self { point, tolerance: 1e-7, vertex_id: 0, is_done: false }
    }

    pub fn with_tolerance(mut self, tol: f64) -> Self { self.tolerance = tol.max(1e-14); self }

    pub fn build(&mut self, builder: &mut BrepBuilder) -> u32 {
        self.vertex_id = builder.alloc_id();
        self.is_done = true;
        self.vertex_id
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn vertex_id(&self) -> u32 { self.vertex_id }
}

/// Wire (closed edge loop) builder.
// occt-ref: BRepBuilderAPI_MakeWire
#[derive(Clone, Debug, Default)]
pub struct BrepBuilderMakeWire {
    pub edge_ids: Vec<u32>,
    pub is_done: bool,
    pub wire_id: u32,
}

impl BrepBuilderMakeWire {
    pub fn new() -> Self { Self::default() }

    pub fn add_edge(&mut self, edge_id: u32) { self.edge_ids.push(edge_id); }

    pub fn build(&mut self, builder: &mut BrepBuilder) -> Result<u32, String> {
        if self.edge_ids.is_empty() {
            return Err("no edges".into());
        }
        self.wire_id = builder.alloc_id();
        self.is_done = true;
        Ok(self.wire_id)
    }

    pub fn nb_edges(&self) -> usize { self.edge_ids.len() }
    pub fn is_done(&self) -> bool { self.is_done }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compound_add_remove() {
        let mut c = TopoDsCompound::new();
        c.add(TopoDsShapeEntry::new(1, TopAbsShapeEnum::Face));
        c.add(TopoDsShapeEntry::new(2, TopAbsShapeEnum::Edge));
        assert_eq!(c.nb_children(), 2);
        c.remove(1);
        assert_eq!(c.nb_children(), 1);
    }

    #[test]
    fn compound_children_of_type() {
        let mut c = TopoDsCompound::new();
        c.add(TopoDsShapeEntry::new(1, TopAbsShapeEnum::Face));
        c.add(TopoDsShapeEntry::new(2, TopAbsShapeEnum::Face));
        c.add(TopoDsShapeEntry::new(3, TopAbsShapeEnum::Edge));
        let faces = c.children_of_type(TopAbsShapeEnum::Face);
        assert_eq!(faces.len(), 2);
    }

    #[test]
    fn builder_make_compound() {
        let mut b = BrepBuilder::new();
        let mut comp = b.make_compound();
        b.add(&mut comp, TopoDsShapeEntry::new(10, TopAbsShapeEnum::Solid));
        assert_eq!(comp.nb_children(), 1);
    }

    #[test]
    fn make_vertex() {
        let mut b = BrepBuilder::new();
        let mut mv = BrepBuilderMakeVertex::new([1.0, 2.0, 3.0]);
        let id = mv.build(&mut b);
        assert!(mv.is_done());
        assert!(id > 0);
    }

    #[test]
    fn make_wire() {
        let mut b = BrepBuilder::new();
        let mut mw = BrepBuilderMakeWire::new();
        mw.add_edge(5);
        mw.add_edge(6);
        let res = mw.build(&mut b);
        assert!(res.is_ok());
        assert!(mw.is_done());
        assert_eq!(mw.nb_edges(), 2);
    }

    #[test]
    fn orientation_reversed() {
        let o = TopAbsOrientation::Forward;
        assert_eq!(o.reversed(), TopAbsOrientation::Reversed);
        assert_eq!(o.reversed().reversed(), TopAbsOrientation::Forward);
    }

    #[test]
    fn comp_solid_dedup() {
        let mut cs = TopoDsCompSolid::new();
        cs.add_solid(1);
        cs.add_solid(1);
        cs.add_solid(2);
        assert_eq!(cs.nb_solids(), 2);
    }
}
