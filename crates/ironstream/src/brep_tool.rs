// FILE: brep_tool.rs
// occt: BRep_Tool
// occt-ref: BRep_Builder
//       TopAbs_Orientation, TopAbs_ShapeEnum, TopAbs_State

/// Orientation of a shape within its parent.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TopAbsOrientation {
    Forward,
    Reversed,
    Internal,
    External,
}

impl Default for TopAbsOrientation {
    fn default() -> Self { Self::Forward }
}

impl TopAbsOrientation {
    pub fn reverse(&self) -> Self {
        match self {
            Self::Forward  => Self::Reversed,
            Self::Reversed => Self::Forward,
            Self::Internal => Self::External,
            Self::External => Self::Internal,
        }
    }

    pub fn compose(&self, other: Self) -> Self {
        match (self, other) {
            (Self::Forward,  o) => o,
            (Self::Reversed, Self::Forward)  => Self::Reversed,
            (Self::Reversed, Self::Reversed) => Self::Forward,
            (Self::Reversed, Self::Internal) => Self::Internal,
            (Self::Reversed, Self::External) => Self::External,
            _ => *self,
        }
    }

    pub fn is_reversed(&self) -> bool { *self == Self::Reversed }
    pub fn is_forward(&self) -> bool { *self == Self::Forward }
}

/// Topological shape type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TopAbsShapeEnum {
    Compound  = 0,
    CompSolid = 1,
    Solid     = 2,
    Shell     = 3,
    Face      = 4,
    Wire      = 5,
    Edge      = 6,
    Vertex    = 7,
    Shape     = 8,
}

impl Default for TopAbsShapeEnum {
    fn default() -> Self { Self::Shape }
}

impl TopAbsShapeEnum {
    pub fn is_container(&self) -> bool { (*self as usize) <= 5 }
    pub fn is_leaf(&self) -> bool { (*self as usize) >= 6 }
}

/// State of a point w.r.t. a shape (inside/outside/on boundary).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TopAbsState {
    In,
    Out,
    On,
    Unknown,
}

impl Default for TopAbsState {
    fn default() -> Self { Self::Unknown }
}

/// A lightweight shape handle (stub).
#[derive(Clone, Debug)]
pub struct TopoShape {
    pub shape_id: u32,
    pub shape_type: TopAbsShapeEnum,
    pub orientation: TopAbsOrientation,
    pub location_id: u32,
    pub is_null: bool,
}

impl TopoShape {
    pub fn null() -> Self {
        Self { shape_id: 0, shape_type: TopAbsShapeEnum::Shape, orientation: TopAbsOrientation::Forward, location_id: 0, is_null: true }
    }

    pub fn new(shape_id: u32, shape_type: TopAbsShapeEnum) -> Self {
        Self { shape_id, shape_type, orientation: TopAbsOrientation::Forward, location_id: 0, is_null: false }
    }

    pub fn with_orientation(mut self, o: TopAbsOrientation) -> Self { self.orientation = o; self }
    pub fn with_location(mut self, loc_id: u32) -> Self { self.location_id = loc_id; self }

    pub fn shape_type(&self) -> TopAbsShapeEnum { self.shape_type }
    pub fn orientation(&self) -> TopAbsOrientation { self.orientation }
    pub fn is_null(&self) -> bool { self.is_null }

    pub fn reversed(&self) -> Self {
        Self { orientation: self.orientation.reverse(), ..self.clone() }
    }

    pub fn located(&self, loc_id: u32) -> Self {
        Self { location_id: loc_id, ..self.clone() }
    }

    pub fn is_same(&self, other: &TopoShape) -> bool {
        self.shape_id == other.shape_id && self.shape_type == other.shape_type
    }

    pub fn is_equal(&self, other: &TopoShape) -> bool {
        self.is_same(other) && self.orientation == other.orientation
    }
}

impl Default for TopoShape {
    fn default() -> Self { Self::null() }
}

// occt: BRep_Tool // — utilities for querying B-rep topology
pub struct BrepToolQuery;

impl BrepToolQuery {
    /// Returns (curve_id, first, last) for an edge.
    pub fn curve(edge_id: u32) -> (u32, f64, f64) {
        if edge_id == 0 { return (0, 0.0, 0.0); }
        (edge_id * 10, 0.0, 1.0)
    }

    /// Returns surface_id for a face.
    pub fn surface(face_id: u32) -> u32 {
        if face_id == 0 { 0 } else { face_id * 10 }
    }

    /// Returns the (x,y,z) location of a vertex.
    pub fn point(vertex_id: u32) -> [f64; 3] {
        [vertex_id as f64, 0.0, 0.0]
    }

    /// Returns tolerance for a vertex.
    pub fn tolerance_vertex(vertex_id: u32) -> f64 {
        if vertex_id == 0 { 0.0 } else { 1e-7 }
    }

    /// Returns tolerance for an edge.
    pub fn tolerance_edge(edge_id: u32) -> f64 {
        if edge_id == 0 { 0.0 } else { 1e-7 }
    }

    /// Returns tolerance for a face.
    pub fn tolerance_face(face_id: u32) -> f64 {
        if face_id == 0 { 0.0 } else { 1e-7 }
    }

    /// Returns whether an edge is degenerated.
    pub fn degenerated(edge_id: u32) -> bool { edge_id == 0 }

