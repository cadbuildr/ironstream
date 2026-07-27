// FILE: topo_ds_shape.rs
// occt: TopoDS_Shape // (extended), TopoDS_Iterator, TopAbs_Orientation

// occt-ref: TopAbs_Orientation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Orientation {
    Forward,
    Reversed,
    Internal,
    External,
}

impl Orientation {
    pub fn is_reversed(&self) -> bool { *self == Self::Reversed }
    pub fn complement(&self) -> Self {
        match self {
            Self::Forward => Self::Reversed,
            Self::Reversed => Self::Forward,
            o => *o,
        }
    }
}

// occt-ref: TopAbs_ShapeEnum
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShapeEnum {
    Compound,
    CompSolid,
    Solid,
    Shell,
    Face,
    Wire,
    Edge,
    Vertex,
    Shape,
}

impl ShapeEnum {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Compound => "Compound",
            Self::CompSolid => "CompSolid",
            Self::Solid => "Solid",
            Self::Shell => "Shell",
            Self::Face => "Face",
            Self::Wire => "Wire",
            Self::Edge => "Edge",
            Self::Vertex => "Vertex",
            Self::Shape => "Shape",
        }
    }

    pub fn is_higher_than(&self, other: &ShapeEnum) -> bool {
        (*self as u8) < (*other as u8)
    }
}

// occt: TopoDS_Shape // (simplified)
#[derive(Clone, Debug)]
pub struct Shape {
    pub shape_type: ShapeEnum,
    pub orientation: Orientation,
    pub location_id: u64,
    pub children: Vec<Shape>,
    pub free: bool,
    pub modified: bool,
    pub checked: bool,
    pub orientable: bool,
    pub closed: bool,
    pub infinite: bool,
    pub convex: bool,
}

impl Shape {
    pub fn new(shape_type: ShapeEnum) -> Self {
        Self {
            shape_type,
            orientation: Orientation::Forward,
            location_id: 0,
            children: Vec::new(),
            free: true,
            modified: false,
            checked: false,
            orientable: true,
            closed: false,
            infinite: false,
            convex: false,
        }
    }

    pub fn is_null(&self) -> bool { false }
    pub fn shape_type(&self) -> ShapeEnum { self.shape_type }
    pub fn orientation(&self) -> Orientation { self.orientation }

    pub fn reversed(&self) -> Self {
        let mut s = self.clone();
        s.orientation = s.orientation.complement();
        s
    }

    pub fn oriented(&self, o: Orientation) -> Self {
        let mut s = self.clone();
        s.orientation = o;
        s
    }

    pub fn is_partner(&self, other: &Shape) -> bool {
        self.shape_type == other.shape_type && self.location_id == other.location_id
    }

    pub fn is_same(&self, other: &Shape) -> bool {
        self.is_partner(other) && self.orientation == other.orientation
    }

    pub fn add_child(&mut self, child: Shape) { self.children.push(child); }
    pub fn nb_children(&self) -> usize { self.children.len() }
}

// occt-ref: TopoDS_Iterator
pub struct ShapeIterator<'a> {
    shape: &'a Shape,
    index: usize,
    cumulative: bool,
}

impl<'a> ShapeIterator<'a> {
    pub fn new(shape: &'a Shape) -> Self {
        Self { shape, index: 0, cumulative: false }
    }

    pub fn with_orientation(shape: &'a Shape, _cum: bool) -> Self {
        Self { shape, index: 0, cumulative: _cum }
    }

    pub fn more(&self) -> bool { self.index < self.shape.children.len() }

    pub fn next(&mut self) { self.index += 1; }

    pub fn value(&self) -> Option<&Shape> { self.shape.children.get(self.index) }
}

impl<'a> Iterator for ShapeIterator<'a> {
    type Item = &'a Shape;
    fn next(&mut self) -> Option<Self::Item> {
        let v = self.shape.children.get(self.index);
        self.index += 1;
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orientation_complement() {
        assert_eq!(Orientation::Forward.complement(), Orientation::Reversed);
        assert_eq!(Orientation::Reversed.complement(), Orientation::Forward);
        assert_eq!(Orientation::Internal.complement(), Orientation::Internal);
    }

    #[test]
    fn shape_enum_order() {
        assert!(ShapeEnum::Solid.is_higher_than(&ShapeEnum::Face));
        assert!(!ShapeEnum::Edge.is_higher_than(&ShapeEnum::Face));
    }

    #[test]
    fn shape_reversed() {
        let s = Shape::new(ShapeEnum::Edge);
        let r = s.reversed();
        assert_eq!(r.orientation(), Orientation::Reversed);
    }

    #[test]
    fn shape_iterator() {
        let mut parent = Shape::new(ShapeEnum::Wire);
        parent.add_child(Shape::new(ShapeEnum::Edge));
        parent.add_child(Shape::new(ShapeEnum::Edge));
        assert_eq!(parent.nb_children(), 2);
        let count = ShapeIterator::new(&parent).count();
        assert_eq!(count, 2);
    }

    #[test]
    fn shape_is_same() {
        let a = Shape::new(ShapeEnum::Face);
        let b = a.clone();
        assert!(a.is_same(&b));
        let c = a.reversed();
        assert!(!a.is_same(&c));
        assert!(a.is_partner(&c));
    }
}
