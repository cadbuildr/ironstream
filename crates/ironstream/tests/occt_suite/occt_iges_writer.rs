extern crate ironstream;
use ironstream::iges_writer::*;

#[test]
fn test_iges_writer_basic() {
    // IgesMode variants
    let _ = IgesMode::BRep;
    let _ = IgesMode::FacettedBRep;
    let _ = IgesMode::WireFrame;

    // IgesWriter construction and builder pattern
    let mut w = IgesWriter::new()
        .with_mode(IgesMode::BRep)
        .with_units("MM");
    assert_eq!(w.mode, IgesMode::BRep);
    assert_eq!(w.units, "MM");
    assert_eq!(w.shape_count(), 0);

    w.add_shape("solid_a");
    w.add_shape("solid_b");
    assert_eq!(w.shape_count(), 2);

    // write to /dev/null
    let result = w.write("/dev/null");
    assert!(result.is_ok());

    // IgesEntity
    let e = IgesEntity::new(110, "line_entity");
    assert_eq!(e.entity_type, 110);
    assert_eq!(e.label, "line_entity");
    let line = e.to_iges_line();
    assert!(!line.is_empty());
}