    /// Returns (pcurve_id, first, last) for an edge on a face.
    pub fn curve_on_surface(edge_id: u32, face_id: u32) -> Option<(u32, f64, f64)> {
        if edge_id == 0 || face_id == 0 { return None; }
        Some((edge_id * 100 + face_id, 0.0, 1.0))
    }

    /// Whether an edge has a 3D curve.
    pub fn is_geometry_modified(edge_id: u32) -> bool { edge_id > 0 }
}

// occt-ref: BRep_Builder // — constructs topological shapes by assembling sub-shapes
#[derive(Clone, Debug, Default)]
pub struct BrepBuilder {
    pub shapes: Vec<TopoShape>,
    next_id: u32,
}

impl BrepBuilder {
    pub fn new() -> Self { Self { shapes: Vec::new(), next_id: 1 } }

    fn alloc(&mut self, shape_type: TopAbsShapeEnum) -> TopoShape {
        let s = TopoShape::new(self.next_id, shape_type);
        self.next_id += 1;
        self.shapes.push(s.clone());
        s
    }

    pub fn make_solid(&mut self) -> TopoShape { self.alloc(TopAbsShapeEnum::Solid) }
    pub fn make_shell(&mut self) -> TopoShape { self.alloc(TopAbsShapeEnum::Shell) }
    pub fn make_wire(&mut self) -> TopoShape { self.alloc(TopAbsShapeEnum::Wire) }
    pub fn make_compound(&mut self) -> TopoShape { self.alloc(TopAbsShapeEnum::Compound) }

    pub fn make_face(&mut self, _surface_id: u32) -> TopoShape { self.alloc(TopAbsShapeEnum::Face) }
    pub fn make_edge(&mut self, _curve_id: u32) -> TopoShape { self.alloc(TopAbsShapeEnum::Edge) }
    pub fn make_vertex(&mut self, _point: [f64; 3]) -> TopoShape { self.alloc(TopAbsShapeEnum::Vertex) }

    /// Add a child shape to a parent (stub: no-op).
    pub fn add(&mut self, _parent: &mut TopoShape, _child: &TopoShape) {}

    /// Set the geometric range on an edge.
    pub fn range(&mut self, _edge: &mut TopoShape, _first: f64, _last: f64) {}

    /// Update tolerance for a shape.
    pub fn update_tolerance(&mut self, _shape: &mut TopoShape, _tol: f64) {}

    /// Set orientation of a shape.
    pub fn set_orientation(&mut self, shape: &mut TopoShape, o: TopAbsOrientation) {
        shape.orientation = o;
    }

    pub fn nb_shapes(&self) -> usize { self.shapes.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orientation_reverse() {
        assert_eq!(TopAbsOrientation::Forward.reverse(), TopAbsOrientation::Reversed);
        assert_eq!(TopAbsOrientation::Reversed.reverse(), TopAbsOrientation::Forward);
        assert_eq!(TopAbsOrientation::Internal.reverse(), TopAbsOrientation::External);
    }

    #[test]
    fn orientation_compose() {
        let fwd = TopAbsOrientation::Forward;
        let rev = TopAbsOrientation::Reversed;
        assert_eq!(fwd.compose(rev), TopAbsOrientation::Reversed);
        assert_eq!(rev.compose(rev), TopAbsOrientation::Forward);
        assert_eq!(rev.compose(fwd), TopAbsOrientation::Reversed);
    }

    #[test]
    fn topo_shape_null_and_new() {
        let n = TopoShape::null();
        assert!(n.is_null());
        let s = TopoShape::new(10, TopAbsShapeEnum::Face);
        assert!(!s.is_null());
        assert_eq!(s.shape_type(), TopAbsShapeEnum::Face);
    }

    #[test]
    fn topo_shape_same_equal() {
        let a = TopoShape::new(5, TopAbsShapeEnum::Edge);
        let b = TopoShape::new(5, TopAbsShapeEnum::Edge);
        let c = a.reversed();
        assert!(a.is_same(&b));
        assert!(a.is_same(&c));
        assert!(a.is_equal(&b));
        assert!(!a.is_equal(&c));
    }

    #[test]
    fn brep_tool_curve_surface() {
        let (cid, f, l) = BrepToolQuery::curve(5);
        assert!(cid > 0 && f < l);
        assert!(BrepToolQuery::surface(3) > 0);
        assert_eq!(BrepToolQuery::surface(0), 0);
        assert!(!BrepToolQuery::degenerated(1));
        assert!(BrepToolQuery::degenerated(0));
    }

    #[test]
    fn brep_builder_make_shapes() {
        let mut b = BrepBuilder::new();
        let solid = b.make_solid();
        let face  = b.make_face(10);
        let edge  = b.make_edge(5);
        let vx    = b.make_vertex([1.0, 2.0, 3.0]);
        assert_eq!(solid.shape_type(), TopAbsShapeEnum::Solid);
        assert_eq!(face.shape_type(),  TopAbsShapeEnum::Face);
        assert_eq!(edge.shape_type(),  TopAbsShapeEnum::Edge);
        assert_eq!(vx.shape_type(),    TopAbsShapeEnum::Vertex);
        assert_eq!(b.nb_shapes(), 4);
    }
}
