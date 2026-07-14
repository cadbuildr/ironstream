extern crate ironstream;
use ironstream::topo_ds_shape::*;

#[test]
fn orientation_complement_and_reversed() {
    assert_eq!(Orientation::Forward.complement(), Orientation::Reversed);
    assert_eq!(Orientation::Reversed.complement(), Orientation::Forward);
    assert_eq!(Orientation::Internal.complement(), Orientation::Internal);
    assert_eq!(Orientation::External.complement(), Orientation::External);
    assert!(Orientation::Reversed.is_reversed());
    assert!(!Orientation::Forward.is_reversed());
}

#[test]
fn shape_enum_ordering() {
    assert!(ShapeEnum::Solid.is_higher_than(&ShapeEnum::Face));
    assert!(ShapeEnum::Wire.is_higher_than(&ShapeEnum::Edge));
    assert!(!ShapeEnum::Vertex.is_higher_than(&ShapeEnum::Edge));
    assert_eq!(ShapeEnum::Face.name(), "Face");
}

#[test]
fn shape_reversed_and_oriented() {
    let s = Shape::new(ShapeEnum::Edge);
    assert_eq!(s.orientation(), Orientation::Forward);
    let r = s.reversed();
    assert_eq!(r.orientation(), Orientation::Reversed);
    let o = s.oriented(Orientation::Internal);
    assert_eq!(o.orientation(), Orientation::Internal);
}

#[test]
fn shape_children_iterator() {
    let mut wire = Shape::new(ShapeEnum::Wire);
    wire.add_child(Shape::new(ShapeEnum::Edge));
    wire.add_child(Shape::new(ShapeEnum::Edge));
    wire.add_child(Shape::new(ShapeEnum::Edge));
    assert_eq!(wire.nb_children(), 3);
    let count = ShapeIterator::new(&wire).count();
    assert_eq!(count, 3);
}

#[test]
fn shape_is_same_and_partner() {
    let a = Shape::new(ShapeEnum::Face);
    let b = a.clone();
    assert!(a.is_same(&b));
    let c = a.reversed();
    assert!(a.is_partner(&c));
    assert!(!a.is_same(&c));
}
