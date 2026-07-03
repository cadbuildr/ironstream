extern crate ironstream;
use ironstream::brep_compound_builder::*;

#[test]
fn test_make_compound_add_and_count() {
    let mut c = MakeCompound::new();
    c.add(CompoundShape::new(0, CompoundShapeType::Solid));
    c.add(CompoundShape::new(1, CompoundShapeType::Face));
    c.add(CompoundShape::new(2, CompoundShapeType::Face));
    assert_eq!(c.nb_shapes(), 3);
    assert!(c.is_done());
    assert_eq!(c.shape_type_count(CompoundShapeType::Face), 2);
    assert_eq!(c.shape_type_count(CompoundShapeType::Solid), 1);
}

#[test]
fn test_make_compound_remove() {
    let mut c = MakeCompound::new();
    c.add(CompoundShape::new(10, CompoundShapeType::Wire));
    c.add(CompoundShape::new(20, CompoundShapeType::Wire));
    c.remove(10);
    assert_eq!(c.nb_shapes(), 1);
    assert_eq!(c.shapes[0].id, 20);
}

#[test]
fn test_compound_global_bbox() {
    let mut c = MakeCompound::new();
    c.add(CompoundShape::new(0, CompoundShapeType::Solid).with_bbox([0.0, 0.0, 0.0], [1.0, 2.0, 3.0]));
    c.add(CompoundShape::new(1, CompoundShapeType::Solid).with_bbox([-1.0, 0.0, 0.0], [2.0, 3.0, 4.0]));
    let bbox = c.global_bbox().unwrap();
    assert!((bbox[0][0] - (-1.0)).abs() < 1e-10);
    assert!((bbox[1][2] - 4.0).abs() < 1e-10);
}

#[test]
fn test_compound_ops_flatten() {
    let shapes = vec![
        CompoundShape::new(0, CompoundShapeType::Compound),
        CompoundShape::new(1, CompoundShapeType::Solid),
        CompoundShape::new(2, CompoundShapeType::Edge),
    ];
    let flat = CompoundOps::flatten(&shapes);
    assert_eq!(flat.len(), 2);
    assert!(flat.iter().all(|s| s.shape_type != CompoundShapeType::Compound));
}

#[test]
fn test_compound_ops_homogeneous_and_count_by_type() {
    let shapes = vec![
        CompoundShape::new(0, CompoundShapeType::Face),
        CompoundShape::new(1, CompoundShapeType::Face),
    ];
    assert!(CompoundOps::is_homogeneous(&shapes));
    let counts = CompoundOps::count_by_type(&shapes);
    let face_count = counts.iter().find(|(t, _)| *t == CompoundShapeType::Face).map(|(_, c)| *c).unwrap_or(0);
    assert_eq!(face_count, 2);
}
